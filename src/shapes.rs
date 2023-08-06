use std::collections::HashSet;



#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Pos(pub i32, pub i32);

#[derive(Debug, Clone)]
pub struct Shape {
    positions: HashSet<Pos>
}


impl Shape {
     
     
}