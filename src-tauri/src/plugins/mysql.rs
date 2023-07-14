use super::PluginDescription;

pub(super) fn description() -> PluginDescription {
    PluginDescription { name: "mysql".into(), friendly_name: "MySQL".into() }
}