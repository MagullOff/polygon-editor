use rand::Rng;
use crate::data_models::*;

const LINE_MARGIN: f64 = 15.0;
const POINT_MARGIN: f64 = 200.0;

pub fn calcualate_new_lines<'a>(points: Vec<&'a Point>) -> Vec<Line>{
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
                length: get_line_length(PointCords(last_point.x, last_point.y), PointCords(point.x, point.y)),
                id: rng.gen()
            });
            last_point = point;
            last_point_id = point.id;
        });
    lines
}

pub fn get_click_point(p1: PointCords, p2: PointCords, offset_x: f64) -> PointCords {
    let x: f64 = p1.0 + offset_x;
    let y: f64 = (offset_x * (p2.1 - p1.1))/(p2.0-p1.0) + p1.1;
    PointCords(x,y)
}

pub fn get_line_length(p1: PointCords, p2: PointCords) -> f64{
    ((p1.0 - p2.0)*(p1.0 - p2.0)+(p1.1 - p2.1)*(p1.1 - p2.1)).sqrt()
}

pub fn check_point_hover(p1: PointCords, p2: PointCords) -> bool{
    ((p1.0 - p2.0)*(p1.0 - p2.0) + (p1.1 - p2.1)*(p1.1 - p2.1)) < POINT_MARGIN
}

pub fn check_line_hover(l1: PointCords, l2: PointCords,p: PointCords) -> bool {
    if !(p.0 > l1.0.min(l2.0)-LINE_MARGIN && p.0 < l1.0.max(l2.0)+LINE_MARGIN && p.1 > l1.1.min(l2.1)-LINE_MARGIN && p.1 < l1.1.max(l2.1)+LINE_MARGIN) {
        return false;
    }
    (((l2.0 - l1.0)*(l1.1 - p.1) - (l1.0-p.0)*(l2.1-l1.1)).abs())/(((l2.0 - l1.0)*(l2.0 - l1.0) + (l2.1 - l1.1)*(l2.1 - l1.1)).sqrt()) < LINE_MARGIN
}

pub fn calculate_middle_point(l1: PointCords, l2: PointCords) -> PointCords {
    PointCords(l1.0.min(l2.0) + (l1.0 - l2.0).abs()/2.0, l1.1.min(l2.1) + (l1.1 - l2.1).abs()/2.0)
}

pub fn get_centroid(points: &Vec<Point>) -> PointCords {
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    points
        .iter()
        .for_each(|point| {
            sum_y = sum_y + point.y;
            sum_x = sum_x + point.x;
        });
    PointCords(sum_x/f64::from(points.len() as u8),sum_y/f64::from(points.len() as u8))
}
