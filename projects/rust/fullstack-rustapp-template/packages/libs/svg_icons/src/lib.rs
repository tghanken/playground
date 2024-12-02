use std::fmt::{Display, Formatter};

mod icon_defs;

#[non_exhaustive]
pub struct Icons;

pub struct IconRef {
    svg: &'static str,
}

impl IconRef {
    pub fn get_svg(&self) -> &'static str {
        self.svg
    }
}

impl Display for IconRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.svg)
    }
}
