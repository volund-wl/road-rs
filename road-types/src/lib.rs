#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]

use abi_stable::{
    declare_root_module_statics, library::RootModule, package_version_strings, sabi_trait,
    sabi_types::VersionStrings, std_types::*, StableAbi,
};
use async_ffi::FfiFuture;

#[repr(C)]
#[derive(StableAbi, Debug)]
pub enum Error {
    IoError(RIoError),
    NotImpld,
}

//pub enum SafeError {
//    IoError()
//}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::NotImpld => "The plugin or compositor doesn't implement this, please refer to the plugin author for help.".to_string(),
            Self::IoError(e) => e.to_string(),
            //Self::LibraryError(e) => e.to_string()
        })
    }
}

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
    pub id: i64,
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

#[derive(StableAbi, Debug, Clone)]
#[repr(C)]
pub struct PluginInfo {
    pub name: RString,
    pub version: RString,
    pub author: RString,
}

#[sabi_trait]
pub trait Listeners: Clone + Sync + Send {
    fn active_work(self, _work: Workspace) {}
}

pub type ListenersObj = Listeners_TO<'static, RBox<()>>;

macro_rules! empty_future {
    () => {{
        async fn future() {}
        FfiFuture::new(future())
    }};
}

// macro_rules! init_fn {
//     (async;$name:ident,$param:ident,$ptype:ty) => {{
//         fn $name(self, $param: $ptype) -> FfiFuture<()> {
//             async fn _future() {}
//             FfiFuture::new(_future())
//         }
//     }};
// }

#[sabi_trait]
pub trait ListenersAsync: Clone + Sync + Send {
    fn active_work(self, _work: Workspace) -> FfiFuture<()> {
        empty_future!()
    }
    fn active_mon(self, _mon: Workspace) -> FfiFuture<()> {
        empty_future!()
    }
    fn active_win(self, _win: Window) -> FfiFuture<()> {
        empty_future!()
    }
    //init_fn! {async;active_win,_win,Window}
}

pub type ListenersAsyncObj = ListenersAsync_TO<'static, RBox<()>>;

#[sabi_trait]
pub trait Plugin: Clone + Sync + Send {
    fn fetch_comp_info(self, dtype: CompInfoTypes) -> CompInfo;
    fn fetch_comp_info_async(self, dtype: CompInfoTypes) -> FfiFuture<CompInfo>;
    fn listener(self, listeners: ListenersObj) -> RResult<(), Error>;
    fn listener_async(self, listeners: &ListenersAsyncObj) -> FfiFuture<()>;
    fn should_run(self) -> bool;
    fn info(self) -> PluginInfo;
}

pub trait PluginInit {
    fn init() -> PluginType<'static>;
}

#[repr(C)]
#[derive(Clone, StableAbi)]
#[sabi(kind(Prefix(prefix_ref = PluginRef)))]
#[sabi(missing_field(panic))]
pub struct PluginWrapper(
    /// Initializes the plugin
    pub extern "C" fn() -> RResult<PluginType<'static>, Error>,
);

impl RootModule for PluginRef {
    declare_root_module_statics! {PluginRef}

    const BASE_NAME: &'static str = "road_plugin";
    const NAME: &'static str = "road_plugin";
    const VERSION_STRINGS: VersionStrings = package_version_strings!();
}

pub type PluginType<'a> = Plugin_TO<'a, RBox<()>>;
