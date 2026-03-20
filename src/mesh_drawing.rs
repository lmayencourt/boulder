/* SPDX-License-Identifier: MIT
* Copyright (c) 2026 Louis Mayencourt
*/

use bevy::{
    prelude::*,
    color::palettes::tailwind::*,
};
use bevy_prototype_lyon::prelude::*;


/// Return an extended vector with points flipped on the Y axis
pub fn mirror_mesh_on_y_axis(points: &[Vec2]) -> Vec<Vec2> {
    let mut output = points.to_vec();

    for point in points.iter().rev() {
        output.push(Vec2 { x: -point.x, y: point.y });
    }

    output
}

/// Return an extended vector with points flipped on the X axis
pub fn mirror_mesh_on_x_axis(points: &[Vec2]) -> Vec<Vec2> {
    let mut output = points.to_vec();

    for point in points.iter().rev() {
        output.push(Vec2 { x: point.x, y: -point.y });
    }

    output
}

/// Flip the full point list on the Y axis
pub fn flip_on_y_axis(points: &[Vec2]) -> Vec<Vec2> {
    let mut output = Vec::new();

    for point in points {
        output.push(Vec2::new(-point.x, point.y));
    }

    output
}