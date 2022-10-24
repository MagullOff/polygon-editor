use crate::{polygon::Polygon, data_models::PointCords};
use web_sys::CanvasRenderingContext2d;
use wasm_bindgen::JsValue;
use crate::utils::calculate_middle_point;
use std::collections::HashMap;

const CANVAS_X: f64 = 100000.0;
const CANVAS_Y: f64 = 70000.0;
pub const POINT_RADIUS: f64 = 5.0;
pub const CENTER_RADIUS: f64 = 7.0;
const HL_RADIUS: f64 = 8.0;
pub const BASIC_COLOR: &str = "rgb(44, 0, 117)";
pub const HIGHLIGHT_COLOR: &str = "rgb(207, 52, 121)";
pub const CONSTANT_COLOR: &str = "rgb(141, 55, 179)";

impl Polygon{
    pub fn draw(&self, context: &CanvasRenderingContext2d){
        let mut relation_number: u32 = 0;
        let mut relation_map: HashMap<u32, u32> = HashMap::new();
        context.set_line_width(3.0);
        self.lines
            .iter()
            .for_each(|line| {
                context.begin_path();
                let p1 = self.get_point_by_id(line.points.0);
                let p2 = self.get_point_by_id(line.points.1);
                let mid = calculate_middle_point(p1, p2);

                match line.relation {
                    Some(l) => {
                        let rel_num = if relation_map.contains_key(&line.id) {
                            relation_map[&line.id]
                        } else {
                            relation_number = relation_number + 1;
                            relation_map.insert(l, relation_number);
                            relation_number
                        };
                        context.set_font("30px serif");
                        context.fill_text(rel_num.to_string().as_str(), mid.0, mid.1).unwrap();
                    },
                    _ => {}
                }
                context.move_to(p1.0, p1.1);
                context.line_to(p2.0,p2.1);
                match line.is_const {
                    true => {context.set_stroke_style(&JsValue::from_str(CONSTANT_COLOR));},
                    false => {context.set_stroke_style(&JsValue::from_str(BASIC_COLOR));}
                }
                context.stroke();
            });

        context.set_line_width(4.0);
        self.points
            .iter()
            .for_each(|point| {
                draw_point(context, PointCords(point.x,point.y), POINT_RADIUS)
            });

        draw_point(context, self.center, CENTER_RADIUS);
    }
}

pub fn clear_canvas(context: &CanvasRenderingContext2d){
    context.clear_rect(0.0,0.0,CANVAS_X,CANVAS_Y);
    context.stroke();
    context.begin_path();
}

pub fn highlight_point(context: &CanvasRenderingContext2d, p: PointCords){
    context.begin_path();
    context.set_stroke_style(&JsValue::from_str(HIGHLIGHT_COLOR));
    context.arc(p.0, p.1, HL_RADIUS, 0.0, 2.0 * 3.14).unwrap();
    context.fill();
    context.stroke();
    context.set_fill_style(&JsValue::from_str(BASIC_COLOR));
    context.set_stroke_style(&JsValue::from_str(BASIC_COLOR));
}

pub fn draw_point(context: &CanvasRenderingContext2d, p: PointCords, radius: f64) {
    context.begin_path();
    context.set_stroke_style(&JsValue::from_str(BASIC_COLOR));
    context.arc(p.0, p.1, radius, 0.0, 2.0 * 3.14).unwrap();
    context.fill();
    context.stroke();
}

pub fn highlight_line(context: &CanvasRenderingContext2d, l1: PointCords, l2: PointCords){
    context.begin_path();
    context.set_line_width(4.0);
    context.move_to(l1.0, l1.1);
    context.line_to(l2.0, l2.1);
    context.set_stroke_style(&JsValue::from_str(HIGHLIGHT_COLOR));
    context.stroke();
    context.set_line_width(3.0);
    context.set_stroke_style(&JsValue::from_str(BASIC_COLOR));
}

