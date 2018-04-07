// TODO: use euclid library
pub struct Mass {
    pub coords: f64,
}

pub struct Spring<I> {
    pub endpoints: [I; 2],
}
