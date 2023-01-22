pub mod reexports {
    pub use abi_stable as abi;
}

pub mod types {
    pub use road_types::*;
}

#[cfg(feature = "hyprland")]
mod hypr_road {
    pub use road_plugins::hypr::*;
}

#[cfg(feature = "sway")]
mod hypr_road {
    pub use road_plugins::sway::*;
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

    #[cfg(not(feature = "hyprland"))]
    pub fn load_all() -> col::BTreeMap<String, PluginType<'static>> {
        let mut map = col::BTreeMap::new();
        let plugins_paths = get_all_plugin_paths();
        #[cfg(feature = "hyprland")]
        map.insert("hyprland-builtin".to_string(), load_hyprland_plugin());
        #[cfg(feature = "sway")]
        map.insert("sway-builtin".to_string(), load_sway_plugin());
        for i in plugins_paths {
            let name: &str = match i.file_name() {
                Some(name) => name.to_str().expect("Couldn't convert a OsStr to a &str"),
                None => panic!("Broken plugin file name"),
            };
            let plugin = load_plugin_by_path(i.clone());
            println!("Loaded External Plugin: {:#?}", plugin.clone().info());
            map.insert(name.to_string(), plugin);
        }
        map
    }

    #[cfg(feature = "hyprland")]
    fn load_hyprland_plugin() -> PluginType<'static> {
        use abi_stable::sabi_trait::TD_Opaque;
        let plugin = hypr_road::HyprRoadPlugin::init();
        Plugin_TO::from_value(plugin, TD_Opaque)
    }
    #[cfg(feature = "sway")]
    fn load_sway_plugin() -> PluginType<'static> {
        use abi_stable::sabi_trait::TD_Opaque;
        let plugin = sway_road::SwayRoadPlugin::init();
        Plugin_TO::from_value(plugin, TD_Opaque)
    }

    fn load_plugin_by_path(path: path::PathBuf) -> PluginType<'static> {
        //let plugin = hypr_road::HyprRoadPlugin::init();
        let header =
            abi_stable::library::lib_header_from_path(&path).expect("Error loading plugin");
        header
            .init_root_module::<PluginRef>()
            .expect("Error initializing plugin")
            .field_0()()
        .expect("Error getting plugin from PluginRef")
    }
}

pub mod caller {
    use super::*;
    use std::env::{var, VarError};
    use types::*;
    fn get_best_plugin<'a>() -> PluginType<'a> {
        let _desk = match var("XDG_CURRENT_DESKTOP") {
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
            if v.clone().should_run() {
                plugin = Some(v.clone());
                break;
            } else {
                continue;
            }
        }

        match plugin {
            Some(plug) => plug,
            None => panic!("No suitable plugin was found!"),
        }
    }

    pub mod data {
        use super::*;

        pub struct ActiveWorkspace;
        impl ActiveWorkspace {
            pub fn get() {
                let plugin = get_best_plugin();
                let out = plugin.fetch_comp_info(CompInfoTypes::ActiveWorkspace);
                println!("{out:#?}");
            }
        }
    }
}
