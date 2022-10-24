use wasm_bindgen::prelude::*;
use crate::polygon::*;
use crate::data_models::*;
use crate::utils::*;
use crate::draw::*;
use super::{Canvas, State, PressedObject};


#[wasm_bindgen]
impl Canvas{
    pub fn set_create_state(&mut self){
        self.state = State::Create;
    }

    pub fn set_edit_state(&mut self){
        self.state = State::Edit;
        self.clear_current_points();
        clear_canvas(&self.context);
        self.draw();
    }

    pub fn set_rules_state(&mut self){
        self.state = State::Rules(None);
    }

    pub fn set_predefined_scene(&mut self){
        clear_canvas(&self.context);
        let points1 = vec![
            Point {
                x: 330.0,
                y: 220.0,
                id: 1
            },
            Point {
                x: 200.0,
                y: 50.0,
                id: 2
            },
            Point {
                x: 160.0,
                y: 350.0,
                id: 3
            },
        ];

        let lines1 = calcualate_new_lines(points1.iter().collect());
        let center1 = get_centroid(&points1);

        let polygon1 = Polygon {
            points: points1,
            lines: lines1,
            center: center1 
        };

        let points2 = vec![
            Point {
                x: 80.0,
                y: 70.0,
                id: 4
            },
            Point {
                x: 250.0,
                y: 90.0,
                id: 5
            },
            Point {
                x: 220.0,
                y: 300.0,
                id: 6
            },
            Point {
                x: 50.0,
                y: 150.0,
                id: 7
            },
        ];

        let lines2 = calcualate_new_lines(points2.iter().collect());
        let center2 = get_centroid(&points2);

        let polygon2 = Polygon {
            points: points2,
            lines: lines2,
            center: center2 
        };

        self.polygons = vec![polygon1, polygon2];
        self.current_points = vec![];
        
        self.draw();
    }
    
    pub fn set_line_length(&mut self){
        match self.state {
            State::Rules(Some((polygon_id, line_id))) => {
                let mut line = self.polygons[polygon_id].get_line_reference(line_id);
                let new_length = self.length_selector.value_as_number();
                let extention = (new_length - line.length)/2.0;
                line.length = new_length;
                let (p1_id, p2_id) = line.points;
                self.correct_line_mid(extention, line_id, polygon_id);
                self.correct_line_length(p2_id, polygon_id, true);
                self.correct_line_length(p1_id, polygon_id, false);
                self.polygons[polygon_id].recalculate();
                self.draw();
            },
            _ => {}
        }
    }

    pub fn set_const_state(&mut self){
        match self.state {
            State::Rules(Some((polygon_id, line_id))) => {
                let mut line = self.polygons[polygon_id].get_line_reference(line_id);
                line.is_const = self.is_const.checked();
            },
            _ => {}
        }
    }

    pub fn on_down_click(&mut self, x: f64, y: f64){
        match &self.state {
            State::Edit => {
                self.draw();
                let mut i = 0;
                while i<self.polygons.len() {
                    match self.polygons[i].check_hover(x, y){
                        Some(pressed_object) => {self.state = State::Moving((i, pressed_object)); break;},
                        _ => {}
                    }
                    i = i+1;
                }
            },
            _ => {}
        }
    }

    pub fn on_left_click(&mut self, x: f64, y: f64){
        match self.state {
            State::Create => {
                self.current_points.push(Point{x,y, id:self.current_id});
                self.current_id = self.current_id + 1;
                clear_canvas(&self.context);
                self.draw();
            },
            State::Moving(_) => {self.state = State::Edit},
            State::Rules(_) => {
                self.draw();
                use web_sys::console;
                let mut i = 0;
                while i<self.polygons.len() {
                    match self.polygons[i].check_hover(x, y){
                        Some(PressedObject::Line(id,_)) => {
                            self.state = State::Rules(Some((i, id)));
                            console::log_2(
                                &JsValue::from_f64(i as f64),
                                &JsValue::from_f64(id as f64)
                            );
                            let (x_id,y_id) = self.polygons[i].get_line_by_id(id);
                            let x = self.polygons[i].get_point_by_id(x_id);
                            let y = self.polygons[i].get_point_by_id(y_id);
                            highlight_line(&self.context, x, y);
                            let line = self.polygons[i].get_line_reference(id);
                            self.length_selector.set_value(format!("{:.2}", line.length).as_str());
                            self.is_const.set_checked(line.is_const);
                            break;
                        },
                        _ => {}
                    }
                    i = i+1;
                }
            },
            _ => {},
        }
    }

