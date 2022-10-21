use crate::canvas::PressedObject;
use crate::data_models::*;
use crate::utils::*;

impl Polygon {
    pub fn get_point_by_id(&self, id: u32) -> PointCords{
        self.points
            .iter()
            .find(|point| point.id == id)
            .and_then(|point| Some(PointCords(point.x, point.y)))
            .unwrap()
    }

    pub fn get_point_reference(&mut self, id: u32) -> &mut Point {
        self.points
            .iter_mut()
            .find(|point| point.id == id)
            .unwrap()
    }

    pub fn modify_point_coordinates(&mut self, id: u32, coordinates: (f64, f64)) {
        let edited_point = self.points
            .iter_mut()
            .find(|point| point.id == id)
            .unwrap();

        edited_point.x = edited_point.x + coordinates.0;
        edited_point.y = edited_point.y + coordinates.1;
    }

    pub fn get_line_reference(&mut self, id: u32) -> &mut Line {
        self.lines
            .iter_mut()
            .find(|line| line.id == id)
            .unwrap()
    }

    pub fn get_line_reference_inmut(&self, id: u32) -> &Line {
        self.lines
            .iter()
            .find(|line| line.id == id)
            .unwrap()
    }

    pub fn get_line_by_id(&self, id: u32) -> (u32, u32) {
        self.lines
            .iter()
            .find(|line| line.id == id)
            .and_then(|line| Some(line.points))
            .unwrap()
    }

    pub fn remove_point_of_id(&mut self, id: u32) {
        let mut i = 0;
        while i<self.points.len() {
            if self.points[i].id == id {
                self.points.remove(i);
                break;
            }
            i = i + 1;
        }
        self.lines = calcualate_new_lines(self.points.iter().collect());
        self.center = get_centroid(&self.points);
    }

    pub fn check_hover(&self, x: f64, y: f64) -> Option<PressedObject> {
        if check_point_hover(self.center, PointCords(x,y)) {
            return Some(PressedObject::Center);
        }

        let mut i = 0;
        while i<self.points.len() {
            if check_point_hover(PointCords(self.points[i].x, self.points[i].y), PointCords(x,y)) {
                return Some(PressedObject::Point(self.points[i].id));
            }
            i = i + 1;
        }

        i = 0;
        while i<self.lines.len() {
            let p1 = self.get_point_by_id(self.lines[i].points.0);
            let p2 = self.get_point_by_id(self.lines[i].points.1);
            if check_line_hover(p1, p2, PointCords(x,y)) {
                return Some(PressedObject::Line(self.lines[i].id, x - p1.0));
            }
            i = i + 1;
        }

        None
    }

    pub fn recalculate(&mut self){
        self.center = get_centroid(&self.points);

        let mut i = 0;
        while i < self.lines.len() {
            let p1 = self.get_point_by_id(self.lines[i].points.0);
            let p2 = self.get_point_by_id(self.lines[i].points.1);

            self.lines[i].length = get_line_length(p1, p2);
            i = i + 1;
        }
    }

    pub fn correct_line_mid(&mut self, extention: f64, line_id: u32){
        let line = self.get_line_reference_inmut(line_id);

        let mut i=0;
        let mut p0_index = 0;
        let mut p1_index = 0;
        while i<self.points.len() {
            if self.points[i].id == line.points.1 {
                p0_index = i;
            }
            if self.points[i].id == line.points.0 {
                p1_index = i;
            }
            i = i + 1;
        }

        let ratio = (self.points[p1_index].x - self.points[p0_index].x).abs()/(self.points[p1_index].y - self.points[p0_index].y).abs();
        let ratio_x = 1.0/(1.0 + 1.0/(ratio*ratio)).sqrt();
        let ratio_y = 1.0/(1.0 + ratio*ratio).sqrt();

        if self.points[p1_index].x > self.points[p0_index].x {
            self.points[p1_index].x = self.points[p1_index].x + extention*ratio_x;
            self.points[p0_index].x = self.points[p0_index].x - extention*ratio_x;
        } else {
            self.points[p1_index].x = self.points[p1_index].x - extention*ratio_x;
            self.points[p0_index].x = self.points[p0_index].x + extention*ratio_x;
        }

        if self.points[p1_index].y > self.points[p0_index].y {
            self.points[p1_index].y = self.points[p1_index].y + extention*ratio_y;
            self.points[p0_index].y = self.points[p0_index].y - extention*ratio_y;
        } else {
            self.points[p1_index].y = self.points[p1_index].y - extention*ratio_y;
            self.points[p0_index].y = self.points[p0_index].y + extention*ratio_y
        }
    }

