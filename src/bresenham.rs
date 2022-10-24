use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use crate::{polygon::Polygon, data_models::PointCords, draw::{draw_point, POINT_RADIUS, CENTER_RADIUS, BASIC_COLOR}};

impl Polygon {
    pub fn draw_bresenham(&self, context: &CanvasRenderingContext2d){
        context.set_line_width(4.0);
        
        self.lines
            .iter()
            .for_each(|line| {
                let p1 = self.get_point_by_id(line.points.0);
                let p2 = self.get_point_by_id(line.points.1);
                draw_bresenham_line(context, p1, p2);
            });

        self.points
            .iter()
            .for_each(|point| {
                draw_point(context, PointCords(point.x,point.y), POINT_RADIUS)
            });

        draw_point(context, self.center, CENTER_RADIUS);
    }
}

fn draw_bresenham_line(context: &CanvasRenderingContext2d, x: PointCords, y: PointCords){
    context.set_fill_style(&JsValue::from_str(BASIC_COLOR));
    let mut x0: f64 = x.0.floor();
    let mut x1: f64 = y.0.floor();
    let mut y0: f64 = x.1.floor();
    let mut y1: f64 = y.1.floor();

    let steep = (x0 - x1).abs() < (y0 - y1).abs();

    if steep {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
    }
    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
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
        } else {
            context.rect(x, y, 1.0, 1.0);
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