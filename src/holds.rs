
use std::fmt::Debug;

use bevy::{
    prelude::*,
    color::palettes::tailwind::*,
};

#[derive(Component)]
pub struct Hold {
    pub position: Vec2,
}

impl Hold {
    pub fn new(position: Vec2) -> Self {
        Self { position }
    }

    pub fn spawn(
        self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        println!("Spawning hold at position: {:?}", self.position);

        let default_matl = materials.add(Color::from(GRAY_300));
        let hover_matl = materials.add(Color::from(CYAN_300));

        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(30.0, 30.0))),
            MeshMaterial2d(default_matl.clone()),
            Transform::from_xyz(self.position.x, self.position.y, 0.0),
            self,
        ))
            .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
            .observe(update_material_on::<Pointer<Out>>(default_matl.clone()));
    }
}

/// Returns an observer that updates the entity's material to the one specified.
fn update_material_on<E: EntityEvent>(
    new_material: Handle<ColorMaterial>,
) -> impl Fn(On<E>, Query<&mut MeshMaterial2d<ColorMaterial>>) {
    // An observer closure that captures `new_material`. We do this to avoid needing to write four
    // versions of this observer, each triggered by a different event and with a different hardcoded
    // material. Instead, the event type is a generic, and the material is passed in.
    move |event, mut query| {
        if let Ok(mut material) = query.get_mut(event.event_target()) {
            material.0 = new_material.clone();
        }
    }
}