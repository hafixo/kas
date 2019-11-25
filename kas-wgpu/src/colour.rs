// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE-APACHE file or at:
//     https://www.apache.org/licenses/LICENSE-2.0

//! Colour type and theming

/// Standard colour description
#[derive(Clone, Copy, Debug, Default)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Colour {
    /// Constructor
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Colour { r, g, b, a: 1.0 }
    }

    /// Construct from grey-scale
    pub const fn grey(s: f32) -> Self {
        Colour::new(s, s, s)
    }
}

impl From<Colour> for wgpu::Color {
    fn from(c: Colour) -> Self {
        wgpu::Color {
            r: c.r as f64,
            g: c.g as f64,
            b: c.b as f64,
            a: c.a as f64,
        }
    }
}

impl From<Colour> for [f32; 4] {
    fn from(c: Colour) -> Self {
        [c.r, c.g, c.b, c.a]
    }
}
