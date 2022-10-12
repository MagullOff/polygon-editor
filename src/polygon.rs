use web_sys::{CanvasRenderingContext2d};
use crate::canvas::PressedObject;
use crate::data_models::*;
use crate::utils::*;

impl Polygon {
    pub fn get_point_by_id(&self, id: u32) -> (f64,f64){
        self.points
            .iter()
            .find(|point| point.id == id)
            .and_then(|point| Some((point.x, point.y)))
            .unwrap_or((0.0,0.0))
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

    pub fn remove_point_of_id(&mut self, id: u32) {
        let mut i = 0;
        while i<self.points.len() {
            if self.points[i].id == id {
                self.points.remove(i);
            }
            i = i+1;
        }
    }

    pub fn check_move(&self, x: f64, y: f64) -> Option<PressedObject> {
        if check_point((self.center.x, self.center.y), (x,y)) {
            return Some(PressedObject::Center);
        }

        let mut i = 0;
        while i<self.points.len() {
            if check_point((self.points[i].x, self.points[i].y), (x,y)) {
                return Some(PressedObject::Point(self.points[i].id));
            }
            i = i + 1;
        }

        i = 0;
        while i<self.lines.len() {
            let p1 = self.get_point_by_id(self.lines[i].points.0);
            let p2 = self.get_point_by_id(self.lines[i].points.1);
            if check_line(p1, p2, (x,y)) {
                return Some(PressedObject::Line((self.lines[i].id, x - p1.0)));
            }
            i = i + 1;
        }

        None
    }

    pub fn check_hl(&self,context: &CanvasRenderingContext2d, x: f64, y: f64) -> Option<()> {
        if check_point((self.center.x, self.center.y), (x,y)) {
            hl_point(context, (self.center.x,self.center.y));
            return Some(());
        }

        let mut i = 0;
        while i<self.points.len() {
            if check_point((self.points[i].x, self.points[i].y), (x,y)) {
                hl_point(context, (self.points[i].x,self.points[i].y));
                return Some(());
            }
            i = i + 1;
        }

        i = 0;
        while i<self.lines.len() {
            let l1 = self.get_point_by_id(self.lines[i].points.0);
            let l2 = self.get_point_by_id(self.lines[i].points.1);
            if check_line(l1, l2, (x,y)) {
                hl_line(context, l1, l2);
                return Some(());
            }
            i = i + 1;
        }

        None
    }

    pub fn check_split(&mut self, x: f64, y: f64, new_id: u32) -> Option<()> {
        let mut i = 0;
        while i<self.lines.len() {
            let l1 = self.get_point_by_id(self.lines[i].points.0);
            let l2 = self.get_point_by_id(self.lines[i].points.1);
            if check_line(l1, l2, (x,y)) {
                let mut j = 0;
                while j < self.points.len() {
                    if self.points[j].id == self.lines[i].points.0 {
                        break;
                    }
                    j = j + 1;
                }
                let new_point_pos = calculate_point_position(l1, l2);
                self.points.insert(j, Point { x: new_point_pos.0, y: new_point_pos.1, id: new_id });
                self.lines = get_lines(self.points.iter().collect());
                self.center = get_center(&self.points);
                return Some(());
            }
            i = i + 1;
        }
        None
    }

    pub fn check_del(&mut self, x: f64, y: f64) -> Option<bool> {
        if check_point((self.center.x, self.center.y), (x,y)) {
            return Some(true);
        }

        let mut i = 0;
        while i<self.points.len() {
            if check_point((self.points[i].x, self.points[i].y), (x,y)) {
                self.points.remove(i);
                self.lines = get_lines(self.points.iter().collect());
                self.center = get_center(&self.points);
                return Some(false);
            }
            i = i + 1;
        }

        i = 0;
        while i<self.lines.len() {
            let l1 = self.get_point_by_id(self.lines[i].points.0);
            let l2 = self.get_point_by_id(self.lines[i].points.1);
            if check_line(l1, l2, (x,y)) {
                self.remove_point_of_id(self.lines[i].points.0);
                self.remove_point_of_id(self.lines[i].points.1);
                self.lines = get_lines(self.points.iter().collect());
                self.center = get_center(&self.points);
                return Some(false);
            }
            i = i + 1;
        }

        None
    }

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
        context.fill_rect(self.center.x-10.0, self.center.y-10.0, 20.0, 20.0);
        context.stroke();
    }
}

pub struct Polygon{
    pub points: Vec<Point>,
    pub lines: Vec<Line>,
    pub center: Point
}
