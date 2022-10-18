use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Document;
use web_sys::{CanvasRenderingContext2d};
use crate::polygon::*;
use crate::data_models::*;
use crate::utils::*;
use crate::draw::*;

pub enum State{
    Create,
    Edit,
    Rules,
    Moving((usize, PressedObject))
}

pub enum PressedObject {
    Center,
    Line(u32,f64),
    Point(u32)
}

#[wasm_bindgen]
pub struct Canvas{
   context: CanvasRenderingContext2d,
   state: State,
   current_points: Vec<Point>,
   polygons: Vec<Polygon>,
   current_id: u32,
}

#[wasm_bindgen]
impl Canvas{
    fn clear_current_points(&mut self) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];
        while !self.current_points.is_empty() {
            let point = self.current_points.pop().unwrap();
            points.push(point);
        }
        points
    }

    pub fn draw(&self){
        clear_canvas(&self.context);

        self.polygons
            .iter()
            .for_each(|polygon| polygon.draw(&self.context));

        self.current_points
            .first()
            .and_then(|point| {
                self.context.move_to(point.x,point.y);
                Some(point)
            });

        self.current_points
            .iter()
            .for_each(|Point{x,y, id}| {
                if *id != 0 {
                    self.context.set_line_width(4.0);
                    self.context.arc(*x, *y, 5.0, 0.0, 2.0*3.14);
                    self.context.fill();
                }
                self.context.set_line_width(3.0);
                self.context.line_to(*x,*y);
                self.context.move_to(*x,*y);
            });
            self.context.stroke();
    }

    pub fn draw_bresenham(&self){
        clear_canvas(&self.context);

        self.polygons
            .iter()
            .for_each(|polygon| polygon.draw_bresenham(&self.context));
    }

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
        self.state = State::Rules;
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
    
    pub fn new(document: Document) -> Canvas{
        let canvas_ref = document.get_element_by_id("board").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas_ref
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        context.set_line_width(3.0);
        Canvas{
            context,
            state: State::Create,
            current_points: vec![],
            current_id: 1,
            polygons: vec![]
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
            State::Rules => {},
            State::Moving(_) => {self.state = State::Edit},
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
                        let polygon = &mut self.polygons[*id];
                        let line = polygon.get_line_reference(*line_id);
                        let p1_id = line.points.0;
                        let p2_id = line.points.1;

                        let p1_val = polygon.get_point_by_id(p1_id);
                        let p2_val = polygon.get_point_by_id(p2_id);

                        let last_click_point = get_click_point(p1_val, p2_val, *offset);

                        let difference_vec = (x-last_click_point.0, y-last_click_point.1);

                        polygon.modify_point_coordinates(p1_id, difference_vec);
                        polygon.modify_point_coordinates(p2_id, difference_vec);

                        self.polygons[*id].center = get_centroid(&self.polygons[*id].points);
                        self.draw();
                        highlight_line(&self.context, p1_val, p2_val);
                    },
                    PressedObject::Point(point_id) => {
                        let mut point = self.polygons[*id].get_point_reference(*point_id);
                        point.x = x;
                        point.y = y;
                        self.polygons[*id].center = get_centroid(&self.polygons[*id].points);
                        self.draw();
                        highlight_point(&self.context, PointCords(x,y));
                    }
                }
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
                            current_polygon.lines = calcualate_new_lines(current_polygon.points.iter().collect());
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
            }
            _ => {}
        }
    }
}