    pub fn on_move_mouse(&mut self, x: f64, y: f64){
        match &self.state {
            State::Create => {
                self.current_points.push(Point{x,y, id: 0});
                self.draw();
                self.current_points.pop();
            },
            State::Moving((id, pressed_object)) => {
                match pressed_object {
                    PressedObject::Center => {
                        let mut polygon = &mut self.polygons[*id];
                        
                        let difference_vec = (x-polygon.center.0, y-polygon.center.1);

                        polygon.points
                            .iter_mut()
                            .for_each(|point| {
                                point.x = point.x + difference_vec.0;
                                point.y = point.y + difference_vec.1;
                            });

                        polygon.center.0 = x;
                        polygon.center.1 = y;

                        self.polygons[*id].center = get_centroid(&self.polygons[*id].points);
                        self.draw();
                        highlight_point(&self.context, PointCords(x,y));
                    },
                    PressedObject::Line(line_id, offset) => {
                        let (p1_id, p2_id)= self.polygons[*id].get_line_by_id(*line_id);
                        let p_id = *id;
                        let p1_val = self.polygons[*id].get_point_by_id(p1_id);
                        let p2_val = self.polygons[*id].get_point_by_id(p2_id);

                        let last_click_point = get_click_point(p1_val, p2_val, *offset);

                        let difference_vec = (x-last_click_point.0, y-last_click_point.1);

                        self.polygons[*id].modify_point_coordinates(p1_id, difference_vec);
                        self.polygons[*id].modify_point_coordinates(p2_id, difference_vec);
                        highlight_line(&self.context, p1_val, p2_val);
                        self.correct_line_length(p1_id, p_id, false);
                        self.correct_line_length(p2_id, p_id, true);
                        self.polygons[p_id].recalculate();
                        self.draw();
                    },
                    PressedObject::Point(point_id) => {
                        let mut point = self.polygons[*id].get_point_reference(*point_id);
                        let px_id = *point_id;
                        let p_id = *id;
                        point.x = x;
                        point.y = y;
                        self.correct_line_length(px_id, p_id, false);
                        self.correct_line_length(px_id, p_id, true);
                        self.polygons[p_id].recalculate();
                        self.draw();
                        highlight_point(&self.context, PointCords(x,y));
                    }
                }
            },
            State::Rules(Some((polygon_id, line_id))) => {
                self.draw();
                let mut i = 0;
                while i<self.polygons.len() {
                    match self.polygons[i].check_hover(x,y){
                        Some(PressedObject::Line(id, _)) =>{
                            let (p1_id, p2_id) = self.polygons[i].get_line_by_id(id);
                            let p1 = self.polygons[i].get_point_by_id(p1_id);
                            let p2 = self.polygons[i].get_point_by_id(p2_id);
                            highlight_line(&self.context, p1, p2);
                            break;
                        },
                        _ => {}
                    }
                    i = i+1;
                }

                let (x_id,y_id) = self.polygons[*polygon_id].get_line_by_id(*line_id);
                let x = self.polygons[*polygon_id].get_point_by_id(x_id);
                let y = self.polygons[*polygon_id].get_point_by_id(y_id);
                highlight_line(&self.context, x, y);
            },
            _ => {
                self.draw();
                let mut i = 0;
                while i<self.polygons.len() {
                    match self.polygons[i].check_hover(x,y){
                        Some(PressedObject::Center) => {highlight_point(&self.context, self.polygons[i].center); break;},
                        Some(PressedObject::Line(id, _)) =>{
                            let (p1_id, p2_id) = self.polygons[i].get_line_by_id(id);
                            let p1 = self.polygons[i].get_point_by_id(p1_id);
                            let p2 = self.polygons[i].get_point_by_id(p2_id);
                            highlight_line(&self.context, p1, p2);
                            break;
                        },
                        Some(PressedObject::Point(id)) => {
                            let hovered_point_cords = self.polygons[i].get_point_by_id(id);
                            highlight_point(&self.context, hovered_point_cords);
                            break;
                        }
                        None => {}
                    }
                    i = i+1;
                }
            },
        }
    }

