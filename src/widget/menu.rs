// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE-APACHE file or at:
//     https://www.apache.org/licenses/LICENSE-2.0

//! Menus

use super::{Column, List};
use kas::class::HasText;
use kas::draw::{DrawHandle, SizeHandle, TextClass};
use kas::event::{Event, GrabMode, Handler, Manager, Response, SendEvent};
use kas::layout::{AxisInfo, SizeRules};
use kas::prelude::*;
use kas::WindowId;

/// A pop-up menu
///
/// This widget opens another widget as a pop-up when clicked. It also supports
/// drag interactions which send [`Event::Activate`] to the pop-up widget under
/// the mouse on click-release.
///
/// Messages from the pop-up widget are propegated to this widget's parent when
/// emitted, and the menu is closed when this happens. Because of this it is
/// important that interactive widgets do emit a message when activated.
#[widget(config(key_nav = true))]
#[handler(noauto)]
#[derive(Clone, Debug, Widget)]
pub struct MenuButton<W: Widget> {
    #[widget_core]
    core: CoreData,
    label: CowString,
    #[widget]
    popup: W,
    opening: bool,
    popup_id: Option<WindowId>,
}

impl<W: Widget> MenuButton<W> {
    /// Construct a pop-up menu
    #[inline]
    pub fn new<S: Into<CowString>>(label: S, popup: W) -> Self {
        MenuButton {
            core: Default::default(),
            label: label.into(),
            popup,
            opening: false,
            popup_id: None,
        }
    }

    fn open_menu(&mut self, mgr: &mut Manager) {
        if self.popup_id.is_none() {
            let id = mgr.add_popup(kas::Popup {
                id: self.popup.id(),
                parent: self.id(),
                direction: Direction::Down,
            });
            self.popup_id = Some(id);
        }
    }
    fn close_menu(&mut self, mgr: &mut Manager) {
        if let Some(id) = self.popup_id {
            mgr.close_window(id);
            self.popup_id = None;
        }
    }
}

impl<W: Widget> kas::Layout for MenuButton<W> {
    fn size_rules(&mut self, size_handle: &mut dyn SizeHandle, axis: AxisInfo) -> SizeRules {
        let sides = size_handle.button_surround();
        let margins = size_handle.outer_margins();
        let frame_rules = SizeRules::extract_fixed(axis.is_vertical(), sides.0 + sides.1, margins);

        let content_rules = size_handle.text_bound(&self.label, TextClass::Button, axis);
        content_rules.surrounded_by(frame_rules, true)
    }

    fn set_rect(&mut self, rect: Rect, _align: kas::AlignHints) {
        self.core.rect = rect;
    }

    fn spatial_range(&self) -> (usize, usize) {
        // We have no child within our rect; return an empty range
        (0, std::usize::MAX)
    }

    fn draw(&self, draw_handle: &mut dyn DrawHandle, mgr: &event::ManagerState, disabled: bool) {
        let mut state = self.input_state(mgr, disabled);
        if self.popup_id.is_some() {
            state.depress = true;
        }
        draw_handle.button(self.core.rect, state);
        let align = (Align::Centre, Align::Centre);
        draw_handle.text(self.core.rect, &self.label, TextClass::Button, align);
    }
}

impl<M, W: Widget<Msg = M>> event::Handler for MenuButton<W> {
    type Msg = M;

    fn handle(&mut self, mgr: &mut Manager, event: Event) -> Response<M> {
        match event {
            Event::Activate => {
                if self.popup_id.is_none() {
                    self.open_menu(mgr);
                } else {
                    self.close_menu(mgr);
                }
            }
            Event::PressStart {
                source,
                start_id,
                coord,
            } => {
                if self.is_ancestor_of(start_id) {
                    if source.is_primary() {
                        mgr.request_grab(self.id(), source, coord, GrabMode::Grab, None);
                        mgr.set_grab_depress(source, Some(start_id));
                        self.opening = self.popup_id.is_none();
                    }
                } else {
                    if let Some(id) = self.popup_id {
                        mgr.close_window(id);
                        self.popup_id = None;
                    }
                    return Response::Unhandled(Event::None);
                }
            }
            Event::PressMove { source, cur_id, .. } => {
                if cur_id == Some(self.id()) {
                    self.open_menu(mgr);
                    mgr.set_grab_depress(source, cur_id);
                }
            }
            Event::PressEnd { end_id, coord, .. } => {
                if self.rect().contains(coord) {
                    if end_id == Some(self.id()) && self.opening {
                        self.open_menu(mgr);
                    } else {
                        self.close_menu(mgr);
                    }
                } else if self.popup_id.is_some() && self.popup.rect().contains(coord) {
                    if let Some(id) = end_id {
                        let r = self.popup.send(mgr, id, Event::Activate);
                        self.close_menu(mgr);
                        return r;
                    }
                }
            }
            event => return Response::Unhandled(event),
        }
        Response::None
    }
}

