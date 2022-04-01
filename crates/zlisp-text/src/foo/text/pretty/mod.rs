use super::config::WhitespaceConfig;
use crate::error::{Error, ErrorCode, Result};
use crate::traits::{LispWriter, Location};
use std::vec;

#[derive(Debug, Clone)]
enum Compact {
    Unit,
    Scalar(String),
    NewType(Box<Compact>),
}

#[derive(Debug, Clone)]
enum Element {
    Compact(Compact),
    List(Box<HeadVec>),
}

#[derive(Debug, Clone)]
struct HeadVec {
    head: Option<Element>,
    rest: Vec<Element>,
}

impl HeadVec {
    const fn new() -> Self {
        Self {
            head: None,
            rest: Vec::new(),
        }
    }

    fn push(&mut self, value: Element) {
        if self.head.is_none() {
            self.head = Some(value);
        } else {
            self.rest.push(value);
        }
    }

    fn into_element(self) -> Element {
        if !self.rest.is_empty() {
            Element::List(Box::new(self))
        } else {
            match self.head {
                None => Element::Compact(Compact::Unit),
                Some(Element::Compact(c)) => Element::Compact(Compact::NewType(Box::new(c))),
                Some(Element::List(_)) => Element::List(Box::new(self)),
            }
        }
    }

    fn iter_all(&self) -> std::iter::Chain<std::option::Iter<Element>, std::slice::Iter<Element>> {
        self.head.iter().chain(self.rest.iter())
    }

    fn into_iter_all(
        self,
    ) -> std::iter::Chain<std::option::IntoIter<Element>, std::vec::IntoIter<Element>> {
        self.head.into_iter().chain(self.rest.into_iter())
    }
}

#[derive(Debug, Clone)]
pub struct PrettyWriter {
    inner: String,
}

pub struct Scope<'a> {
    writer: &'a mut PrettyWriter,
    level: usize,
    stack: Vec<HeadVec>,
}

impl PrettyWriter {

    pub fn new() -> Self {
        Self {
            inner: String,
        }
    }

    pub fn finish(self) -> Result<String> {
        self.inner
    }
}

impl<'a> Scope<'a> {
    fn push(&mut self, value: Element) -> Result<()> {
        self.stack
            .last_mut()
            .ok_or_else(|| {
                let code = ErrorCode::Custom("mismatched lists".to_owned());
                Error::new(code, Location::Unknown)
            })
            .map(|v| v.push(value))
    }

    fn push_list(&mut self) -> Result<()> {
        self.stack.push(HeadVec::new());
        Ok(())
    }

    fn pop_list(&mut self) -> Result<HeadVec> {
        self.stack.pop().ok_or_else(|| {
            let code = ErrorCode::Custom("mismatched lists".to_owned());
            Error::new(code, Location::Unknown)
        })
    }
}

#[derive(Debug, Clone)]
struct PrettyOutput<'a, 'b: 'a> {
    config: &'a WhitespaceConfig<'b>,
    buffer: String,
    last_write_was_string: bool,
}

impl<'a, 'b: 'a> PrettyOutput<'a, 'b> {
    const fn new(config: &'a WhitespaceConfig<'b>) -> Self {
        Self {
            config,
            buffer: String::new(),
            last_write_was_string: false,
        }
    }

    fn write_compact(&mut self, compact: Compact) {
        match compact {
            Compact::Unit => {
                self.buffer.push_str("()");
                self.last_write_was_string = false;
            }
            Compact::Scalar(string) => {
                self.buffer.push_str(&string);
                self.last_write_was_string = true;
            }
            Compact::NewType(value) => {
                self.buffer.push('(');
                self.write_compact(*value);
                self.buffer.push(')');
                self.last_write_was_string = false;
            }
        }
    }

    fn write_element(&mut self, level: usize, value: Element) {
        match value {
            Element::Compact(compact) => self.write_compact(compact),
            Element::List(list) => {
                // for _ in 0..level {
                //     s.push_str(self.config.indent);
                // }
                self.buffer.push('(');

                let has_sub_list = list.iter_all().any(|v| matches!(v, Element::List(_)));
                if has_sub_list {
                    // have "("
                    // push newline for "(\n"
                    self.buffer.push_str(self.config.newline);
                    for value in list.into_iter_all() {
                        // push "<indent+1><value>\n" for each value
                        for _ in 0..=level {
                            self.buffer.push_str(self.config.indent);
                        }
                        self.write_element(level + 1, value);
                        self.buffer.push_str(self.config.newline);
                    }
                    // push "<indent>)\n"
                    for _ in 0..level {
                        self.buffer.push_str(self.config.indent);
                    }
                    self.buffer.push(')');
                } else {
                    // have "<indent>("
                    // push value for "<indent>(<value>"
                    if let Some(head) = list.head {
                        match head {
                            Element::List(_) => panic!(),
                            Element::Compact(compact) => self.write_compact(compact),
                        }
                    }
                    // push delimiter + value for "<indent>(<value><delim><value>..."
                    for value in list.rest {
                        match value {
                            Element::List(_) => panic!(),
                            Element::Compact(compact) => {
                                // for string + newtype/scalar situations, we don't want to output a delimiter
                                let no_delimiter = self.last_write_was_string
                                    && !matches!(compact, Compact::Scalar(_));
                                if !no_delimiter {
                                    self.buffer.push_str(self.config.delimiter);
                                }
                                self.write_compact(compact)
                            }
                        }
                    }
                    // push ")\n"
                    self.buffer.push(')');
                }
                self.last_write_was_string = false;
            }
        }
    }

    fn write(&mut self, value: Element) {
        self.write_element(0, value)
    }

    fn finish(mut self) -> String {
        self.buffer.push_str(self.config.newline);
        self.buffer
    }
}

impl<'a, 'b> LispWriter for PrettyWriter<'a, 'b> {
    type Output = String;


    fn is_human_readable(&self) -> bool {
        true
    }

    fn write_i32(&mut self, v: i32) -> Result<()> {
        self.push(Element::Compact(Compact::Scalar(format!("{}", v))))
    }

    fn write_f32(&mut self, v: f32) -> Result<()> {
        self.push(Element::Compact(Compact::Scalar(format!("{:.6}", v))))
    }

    fn write_str(&mut self, v: &str) -> Result<()> {
        self.push(Element::Compact(Compact::Scalar(v.to_owned())))
    }

    fn write_list_start(&mut self, count: Option<usize>) -> Result<()> {
        let count = count
            .ok_or_else(|| Error::new(ErrorCode::SequenceMustHaveLength, Location::Unknown))?;
        let _len: i32 = count
            .try_into()
            .map_err(|_| Error::new(ErrorCode::SequenceTooLong, Location::Unknown))?;

        // push a new list
        self.push_list()
    }

    fn write_list_end(&mut self) -> Result<()> {
        // pop the list, and add it to the previous list
        let list = self.pop_list()?;
        self.push(list.into_element())
    }

    fn finish(mut self) -> Result<Self::Output> {
        if self.stack.len() != 1 {
            let code = ErrorCode::Custom("mismatched lists".to_owned());
            return Err(Error::new(code, Location::Unknown));
        }

        let root = self.stack.pop().unwrap();

        let mut output = PrettyOutput::new(self.config);
        if let Some(head) = root.head {
            output.write(head);
        }
        for value in root.rest {
            output.write(value);
        }
        Ok(output.finish())
    }
}

// #[cfg(test)]
// mod tests;
