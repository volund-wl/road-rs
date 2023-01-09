pub mod reexports {
    pub use abi_stable as abi;
}

pub mod types {
    pub use road_types::*;
}

#[cfg(feature = "hyprland")]
mod hypr_road {
    pub use road_plugins::hypr_road::*;
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

    pub fn load_all<'a>() -> col::BTreeMap<String, PluginWrapper<'a>> {
        let mut map = col::BTreeMap::new();
        let plugins_paths = get_all_plugin_paths();
        map.insert("hyprland".to_string(), load_hyprland_plugin());
        for i in plugins_paths {
            let name: &str = match i.file_name() {
                Some(name) => name.to_str().expect("Couldn't convert a OsStr to a &str"),
                None => panic!("Broken plugin file name"),
            };
            map.insert(name.to_string(), load_plugin_by_path(&i));
        }
        map
    }

    fn load_hyprland_plugin<'a>() -> PluginWrapper<'a> {
        let plugin = hypr_road::HyprRoadPlugin::init();
        plugin
    }
    fn load_plugin_by_path<'a>(_path: &path::Path) -> PluginWrapper<'a> {
        let plugin = hypr_road::HyprRoadPlugin::init();
        plugin
    }
}

pub mod caller {
    use super::*;
    use std::env::{var, VarError};
    use types::*;
    fn get_best_plugin<'a>() -> PluginType<'a> {
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
        let mut plugin: Option<PluginType> = None;
        for (_, v) in plugins.iter() {
            if v.xdg_current_desktop == desk {
                plugin = Some(v.clone().plugin);
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
        use super::*;

        pub struct Workspaces;
        impl Workspaces {
            pub fn get() {
                let plugin = get_best_plugin();
                let out = plugin.fetch_comp_info(CompInfoTypes::Workspaces);
                println!("{out:#?}");
            }
        }
    }
}
