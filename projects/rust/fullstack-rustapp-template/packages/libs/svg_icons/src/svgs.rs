use crate::{IconRef, Icons};

impl Icons {
    pub const ADD_DOCUMENT: IconRef = IconRef {
        name: "Add Document Icon",
        svg_data: include_str!("svgs/add_document.svg"),
    };
    pub const FOLDER: IconRef = IconRef {
        name: "Folder Icon",
        svg_data: include_str!("svgs/folder.svg"),
    };
    pub const GRAPES: IconRef = IconRef {
        name: "Grapes Icon",
        svg_data: include_str!("svgs/grapes.svg"),
    };
    pub const PANEL_LEFT: IconRef = IconRef {
        name: "Panel Left Icon",
        svg_data: include_str!("svgs/panel_left.svg"),
    };
}
