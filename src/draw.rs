use crate::{polygon::Polygon, data_models::PointCords};
use web_sys::CanvasRenderingContext2d;
use wasm_bindgen::JsValue;

const CANVAS_X: f64 = 100000.0;
const CANVAS_Y: f64 = 70000.0;

impl Polygon{
    pub fn draw(&self, context: &CanvasRenderingContext2d){
        self.points
            .iter()
            .for_each(|point| {
                context.move_to(point.x, point.y);
                context.set_line_width(7.0);
                context.arc(point.x, point.y, 5.0, 0.0, 2.0*3.14);
                context.set_line_width(3.0);
            });
        self.lines
            .iter()
            .for_each(|line| {
                let p1 = self.get_point_by_id(line.points.0);
                let p2 = self.get_point_by_id(line.points.1);
                context.move_to(p1.0, p1.1);
                context.line_to(p2.0,p2.1);
            });
        context.fill_rect(self.center.0-10.0, self.center.1-10.0, 20.0, 20.0);
        context.stroke();
    }
}

pub fn clear_canvas(context: &CanvasRenderingContext2d){
    context.clear_rect(0.0,0.0,CANVAS_X,CANVAS_Y);
    context.stroke();
    context.begin_path();
}

pub fn highlight_point(context: &CanvasRenderingContext2d, p: PointCords){
    context.begin_path();
    context.set_stroke_style(&JsValue::from_str("red"));
    context.move_to(p.0, p.1);
    context.arc(p.0, p.1, 15.0, 0.0, 2.0 * 3.14);
    context.stroke();
    context.set_stroke_style(&JsValue::from_str("black"));
}

pub fn highlight_line(context: &CanvasRenderingContext2d, l1: PointCords, l2: PointCords){
    context.begin_path();
    context.move_to(l1.0, l1.1);
    context.line_to(l2.0, l2.1);
    context.set_stroke_style(&JsValue::from_str("red"));
    context.stroke();
    context.set_stroke_style(&JsValue::from_str("black"));
}

