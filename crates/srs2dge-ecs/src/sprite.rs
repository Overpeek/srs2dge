use crate::{plugin::Plugin, prelude::Time, rigidbody::RigidBody2D, transform::Transform2D, World};
use legion::{component, maybe_changed, system};
use srs2dge_core::prelude::{BatchRenderer, Color, Idx, QuadMesh, TexturePosition};

//

#[derive(Debug, Clone, Copy, Default)]
pub struct Sprite {
    pub sprite: TexturePosition,
    pub color: Color,

    pub idx: Option<Idx>,

    pub lerp_transform: Transform2D,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SpritePlugin;

//

impl Plugin for SpritePlugin {
    fn build(&self, world: &mut World) {
        world.insert_internal_update_system(200, set_pos_static_system);
        world.insert_internal_frame_system(200, set_pos_body_system);
        world.insert_internal_frame_system(201, set_sprite_system);
    }
}

#[cfg_attr(target_arch = "wasm32", system(for_each))]
#[cfg_attr(not(target_arch = "wasm32"), system(par_for_each))]
#[filter(!component::<RigidBody2D>())]
fn set_pos_static(sprite: &mut Sprite, transform: &Transform2D) {
    // println!("move sprite 0");
    sprite.lerp_transform = *transform;
}

#[cfg_attr(target_arch = "wasm32", system(for_each))]
#[cfg_attr(not(target_arch = "wasm32"), system(par_for_each))]
fn set_pos_body(
    sprite: &mut Sprite,
    transform: &Transform2D,
    rigid_body: &RigidBody2D,
    #[resource] time: &Time,
) {
    // println!("move sprite 1");
    sprite.lerp_transform.translation =
        transform.translation + rigid_body.linear_velocity * time.delta_mult();
    sprite.lerp_transform.scale = transform.scale;
    sprite.lerp_transform.rotation =
        transform.rotation + rigid_body.angular_velocity * time.delta_mult();
}

#[system(for_each)]
#[filter(maybe_changed::<Sprite>())]
fn set_sprite(sprite: &mut Sprite, #[resource] batcher: &mut BatchRenderer) {
    // println!("set sprite");
    if let Some(idx) = sprite.idx {
        let mesh = batcher.get(idx);
        if mesh.pos != sprite.lerp_transform.translation
            || mesh.size != sprite.lerp_transform.scale
            || mesh.col != sprite.color
            || mesh.tex != sprite.sprite
        {
            let mesh = batcher.get_mut(idx);
            mesh.pos = sprite.lerp_transform.translation;
            mesh.size = sprite.lerp_transform.scale;
            mesh.col = sprite.color;
            mesh.tex = sprite.sprite;
        }
    } else {
        sprite.idx = Some(batcher.push_with(QuadMesh {
            pos: sprite.lerp_transform.translation,
            size: sprite.lerp_transform.scale,
            col: sprite.color,
            tex: sprite.sprite,
        }));
    }
}
