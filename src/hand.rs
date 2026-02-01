use bevy::{
    prelude::*,
    color::palettes::tailwind::*,
};

// #[derive(Component)]
// pub struct Hand;

#[derive(Component)]
pub struct LeftHand;

#[derive(Component)]
pub struct RightHand;

// impl Hand {
    pub fn spawn(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {

        commands.spawn((
            Mesh2d(meshes.add(Circle::new(10.0))),
            MeshMaterial2d(materials.add(Color::from(BLUE_300))),
            Transform::from_xyz(0.0, 0.0, 0.0),
            LeftHand,
            Pickable::IGNORE
        ));

        commands.spawn((
            Mesh2d(meshes.add(Circle::new(10.0))),
            MeshMaterial2d(materials.add(Color::from(RED_300))),
            Transform::from_xyz(0.0, 0.0, 0.0),
            RightHand,
            Pickable::IGNORE
        ));
            // .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
            // .observe(update_material_on::<Pointer<Out>>(default_matl.clone()));
    }
// }