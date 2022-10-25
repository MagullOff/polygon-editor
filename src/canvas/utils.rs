use crate::{data_models::{PointCords, Point}, utils::get_line_length};
use super::Canvas;

impl Canvas {
    fn get_line_by_id(&self, id: u32) -> (PointCords, PointCords) {
        let mut i = 0;
        while i < self.polygons.len() {
            if self.polygons[i].lines.iter().any(|line| line.id == id) {
                let (p1, p2) = self.polygons[i].get_line_by_id(id);
                return (self.polygons[i].get_point_by_id(p1), self.polygons[i].get_point_by_id(p2))
            }
            i = i + 1;
        }
        (PointCords(0.0, 0.0), PointCords(0.0, 0.0))
    }

    pub fn enforce_relation(&mut self, line_id: u32, related_line_id: u32) {
        let mut j = 0;
        while j < self.polygons.len() {
            let mut k = 0;
            while k < self.polygons[j].lines.len() {
                if self.polygons[j].lines[k].id == related_line_id {
                    if self.polygons[j].lines[k].visited == true {return}
                    self.polygons[j].lines[k].visited = true;
                    let (p1, p2) = self.get_line_by_id(line_id);
                    let (p3_id, p4_id) = self.polygons[j].lines[k].points;
                    let p3 = self.polygons[j].get_point_by_id(p3_id);
                    let px = (self.polygons[j].lines[k].length * (p1.0 - p2.0))/get_line_length(p1,p2) + p3.0;
                    let py = (if p1.1 > p2.1 {1.0} else {-1.0})*(self.polygons[j].lines[k].length.powi(2) - (px - p3.0).powi(2)).sqrt() + p3.1;

                    let mut l = 0;
                    while l < self.polygons[j].points.len() {
                        if self.polygons[j].points[l].id == p4_id {
                            self.polygons[j].points[l] = Point{id: p4_id, x: px, y: py};
                            self.correct_line_length(p4_id, j, true);
                        }
                        l = l + 1;
                    }
                }
                k = k + 1;
            }
            j = j + 1;
        }
    }

    pub fn correct_line_mid(&mut self, extention: f64, line_id: u32, polygon_id: usize){
        let line = self.polygons[polygon_id].get_line_reference_inmut(line_id);

        let mut i=0;
        let mut p0_index = 0;
        let mut p1_index = 0;
        while i<self.polygons[polygon_id].points.len() {
            if self.polygons[polygon_id].points[i].id == line.points.1 {
                p0_index = i;
            }
            if self.polygons[polygon_id].points[i].id == line.points.0 {
                p1_index = i;
            }
            i = i + 1;
        }

        let ratio = (self.polygons[polygon_id].points[p1_index].x - self.polygons[polygon_id].points[p0_index].x).abs()/(self.polygons[polygon_id].points[p1_index].y - self.polygons[polygon_id].points[p0_index].y).abs();
        let ratio_x = 1.0/(1.0 + 1.0/(ratio*ratio)).sqrt();
        let ratio_y = 1.0/(1.0 + ratio*ratio).sqrt();

        if self.polygons[polygon_id].points[p1_index].x > self.polygons[polygon_id].points[p0_index].x {
            self.polygons[polygon_id].points[p1_index].x = self.polygons[polygon_id].points[p1_index].x + extention*ratio_x;
            self.polygons[polygon_id].points[p0_index].x = self.polygons[polygon_id].points[p0_index].x - extention*ratio_x;
        } else {
            self.polygons[polygon_id].points[p1_index].x = self.polygons[polygon_id].points[p1_index].x - extention*ratio_x;
            self.polygons[polygon_id].points[p0_index].x = self.polygons[polygon_id].points[p0_index].x + extention*ratio_x;
        }

        if self.polygons[polygon_id].points[p1_index].y > self.polygons[polygon_id].points[p0_index].y {
            self.polygons[polygon_id].points[p1_index].y = self.polygons[polygon_id].points[p1_index].y + extention*ratio_y;
            self.polygons[polygon_id].points[p0_index].y = self.polygons[polygon_id].points[p0_index].y - extention*ratio_y;
        } else {
            self.polygons[polygon_id].points[p1_index].y = self.polygons[polygon_id].points[p1_index].y - extention*ratio_y;
            self.polygons[polygon_id].points[p0_index].y = self.polygons[polygon_id].points[p0_index].y + extention*ratio_y
        }
    }

