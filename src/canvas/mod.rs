use wasm_bindgen::JsCast;
pub use wasm_bindgen::prelude::*;
use web_sys::Document;
use web_sys::HtmlInputElement;
use web_sys::CanvasRenderingContext2d;
use crate::polygon::*;
use crate::data_models::*;

pub mod handlers;
pub mod draw;
pub mod utils;
pub mod predefined;

pub enum State{
    Create,
    Edit,
    Rules(Option<(usize, u32)>),
    Moving((usize, PressedObject))
}

pub enum PressedObject {
    Center,
    Line(u32,f64),
    Point(u32)
}

#[wasm_bindgen]
pub struct Canvas{
   context: CanvasRenderingContext2d,
   state: State,
   current_points: Vec<Point>,
   polygons: Vec<Polygon>,
   current_id: u32,
   length_selector: HtmlInputElement,
   is_const: HtmlInputElement
}

#[wasm_bindgen]
impl Canvas {
    pub fn new(document: Document) -> Canvas{
        let num_field_ref = document.get_element_by_id("LengthSelector").unwrap();
        let num_field: web_sys::HtmlInputElement = num_field_ref
            .dyn_into::<web_sys::HtmlInputElement>()
            .map_err(|_| ())
            .unwrap();

        let is_const_ref = document.get_element_by_id("IsConst").unwrap();
        let is_const: web_sys::HtmlInputElement = is_const_ref
            .dyn_into::<web_sys::HtmlInputElement>()
            .map_err(|_| ())
            .unwrap();

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
        context.set_line_width(3.0);
        context.set_font("30px serif");
        Canvas{
            context,
            state: State::Create,
            current_points: vec![],
            current_id: 1,
            polygons: vec![],
            is_const,
            length_selector: num_field
        }
    }

}