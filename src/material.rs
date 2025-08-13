use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use rand::Rng;
use std::sync::Arc;

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        rng: &mut dyn rand::RngCore,
    ) -> Option<(Vec3, Ray)>;
}
