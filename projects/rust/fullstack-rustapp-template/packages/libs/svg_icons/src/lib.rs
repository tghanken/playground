use std::fmt::{Display, Formatter};

mod svgs;

#[non_exhaustive]
pub struct Icons;

pub struct IconRef {
    /// A user-friendly name to refer to the icon as
    name: &'static str,
    /// The raw svg, excluding any "class" attributes
    svg_data: &'static str,
}

impl Display for IconRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.svg_data)
    }
}

impl IconRef {
    pub fn get_classed_svg(&self, class: &str) -> String {
        const SVG_PREFIX: &str = "<svg";
        let (_, suffix) = self.svg_data.split_at(SVG_PREFIX.len());

        format!("{SVG_PREFIX} class=\"{class}\" {suffix}")
    }
    
    pub fn get_icon_name(&self) -> &str {
        self.name
    }
}
