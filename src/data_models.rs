#[derive(Clone, Copy)]
pub struct Line{
    pub points: (u32, u32),
    pub length: f64,
    pub id:  u32,
    pub is_const: bool
}

pub struct Relation<'a>{
    pub lines: Vec<&'a Line>,
    pub id: u32
}

#[derive(Copy,Clone)]
pub struct Point{
    pub x: f64,
    pub y: f64,
    pub id: u32
}

#[derive(Clone, Copy)]
pub struct PointCords(pub f64, pub f64);