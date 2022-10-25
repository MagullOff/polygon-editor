use std::collections::HashMap;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

use crate::{data_models::Point, draw::{clear_canvas, BASIC_COLOR}};
use super::Canvas;

#[wasm_bindgen]
impl Canvas {
    pub fn draw(&self){
        let relation_number = 0;
        let mut relation_map: HashMap<u32, u32> = HashMap::new();
        clear_canvas(&self.context);

        self.polygons
            .iter()
            .for_each(|polygon| polygon.draw(&self.context, relation_number, &mut relation_map));

        self.current_points
            .first()
            .and_then(|point| {
                self.context.move_to(point.x,point.y);
                Some(point)
            });

        self.current_points
            .iter()
            .for_each(|Point{x,y, id}| {
                if *id != 0 {
                    self.context.set_line_width(4.0);
                    self.context.arc(*x, *y, 5.0, 0.0, 2.0*std::f64::consts::PI).unwrap();
                    self.context.fill();
                }
                self.context.set_line_width(3.0);
                self.context.line_to(*x,*y);
                self.context.move_to(*x,*y);
            });
            self.context.set_stroke_style(&JsValue::from_str(BASIC_COLOR));
            self.context.stroke();
    }

    pub fn draw_bresenham(&self){
        clear_canvas(&self.context);

        self.polygons
            .iter()
            .for_each(|polygon| polygon.draw_bresenham(&self.context));
    }
}