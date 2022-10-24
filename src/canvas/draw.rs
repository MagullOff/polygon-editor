use wasm_bindgen::JsValue;

use crate::{data_models::Point, draw::{clear_canvas, BASIC_COLOR}};
use super::Canvas;

impl Canvas {
    pub fn clear_current_points(&mut self) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];
        while !self.current_points.is_empty() {
            let point = self.current_points.pop().unwrap();
            points.push(point);
        }
        points
    }

    pub fn draw(&self){
        clear_canvas(&self.context);

        self.polygons
            .iter()
            .for_each(|polygon| polygon.draw(&self.context));

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
                    self.context.arc(*x, *y, 5.0, 0.0, 2.0*3.14).unwrap();
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