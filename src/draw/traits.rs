// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE-APACHE file or at:
//     https://www.apache.org/licenses/LICENSE-2.0

//! Drawing API

use std::any::Any;

use super::{Colour, Quad, Vec2};
use kas::geom::Rect;

/// Style of drawing
pub enum Style {
    /// Flat shading
    Flat,
    /// Square corners, shading according to the given normals
    ///
    /// Normal has two components, `(outer, inner)`, interpreted as the
    /// horizontal component of the direction vector outwards from the drawn
    /// feature. Both values are constrained to the closed range `[-1, 1]`.
    Square(Vec2),
    /// Round corners, shading according to the given normals
    ///
    /// Normal has two components, `(outer, inner)`, interpreted as the
    /// horizontal component of the direction vector outwards from the drawn
    /// feature. Both values are constrained to the closed range `[-1, 1]`.
    Round(Vec2),
}

/// Abstraction over drawing commands
///
/// Implementations may support drawing each feature with multiple styles, but
/// do not guarantee an exact match in each case.
///
/// Certain bounds on input are expected in each case. In case these are not met
/// the implementation may tweak parameters to ensure valid drawing. In the case
/// that the outer region does not have positive size or has reversed
/// coordinates, drawing may not occur at all.
pub trait Draw {
    /// Type returned by [`Draw::add_clip_region`].
    ///
    /// Supports [`Default`], which may be used to target the root region.
    type Region: Copy + Clone + Default;

    /// Cast self to [`std::any::Any`] reference.
    ///
    /// A downcast on this value may be used to obtain a reference to a
    /// toolkit-specific API.
    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// Add a clip region
    ///
    /// Clip regions are cleared each frame and so must be recreated on demand.
    fn add_clip_region(&mut self, region: Rect) -> Self::Region;

    /// Add a rectangle with flat shading to the draw buffer.
    fn rect(&mut self, region: Self::Region, rect: Rect, col: Colour);

    /// Add a frame with flat shading to the draw buffer.
    ///
    /// It is expected that the `outer` rect contains the `inner` rect.
    /// Failure may result in graphical glitches.
    fn frame(&mut self, region: Self::Region, outer: Rect, inner: Rect, col: Colour);

    /// Add a rectangle to the draw buffer.
    ///
    /// Expected componentwise bounds on input: `q.0 < q.1`.
    fn draw_quad(&mut self, region: Self::Region, quad: Quad, style: Style, col: Colour);

    /// Add a frame to the draw buffer.
    ///
    /// Expected componentwise bounds on input:
    /// `outer.0 < inner.0 < inner.1 < outer.1` and `-1 ≤ norm ≤ 1`.
    fn draw_frame(
        &mut self,
        region: Self::Region,
        outer: Quad,
        inner: Quad,
        style: Style,
        col: Colour,
    );
}