impl<W: Widget> event::SendEvent for MenuButton<W> {
    fn send(&mut self, mgr: &mut Manager, id: WidgetId, event: Event) -> Response<Self::Msg> {
        if self.is_disabled() {
            return Response::Unhandled(event);
        }

        if id <= self.popup.id() {
            let r = self.popup.send(mgr, id, event);
            if r.is_msg() {
                self.close_menu(mgr);
            }
            r
        } else {
            Manager::handle_generic(self, mgr, event)
        }
    }
}

impl<W: Widget> HasText for MenuButton<W> {
    fn get_text(&self) -> &str {
        &self.label
    }

    fn set_cow_string(&mut self, text: CowString) -> TkAction {
        self.label = text;
        TkAction::Redraw
    }
}

/// A sub-menu
#[handler(noauto)]
#[derive(Clone, Debug, Widget)]
pub struct SubMenu<W: Widget> {
    #[widget_core]
    core: CoreData,
    label: CowString,
    #[widget]
    pub list: Column<W>,
    popup_id: Option<WindowId>,
}

impl<W: Widget> SubMenu<W> {
    /// Construct a sub-menu
    #[inline]
    pub fn new<S: Into<CowString>>(label: S, list: Vec<W>) -> Self {
        SubMenu {
            core: Default::default(),
            label: label.into(),
            list: Column::new(list),
            popup_id: None,
        }
    }

    fn menu_is_open(&self) -> bool {
        self.popup_id.is_some()
    }
    fn open_menu(&mut self, mgr: &mut Manager) {
        if self.popup_id.is_none() {
            let id = mgr.add_popup(kas::Popup {
                id: self.list.id(),
                parent: self.id(),
                direction: Direction::Down,
            });
            self.popup_id = Some(id);
        }
    }
    fn close_menu(&mut self, mgr: &mut Manager) {
        if let Some(id) = self.popup_id {
            mgr.close_window(id);
            self.popup_id = None;
        }
    }
}

impl<W: Widget> kas::Layout for SubMenu<W> {
    fn size_rules(&mut self, size_handle: &mut dyn SizeHandle, axis: AxisInfo) -> SizeRules {
        let sides = size_handle.button_surround();
        let margins = size_handle.outer_margins();
        let frame_rules = SizeRules::extract_fixed(axis.is_vertical(), sides.0 + sides.1, margins);

        let content_rules = size_handle.text_bound(&self.label, TextClass::Button, axis);
        content_rules.surrounded_by(frame_rules, true)
    }

    fn set_rect(&mut self, rect: Rect, _align: kas::AlignHints) {
        self.core.rect = rect;
    }

    fn spatial_range(&self) -> (usize, usize) {
        // We have no child within our rect; return an empty range
        (0, std::usize::MAX)
    }

    fn draw(&self, draw_handle: &mut dyn DrawHandle, mgr: &event::ManagerState, disabled: bool) {
        let mut state = self.input_state(mgr, disabled);
        if self.popup_id.is_some() {
            state.depress = true;
        }
        draw_handle.button(self.core.rect, state);
        let align = (Align::Centre, Align::Centre);
        draw_handle.text(self.core.rect, &self.label, TextClass::Button, align);
    }
}

impl<M, W: Widget<Msg = M>> event::Handler for SubMenu<W> {
    type Msg = M;

    fn handle(&mut self, mgr: &mut Manager, event: Event) -> Response<M> {
        match event {
            Event::Activate | Event::OpenPopup => {
                if self.popup_id.is_none() {
                    self.open_menu(mgr);
                }
            }
            event => return Response::Unhandled(event),
        }
        Response::None
    }
}

impl<W: Widget> event::SendEvent for SubMenu<W> {
    fn send(&mut self, mgr: &mut Manager, id: WidgetId, event: Event) -> Response<Self::Msg> {
        if self.is_disabled() {
            return Response::Unhandled(event);
        }

        if id <= self.list.id() {
            let r = self.list.send(mgr, id, event);
            if r.is_msg() {
                self.close_menu(mgr);
            }
            r
        } else {
            Manager::handle_generic(self, mgr, event)
        }
    }
}