    pub fn correct_line_length_right(&mut self, point_id: u32){
        let mut point_index = 0;
        while self.points[point_index].id != point_id {point_index = point_index + 1;}

        let mut line_index = 0;
        while self.lines[line_index].points.0 != point_id {line_index = line_index + 1;}


        while self.lines[line_index].is_const == true || self.lines[line_index].points.0 == point_id {
            let p1 = self.get_point_by_id(self.lines[line_index].points.0);
            let p2 = self.get_point_by_id(self.lines[line_index].points.1);
            let current_len = get_line_length(p1, p2);
            let len = self.lines[line_index].length;
            let extention = len - current_len;

            let ratio = (p2.0 - p1.0).abs()/(p2.1 - p1.1).abs();
            let ratio_x = 1.0/(1.0 + 1.0/(ratio*ratio)).sqrt();
            let ratio_y = 1.0/(1.0 + ratio*ratio).sqrt();

            let mut i=0;
            let mut p2_index = 0;
            while i<self.points.len() {
                if self.points[i].id == self.lines[line_index].points.1 {
                    p2_index = i;
                    break;
                }
                i = i + 1;
            }

            if p2.0 > p1.0 {
                self.points[p2_index].x = self.points[p2_index].x + extention*ratio_x;
            } else {
                self.points[p2_index].x = self.points[p2_index].x - extention*ratio_x;
            }

            if p2.1 > p1.1 {
                self.points[p2_index].y = self.points[p2_index].y + extention*ratio_y;
            } else {
                self.points[p2_index].y = self.points[p2_index].y - extention*ratio_y;
            }
            line_index = if line_index == self.lines.len() - 1 {0} else {line_index + 1};
        }
    }

    pub fn correct_line_length_left(&mut self, point_id: u32){
        let mut point_index = 0;
        while self.points[point_index].id != point_id {point_index = point_index + 1;}

        let mut line_index = 0;
        while self.lines[line_index].points.1 != point_id {line_index = line_index + 1;}


        while self.lines[line_index].is_const == true || self.lines[line_index].points.1 == point_id{
            let p1 = self.get_point_by_id(self.lines[line_index].points.0);
            let p2 = self.get_point_by_id(self.lines[line_index].points.1);
            let current_len = get_line_length(p1, p2);
            let len = self.lines[line_index].length;
            let extention = len - current_len;

            let ratio = (p2.0 - p1.0).abs()/(p2.1 - p1.1).abs();
            let ratio_x = 1.0/(1.0 + 1.0/(ratio*ratio)).sqrt();
            let ratio_y = 1.0/(1.0 + ratio*ratio).sqrt();

            let mut i=0;
            let mut p1_index = 0;
            while i<self.points.len() {
                if self.points[i].id == self.lines[line_index].points.0 {
                    p1_index = i;
                    break;
                }
                i = i + 1;
            }

            if p2.0 > p1.0 {
                self.points[p1_index].x = self.points[p1_index].x - extention*ratio_x;
            } else {
                self.points[p1_index].x = self.points[p1_index].x + extention*ratio_x;
            }

            if p2.1 > p1.1 {
                self.points[p1_index].y = self.points[p1_index].y - extention*ratio_y;
            } else {
                self.points[p1_index].y = self.points[p1_index].y + extention*ratio_y;
            }
            line_index = if line_index == 0 {self.lines.len() - 1} else {line_index - 1};
        }
    }
}

pub struct Polygon {
    pub points: Vec<Point>,
    pub lines: Vec<Line>,
    pub center: PointCords
}
