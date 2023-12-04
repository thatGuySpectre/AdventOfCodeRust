use std::fmt::{self, Display, Formatter};

#[derive(PartialEq)]
pub enum Answer {
    String(String),
    Number(u64),
    Float(f64),
    Unimplemented,
}

impl Display for Answer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Answer::String(s) => write!(f, "{s}"),
            Answer::Number(n) => write!(f, "{n}"),
            Answer::Float(d) => write!(f, "{d}"),
            Answer::Unimplemented => write!(f, "Unimplemented"),
        }
    }
}

impl From<String> for Answer {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Answer {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<f64> for Answer {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<f32> for Answer {
    fn from(value: f32) -> Self {
        Self::Float(value as f64)
    }
}

macro_rules! impl_from_numeric {
    ($($type:ty),*) => {
        $(
            impl From<$type> for Answer {
                fn from(item: $type) -> Self {
                    Answer::Number(item as u64)
                }
            }
        )*
    };
}

impl_from_numeric!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);

