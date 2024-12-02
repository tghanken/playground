use crate::{IconRef, Icons};
use crate::svg_data::geometry::{GeometryData, ViewBox};
use crate::svg_data::SvgData;

impl Icons {
    pub const FOLDER: IconRef = IconRef {
        name: "Folder Icon",
        svg_data: SvgData {
            geometry_data: Some(GeometryData {
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
                    d=\"M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z\">
                    </path>",
        },
    };
}
