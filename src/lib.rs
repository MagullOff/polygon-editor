use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Document;
use web_sys::{CanvasRenderingContext2d};

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Clone, Copy)]
pub struct Point{
    pub x: f64,
    pub y: f64,
    pub id: i32
}

pub struct Relation<'a>{
    pub lines: Vec<&'a Line>,
    pub id: i32
}

pub struct Polygon{
    pub points: Vec<Point>,
    pub lines: Vec<Line>,
    pub center: Point
}

fn get_lines<'a>(points: Vec<&'a Point>) -> Vec<Line>{
    if points.len() < 3 {
        return vec![];
    }
    let mut lines: Vec<Line> = vec![];
    let mut last_point= points.last().unwrap();
    let mut last_point_id = last_point.id;
    points
        .iter()
        .for_each(|point| {
            lines.push(Line {
                points: (last_point_id, point.id),
                length: get_length((last_point.x, last_point.y), (point.x, point.y))
            });
            last_point = point;
            last_point_id = point.id;
        });
    lines
}

fn get_length(p1: (f64, f64), p2: (f64,f64)) -> f64{
    ((p1.0 - p2.0)*(p1.0 - p2.0)+(p1.1 - p2.1)*(p1.1 - p2.1)).sqrt()
}

fn hl_point(context: &CanvasRenderingContext2d, p: (f64,f64)){
    context.move_to(p.0, p.1);
    context.arc(p.0, p.1, 15.0, 0.0, 2.0 * 3.14);
    context.stroke();
}

fn hl_line(context: &CanvasRenderingContext2d, l1: (f64,f64), l2: (f64,f64)){
    context.begin_path();
    context.move_to(l1.0, l1.1);
    context.line_to(l2.0, l2.1);
    context.set_stroke_style(&JsValue::from_str("red"));
    context.stroke();
    context.set_stroke_style(&JsValue::from_str("black"));
    /* 
    hl_point(context, l1);
    hl_point(context, l2);
    */
}

fn check_point(p1: (f64,f64), p2: (f64,f64)) -> bool{
    ((p1.0 - p2.0)*(p1.0 - p2.0) + (p1.1 - p2.1)*(p1.1 - p2.1)) < 200.0
}

fn check_line(l1: (f64,f64), l2: (f64,f64),p: (f64,f64)) -> bool {
    if !(p.0 > l1.0.min(l2.0) && p.0 < l1.0.max(l2.0) && p.1 > l1.1.min(l2.1) && p.1 < l1.1.max(l2.1)) {
        return false;
    }
    (((l2.0 - l1.0)*(l1.1 - p.1) - (l1.0-p.0)*(l2.1-l1.1)).abs())/(((l2.0 - l1.0)*(l2.0 - l1.0) + (l2.1 - l1.1)*(l2.1 - l1.1)).sqrt()) < 10.0
}

impl Polygon {
    fn get_point_by_id(&self, id: i32) -> (f64,f64){
        let filtered_points: Vec<&Point> = self.points
            .iter()
            .filter(|point| point.id == id)
            .collect();
        
        filtered_points
            .first()
            .and_then(|point| Some((point.x, point.y)))
            .unwrap_or((0.0,0.0))
    }

    fn remove_point_of_id(&mut self, id: i32) {
        let mut i = 0;
        while i<self.points.len() {
            if self.points[i].id == id {
                self.points.remove(i);
            }
            i = i+1;
        }
    }

    fn check_hl(&self,context: &CanvasRenderingContext2d, x: f64, y: f64) -> Option<()> {
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

    fn check_del(&mut self,context: &CanvasRenderingContext2d, x: f64, y: f64) -> Option<bool> {
        if check_point((self.center.x, self.center.y), (x,y)) {
            return Some(true);
        }

        let mut i = 0;
        while i<self.points.len() {
            if check_point((self.points[i].x, self.points[i].y), (x,y)) {
                self.points.remove(i);
                self.lines = get_lines(self.points.iter().collect());
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
                return Some(false);
            }
            i = i + 1;
        }

        None
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d){
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

pub struct Line{
    pub points: (i32,i32),
    pub length: f64
}

#[wasm_bindgen]
pub struct Canvas{
   context: CanvasRenderingContext2d,
   state: State,
   current_points: Vec<Point>,
   polygons: Vec<Polygon>,
   current_id: i32,
}

pub enum State{
    Edit,
    Highlight
}

#[wasm_bindgen]
impl Canvas{
    fn clear(&self){
        self.context.clear_rect(0.0,0.0,1000.0,700.0);
        self.context.stroke();
        self.context.begin_path();
    }

    fn draw(&self){
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
        context.set_line_width(5.0);
        Canvas{
            context,
            state: State::Edit,
            current_points: vec![],
            current_id: 0,
            polygons: vec![]
        }
    }

    pub fn on_left_click(&mut self, x: f64, y: f64){
        match self.state {
            State::Edit => {
                self.current_points.push(Point{x,y, id:self.current_id});
                self.current_id = self.current_id + 1;
                self.draw();
            },
            State::Highlight => {

            }
        }
    }

    pub fn on_move_mouse(&mut self, x: f64, y: f64){
        match self.state {
            State::Edit => {
                self.current_points.push(Point{x,y, id: -1});
                self.draw();
                self.current_points.pop();
            },
            State::Highlight => {
                self.draw();
                let mut i = 0;
                while i<self.polygons.len() {
                    match self.polygons[i].check_hl(&self.context,x,y){
                        Some(()) => {break;},
                        _ => {}
                    }
                    i = i+1;
                }
            }
        }
    }

    fn clear_current_points(&mut self) -> Vec<Point> {
        let mut points: Vec<Point> = vec![];
        while !self.current_points.is_empty() {
            let point = self.current_points.pop().unwrap();
            points.push(point);
        }
        points
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
                    match self.polygons[i].check_del(&self.context ,x ,y){
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
        }
        self.draw();
    }
}

fn get_center(points: &Vec<Point>) -> Point{
    let first_point = points.first().unwrap_or(&Point{x:0.0,y:0.0,id: -1});
    let mut xmin = first_point.x;
    let mut xmax = first_point.x;
    let mut ymin = first_point.y;
    let mut ymax = first_point.y;

    points
        .into_iter()
        .for_each(|Point{x,y, id: _}| {
            xmin = xmin.min(*x);
            ymin = ymin.min(*y);
            xmax = xmax.max(*x);
            ymax = ymax.max(*y);
        });
    Point {
        x: (xmax-xmin)/2.0+xmin,
        y: (ymax-ymin)/2.0+ymin,
        id: -1
    }
}
