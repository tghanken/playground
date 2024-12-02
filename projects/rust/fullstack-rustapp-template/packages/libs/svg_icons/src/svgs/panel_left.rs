use crate::{IconRef, Icons};
use crate::svg_data::geometry::{Coordinate, GeometryData, ViewBox};
use crate::svg_data::SvgData;

impl Icons {
    pub const PANEL_LEFT: IconRef = IconRef {
        name: "Panel Left Icon",
        svg_data: SvgData {
            geometry_data: Some(GeometryData {
                view_box: Some(ViewBox(0, 0, 24, 24)),
                x: Some(Coordinate(24)),
                y: None,
                height: None,
                width: None,
            }),
            paint_data: None,
            preserve_aspect_ratio: None,
            path: "<rect width=\"18\" height=\"18\" x=\"3\" y=\"3\" rx=\"2\"/>
                    <path d=\"M9 3v18\"/>",
        },
    };
}
