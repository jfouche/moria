mod progressbar;
use bevy::app::{PluginGroup, PluginGroupBuilder};

pub use progressbar::*;

pub struct UiPlugins;

impl PluginGroup for UiPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(progressbar::plugin)
    }
}
