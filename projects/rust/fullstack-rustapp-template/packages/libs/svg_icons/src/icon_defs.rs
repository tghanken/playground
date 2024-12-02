use crate::{IconRef, Icons};

impl Icons {
    pub const ADD_DOCUMENT: IconRef = IconRef {
        svg: include_str!("svgs/add_document.svg"),
    };
    pub const FOLDER: IconRef = IconRef {
        svg: include_str!("svgs/folder.svg"),
    };
    pub const GRAPES: IconRef = IconRef {
        svg: include_str!("svgs/grapes.svg"),
    };
    pub const PANEL_LEFT: IconRef = IconRef {
        svg: include_str!("svgs/panel_left.svg"),
    };
}

