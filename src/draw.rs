use std::mem::swap;

use crate::{polygon::Polygon, data_models::PointCords};
use web_sys::CanvasRenderingContext2d;
use wasm_bindgen::JsValue;

const CANVAS_X: f64 = 100000.0;
const CANVAS_Y: f64 = 70000.0;
const POINT_RADIUS: f64 = 5.0;
const CENTER_RADIUS: f64 = 7.0;
const HL_RADIUS: f64 = 8.0;

impl Polygon{
    pub fn draw_bresenham(&self, context: &CanvasRenderingContext2d){
        context.set_line_width(4.0);
        self.points
            .iter()
            .for_each(|point| {
                draw_point(context, PointCords(point.x,point.y), POINT_RADIUS)
            });
        
        self.lines
            .iter()
            .for_each(|line| {
                let p1 = self.get_point_by_id(line.points.0);
                let p2 = self.get_point_by_id(line.points.1);
                draw_bresenham_line(context, p1, p2);
            });
        draw_point(context, self.center, CENTER_RADIUS);
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d){
        context.set_line_width(4.0);
        self.points
            .iter()
            .for_each(|point| {
                draw_point(context, PointCords(point.x,point.y), POINT_RADIUS)
            });

        context.set_line_width(3.0);
        self.lines
            .iter()
            .for_each(|line| {
                let p1 = self.get_point_by_id(line.points.0);
                let p2 = self.get_point_by_id(line.points.1);
                context.move_to(p1.0, p1.1);
                context.line_to(p2.0,p2.1);
            });
        context.stroke();
        draw_point(context, self.center, CENTER_RADIUS);
    }
}

pub fn draw_bresenham_line(context: &CanvasRenderingContext2d, x: PointCords, y: PointCords){
    let mut x0: f64 = x.0.floor();
    let mut x1: f64 = y.0.floor();
    let mut y0: f64 = x.1.floor();
    let mut y1: f64 = y.1.floor();

    let steep = (x0 - x1).abs() < (y0 - y1).abs();

    if steep {
        swap(&mut x0, &mut y0);
        swap(&mut x1, &mut y1);
    }
    if x0 > x1 {
        swap(&mut x0, &mut x1);
        swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;
    let derror2 = dy.abs() * 2.0;
    let mut error2 = 0.0;
    let mut y = y0;

    let mut x = x0;
    while x <= x1 {
        if steep {
            context.rect(y, x, 1.0, 1.0);
            //image.set(y as usize, x as usize, color);
        } else {
            context.rect(x, y, 1.0, 1.0);
            //image.set(x as usize, y as usize, color);
        }

        error2 += derror2;

        if error2 > dx {
            y += if y1 > y0 { 1.0 } else { -1.0 };
            error2 -= dx * 2.0;
        }
        x += 1.0;
    }
    context.stroke();
}

pub fn clear_canvas(context: &CanvasRenderingContext2d){
    context.clear_rect(0.0,0.0,CANVAS_X,CANVAS_Y);
    context.stroke();
    context.begin_path();
}

pub fn highlight_point(context: &CanvasRenderingContext2d, p: PointCords){
    context.begin_path();
    context.set_stroke_style(&JsValue::from_str("red"));
    context.arc(p.0, p.1, HL_RADIUS, 0.0, 2.0 * 3.14);
    context.fill();
    context.stroke();
    context.set_fill_style(&JsValue::from_str("black"));
    context.set_stroke_style(&JsValue::from_str("black"));
}

pub fn draw_point(context: &CanvasRenderingContext2d, p: PointCords, radius: f64) {
    context.begin_path();
    context.arc(p.0, p.1, radius, 0.0, 2.0 * 3.14);
    context.fill();
    context.stroke();
}

pub fn highlight_line(context: &CanvasRenderingContext2d, l1: PointCords, l2: PointCords){
    context.begin_path();
    context.move_to(l1.0, l1.1);
    context.line_to(l2.0, l2.1);
    context.set_stroke_style(&JsValue::from_str("red"));
    context.stroke();
    context.set_stroke_style(&JsValue::from_str("black"));
}

