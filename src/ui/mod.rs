mod menu;
mod progressbar;

pub use menu::*;
pub use progressbar::*;

use bevy::app::{PluginGroup, PluginGroupBuilder};

pub struct UiPlugins;

impl PluginGroup for UiPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(progressbar::plugin)
    }
}
