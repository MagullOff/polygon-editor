use rand::Rng;
use crate::{data_models::*, polygon::Polygon};

const LINE_MARGIN: f64 = 15.0;
const POINT_MARGIN: f64 = 200.0;

pub fn calcualate_new_lines(points: Vec<&Point>) -> Vec<Line>{
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
                id: rng.gen(),
                is_const: false,
                relation: None,
                visited: false,
                bezier: None
            });
            last_point = point;
            last_point_id = point.id;
        });
    lines
}

pub fn calculate_new_lines_preserving_relations(points: Vec<&Point>, old_lines: Vec<&Line>) -> Vec<Line>{
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
            let mut rel: Option<u32> =  None;
            for i in 0..old_lines.len() {
                if (old_lines[i].points.1 == last_point_id  && old_lines[i].points.0 == point.id) 
                    || (old_lines[i].points.0 == last_point_id  && old_lines[i].points.1 == point.id) {
                    rel = old_lines[i].relation;
                    break;
                }
            }
            lines.push(Line {
                points: (last_point_id, point.id),
                length: get_line_length(PointCords(last_point.x, last_point.y), PointCords(point.x, point.y)),
                id: rng.gen(),
                is_const: false,
                relation: rel,
                visited: false,
                bezier: None
            });
            last_point = point;
            last_point_id = point.id;
        });
    lines
}

pub fn get_click_point(p1: PointCords, offset: (f64, f64)) -> PointCords {
    let x: f64 = p1.0 + offset.0;
    let y: f64 = offset.1 + p1.1;
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

pub fn get_new_split_lines(polygon: &Polygon, x: u32, y: u32, s: u32) -> (Line, Line) {
    let mut rng = rand::thread_rng();
    (Line {
            id: rng.gen(),
            points: (x, s),
            length: get_line_length(
                polygon.get_point_by_id(x),
                polygon.get_point_by_id(s)
            ),
            is_const: false,
            relation: None,
            visited: false,
            bezier: None
        },
        Line{
            id: rng.gen(),
            points: (s, y),
            length: get_line_length(
                polygon.get_point_by_id(s),
                polygon.get_point_by_id(y)
            ),
            is_const: false,
            relation: None,
            visited: false,
            bezier: None
        })
}

pub fn check_if_parallel(l1: (PointCords, PointCords), l2: (PointCords, PointCords)) -> bool {
    (((l1.1.0 - l1.0.0)/(l1.1.1 - l1.1.0)) - ((l2.1.0 - l2.0.0)/(l2.1.1 - l2.1.0))).abs() < 0.01
}

pub fn get_bezier_cords(point_cords: (PointCords, PointCords)) -> (PointCords, PointCords) {
    let p1 = point_cords.0;
    let p2 = point_cords.1;
    let x_len = p2.0 - p1.0;
    let y_len = p2.1 - p1.1;
    let r1 = PointCords(p1.0 + (x_len/4.0), p1.1 + (y_len/4.0));
    let r2 = PointCords(p1.0 + 3.0*(x_len/4.0), p1.1 + 3.0*(y_len/4.0));
    (r1,r2)
}
