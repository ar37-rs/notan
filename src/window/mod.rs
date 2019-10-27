use glow::*;

//add #[cfg(target_arch = "wasm32")]
use crate::graphics;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;

fn default_cb() {}

pub struct Window {
    //    window: web_sys::Window,
    canvas: web_sys::HtmlCanvasElement,
    //    ctx: graphics::Context,
    //gl: glow::Context
    //    cb: fn()
}

impl Window {
    pub fn new() -> Window {
        let window = web_sys::window().unwrap();

        let canvas = window
            .document()
            .unwrap()
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        //        let ctx = graphics::Context::new(&canvas).unwrap(); //TODO manage error

        Window {
            //            window: window,
            canvas: canvas,
            //            cb: default_cb
            //            ctx: ctx, //gl: glow::Context::from_webgl2_context(gl)
        }
    }

    pub fn window(&self) -> &web_sys::HtmlCanvasElement {
        return &self.canvas;
    }

    //window_rect()? top_right, top_left, center, etc...
    //    pub fn top_right(&self) -> (f32, f32) {
    //        (self.canvas.width() as f32 * -0.5, self.canvas.height() as f32 * -0.5)
    //    }
}

fn request_animation_frame(win: web_sys::Window, f: &Closure<dyn FnMut()>) {
    win.request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn run<F>(callback: F)
where
    F: FnMut() + 'static,
{
    //        self.cb = cb;
    let cb = Rc::new(RefCell::new(None));
    let cb_copy = cb.clone();
    let callback = Rc::new(RefCell::new(callback));

    *cb_copy.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        //            let mut ctx = ctx.borrow_mut();
        //            ctx.fps_tracker.tick();

        let mut tick_handler = callback.borrow_mut();
        (&mut *tick_handler)();

        //            if ctx.running {
        //Web always run at max speed using raf (setTimeout has drawbacks)
        let win = web_sys::window().unwrap();
        request_animation_frame(win, cb.borrow().as_ref().unwrap());
        //            }
    }) as Box<FnMut()>));

    let win = web_sys::window().unwrap();
    request_animation_frame(win, cb_copy.borrow().as_ref().unwrap());
}