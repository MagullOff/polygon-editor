use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Document;
use web_sys::{CanvasRenderingContext2d};
use crate::polygon::*;
use crate::data_models::*;
use crate::utils::*;

const CANVAS_X: f64 = 100000.0;
const CANVAS_Y: f64 = 70000.0;

pub enum State{
    Edit,
    Highlight,
    Split,
    Moving((usize, PressedObject))
}

pub enum PressedObject {
    Center,
    Line((u32,f64)),
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
    fn clear(&self){
        self.context.clear_rect(0.0,0.0,CANVAS_X,CANVAS_Y);
        self.context.stroke();
        self.context.begin_path();
    }

    fn clear_current_points(&mut self) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];
        while !self.current_points.is_empty() {
            let point = self.current_points.pop().unwrap();
            points.push(point);
        }
        points
    }

    pub fn draw(&self){
        self.clear();

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
            .for_each(|Point{x,y, id: _}| {
                self.context.set_line_width(7.0);
                self.context.arc(*x, *y, 5.0, 0.0, 2.0*3.14);
                self.context.set_line_width(3.0);
                self.context.line_to(*x,*y);
                self.context.move_to(*x,*y);
            });
        self.context.stroke();
    }

    pub fn set_edit_state(&mut self){
        self.state = State::Edit;
    }

    pub fn set_highlight_state(&mut self){
        self.state = State::Highlight;
        self.clear_current_points();
        self.clear();
        self.draw();
    }

    pub fn set_split_state(&mut self){
        self.state = State::Split;
    }

    pub fn set_predefined_scene(&mut self){
        // wylosuj jedną i druga tablice punktów a pozniej przypisz co trzeba
        let mut polygon1 = Polygon {
            points: vec![],
            lines: vec![],
            center: Point { x: 0.0, y: 0.0, id: 0 }
        };

        let mut polygon2 = Polygon {
            points: vec![],
            lines: vec![],
            center: Point { x: 0.0, y: 0.0, id: 0 }
        };
        
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
            state: State::Edit,
            current_points: vec![],
            current_id: 0,
            polygons: vec![]
        }
    }

    pub fn on_down_click(&mut self, x: f64, y: f64){
        match &self.state {
            State::Highlight => {
                self.draw();
                let mut i = 0;
                while i<self.polygons.len() {
                    match self.polygons[i].check_move(x, y){
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
            State::Edit => {
                self.current_points.push(Point{x,y, id:self.current_id});
                self.current_id = self.current_id + 1;
                self.draw();
            },
            State::Split => {
                self.draw();
                let mut i = 0;
                while i<self.polygons.len() {
                    match self.polygons[i].check_split(x, y, self.current_id){
                        Some(()) => {self.current_id = self.current_id + 1; break;},
                        _ => {}
                    }
                    i = i+1;
                }
            },
            State::Moving(_) => {self.state = State::Highlight}
            _ => {},
        }
    }

    pub fn on_move_mouse(&mut self, x: f64, y: f64){
        match &self.state {
            State::Edit => {
                self.current_points.push(Point{x,y, id: 0});
                self.draw();
                self.current_points.pop();
            },
            State::Moving((id, pressed_object)) => {
                match pressed_object {
                    PressedObject::Center => {
                        let mut polygon = &mut self.polygons[*id];
                        
                        let difference_vec = (x-polygon.center.x, y-polygon.center.y);

                        polygon.points
                            .iter_mut()
                            .for_each(|point| {
                                point.x = point.x + difference_vec.0;
                                point.y = point.y + difference_vec.1;
                            });

                        polygon.center.x = x;
                        polygon.center.y = y;
                        hl_point(&self.context, (x,y));
                    },
                    PressedObject::Line((line_id, offset)) => {
                        let mut polygon = &mut self.polygons[*id];
                        let line = polygon.get_line_reference(*line_id);
                        let p1_id = line.points.0;
                        let p2_id = line.points.1;

                        let p1_val = polygon.get_point_by_id(p1_id);
                        let p2_val = polygon.get_point_by_id(p2_id);

                        let last_click_point = get_click_point(p1_val, p2_val, *offset);

                        let difference_vec = (x-last_click_point.0, y-last_click_point.1);

                        polygon.modify_point_coordinates(p1_id, difference_vec);
                        polygon.modify_point_coordinates(p2_id, difference_vec);

                        hl_line(&self.context, p1_val, p2_val);
                    },
                    PressedObject::Point(point_id) => {
                        let mut point = self.polygons[*id].get_point_reference(*point_id);
                        point.x = x;
                        point.y = y;
                        hl_point(&self.context, (x,y));
                    }
                }
                self.polygons[*id].center = get_center(&self.polygons[*id].points);
                self.draw();
            },
            _ => {
                self.draw();
                let mut i = 0;
                while i<self.polygons.len() {
                    match self.polygons[i].check_hl(&self.context,x,y){
                        Some(()) => {break;},
                        _ => {}
                    }
                    i = i+1;
                }
            },
        }
    }

    pub fn on_right_click(&mut self, x: f64, y: f64){
        match self.state {
            State::Edit => {
                let temp: Vec<&Point> = self.current_points.iter().collect();
                let lines = get_lines(temp);
                let points = self.clear_current_points();
                if points.len() >= 3 {
                    let center = get_center(&points);
                    let new_polygon = Polygon { lines, points, center};
                    self.polygons.push(new_polygon);
                }
                self.draw();
            },
            State::Highlight => {
                self.draw();
                let mut i = 0;
                while i<self.polygons.len() {
                    match self.polygons[i].check_del(x, y){
                        Some(false) => {
                            if self.polygons[i].points.len() < 3 {
                                self.polygons.remove(i);
                            }
                            break;
                        },
                        Some(true) => {
                            self.polygons.remove(i);
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
        self.draw();
    }
}
