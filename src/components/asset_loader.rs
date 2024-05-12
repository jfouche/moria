use crate::bevy_gltf_collider::get_scene_colliders;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::{collections::HashMap, slice::Iter};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, States, Default)]
pub enum AssetsLoadingState {
    #[default]
    Loading,
    Loaded,
}

#[derive(Resource, Default)]
pub struct AssetsLoaderRegister {
    assets: HashMap<String, bool>,
}

impl AssetsLoaderRegister {
    pub fn register<T: Resource>(&mut self) {
        let name = std::any::type_name::<T>();
        self.assets.insert(String::from(name), false);
    }

    pub fn handle(&mut self, event: &AssetsLoadedEvent) {
        match self.assets.get_mut(&event.0) {
            Some(loaded) => *loaded = true,
            None => error!("Unknown ressource [{}]", event.0),
        }
    }

    pub fn loaded(&self) -> bool {
        self.assets.values().all(|&loaded| loaded)
    }
}

#[derive(Event)]
pub struct AssetsLoadedEvent(String);

impl AssetsLoadedEvent {
    pub fn from<T: Resource>() -> Self {
        AssetsLoadedEvent(String::from(std::any::type_name::<T>()))
    }
}

pub struct SceneWithCollidersAssets {
    loaded: bool,
    scene: Handle<Scene>,
    colliders: Vec<(Collider, Transform)>,
}

impl SceneWithCollidersAssets {
    pub fn load(scene: Handle<Scene>) -> Self {
        SceneWithCollidersAssets {
            loaded: false,
            scene,
            colliders: Vec::new(),
        }
    }

    // Return a clone of the scene handle
    pub fn scene(&self) -> Handle<Scene> {
        self.scene.clone()
    }

    pub fn just_loaded(
        &mut self,
        mut scenes: ResMut<Assets<Scene>>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) -> bool {
        if !self.loaded {
            match scenes.get_mut(&self.scene) {
                Some(scene) => {
                    self.colliders = get_scene_colliders(&mut meshes, &mut scene.world)
                        .expect("Can't get colliders");
                    self.loaded = true;
                    true
                }
                None => false,
            }
        } else {
            false
        }
    }

    pub fn colliders(&self) -> Iter<(Collider, Transform)> {
        self.colliders.iter()
    }
}
