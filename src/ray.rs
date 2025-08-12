use crate::vec3::{Point3, Vec3};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    #[inline]
    pub const fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            orig: origin,
            dir: direction,
        }
    }

    #[inline]
    pub const fn origin(self) -> Point3 {
        self.orig
    }

    #[inline]
    pub const fn direction(self) -> Vec3 {
        self.dir
    }

    #[inline]
    pub fn at(self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
