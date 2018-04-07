use cgmath::Vector2;

pub type Unit = f64;

pub struct Mass {
    pub pos: Vector2<Unit>,
    pub vel: Vector2<Unit>,
    pub fixed: bool,
}

pub struct Spring<I> {
    pub endpoints: [I; 2],
    pub length: Unit,
    pub stiffness: Option<Unit>,
}
