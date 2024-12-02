use crate::{IconRef, Icons};
use crate::svg_data::{geometry::ViewBox, SvgData};
use crate::svg_data::geometry::GeometryData;

impl Icons {
    pub const ADD_DOCUMENT: IconRef = IconRef {
        name: "Add Document Icon",
        svg_data: SvgData {
            geometry_data: Some(GeometryData{
                view_box: Some(ViewBox(0, 0, 24, 24)),
                x: None,
                y: None,
                height: None,
                width: None,
            }),
            paint_data: None,
            preserve_aspect_ratio: None,
            path: "<path
                stroke-linecap=\"round\"
                stroke-linejoin=\"round\"
                stroke-width=\"2\"
                d=\"M9 13h6m-3-3v6m5 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z\">
                </path>",

        }
    };
}
