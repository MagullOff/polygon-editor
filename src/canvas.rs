use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Document;
use web_sys::HtmlInputElement;
use web_sys::CanvasRenderingContext2d;
use crate::polygon::*;
use crate::data_models::*;
use crate::utils::*;
use crate::draw::*;
use crate::relations::*;

pub enum State{
    Create,
    Edit,
    Rules(Option<(usize, u32)>),
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
   relations: Vec<Relation>,
   current_id: u32,
   length_selector: HtmlInputElement,
   is_const: HtmlInputElement
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
                    self.context.arc(*x, *y, 5.0, 0.0, 2.0*3.14).unwrap();
                    self.context.fill();
                }
                self.context.set_line_width(3.0);
                self.context.line_to(*x,*y);
                self.context.move_to(*x,*y);
            });
            self.context.set_stroke_style(&JsValue::from_str(BASIC_COLOR));
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
                self.correct_line_length_right(p2_id, polygon_id);
                self.correct_line_length_left(p1_id, polygon_id);
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

    pub fn new(document: Document) -> Canvas{
        let num_field_ref = document.get_element_by_id("LengthSelector").unwrap();
        let num_field: web_sys::HtmlInputElement = num_field_ref
            .dyn_into::<web_sys::HtmlInputElement>()
            .map_err(|_| ())
            .unwrap();

        let is_const_ref = document.get_element_by_id("IsConst").unwrap();
        let is_const: web_sys::HtmlInputElement = is_const_ref
            .dyn_into::<web_sys::HtmlInputElement>()
            .map_err(|_| ())
            .unwrap();

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
        context.set_font("30px serif");
        Canvas{
            context,
            state: State::Create,
            current_points: vec![],
            current_id: 1,
            polygons: vec![],
            relations: vec![],
            is_const,
            length_selector: num_field
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
                        self.correct_line_length_left(p1_id, p_id);
                        self.correct_line_length_right(p2_id, p_id);
                        self.polygons[p_id].recalculate();
                        self.draw();
                    },
                    PressedObject::Point(point_id) => {
                        let mut point = self.polygons[*id].get_point_reference(*point_id);
                        let px_id = *point_id;
                        let p_id = *id;
                        point.x = x;
                        point.y = y;
                        self.correct_line_length_left(px_id, p_id);
                        self.correct_line_length_right(px_id, p_id);
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
            State::Rules(Some((_, old_line_id))) => {
                self.draw();
                let mut i = 0;
                while i<self.polygons.len() {
                    match self.polygons[i].check_hover(x, y){
                        Some(PressedObject::Line(new_line_id,_)) => {
                            let is_old_in_relation = Canvas::is_line_in_relation(&self.relations, old_line_id);
                            let is_new_in_relation = Canvas::is_line_in_relation(&self.relations, new_line_id);

                            match (is_old_in_relation, is_new_in_relation) {
                                (None, None) => {
                                    self.create_new_relation(new_line_id, old_line_id);
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

    fn is_line_in_relation(relations: &Vec<Relation>, line_id: u32) -> Option<u32> {
        relations
            .iter()
            .find_map(|relation| {
                if relation.lines.0 == line_id || relation.lines.1 == line_id {
                    Some(relation.id)
                } else {
                    None
                }
            })
            .map(|id| id)
    }


    fn create_new_relation(&mut self, l1: u32, l2: u32) -> u32 {
        let new_relation_id = (self.relations
            .iter()
            .map(|relation| relation.id)
            .min()
            .unwrap_or(0)) + 1;

        self.relations.push(Relation { lines: (l1, l2), id: new_relation_id });

        new_relation_id
    }

    pub fn print_relations(&self){
        use web_sys::console;

        self.relations
            .iter()
            .for_each(|rel| {
                console::log_2(&JsValue::from_str("relation"), &JsValue::from_f64(rel.id as f64));
                console::log_2(&JsValue::from_f64(rel.lines.0 as f64), &JsValue::from_f64(rel.lines.1 as f64));
            });
    }

    pub fn is_relation_ok(&self, relation_id: u32) -> bool {
        let (l1, l2) = self.relations
            .iter()
            .find_map(|relation| {
                if relation.id == relation_id {
                    Some(relation.lines)
                } else {None}}).unwrap();

        let l1_cords = self.get_line_by_id(l1);
        let l2_cords = self.get_line_by_id(l2);

        check_if_parallel(l1_cords, l2_cords)
    }

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

    pub fn enforce_relation(&mut self, relation_id: u32, line_id: u32) {
        for(let mut i = 0; i < self.relations.len(); i++ {
            if self.relations[i].id == relation_id {
                let change_line_id = if self.relations[i].lines.0 == line_id {self.relations[i].lines.1} else {self.relations[i].lines.0};
                let mut j = 0;
                while j < self.polygons.len() {
                    let mut k = 0;
                    while k < self.polygons[j].lines.len() {
                        if self.polygons[j].lines[k].id == change_line_id {
                            let (p1, p2) = self.get_line_by_id(line_id);
                            let (p3_id, p4_id) = self.polygons[j].lines[k].points;
                            let p3 = self.polygons[j].get_point_by_id(p3_id);
                            let px = (self.polygons[j].lines[k].length * (p1.0 - p2.0))/get_line_length(p1,p2) + p3.0;
                            let py = (if p1.1 > p2.1 {1.0} else {-1.0})*(self.polygons[j].lines[k].length.powi(2) - (px - p3.0).powi(2)).sqrt() + p3.1;

                            let mut l = 0;
                            while l < self.polygons[j].points.len() {
                                if self.polygons[j].points[l].id == p4_id {
                                    self.polygons[j].points[l] = Point{id: p4_id, x: px, y: py};
                                }
                                l = l + 1;
                            }
                        }
                        k = k + 1;
                    }
                    j = j + 1;
                }
            }
            i = i + 1;
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

    pub fn correct_line_length_right(&mut self, point_id: u32, polygon_id: usize){
        let mut point_index = 0;
        while self.polygons[polygon_id].points[point_index].id != point_id {point_index = point_index + 1;}

        let mut line_index = 0;
        while self.polygons[polygon_id].lines[line_index].points.0 != point_id {line_index = line_index + 1;}

        let mut x = false;
        while self.polygons[polygon_id].lines[line_index].is_const == true 
        && ((self.polygons[polygon_id].lines[line_index].points.0 != point_id && x == true) || x == false){
            x = true;
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
                if self.polygons[polygon_id].points[i].id == self.polygons[polygon_id].lines[line_index].points.1 {
                    p2_index = i;
                    break;
                }
                i = i + 1;
            }

            if p2.0 > p1.0 {
                self.polygons[polygon_id].points[p2_index].x = self.polygons[polygon_id].points[p2_index].x + extention*ratio_x;
            } else {
                self.polygons[polygon_id].points[p2_index].x = self.polygons[polygon_id].points[p2_index].x - extention*ratio_x;
            }

            if p2.1 > p1.1 {
                self.polygons[polygon_id].points[p2_index].y = self.polygons[polygon_id].points[p2_index].y + extention*ratio_y;
            } else {
                self.polygons[polygon_id].points[p2_index].y = self.polygons[polygon_id].points[p2_index].y - extention*ratio_y;
            }

            match Canvas::is_line_in_relation(&self.relations, self.polygons[polygon_id].lines[line_index].id){
                Some(relation_id) => {
                    self.enforce_relation(relation_id, self.polygons[polygon_id].lines[line_index].id);
                },
                None => {}
            };
            line_index = if line_index == self.polygons[polygon_id].lines.len() - 1 {0} else {line_index + 1};
        }
    }

    pub fn correct_line_length_left(&mut self, point_id: u32, polygon_id: usize){
        let mut point_index = 0;
        let polygon = &mut self.polygons[polygon_id];
        while polygon.points[point_index].id != point_id {point_index = point_index + 1;}

        let mut line_index = 0;
        while polygon.lines[line_index].points.1 != point_id {line_index = line_index + 1;}


        let mut x = false;
        while polygon.lines[line_index].is_const == true 
        && ((polygon.lines[line_index].points.1 != point_id && x == true) || x == false){
            x = true;
            let p1 = polygon.get_point_by_id(polygon.lines[line_index].points.0);
            let p2 = polygon.get_point_by_id(polygon.lines[line_index].points.1);
            let current_len = get_line_length(p1, p2);
            let len = polygon.lines[line_index].length;
            let extention = len - current_len;

            let ratio = (p2.0 - p1.0).abs()/(p2.1 - p1.1).abs();
            let ratio_x = 1.0/(1.0 + 1.0/(ratio*ratio)).sqrt();
            let ratio_y = 1.0/(1.0 + ratio*ratio).sqrt();

            let mut i=0;
            let mut p1_index = 0;
            while i<polygon.points.len() {
                if polygon.points[i].id == polygon.lines[line_index].points.0 {
                    p1_index = i;
                    break;
                }
                i = i + 1;
            }

            if p2.0 > p1.0 {
                polygon.points[p1_index].x = polygon.points[p1_index].x - extention*ratio_x;
            } else {
                polygon.points[p1_index].x = polygon.points[p1_index].x + extention*ratio_x;
            }

            if p2.1 > p1.1 {
                polygon.points[p1_index].y = polygon.points[p1_index].y - extention*ratio_y;
            } else {
                polygon.points[p1_index].y = polygon.points[p1_index].y + extention*ratio_y;
            }
            line_index = if line_index == 0 {polygon.lines.len() - 1} else {line_index - 1};
        }
    }
}
