// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE-APACHE file or at:
//     https://www.apache.org/licenses/LICENSE-2.0

//! Text-drawing API

pub use ab_glyph::{FontArc, PxScale};

use super::{Colour, Draw, DrawShared, Pass};
use crate::geom::{Rect, Vec2};
use crate::Align;

/// Font identifier
///
/// A default font may be obtained with `FontId(0)`, which refers to the
/// first font loaded by the (first) theme.
///
/// Other than this, users should treat this type as an opaque handle.
/// An instance may be obtained by [`DrawTextShared::load_font`].
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct FontId(pub usize);

/// Text properties for use by [`DrawText::text`]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TextProperties {
    /// The font
    pub font: FontId,
    /// Font scale
    ///
    /// This is approximately the pixel-height of a line of text or double the
    /// "pt" size. Usually you want to use the same scale for both components,
    /// e.g. `PxScale::from(18.0)`.
    pub scale: PxScale,
    /// Font colour
    pub col: Colour,
    /// Text alignment in horizontal and vertical directions
    pub align: (Align, Align),
    /// True if text should automatically be line-wrapped
    pub line_wrap: bool,
}

impl Default for TextProperties {
    fn default() -> Self {
        TextProperties {
            font: Default::default(),
            scale: 18.0.into(),
            col: Default::default(),
            align: Default::default(),
            line_wrap: Default::default(),
        }
    }
}

/// Abstraction over type shared by [`DrawText`] implementations
pub trait DrawTextShared: DrawShared {
    /// Load a font
    fn load_font(&mut self, font: FontArc) -> FontId;
}

/// Abstraction over text rendering
///
/// This trait is an extension over [`Draw`] providing basic text rendering.
/// Rendering makes use of transparency and should occur last in
/// implementations which buffer draw commands.
///
/// Note: the current API is designed to meet only current requirements since
/// changes are expected to support external font shaping libraries.
pub trait DrawText: Draw {
    /// Simple text drawing
    ///
    /// This allows text to be drawn according to a high-level API, and should
    /// satisfy most uses.
    fn text(&mut self, pass: Pass, rect: Rect, text: &str, props: TextProperties);

    /// Calculate size bound on text
    ///
    /// This may be used with [`DrawText::text`] to calculate size requirements
    /// within [`kas::Layout::size_rules`].
    ///
    /// Bounds of `(f32::INFINITY, f32::INFINITY)` may be used if there are no
    /// constraints. This parameter allows forcing line-wrapping behaviour
    /// within the given bounds.
    fn text_bound(
        &mut self,
        text: &str,
        font_id: FontId,
        font_scale: f32,
        bounds: (f32, f32),
        line_wrap: bool,
    ) -> (f32, f32);

    /// Find the starting position (top-left) of the glyph at the given index
    ///
    /// May panic on invalid byte index.
    fn text_glyph_pos(
        &mut self,
        rect: Rect,
        text: &str,
        props: TextProperties,
        byte: usize,
    ) -> Vec2;

    /// Find the text index for the glyph nearest the given `pos`
    ///
    /// This includes the index immediately after the last glyph, thus
    /// `result ≤ text.len()`.
    fn text_index_nearest(
        &mut self,
        rect: Rect,
        text: &str,
        props: TextProperties,
        pos: Vec2,
    ) -> usize;
}
