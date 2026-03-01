/* SPDX-License-Identifier: MIT
* Copyright (c) 2026 Louis Mayencourt
*/

use bevy::{
    prelude::*,
    color::palettes::tailwind::*,
};
use rand::prelude::*;

use super::holds::*;

static WALL_WIDTH: f32 = 150.0;

pub struct Path {
    pub points: Vec<Vec2>,
}

impl Path {
    pub fn new(height: f32) -> Self {
        let vertical_spacing = 50.0;
        let number_of_holds = height/vertical_spacing;

        let mut points = Vec::new();
        for i in 0..number_of_holds as i32 {
            let x = rand::rng().random_range(-WALL_WIDTH/2.0..WALL_WIDTH/2.0);
            let y = i as f32 * vertical_spacing;
            points.push(Vec2::new(x, y));
        }

        Self { points }
    }

    pub fn spawn(
        self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) {
        for point in self.points {

            let hold = Hold::new(Vec2::new(point.x, point.y));
            hold.spawn(commands, meshes, materials);
        }
    }
}