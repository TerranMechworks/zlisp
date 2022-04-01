use super::{Element, Variant};
use crate::writer::config::WhitespaceConfig;

#[derive(Debug, Clone)]
pub struct PrettyWriter<'a, 'b> {
    config: &'a WhitespaceConfig<'b>,
    buffer: String,
}

impl<'a, 'b: 'a> PrettyWriter<'a, 'b> {
    pub fn new(config: &'a WhitespaceConfig<'b>) -> Self {
        Self {
            config,
            buffer: String::new(),
        }
    }

    pub fn write(mut self, value: Element) -> String {
        self.write_element(value, 0);

        self.buffer.push_str(self.config.newline);
        self.buffer
    }

    fn push_str(&mut self, string: &str) {
        self.buffer.push_str(string)
    }

    fn push_char(&mut self, ch: char) {
        self.buffer.push(ch)
    }

    fn push_indent(&mut self, level: usize) {
        for _ in 0..level {
            self.buffer.push_str(self.config.indent);
        }
    }

    fn write_seq_items(&mut self, seq: Vec<Element>, is_compact: bool, level: usize) {
        if is_compact {
            let mut iter = seq.into_iter();
            if let Some(element) = iter.next() {
                self.write_element(element, level + 1);
            }
            for element in iter {
                self.push_str(self.config.delimiter);
                self.write_element(element, level + 1);
            }
        } else {
            self.push_str(self.config.newline);
            for element in seq {
                self.push_indent(level + 1);
                self.write_element(element, level + 1);
                self.push_str(self.config.newline);
            }
            self.push_indent(level);
        }
    }

    fn write_struct_items(
        &mut self,
        fields: Vec<(&'static str, Element)>,
        is_compact: bool,
        level: usize,
    ) {
        if is_compact {
            let mut iter = fields.into_iter();
            if let Some((k, v)) = iter.next() {
                self.push_str(k);
                self.push_str(self.config.delimiter);
                self.write_element(v, level + 1);
            }
            for (k, v) in iter {
                self.push_str(self.config.delimiter);
                self.push_str(k);
                self.push_str(self.config.delimiter);
                self.write_element(v, level + 1);
            }
        } else {
            self.push_str(self.config.newline);
            for (k, v) in fields {
                self.push_indent(level + 1);
                self.push_str(k);
                self.push_str(self.config.delimiter);
                self.write_element(v, level + 1);
                self.push_str(self.config.newline);
            }
            self.push_indent(level);
        }
    }

    fn write_element(&mut self, value: Element, level: usize) {
        // the outside structure is responsible for the starting indent and
        // the termination.
        match value {
            Element::Unit => self.push_str("()"),
            Element::Scalar(string) => self.push_str(&string),
            Element::Some(inner) => {
                // this does not need to know if inner is compact, since it
                // just wraps the inner value in "(...)".
                self.push_char('(');
                self.write_element(*inner, level);
                self.push_char(')');
            }
            Element::Seq(seq, is_compact) => {
                self.push_char('(');
                self.write_seq_items(seq, is_compact, level);
                self.push_char(')');
            }
            Element::Map(inner) => {
                self.push_char('(');
                self.push_str(self.config.newline);
                for (k, v) in inner {
                    self.push_indent(level + 1);
                    self.write_element(k, level + 1);
                    self.push_str(self.config.delimiter);
                    self.write_element(v, level + 1);
                    self.push_str(self.config.newline);
                }
                self.push_indent(level);
                self.push_char(')');
            }
            Element::Struct(fields, is_compact) => {
                self.push_char('(');
                self.write_struct_items(fields, is_compact, level);
                self.push_char(')');
            }
            Element::Enum(variant, inner, is_compact) => {
                self.push_str(variant);
                if matches!(inner, Variant::Unit) {
                    return;
                }
                self.push_char('(');
                match inner {
                    Variant::Unit => panic!(),
                    Variant::NewType(element) => self.write_element(*element, level),
                    Variant::Tuple(seq) => self.write_seq_items(seq, is_compact, level),
                    Variant::Struct(fields) => self.write_struct_items(fields, is_compact, level),
                }
                self.push_char(')');
            }
        }
    }
}
