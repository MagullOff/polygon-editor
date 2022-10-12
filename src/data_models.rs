pub struct Line{
    pub points: (u32, u32),
    pub length: f64,
    pub id:  u32
}

pub struct Relation<'a>{
    pub lines: Vec<&'a Line>,
    pub id: u32
}

pub struct Point{
    pub x: f64,
    pub y: f64,
    pub id: u32
}
