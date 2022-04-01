use super::Value;
use std::fmt;

trait Scope {
    fn write_list(&self, f: &mut fmt::Formatter<'_>, entries: &[Value]) -> fmt::Result;
    fn inc(&self) -> Self;
}

struct DefaultScope;

impl Scope for DefaultScope {
    fn write_list(&self, f: &mut fmt::Formatter<'_>, v: &[Value]) -> fmt::Result {
        f.write_str("(")?;
        if !v.is_empty() {
            Display::fmt(&v[0], self, f)?;
            for item in &v[1..] {
                f.write_str(" ")?;
                Display::fmt(item, self, f)?;
            }
        }
        f.write_str(")")
    }

    fn inc(&self) -> Self {
        Self
    }
}

struct PrettyScope(usize);

impl PrettyScope {
    fn write_indent(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for _ in 0..self.0 {
            f.write_str("\t")?;
        }
        Ok(())
    }
}

impl Scope for PrettyScope {
    fn write_list(&self, f: &mut fmt::Formatter<'_>, v: &[Value]) -> fmt::Result {
        // the indent will be inherited
        if v.is_empty() {
            return f.write_str("()");
        }

        let nested_list = v.iter().any(|item| matches!(item, Value::List(_)));
        if nested_list {
            f.write_str("(\n")?;
            {
                let scope = self.inc();
                for item in v {
                    scope.write_indent(f)?;
                    Display::fmt(item, &scope, f)?;
                    f.write_str("\n")?;
                }
            }
            self.write_indent(f)?;
            f.write_str(")")
        } else {
            f.write_str("(")?;
            Display::fmt(&v[0], self, f)?;
            for item in &v[1..] {
                f.write_str("\t")?;
                Display::fmt(item, self, f)?;
            }
            f.write_str(")")
        }
    }

    fn inc(&self) -> Self {
        Self(self.0 + 1)
    }
}

trait Display<S: Scope> {
    fn fmt(&self, scope: &S, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

impl<S: Scope> Display<S> for Value {
    fn fmt(&self, scope: &S, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::List(v) => scope.write_list(f, v),
            Self::Int(v) => write!(f, "{}", v),
            Self::Float(v) => write!(f, "{:.6}", v),
            Self::String(v) => f.write_str(v),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            Display::fmt(self, &PrettyScope(0), f)
        } else {
            Display::fmt(self, &DefaultScope, f)
        }
    }
}
