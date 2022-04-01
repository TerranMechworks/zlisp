const DEFAULT_INDENT: &str = "\t";
const DEFAULT_NEWLINE: &str = "\r\n";
const DEFAULT_DELIM: &str = "\t";

/// A builder of whitespace configuration.
///
/// This cannot be constructed, use [`WhitespaceConfig::builder`].
#[derive(Debug, Clone)]
pub struct WhitespaceConfigBuilder<'a> {
    indent: &'a str,
    newline: &'a str,
    delimiter: &'a str,
}

impl<'a> WhitespaceConfigBuilder<'a> {
    /// The indent to output when writing text.
    ///
    /// The default is `\t`/tab.
    #[inline]
    pub const fn indent(mut self, indent: &'a str) -> Self {
        self.indent = indent;
        self
    }

    /// The newline to output when writing text.
    ///
    /// The default is `\r\n`/a Windows newline.
    #[inline]
    pub const fn newline(mut self, newline: &'a str) -> Self {
        self.newline = newline;
        self
    }

    /// The delimiter to output when writing text.
    ///
    /// The default is `\t`/tab.
    #[inline]
    pub const fn delimiter(mut self, delimiter: &'a str) -> Self {
        self.delimiter = delimiter;
        self
    }

    /// Construct a new whitespace configuration.
    #[inline]
    pub const fn build(self) -> WhitespaceConfig<'a> {
        WhitespaceConfig {
            indent: self.indent,
            newline: self.newline,
            delimiter: self.delimiter,
        }
    }
}

/// Whitespace configuration for text writers.
#[derive(Debug, Clone)]
pub struct WhitespaceConfig<'a> {
    /// The indent to output when writing text.
    ///
    /// Canonically, this is `\t`/tab.
    pub(crate) indent: &'a str,
    /// The newline to output when writing text.
    ///
    /// Canonically, this is `\r\n`/a Windows newline.
    pub(crate) newline: &'a str,
    /// The delimiter to output when writing text.
    ///
    /// Canonically, this is `\t`/tab.
    pub(crate) delimiter: &'a str,
}

impl<'a> WhitespaceConfig<'a> {
    /// The default, canonical whitespace configuration.
    ///
    /// This uses tabs for indent and delimiters, as well as Windows newlines.
    pub const DEFAULT: Self = {
        Self {
            indent: DEFAULT_INDENT,
            newline: DEFAULT_NEWLINE,
            delimiter: DEFAULT_DELIM,
        }
    };

    /// The default, cannonical whitespace configuration.
    ///
    /// This uses tabs for indent and delimiters, as well as Windows newlines.
    #[inline(always)]
    pub const fn default() -> &'static Self {
        &Self::DEFAULT
    }

    #[inline]
    /// Construct a builder for a whitespace configuration.
    pub const fn builder() -> WhitespaceConfigBuilder<'a> {
        WhitespaceConfigBuilder {
            indent: DEFAULT_INDENT,
            newline: DEFAULT_NEWLINE,
            delimiter: DEFAULT_DELIM,
        }
    }
    /// The indent to output when writing text.
    #[inline(always)]
    pub const fn indent(&self) -> &'a str {
        self.indent
    }

    /// The newline to output when writing text.
    #[inline(always)]
    pub const fn newline(&self) -> &'a str {
        self.newline
    }

    /// The delimiter to output when writing text.
    #[inline(always)]
    pub const fn delimiter(&self) -> &'a str {
        self.delimiter
    }
}
