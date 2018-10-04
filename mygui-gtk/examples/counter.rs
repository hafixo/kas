//! Counter example (simple button)

#[macro_use]
extern crate mygui;
extern crate mygui_gtk;

use mygui::event::{NoResponse};
use mygui::widget::{
    canvas::Text,
    control::TextButton,
    window::SimpleWindow
};

use mygui::toolkit::Toolkit;
use mygui_gtk::{GtkToolkit, Error};

enum Message {
    None,
    Decr,
    Incr,
}

impl From<NoResponse> for Message {
    fn from(_: NoResponse) -> Self {
        Message::None
    }
}

fn main() -> Result<(), Error> {
    let window = SimpleWindow::new(   // construct with default state and handler
        make_layout!(vertical<BS[Message]>; self, tk, msg;
            display: Text = Text::from("0") => msg,
            buttons: BS = make_layout!(
                horizontal<A[Message], B[Message]>; self, tk, msg;
                decr: A = TextButton::new("−", || Message::Decr) => msg,
                incr: B = TextButton::new("+", || Message::Incr) => msg;;
                Message) =>
            {
                match msg {
                    Message::None => (),
                    Message::Decr => {
                        self.counter = self.counter.saturating_sub(1);
                        self.display.set_text(tk, &self.counter.to_string());
                    }
                    Message::Incr => {
                        self.counter = self.counter.saturating_add(1);
                        self.display.set_text(tk, &self.counter.to_string());
                    }
                };
                NoResponse::None
            };
            counter: usize = 0;
            NoResponse
        )
    );
    
    let mut toolkit = GtkToolkit::new()?;
    toolkit.add(&window);
    toolkit.main();
    Ok(())
}
