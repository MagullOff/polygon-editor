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

    pub fn get_line_relation(&self, line_id: u32) -> Option<u32> {
        self.lines
            .iter()
            .find(|line| line.id == line_id)
            .unwrap()
            .relation
    }

    pub fn set_relation(&mut self, line_id: u32, related_line_id: u32){
        let mut i = 0;
        while i < self.lines.len() {
            if self.lines[i].id == line_id {
                self.lines[i].relation = Some(related_line_id);
                break;
            }
            i = i + 1;
        }
    }
}

pub struct Polygon {
    pub points: Vec<Point>,
    pub lines: Vec<Line>,
    pub center: PointCords
}
