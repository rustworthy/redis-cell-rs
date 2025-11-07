use redis::ToRedisArgs;
use std::borrow::Cow;
use std::fmt::{Debug, Display, Write};

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Key<'a> {
    String(String),
    Str(&'a str),
    Usize(usize),
    Isize(isize),
    #[cfg(feature = "uuid")]
    #[cfg_attr(docsrs, doc(cfg(feature = "uuid")))]
    Uuid(uuid::Uuid),
    Pair(Cow<'a, str>, Cow<'a, str>),
    Triple(Cow<'a, str>, Cow<'a, str>, Cow<'a, str>),
}

impl<'a> Key<'a> {
    pub fn pair(value1: impl Into<Cow<'a, str>>, value2: impl Into<Cow<'a, str>>) -> Self {
        Self::Pair(value1.into(), value2.into())
    }

    pub fn triple(
        value1: impl Into<Cow<'a, str>>,
        value2: impl Into<Cow<'a, str>>,
        value3: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self::Triple(value1.into(), value2.into(), value3.into())
    }
}

impl<'a> From<&'a str> for Key<'a> {
    fn from(value: &'a str) -> Self {
        Self::Str(value)
    }
}

impl From<String> for Key<'_> {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<usize> for Key<'_> {
    fn from(value: usize) -> Self {
        Self::Usize(value)
    }
}

impl From<isize> for Key<'_> {
    fn from(value: isize) -> Self {
        Self::Isize(value)
    }
}

impl Display for Key<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(value) => Display::fmt(value, f),
            Self::Str(value) => Display::fmt(*value, f),
            Self::Usize(value) => Display::fmt(value, f),
            Self::Isize(value) => Display::fmt(value, f),
            #[cfg(feature = "uuid")]
            Self::Uuid(value) => Display::fmt(value, f),
            Self::Pair(value1, value2) => {
                f.write_char('(')?;
                Display::fmt(value1.as_ref(), f)?;
                f.write_str(", ")?;
                Display::fmt(value2.as_ref(), f)?;
                f.write_char(')')
            }
            Self::Triple(value1, value2, value3) => {
                f.write_char('(')?;
                Display::fmt(value1.as_ref(), f)?;
                f.write_str(", ")?;
                Display::fmt(value2.as_ref(), f)?;
                f.write_str(", ")?;
                Display::fmt(value3.as_ref(), f)?;
                f.write_char(')')
            }
        }
    }
}

impl ToRedisArgs for Key<'_> {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        match self {
            Self::String(value) => value.write_redis_args(out),
            Self::Str(value) => (*value).write_redis_args(out),
            Self::Usize(value) => value.write_redis_args(out),
            Self::Isize(value) => value.write_redis_args(out),
            #[cfg(feature = "uuid")]
            Self::Uuid(value) => value.write_redis_args(out),
            impl_display => out.write_arg_fmt(impl_display),
        }
    }
}
