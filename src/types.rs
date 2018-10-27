use cgmath::Vector2;

pub type Unit = f64;

pub struct Mass {
    pub pos: Vector2<Unit>,
    pub vel: Vector2<Unit>,
    pub acc: Vector2<Unit>,
    pub mass: Option<Unit>,
    pub fixed: bool,
}

pub struct Spring<I> {
    pub endpoints: [I; 2],
    pub length: Unit,
    pub stiffness: Option<Unit>,
}

pub struct Environment {
    pub friction: Unit,
    pub gravity: Vector2<Unit>,
    pub springiness: Unit,
    pub width: Unit,
    pub height: Unit,
}

