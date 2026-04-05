/* SPDX-License-Identifier: MIT
* Copyright (c) 2026 Louis Mayencourt
*/

use bevy::{
    prelude::*,
    color::palettes::tailwind::*,
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
};

use crate::mouse::MousePosition;
use crate::player::Body;

/// Used to help identify the camera that transforms screen coordinates to world coordinates.
/// Only one camera should have this component.
#[derive(Component)]
pub struct MainCamera;


#[derive(Component, Default)]
pub struct SmoothFollower {
    current_pos: Vec3,
    start_pos: Vec3,
    target_pos: Vec3,
    lerping: Option<f32>,
}

pub fn follow_player(
    mut q_camera: Query<(&mut Transform, &mut SmoothFollower), With<MainCamera>>,
    q_body: Query<(&Transform, &Body), (Without<MainCamera>)>,
    mouse_position: Res<MousePosition>,
    mut gizmos: Gizmos,
) {
    let (body_transform, body) = q_body.single().unwrap();
    let (mut camera_transform, mut smooth_camera) = q_camera.single_mut().unwrap();

    if body.active_limb.is_none() || !body.active_limb.as_ref().is_some_and(|x| x.is_hand()) {
        smooth_camera.follow_target(&body_transform.translation, &mut camera_transform.translation, gizmos);
    }
}

impl SmoothFollower {
    fn follow_target(
        &mut self,
        object_to_follow: &Vec3,
        following_object : &mut Vec3,
        mut gizmos: Gizmos,
    ) {
        let exclusion_width = 400.0;
        let exclusion_height = 200.0;
        let bounding_box = Aabb2d::new(following_object.truncate(), Vec2::new(exclusion_width/2.0, exclusion_height/2.0));
        let target_box = BoundingCircle::new(object_to_follow.truncate(), 5.0);

        // Gizmos of the no movement zone
        // gizmos.rect_2d(self.start_pos.truncate(), Vec2::new(exclusion_width, exclusion_height), BLUE_100);
        // gizmos.rect_2d(self.current_pos.truncate(), Vec2::new(exclusion_width, exclusion_height), BLUE_400);
        // gizmos.rect_2d(self.target_pos.truncate(), Vec2::new(exclusion_width, exclusion_height), RED_400);

        // Smoothly interpolate the camera's position towards the body if too far from body
        if !bounding_box.intersects(&target_box) && self.lerping.is_none() {
            self.lerping = Some(0.0);
            self.target_pos = *object_to_follow;
            self.start_pos = *following_object
        }

        let target_pos = self.target_pos;
        let start_pos = self.start_pos;
        let mut current_pos = Vec3::default();
        if let Some(ref mut lerp_value) = self.lerping {
            let easing = EasingCurve::new(0.0, 1.0, EaseFunction::CubicInOut);
            let value = easing.sample(*lerp_value).unwrap();
            let easing_dir = (target_pos - start_pos);
            current_pos = start_pos + easing_dir * value;
            
            if *lerp_value < 0.98 {
                *lerp_value += 0.01;
                println!("lerp {}", *lerp_value);
            } else {
                self.lerping = None;
                self.current_pos = *object_to_follow;
                println!("Reached {}", object_to_follow);
            }
        } else {
            current_pos = target_pos;
        }

        self.current_pos = current_pos;
        *following_object = current_pos;
    }
}