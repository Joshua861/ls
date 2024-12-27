use std::fmt::{format, Debug, Display, Write};

pub fn indent(string: &str) -> String {
    string.lines().fold(String::new(), |mut output, l| {
        let _ = writeln!(output, "    {l}");
        output
    })
}

pub trait DotDebug {
    fn debug(&self) -> String;
}

impl<T> DotDebug for T
where
    T: Debug,
{
    fn debug(&self) -> String {
        format!("{:#?}", self)
    }
}

pub trait DotDisplay {
    fn display(&self) -> String;
}

impl<T> DotDisplay for T
where
    T: Display,
{
    fn display(&self) -> String {
        format!("{}", self)
    }
}