    pub fn correct_line_length(&mut self, point_id: u32, polygon_id: usize, is_direction_forward: bool){
        let mut point_index = 0;
        while self.polygons[polygon_id].points[point_index].id != point_id {point_index = point_index + 1;}

        let mut line_index = 0;
        match is_direction_forward {
            true => {while self.polygons[polygon_id].lines[line_index].points.0 != point_id {line_index = line_index + 1;}},
            false => {while self.polygons[polygon_id].lines[line_index].points.1 != point_id {line_index = line_index + 1;}}
        }

        let mut x = false;
        while (self.polygons[polygon_id].lines[line_index].is_const == true || self.polygons[polygon_id].lines[line_index].relation != None)
        && (((if is_direction_forward {self.polygons[polygon_id].lines[line_index].points.0} else {self.polygons[polygon_id].lines[line_index].points.1} != point_id) && x == true)
            || x == false){
            x = true;
            if self.polygons[polygon_id].lines[line_index].visited == true {return}
            self.polygons[polygon_id].lines[line_index].visited = true;
            if self.polygons[polygon_id].lines[line_index].is_const {
                let p1 =self.polygons[polygon_id].get_point_by_id(self.polygons[polygon_id].lines[line_index].points.0);
                let p2 = self.polygons[polygon_id].get_point_by_id(self.polygons[polygon_id].lines[line_index].points.1);
                let current_len = get_line_length(p1, p2);
                let len = self.polygons[polygon_id].lines[line_index].length;
                let extention = len - current_len;

                let ratio = (p2.0 - p1.0).abs()/(p2.1 - p1.1).abs();
                let ratio_x = 1.0/(1.0 + 1.0/(ratio*ratio)).sqrt();
                let ratio_y = 1.0/(1.0 + ratio*ratio).sqrt();

                let mut i=0;
                let mut p2_index = 0;
                while i<self.polygons[polygon_id].points.len() {
                    if self.polygons[polygon_id].points[i].id == if is_direction_forward {self.polygons[polygon_id].lines[line_index].points.1} else {self.polygons[polygon_id].lines[line_index].points.0} {
                        p2_index = i;
                        break;
                    }
                    i = i + 1;
                }

                let multiplier = if is_direction_forward {1.0} else {-1.0};
                if p2.0 > p1.0 {
                    self.polygons[polygon_id].points[p2_index].x = self.polygons[polygon_id].points[p2_index].x + multiplier*extention*ratio_x;
                } else {
                    self.polygons[polygon_id].points[p2_index].x = self.polygons[polygon_id].points[p2_index].x - multiplier*extention*ratio_x;
                }

                if p2.1 > p1.1 {
                    self.polygons[polygon_id].points[p2_index].y = self.polygons[polygon_id].points[p2_index].y + multiplier*extention*ratio_y;
                } else {
                    self.polygons[polygon_id].points[p2_index].y = self.polygons[polygon_id].points[p2_index].y - multiplier*extention*ratio_y;
                }
            }

            match self.polygons[polygon_id].lines[line_index].relation {
                Some(line_id) => {
                    self.enforce_relation(self.polygons[polygon_id].lines[line_index].id, line_id);
                },
                None => {}
            };

            match is_direction_forward {
                true => {line_index = if line_index == self.polygons[polygon_id].lines.len() - 1 {0} else {line_index + 1};},
                false => {line_index = if line_index ==  0 {self.polygons[polygon_id].lines.len() - 1} else {line_index - 1};}
            }
        }
    }

    pub fn reset_visited(&mut self){
        let mut i = 0;
        while i < self.polygons.len() {
            let mut j = 0;
            while j < self.polygons[i].lines.len() {
                self.polygons[i].lines[j].visited = false;
                j = j + 1;
            }
            i = i + 1;
        }
    }

    pub fn recalculate(&mut self){
        let mut i = 0;
        while i < self.polygons.len() {
            self.polygons[i].recalculate();
            i = i + 1;
        }
    }

    pub fn clear_current_points(&mut self) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];
        while !self.current_points.is_empty() {
            let point = self.current_points.pop().unwrap();
            points.push(point);
        }
        points
    }
}