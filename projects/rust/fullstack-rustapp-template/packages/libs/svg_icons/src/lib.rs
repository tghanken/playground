use std::fmt::{Display, Formatter};

pub(crate) mod svg_data;
mod svgs;

#[non_exhaustive]
pub struct Icons;

pub struct IconRef {
    name: &'static str,
    svg_data: svg_data::SvgData,
}

impl Display for IconRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.svg_data)
    }
}
