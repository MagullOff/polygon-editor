use rand::Rng;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d};
use crate::data_models::*;

pub fn get_lines<'a>(points: Vec<&'a Point>) -> Vec<Line>{
    if points.len() < 3 {
        return vec![];
    }
    let mut lines: Vec<Line> = vec![];
    let mut last_point= points.last().unwrap();
    let mut last_point_id = last_point.id;
    let mut rng = rand::thread_rng();
    points
        .iter()
        .for_each(|point| {
            lines.push(Line {
                points: (last_point_id, point.id),
                length: get_length((last_point.x, last_point.y), (point.x, point.y)),
                id: rng.gen()
            });
            last_point = point;
            last_point_id = point.id;
        });
    lines
}

pub fn get_click_point(p1: (f64,f64), p2: (f64,f64), offset_x: f64) -> (f64,f64) {
    let x: f64 = p1.0 + offset_x;
    let y: f64 = (offset_x * (p2.1 - p1.1))/(p2.0-p1.0) + p1.1;
    (x,y)
}

pub fn get_length(p1: (f64, f64), p2: (f64,f64)) -> f64{
    ((p1.0 - p2.0)*(p1.0 - p2.0)+(p1.1 - p2.1)*(p1.1 - p2.1)).sqrt()
}

pub fn hl_point(context: &CanvasRenderingContext2d, p: (f64,f64)){
    context.begin_path();
    context.set_stroke_style(&JsValue::from_str("red"));
    context.move_to(p.0, p.1);
    context.arc(p.0, p.1, 15.0, 0.0, 2.0 * 3.14);
    context.stroke();
    context.set_stroke_style(&JsValue::from_str("black"));
}

pub fn hl_line(context: &CanvasRenderingContext2d, l1: (f64,f64), l2: (f64,f64)){
    context.begin_path();
    context.move_to(l1.0, l1.1);
    context.line_to(l2.0, l2.1);
    context.set_stroke_style(&JsValue::from_str("red"));
    context.stroke();
    context.set_stroke_style(&JsValue::from_str("black"));
}

pub fn check_point(p1: (f64,f64), p2: (f64,f64)) -> bool{
    ((p1.0 - p2.0)*(p1.0 - p2.0) + (p1.1 - p2.1)*(p1.1 - p2.1)) < 200.0
}

pub fn check_line(l1: (f64,f64), l2: (f64,f64),p: (f64,f64)) -> bool {
    if !(p.0 > l1.0.min(l2.0) && p.0 < l1.0.max(l2.0) && p.1 > l1.1.min(l2.1) && p.1 < l1.1.max(l2.1)) {
        return false;
    }
    (((l2.0 - l1.0)*(l1.1 - p.1) - (l1.0-p.0)*(l2.1-l1.1)).abs())/(((l2.0 - l1.0)*(l2.0 - l1.0) + (l2.1 - l1.1)*(l2.1 - l1.1)).sqrt()) < 20.0
}

pub fn calculate_point_position(l1: (f64,f64), l2: (f64,f64)) -> (f64,f64) {
    (l1.0.min(l2.0) + (l1.0 - l2.0).abs()/2.0, l1.1.min(l2.1) + (l1.1 - l2.1).abs()/2.0)
}

pub fn get_center(points: &Vec<Point>) -> Point{
    let first_point = points.first().unwrap_or(&Point{x:0.0,y:0.0,id: 0});
    let mut xmin = first_point.x;
    let mut xmax = first_point.x;
    let mut ymin = first_point.y;
    let mut ymax = first_point.y;

    points
        .into_iter()
        .for_each(|Point{x,y, id: _}| {
            xmin = xmin.min(*x);
            ymin = ymin.min(*y);
            xmax = xmax.max(*x);
            ymax = ymax.max(*y);
        });
    Point {
        x: (xmax-xmin)/2.0+xmin,
        y: (ymax-ymin)/2.0+ymin,
        id: 0
    }
}
