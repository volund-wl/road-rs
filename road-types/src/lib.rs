#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]

use abi_stable::{
    sabi_trait,
    std_types::{RBox, ROption, RString, RVec},
    StableAbi,
};

#[derive(Debug, Clone, StableAbi)]
#[repr(C)]
pub enum CompInfoTypes {
    Workspaces,
    ActiveWorkspace,
    Windows,
    ActiveWindow,
}

#[derive(Debug, Clone, StableAbi)]
#[repr(C)]
pub struct Workspace {
    pub name: ROption<RString>,
    pub id: i32,
    pub monitor_name: RString,
}

#[derive(Debug, Clone, StableAbi)]
#[repr(C)]
pub struct Window {
    pub title: RString,
    pub class: RString,
}

#[derive(Debug, Clone, StableAbi)]
#[repr(C)]
pub enum CompInfo {
    Workspaces(RVec<Workspace>),
    ActiveWorkspace(Workspace),
    Windows(RVec<Window>),
    ActiveWindow(ROption<Window>),
}

#[sabi_trait]
pub trait Plugin: Clone + Sync + Send {
    fn fetch_comp_info(self, dtype: CompInfoTypes) -> CompInfo;
}
pub trait PluginInit {
    fn init<'a>() -> PluginWrapper<'a>;
}
#[derive(Clone)]
pub struct PluginWrapper<'a> {
    pub plugin: PluginType<'a>,
    pub xdg_current_desktop: &'a str,
}

pub type PluginType<'a> = Plugin_TO<'a, RBox<()>>;
