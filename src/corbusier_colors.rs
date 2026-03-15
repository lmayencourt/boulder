/* SPDX-License-Identifier: MIT
* Copyright (c) 2026 Louis Mayencourt
*/

use bevy::prelude::*;
use rand::seq::IndexedRandom;

pub const COLOR_BLUE: Color = Color::srgb(0.5176470588, 0.6509803922, 0.7803921569);
pub const COLOR_LIGHT_BLUE: Color = Color::srgb(0.6862745098, 0.737254902, 0.7764705882);
pub const COLOR_GREEN: Color = Color::srgb(0.6862745098, 0.7529411765, 0.5098039216);
pub const COLOR_RED: Color = Color::srgb(0.6235294118, 0.2941176471, 0.2470588235);
pub const COLOR_BROWN: Color = Color::srgb(0.5137254902, 0.3137254902, 0.2588235294);
pub const COLOR_WHITE: Color = Color::srgb(0.9137254902, 0.8941176471, 0.8509803922);
pub const COLOR_BLACK: Color = Color::srgb(0.2274509804, 0.231372549, 0.231372549);

pub fn random() -> Color {
    let val = [
    COLOR_BLUE,
    COLOR_LIGHT_BLUE,
    COLOR_GREEN,
    COLOR_RED,
    COLOR_WHITE,
    COLOR_BLACK
    ].choose(&mut rand::rng());

    *val.unwrap()
}