use crate::ascii::to_raw;
use crate::error::Result;
use crate::writer::config::WhitespaceConfig;

#[derive(Debug, Clone)]
pub struct StringWriter<'a, 'b> {
    config: &'a WhitespaceConfig<'b>,
    inner: String,
    level: usize,
    last_write_was_string: bool,
}

impl<'a, 'b: 'a> StringWriter<'a, 'b> {
    pub const fn new(config: &'a WhitespaceConfig<'b>) -> Self {
        Self {
            config,
            inner: String::new(),
            level: 0,
            last_write_was_string: false,
        }
    }

    fn push_str(&mut self, s: &str) {
        self.inner.push_str(s)
    }

    fn push_char(&mut self, c: char) {
        self.inner.push(c)
    }

    fn push_indent(&mut self) {
        for _ in 0..self.level {
            self.inner.push_str(self.config.indent);
        }
    }

    fn push_newline(&mut self) {
        self.inner.push_str(self.config.newline);
    }

    fn push_delim(&mut self) {
        self.inner.push_str(self.config.delimiter);
    }

    pub fn write_i32(&mut self, v: i32) {
        self.last_write_was_string = false;
        self.push_indent();
        self.push_str(&format!("{}", v));
        self.push_newline();
    }

    pub fn write_f32(&mut self, v: f32) {
        self.last_write_was_string = false;
        self.push_indent();
        self.push_str(&format!("{:.6}", v));
        self.push_newline();
    }

    pub fn write_str(&mut self, v: &str) -> Result<()> {
        let needs_quoting = to_raw(v)?;
        self.last_write_was_string = true;
        self.push_indent();
        if needs_quoting {
            self.push_char('"');
            self.push_str(v);
            self.push_char('"');
        } else {
            self.push_str(v);
        }
        self.push_newline();
        Ok(())
    }

    pub fn write_list_start_unchecked(&mut self) {
        if self.last_write_was_string {
            self.push_delim();
            self.push_char('(');
            self.push_newline();
        } else {
            self.push_indent();
            self.push_char('(');
            self.push_newline();
        }

        self.level += 1;
        self.last_write_was_string = false;
    }

    pub fn write_list_start(&mut self, _count: i32) -> Result<()> {
        // although the count is not used, require it so that callers might
        // remember to validate it...
        self.write_list_start_unchecked();
        Ok(())
    }

    pub fn write_list_end(&mut self) {
        self.last_write_was_string = false;
        self.level -= 1;
        self.push_indent();
        self.push_char(')');
        self.push_newline();
    }

    pub fn write_unit(&mut self) {
        if self.last_write_was_string {
            self.push_delim();
        } else {
            self.push_indent();
        }
        self.push_str("()");
        self.push_newline();
        self.last_write_was_string = false;
    }

    pub fn finish(self) -> Result<String> {
        Ok(self.inner)
    }
}