impl<W: Widget> HasText for SubMenu<W> {
    fn get_text(&self) -> &str {
        &self.label
    }

    fn set_cow_string(&mut self, text: CowString) -> TkAction {
        self.label = text;
        TkAction::Redraw
    }
}

/// A menu-bar
///
/// This widget houses a sequence of menu buttons, allowing input actions across
/// menus.
#[layout(single)]
#[handler(noauto)]
#[derive(Clone, Debug, Widget)]
pub struct MenuBar<D: Directional, W: Widget> {
    #[widget_core]
    core: CoreData,
    #[widget]
    pub bar: List<D, SubMenu<W>>,
    opening: bool,
}

impl<D: Directional + Default, W: Widget> MenuBar<D, W> {
    /// Construct
    pub fn new(menus: Vec<SubMenu<W>>) -> Self {
        MenuBar::new_with_direction(D::default(), menus)
    }
}

impl<D: Directional, W: Widget> MenuBar<D, W> {
    /// Construct
    pub fn new_with_direction(direction: D, menus: Vec<SubMenu<W>>) -> Self {
        MenuBar {
            core: Default::default(),
            bar: List::new_with_direction(direction, menus),
            opening: false,
        }
    }
}

impl<D: Directional, W: Widget<Msg = M>, M> event::Handler for MenuBar<D, W> {
    type Msg = M;

    fn handle(&mut self, mgr: &mut Manager, event: Event) -> Response<Self::Msg> {
        match event {
            Event::PressStart {
                source,
                start_id,
                coord,
            } => {
                if self.is_ancestor_of(start_id) {
                    if source.is_primary()
                        && mgr.request_grab(self.id(), source, coord, GrabMode::Grab, None)
                    {
                        mgr.set_grab_depress(source, Some(start_id));
                        self.opening = false;
                        if self.rect().contains(coord) {
                            for i in 0..self.bar.len() {
                                let w = &mut self.bar[i];
                                if w.id() == start_id {
                                    if !w.menu_is_open() {
                                        self.opening = true;
                                        w.open_menu(mgr);
                                        mgr.set_press_focus(Some(self.id()));
                                    }
                                    break;
                                }
                            }
                            Response::None
                        } else {
                            self.send(mgr, start_id, Event::OpenPopup)
                        }
                    } else {
                        Response::None
                    }
                } else {
                    // we don't know which, if any, might be open
                    for i in 0..self.bar.len() {
                        self.bar[i].close_menu(mgr);
                    }
                    Response::Unhandled(Event::None)
                }
            }
            Event::PressMove { source, cur_id, .. } => {
                if cur_id.map(|id| self.is_ancestor_of(id)).unwrap_or(false) {
                    let id = cur_id.unwrap();
                    mgr.set_grab_depress(source, Some(id));
                    self.send(mgr, id, Event::OpenPopup)
                } else {
                    Response::None
                }
            }
            Event::PressEnd { coord, end_id, .. } => {
                if end_id.map(|id| self.is_ancestor_of(id)).unwrap_or(false) {
                    // end_id is a child of self
                    let id = end_id.unwrap();

                    if self.rect().contains(coord) {
                        if !self.opening {
                            for i in 0..self.bar.len() {
                                let w = &mut self.bar[i];
                                if w.id() == id {
                                    w.close_menu(mgr);
                                }
                            }
                        }
                        Response::None
                    } else {
                        self.send(mgr, id, Event::Activate)
                    }
                } else {
                    // we don't know which, if any, might be open
                    for i in 0..self.bar.len() {
                        self.bar[i].close_menu(mgr);
                    }
                    Response::None
                }
            }
            e => Response::Unhandled(e),
        }
    }
}

impl<D: Directional, W: Widget> event::SendEvent for MenuBar<D, W> {
    fn send(&mut self, mgr: &mut Manager, id: WidgetId, event: Event) -> Response<Self::Msg> {
        if self.is_disabled() {
            return Response::Unhandled(event);
        }

        if id <= self.bar.id() {
            return match self.bar.send(mgr, id, event) {
                Response::Unhandled(event) => self.handle(mgr, event),
                r => r,
            };
        }

        self.handle(mgr, event)
    }
}
