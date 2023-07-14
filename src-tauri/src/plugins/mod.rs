mod mysql;

use serde::{Serialize, Deserialize};

pub trait Plugin {
    fn install(&self, prefix: String);
    fn start(&self, prefix: String);
    fn stop(&self, prefix: String);
}

#[derive(Serialize, Deserialize)]
pub struct PluginDescription {
    name: String,
    friendly_name: String,
}

#[tauri::command]
pub fn get_plugins() -> Vec<PluginDescription> {
    vec![mysql::description()]
}