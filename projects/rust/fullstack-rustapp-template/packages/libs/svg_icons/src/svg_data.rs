use std::fmt::{Display, Formatter};

use aspect_ratio::PreserveAspectRatio;
use geometry::GeometryData;
use paint::PaintData;

pub mod aspect_ratio {
    use std::fmt::{Display, Formatter};

    pub struct PreserveAspectRatio {
        x_align: AlignValues,
        y_align: AlignValues,
        scale_type: ScaleType,
    }

    pub enum AlignValues {
        Min,
        Mid,
        Max,
    }

    pub enum ScaleType {
        Meet,
        Slice,
    }

    impl Default for PreserveAspectRatio {
        fn default() -> Self {
            PreserveAspectRatio {
                x_align: AlignValues::Mid,
                y_align: AlignValues::Mid,
                scale_type: ScaleType::Meet,
            }
        }
    }

    impl Display for PreserveAspectRatio {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let x_str = match &self.x_align {
                AlignValues::Min => "xMin",
                AlignValues::Mid => "xMid",
                AlignValues::Max => "xMax",
            };
            let y_str = match &self.y_align {
                AlignValues::Min => "yMin",
                AlignValues::Mid => "yMid",
                AlignValues::Max => "yMax",
            };
            let scale_str = match &self.scale_type {
                ScaleType::Meet => "meet",
                ScaleType::Slice => "slice",
            };
            write!(f, "preserveAspectRation=\"{x_str}{y_str} {scale_str}\"")
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn default_works() {
            let desired_val = "preserveAspectRatio=\"xMidYMid meet\"".to_string();
            let val = PreserveAspectRatio::default().to_string();
            assert_eq!(val, desired_val)
        }
    }
}

pub mod geometry {
    use std::fmt::{Display, Formatter};

    pub struct GeometryData {
        pub view_box: Option<ViewBox>,
        pub x: Option<Coordinate>,
        pub y: Option<Coordinate>,
        pub height: Option<Dimension>,
        pub width: Option<Dimension>,
    }

    pub struct ViewBox(pub u32, pub u32, pub u32, pub u32);
    pub struct Coordinate(pub u32);
    pub struct Dimension(pub u32);

    impl Display for GeometryData {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "")
        }
    }
}

pub mod paint {
    use std::fmt::{Display, Formatter};

    pub struct PaintData {
        pub stroke: Option<Stroke>,
        pub fill: Option<Fill>,
        pub stroke_width: Option<u8>,
        pub stroke_linecap: Option<StrokeLinecap>,
        pub stroke_linejoin: Option<StrokeLineJoin>,
    }

    pub enum Stroke {
        CurrentColor,
    }
    pub enum Fill {}
    pub enum StrokeLinecap {
        Round,
    }
    pub enum StrokeLineJoin {
        Round,
    }

    impl Display for PaintData {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "")
        }
    }
}

pub struct SvgData {
    pub geometry_data: Option<GeometryData>,
    pub paint_data: Option<PaintData>,
    pub preserve_aspect_ratio: Option<PreserveAspectRatio>,
    pub path: &'static str,
}

impl Display for SvgData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let xmlns = "xmlns=\"http://www.w3.org/2000/svg\"";
        let geometry = match &self.geometry_data {
            None => "".to_string(),
            Some(val) => val.to_string(),
        };
        let paint = match &self.paint_data {
            None => "".to_string(),
            Some(val) => val.to_string(),
        };
        let aspect_ratio = match &self.preserve_aspect_ratio {
            None => "".to_string(),
            Some(val) => val.to_string(),
        };
        let path = self.path;
        write!(
            f,
            "<svg {xmlns} {geometry} {paint} {aspect_ratio}> {path} </svg>"
        )
    }
}
