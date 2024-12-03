use svg_icons::{IconRef, Icons};

use crate::pages::application::dashboard::DashboardRoute;
use crate::pages::application::themes::ThemeRoute;
use crate::pages::router::HealthzRoute;
use crate::utils::TypedRoute;

pub enum MenuItemType<'a> {
    GroupMenu(GroupMenuData<'a>),
    SingleMenu(SingleMenuItem<'a>),
}

pub struct GroupMenuData<'a> {
    title: &'a str,
    icon: Option<IconRef>,
    child_items: Vec<GroupMenuChild<'a>>,
}

impl<'a> GroupMenuData<'a> {
    pub fn get_icon(&self) -> &Option<IconRef> {
        &self.icon
    }
    pub fn get_title(&self) -> &str {
        self.title
    }
    pub fn get_children(&self) -> &Vec<GroupMenuChild<'a>> {
        &self.child_items
    }
}

pub struct GroupMenuChild<'a> {
    route: TypedRoute,
    title: &'a str,
}

impl<'a> GroupMenuChild<'a> {
    pub fn get_title(&self) -> &str {
        self.title
    }
    pub fn get_route_href(&self) -> String {
        self.route.to_string()
    }
}

pub struct SingleMenuItem<'a> {
    route: TypedRoute,
    icon: Option<IconRef>,
    title: &'a str,
}

impl<'a> SingleMenuItem<'a> {
    pub fn get_icon(&self) -> &Option<IconRef> {
        &self.icon
    }
    pub fn get_title(&self) -> &str {
        self.title
    }
    pub fn get_route_href(&self) -> String {
        self.route.to_string()
    }
}

pub struct SidebarMenu<'a> {
    menu_items: Vec<MenuItemType<'a>>,
}

impl<'a> SidebarMenu<'a> {
    pub fn get_items(&'a self) -> &Vec<MenuItemType<'a>> {
        &self.menu_items
    }
}

impl Default for SidebarMenu<'_> {
    fn default() -> Self {
        SidebarMenu {
            menu_items: vec![
                MenuItemType::SingleMenu(SingleMenuItem {
                    route: TypedRoute::new(DashboardRoute),
                    title: "Dashboard",
                    icon: Some(Icons::GRAPES),
                }),
                MenuItemType::GroupMenu(GroupMenuData {
                    title: "Utilities",
                    icon: Some(Icons::FOLDER),
                    child_items: vec![
                        GroupMenuChild {
                            route: TypedRoute::new(ThemeRoute),
                            title: "Themes",
                        },
                        GroupMenuChild {
                            route: TypedRoute::new(HealthzRoute),
                            title: "Health Check",
                        },
                    ],
                }),
                MenuItemType::SingleMenu(SingleMenuItem {
                    route: TypedRoute::new(DashboardRoute),
                    icon: None,
                    title: "Iconless Dashboard",
                }),
                MenuItemType::GroupMenu(GroupMenuData {
                    title: "Iconless Utilities",
                    icon: None,
                    child_items: vec![
                        GroupMenuChild {
                            route: TypedRoute::new(ThemeRoute),
                            title: "Themes",
                        },
                        GroupMenuChild {
                            route: TypedRoute::new(HealthzRoute),
                            title: "Health Check",
                        },
                    ],
                }),
            ],
        }
    }
}
