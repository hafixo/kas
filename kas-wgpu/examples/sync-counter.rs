// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE-APACHE file or at:
//     https://www.apache.org/licenses/LICENSE-2.0

//! A counter synchronised between multiple windows
#![feature(proc_macro_hygiene)]

use std::cell::RefCell;

use kas::class::HasText;
use kas::event::{Event, Handler, Manager, Response, UpdateHandle, VoidMsg};
use kas::macros::{make_widget, VoidMsg};
use kas::widget::{Label, TextButton, Window};
use kas::{ThemeApi, WidgetConfig, WidgetCore};

#[derive(Clone, Debug, VoidMsg)]
enum Message {
    Decr,
    Incr,
}

thread_local! {
    // Save ourselves usage of thread-safe primitives by keeping to a single thread.
    static COUNTER: RefCell<i32> = RefCell::new(0);
}

fn main() -> Result<(), kas_wgpu::Error> {
    env_logger::init();

    let buttons = make_widget! {
        #[layout(row)]
        #[handler(msg = Message)]
        struct {
            #[widget] _ = TextButton::new("−", Message::Decr),
            #[widget] _ = TextButton::new("+", Message::Incr),
        }
    };

    let handle = UpdateHandle::new();

    let window = Window::new(
        "Counter",
        make_widget! {
            #[layout(column)]
            #[widget(config=noauto)]
            struct {
                #[widget(halign=centre)] display: Label = Label::new("0"),
                #[widget(handler = handle_button)] buttons -> Message = buttons,
                handle: UpdateHandle = handle,
            }
            impl WidgetConfig {
                fn configure(&mut self, mgr: &mut Manager) {
                    mgr.update_on_handle(self.handle, self.id());
                }
            }
            impl Handler {
                type Msg = VoidMsg;
                fn handle(&mut self, mgr: &mut Manager, event: Event) -> Response<VoidMsg> {
                    match event {
                        Event::HandleUpdate { .. } => {
                            let c = COUNTER.with(|c| *c.borrow());
                            *mgr += self.display.set_text(c.to_string());
                            Response::None
                        }
                        event => Response::Unhandled(event),
                    }
                }
            }
            impl {
                fn handle_button(&mut self, mgr: &mut Manager, msg: Message)
                    -> Response<VoidMsg>
                {
                    COUNTER.with(|c| {
                        let mut c = c.borrow_mut();
                        *c += match msg {
                            Message::Decr => -1,
                            Message::Incr => 1,
                        };
                    });
                    mgr.trigger_update(self.handle, 0);
                    Response::None
                }
            }
        },
    );

    let mut theme = kas_theme::ShadedTheme::new();
    theme.set_font_size(24.0);
    let mut toolkit = kas_wgpu::Toolkit::new(theme)?;
    toolkit.add(window.clone())?;
    toolkit.add(window)?;
    toolkit.run()
}
