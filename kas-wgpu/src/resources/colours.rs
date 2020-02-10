// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE-APACHE file or at:
//     https://www.apache.org/licenses/LICENSE-2.0

//! Colour schemes

use kas::draw::Colour;
use kas::event::HighlightState;

/// Provides standard theme colours
#[derive(Clone, Debug)]
pub struct ThemeColours {
    pub background: Colour,
    pub frame: Colour,
    pub text_area: Colour,
    pub text: Colour,
    pub label_text: Colour,
    pub button_text: Colour,
    pub key_nav_focus: Colour,
    pub button: Colour,
    pub button_highlighted: Colour,
    pub button_depressed: Colour,
}

impl ThemeColours {
    /// Construct a default instance
    pub fn new() -> Self {
        ThemeColours {
            background: Colour::grey(0.7),
            frame: Colour::grey(0.7),
            text_area: Colour::grey(1.0),
            text: Colour::grey(0.0),
            label_text: Colour::grey(0.0),
            button_text: Colour::grey(1.0),
            key_nav_focus: Colour::new(1.0, 0.7, 0.5),
            button: Colour::new(0.2, 0.7, 1.0),
            button_highlighted: Colour::new(0.25, 0.8, 1.0),
            button_depressed: Colour::new(0.15, 0.525, 0.75),
        }
    }

    /// Get colour for navigation highlight region, if any
    pub fn nav_region(&self, highlights: HighlightState) -> Option<Colour> {
        if highlights.key_focus {
            Some(self.key_nav_focus)
        } else {
            None
        }
    }

    /// Get colour for a button, depending on state
    pub fn button_state(&self, highlights: HighlightState) -> Colour {
        if highlights.depress {
            self.button_depressed
        } else if highlights.hover {
            self.button_highlighted
        } else {
            self.button
        }
    }

    /// Get colour for a checkbox mark, depending on state
    pub fn check_mark_state(&self, highlights: HighlightState, checked: bool) -> Option<Colour> {
        if highlights.depress {
            Some(self.button_depressed)
        } else if checked && highlights.hover {
            Some(self.button_highlighted)
        } else if checked {
            Some(self.button)
        } else {
            None
        }
    }

    /// Get colour of a scrollbar, depending on state
    #[inline]
    pub fn scrollbar_state(&self, highlights: HighlightState) -> Colour {
        self.button_state(highlights)
    }
}
