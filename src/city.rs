#[derive(Copy, Clone, Debug)]
pub struct City {
    pub x: i32,
    pub y: i32,
}

impl Default for City {
    #[inline]
    fn default() -> City {
        City { x: -1, y: -1 }
    }
}
