use atomic_refcell::{AtomicRef, AtomicRefMut};
use legion::Resources;
use plugin::Plugin;
use prelude::{systems::Systems, time::Time};
use srs2dge_core::{
    batch::BatchRenderer,
    main_game_loop::{
        report::Reporter,
        update::{UpdateLoop, UpdateRate},
    },
};
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

//

pub use legion;

//

pub mod plugin;
pub mod prelude;
pub mod rigidbody;
pub mod sprite;
pub mod systems;
pub mod time;
pub mod transform;

//

#[derive(Default)]
pub struct World {
    world: legion::World,

    update_loop: UpdateLoop,
    update_rate: UpdateRate,

    pub resources: Resources,

    pub updates: Systems,
    pub frames: Systems,

    frame_plugin: bool,
}

//

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_rate(mut self, rate: UpdateRate) -> Self {
        self.update_rate = rate;
        self.update_loop = UpdateLoop::new(rate);
        self
    }

    pub fn with_plugin(mut self, plugin: impl Plugin) -> Self {
        self.add_plugin(plugin);
        self
    }

    pub fn add_plugin(&mut self, plugin: impl Plugin) {
        plugin.build(self);
    }

    pub fn get_batcher(&self) -> AtomicRef<BatchRenderer> {
        self.resources.get().expect("FramePlugin is missing")
    }

    pub fn get_batcher_mut(&self) -> AtomicRefMut<BatchRenderer> {
        self.resources.get_mut().expect("FramePlugin is missing")
    }

    /// returns a bool that is true if update systems ran
    pub fn run(&mut self) -> bool {
        let old_update_rate = self.update_rate;

        // update(s)
        let (delta_seconds, updated) = self.updates.update(
            &mut self.resources,
            &mut self.update_rate,
            &mut self.update_loop,
            &mut self.world,
        );

        // frame
        if self.frame_plugin {
            self.frames.frame(
                &mut self.resources,
                &mut self.update_rate,
                &mut self.world,
                delta_seconds,
            );
        }

        // update rate modified
        if self.update_rate != old_update_rate {
            self.update_loop = UpdateLoop::new(self.update_rate);
        }

        updated
    }

    pub fn reporters(&mut self) -> impl Iterator<Item = (&'static str, &mut Reporter)> {
        [("ECS Updates", &mut self.updates.reporter)]
            .into_iter()
            .chain(
                self.frame_plugin
                    .then_some(("ECS Frames", &mut self.frames.reporter)),
            )
    }
}

impl Deref for World {
    type Target = legion::World;

    fn deref(&self) -> &Self::Target {
        &self.world
    }
}

impl DerefMut for World {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.world
    }
}

impl Debug for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("World")
            .field("world", &self.world)
            .field("update_loop", &self.update_loop)
            .field("update_rate", &self.update_rate)
            .field("resources", &"Resources")
            .field("updates", &true)
            .field("frames", &self.frame_plugin)
            .finish()
    }
}
