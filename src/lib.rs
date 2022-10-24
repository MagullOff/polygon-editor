#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod canvas;
pub mod data_models;
pub mod polygon;
pub mod utils;
pub mod draw;
pub mod bresenham;
