use std::collections::HashSet;

use crate::shapes::Shape;




#[derive(Debug, Clone)]
pub struct Tetris {
    pub width : usize,
    pub height : usize,
    pub current_shape: Shape,
    pub fixed_shapes: Vec<Shape>,
}


impl Tetris {
    pub fn new(width: usize, height: usize) -> Tetris {
        Tetris {
            width,
            height,
            current_shape: Shape::new(),
            fixed_shapes: Vec::new(),
        }
    }
}