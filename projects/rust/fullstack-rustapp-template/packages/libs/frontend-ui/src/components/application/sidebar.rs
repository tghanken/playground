use svg_icons::{IconRef, Icons};

use crate::pages::application::dashboard::DashboardRoute;
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
}

pub struct GroupMenuChild<'a> {
    route: TypedRoute,
    title: &'a str,
}

pub struct SingleMenuItem<'a> {
    route: TypedRoute,
    title: &'a str,
    icon: Option<IconRef>,
}

impl<'a> SingleMenuItem<'a> {
    pub fn get_icon(&self) -> &Option<IconRef> {
        &self.icon
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
            menu_items: vec![MenuItemType::SingleMenu(SingleMenuItem {
                route: TypedRoute::new(DashboardRoute),
                title: "Dashboard",
                icon: Some(Icons::GRAPES),
            })],
        }
    }
}
