use wasm_bindgen::prelude::wasm_bindgen;

use crate::{draw::clear_canvas, data_models::Point, utils::{calcualate_new_lines, get_centroid}, polygon::Polygon};

use super::Canvas;

#[wasm_bindgen]
impl Canvas {
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

        let mut lines1 = calcualate_new_lines(points1.iter().collect());
        let center1 = get_centroid(&points1);

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

        let mut lines2 = calcualate_new_lines(points2.iter().collect());
        let center2 = get_centroid(&points2);

        lines2[1].is_const = true;
        lines2[0].relation = Some(lines1[1].id);
        lines1[1].relation = Some(lines2[0].id);

        let polygon1 = Polygon {
            points: points1,
            lines: lines1,
            center: center1 
        };

        let polygon2 = Polygon {
            points: points2,
            lines: lines2,
            center: center2 
        };

        self.polygons = vec![polygon1, polygon2];
        self.current_points = vec![];
        self.recalculate();
        self.draw();
    }
}