    pub fn on_right_click(&mut self, x: f64, y: f64){
        match self.state {
            State::Create => {
                let points = self.clear_current_points();
                let lines = calcualate_new_lines(points.iter().collect());
                if points.len() >= 3 {
                    let center = get_centroid(&points);
                    let new_polygon = Polygon { lines, points, center};
                    self.polygons.push(new_polygon);
                }
                clear_canvas(&self.context);
                self.draw();
            },
            State::Edit => {
                self.draw();
                let mut i = 0;
                while i<self.polygons.len() {
                    match self.polygons[i].check_hover(x, y){
                        Some(PressedObject::Line(id,_)) => {
                            let current_polygon = &mut self.polygons[i];
                            let (p1_id, p2_id) = current_polygon.get_line_by_id(id);
                            let p1 = current_polygon.get_point_by_id(p1_id);
                            let p2 = current_polygon.get_point_by_id(p2_id);
                            let mut j = 0;
                            while j < current_polygon.points.len() {
                                if current_polygon.points[j].id == p2_id {
                                    break;
                                }
                                j = j + 1;
                            }
                            let new_point_pos = calculate_middle_point(p1, p2);
                            current_polygon.points.insert(j, Point { x: new_point_pos.0, y: new_point_pos.1, id: self.current_id});

                            j = 0;
                            while j < current_polygon.lines.len() {
                                if current_polygon.lines[j].id == id {
                                    break;
                                }
                                j = j + 1;
                            }
                            let (l1, l2) = get_new_split_lines(current_polygon, p1_id, p2_id, self.current_id);
                            current_polygon.lines[j] = l1;
                            current_polygon.lines.insert(j+1, l2);
                            current_polygon.center = get_centroid(&current_polygon.points);
                            self.current_id = self.current_id + 1;
                            self.draw();
                            break;
                        },
                        Some(PressedObject::Point(id)) => {
                            self.polygons[i].remove_point_of_id(id);
                            if self.polygons[i].points.len() < 3 {
                                self.polygons.remove(i);
                            }
                            self.draw();
                            break;
                        }
                        Some(PressedObject::Center) => {
                            self.polygons.remove(i);
                            self.draw();
                            break;
                        },
                        _ => {}
                    }
                    i = i+1;
                }
                self.draw();
            },
            State::Rules(Some((old_polygon_id, old_line_id))) => {
                self.draw();
                let mut i = 0;
                while i<self.polygons.len() {
                    match self.polygons[i].check_hover(x, y){
                        Some(PressedObject::Line(new_line_id,_)) => {
                            if new_line_id == old_line_id {return;}
                            let new_relation = self.polygons[i].get_line_relation(new_line_id);
                            let old_relation = self.polygons[old_polygon_id].get_line_relation(old_line_id);
                            match (new_relation, old_relation) {
                                (None, None) => {
                                    self.polygons[old_polygon_id].set_relation(old_line_id, new_line_id);
                                    self.polygons[i].set_relation(new_line_id, old_line_id);
                                },
                                _ => {}
                            };
                            break;
                        },
                        _ => {}
                    }
                    i = i+1;
                }
                self.draw();
            }
            _ => {}
        }
    }
}
