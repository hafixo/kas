//! Window widgets

use cw;

use std::fmt::{self, Debug};

use Coord;
use event::{self, Handler};
use widget::{Class, Layout, Widget, CoreData, WidgetCore};
use widget::control::{button, TextButton};
use toolkit::Toolkit;

/// A window is a drawable interactive region provided by windowing system.
pub trait Window: Widget {
    /// Upcast
    /// 
    /// Note: needed because Rust does not yet support trait object upcasting
    fn as_widget(&self) -> &Widget;
    /// Upcast, mutably
    /// 
    /// Note: needed because Rust does not yet support trait object upcasting
    fn as_widget_mut(&mut self) -> &mut Widget;
    
    /// Calculate and update positions for all sub-widgets
    fn configure_widgets(&mut self, tk: &Toolkit);
    
    /// Adjust the size of the window, repositioning widgets.
    /// 
    /// `configure_widgets` must be called before this.
    fn resize(&mut self, tk: &Toolkit, size: Coord);
    
    /// Handle an input event.
    fn handle(&mut self, event: event::Event) -> event::Response;
}

/// Main window type
pub struct SimpleWindow<W> {
    core: CoreData,
    min_size: Coord,
    solver: cw::Solver,
    key_end: usize,
    w: W
}

impl<W: Debug> Debug for SimpleWindow<W> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SimpleWindow {{ core: {:?}, min_size: {:?}, solver: <omitted>, key_end: {}, w: {:?} }}",
            self.core, self.min_size, self.key_end, self.w)
    }
}

impl<W: Clone> Clone for SimpleWindow<W> {
    fn clone(&self) -> Self {
        SimpleWindow {
            core: self.core.clone(),
            min_size: self.min_size,
            solver: cw::Solver::new(),
            key_end: 0,
            w: self.w.clone()
        }
    }
}

impl_widget_core!(SimpleWindow<W>, core);

impl<W: Widget> SimpleWindow<W> {
    /// Create
    pub fn new(w: W) -> SimpleWindow<W> {
        SimpleWindow {
            core: Default::default(),
            min_size: (0, 0),
            solver: cw::Solver::new(),
            key_end: 0,
            w
        }
    }
}

impl<W: Layout> Layout for SimpleWindow<W> {
    fn init_constraints(&self, tk: &Toolkit, key: usize,
        s: &mut cw::Solver, use_default: bool) -> usize
    {
        self.w.init_constraints(tk, key, s, use_default)
    }
    
    fn apply_constraints(&mut self, tk: &Toolkit, key: usize,
        s: &cw::Solver, pos: Coord) -> usize
    {
        self.w.apply_constraints(tk, key, s, pos)
    }
    
    fn sync_size(&mut self, tk: &Toolkit) {
        let new_rect = tk.tk_widget().get_rect(self.get_tkd());
        *self.rect_mut() = new_rect;
        
        self.w.sync_size(tk);
    }
}

impl<W: Widget + 'static> Widget for SimpleWindow<W> {
    fn class(&self) -> Class { Class::Window }
    fn label(&self) -> Option<&str> { None }
    
    fn len(&self) -> usize { 1 }
    fn get(&self, index: usize) -> Option<&Widget> {
        match index {
            0 => Some(&self.w),
            _ => None
        }
    }
    fn get_mut(&mut self, index: usize) -> Option<&mut Widget> {
        match index {
            0 => Some(&mut self.w),
            _ => None
        }
    }
}

impl<R, W: Handler<Response = R> + Widget + 'static> Window for SimpleWindow<W>
    where event::Response: From<R>, R: From<event::NoResponse>
{
    fn as_widget(&self) -> &Widget { self }
    fn as_widget_mut(&mut self) -> &mut Widget { self }
    
    fn configure_widgets(&mut self, tk: &Toolkit) {
        let v0 = cw::Variable::from_usize(0);
        let v1 = cw::Variable::from_usize(1);
        
        self.solver.reset();
        
        self.key_end = self.w.init_constraints(tk, 0, &mut self.solver, true);
        
        self.solver.add_edit_variable(v0, cw::strength::MEDIUM * 100.0).unwrap();
        self.solver.add_edit_variable(v1, cw::strength::MEDIUM * 100.0).unwrap();
        
        self.min_size = (self.solver.get_value(v0) as i32, self.solver.get_value(v1) as i32);
        
        let apply_key = self.w.apply_constraints(tk, 0, &self.solver, (0, 0));
        assert_eq!(self.key_end, apply_key);
    }
    
    fn resize(&mut self, tk: &Toolkit, size: Coord) {
        let v0 = cw::Variable::from_usize(0);
        let v1 = cw::Variable::from_usize(1);
        self.solver.suggest_value(v0, size.0 as f64).unwrap();
        self.solver.suggest_value(v1, size.1 as f64).unwrap();
        
        let apply_key = self.w.apply_constraints(tk, 0, &self.solver, (0, 0));
        assert_eq!(self.key_end, apply_key, "resize called without configure_widgets");
    }
    
    fn handle(&mut self, event: event::Event) -> event::Response {
        event::Response::from(self.w.handle(event))
    }
}


pub fn action_close() -> impl Fn() -> event::Response {
    || event::Response::Close
}

#[derive(Clone, Debug)]
pub struct MessageBox<M, H> {
    core: CoreData,
    message: M,
    button: TextButton<H>,
}

impl<M, R, H: Fn() -> R> MessageBox<M, H> {
    // TODO: action parameter shouldn't be necessary, but we need it because
    // H must be derived from function input somehow, not merely unspecified
    // Once existential types are available, H parameter will not be needed.
    pub fn new(message: M, action: H) -> Self {
        MessageBox{
            core: Default::default(),
            message,
            button: button::ok(action)
        }
    }
}

impl_widget_core!(MessageBox<M, H>, core);

impl<M: Debug, H: Debug> Layout for MessageBox<M, H> {}

impl<M: Debug, H: Debug> Widget for MessageBox<M, H> {
    fn class(&self) -> Class { Class::Window }
    fn label(&self) -> Option<&str> { None }
    
    fn len(&self) -> usize { 0 }
    fn get(&self, index: usize) -> Option<&Widget> {
        unimplemented!()
    }
    fn get_mut(&mut self, index: usize) -> Option<&mut Widget> {
        unimplemented!()
    }
}

impl<M: Debug, H: Debug> Window for MessageBox<M, H> {
    fn as_widget(&self) -> &Widget { self }
    fn as_widget_mut(&mut self) -> &mut Widget { self }
    
    fn configure_widgets(&mut self, tk: &Toolkit) {
        unimplemented!()
    }
    
    fn resize(&mut self, tk: &Toolkit, size: Coord) {
        unimplemented!()
    }
    
    fn handle(&mut self, event: event::Event) -> event::Response {
        unimplemented!()
    }
}
