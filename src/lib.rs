use dyn_clone::DynClone;

mod types {
    use super::*;

    pub enum CompInfoTypes {
        Workspaces,
    }
    pub enum CompInfo<'a> {
        Workspaces {
            name: Option<&'a str>,
            id: i32,
            monitor_name: &'a str,
        },
    }
    pub trait Plugin: DynClone + Sync {
        fn fetch_comp_info<'a>(dtype: CompInfoTypes) -> CompInfo<'a>
        where
            Self: Sized;
        fn init() -> Self
        where
            Self: Sized;
        fn xdg_current_desktop<'a>() -> &'a str
        where
            Self: Sized;
    }
    pub type PluginObject = Box<dyn Plugin + 'static>;
}

mod example_plugin {
    use crate::types::Plugin;

    use super::*;

    #[derive(Clone)]
    pub struct ExamplePlugin;

    impl types::Plugin for ExamplePlugin {
        fn fetch_comp_info<'a>(_dtype: types::CompInfoTypes) -> types::CompInfo<'a>
        where
            Self: Sized,
        {
            types::CompInfo::Workspaces {
                name: Some("example"),
                id: 64,
                monitor_name: "eDP-1",
            }
        }
        fn init() -> Self {
            Self
        }
        fn xdg_current_desktop<'a>() -> &'a str
        where
            Self: Sized,
        {
            "Example"
        }
    }
}

mod plugin_loader {
    use super::*;
    use std::{collections as col, path};
    use types::*;
    use xdg::BaseDirectories;

    fn get_all_plugin_paths() -> Vec<path::PathBuf> {
        let xdg_dirs =
            BaseDirectories::with_prefix("road-rs").expect("Error creating `xdg::BaseDirectories`");
        let plugins = xdg_dirs.list_data_files(path::Path::new("./plugins/"));
        plugins
    }

    pub fn load_all() -> col::BTreeMap<String, PluginObject> {
        let mut map = col::BTreeMap::new();
        let plugins_paths = get_all_plugin_paths();
        map.insert("example".to_string(), load_example_plugin());
        for i in plugins_paths {
            let name: &str = match i.file_name() {
                Some(name) => name.to_str().expect("Couldn't convert a OsStr to a &str"),
                None => panic!("Broken plugin file name"),
            };
            map.insert(name.to_string(), load_plugin_by_path(&i));
        }
        map
    }

    fn load_example_plugin() -> PluginObject {
        let plugin = example_plugin::ExamplePlugin::init();
        Box::new(plugin)
    }

    fn load_plugin_by_path<'p>(_path: &'p path::Path) -> PluginObject {
        let plugin = example_plugin::ExamplePlugin::init();
        Box::new(plugin)
    }
}

mod caller {
    use super::*;
    use std::env::{var, VarError};
    use types::*;
    fn get_best_plugin() -> PluginObject {
        let desk = match var("XDG_CURRENT_DESKTOP") {
            Ok(name) => name,
            Err(err) => match err {
                VarError::NotPresent => {
                    panic!("Not in a running session, or `XDG_CURRENT_DESKTOP` is not set")
                }
                VarError::NotUnicode(_) => panic!("Only unicode env vars are allowed!"),
            },
        };
        let plugins = plugin_loader::load_all();
        let mut plugin: Option<PluginObject> = None;
        for (_, v) in plugins.iter() {
            if (*(*v)).xdg_current_desktop() == desk {
                plugin = Some(*v);
                break;
            } else {
                continue;
            }
        }

        //let plugin = example_plugin::ExamplePlugin::init();
        match plugin {
            Some(plug) => plug,
            None => panic!("No suitable plugin was found!"),
        }
    }

    pub mod data {
        pub struct Workspaces;
        impl Workspaces {
            fn get() {}
        }
    }
}
