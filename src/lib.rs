use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Document;
use web_sys::{console,Element,HtmlCanvasElement,CanvasRenderingContext2d};


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Clone, Copy)]
pub struct Point{
    pub x: f64,
    pub y: f64

}

pub struct Line<'a>(&'a Point,&'a Point);

#[wasm_bindgen]
pub struct Canvas{
   lines: Vec<Line<'static>>,
   context: CanvasRenderingContext2d,
   state: State,
   points: Vec<Point>,
   current_points: Vec<Point>
}

pub enum State{
    Edit(Vec<Point>),
    Es
}

#[wasm_bindgen]
impl Canvas{
    fn draw(&self){
        self.context.clear_rect(0.0,0.0,1000.0,700.0);
        self.context.stroke();
        self.context.clear_rect(0.0,0.0,1000.0,700.0);
        self.current_points
            .first()
            .and_then(|point| {
                self.context.move_to(point.x,point.y);
                Some(point)
            });
        self.current_points
            .iter()
            .for_each(|Point{x,y}| {
                self.context.line_to(*x,*y);
                self.context.stroke();
                self.context.move_to(*x,*y);
            })
    }

    pub fn new(document: Document) -> Canvas{
        let canvas_ref = document.get_element_by_id("board").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas_ref
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        Canvas{
            context,
            lines: vec![],
            state: State::Edit(vec![]),
            points: vec![],
            current_points: vec![],
        }
    }

    pub fn onclick(&mut self, x: f64, y: f64){
        self.context.clear_rect(0.0,0.0,1000.0,700.0);
        self.context.begin_path();
        self.current_points.push(Point{x,y});
        self.draw();
    }

    pub fn movemouse(&mut self, x: f64, y: f64){
        self.context.clear_rect(0.0,0.0,1000.0,700.0);
        self.context.stroke();
        self.context.begin_path();
        self.current_points.push(Point{x,y});
        self.draw();
        self.current_points.pop();
    }
}
