use crate::components::*;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_state::<AssetsLoadingState>()
        .init_resource::<AssetsLoaderRegister>()
        .add_event::<AssetsLoadedEvent>()
        .add_systems(
            Update,
            on_assets_loaded.run_if(in_state(AssetsLoadingState::Loading)),
        );
}

pub fn on_assets_loaded(
    mut events: EventReader<AssetsLoadedEvent>,
    mut assets_register: ResMut<AssetsLoaderRegister>,
    mut load_state: ResMut<NextState<AssetsLoadingState>>,
) {
    for event in events.read() {
        assets_register.handle(event);
    }

    if assets_register.loaded() {
        info!("AssetsLoaderRegister is loaded");
        load_state.set(AssetsLoadingState::Loaded);
    }
}
