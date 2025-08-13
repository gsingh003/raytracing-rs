use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub point: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn with(objects: Vec<Arc<dyn Hittable>>) -> Self {
        Self { objects }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_rec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for obj in &self.objects {
            if let Some(rec) = obj.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                hit_rec = Some(rec);
            }
        }

        hit_rec
    }
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = Vec3::dot(r.direction(), r.direction());
        let half_b = Vec3::dot(oc, r.direction());
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let sqrtd = discriminant.sqrt();

            let mut root = (-half_b - sqrtd) / a;
            if root > t_min && root < t_max {
                let p = r.at(root);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord {
                    t: root,
                    point: p,
                    normal,
                    material: Arc::clone(&self.material),
                });
            }

            root = (-half_b + sqrtd) / a;
            if root > t_min && root < t_max {
                let p = r.at(root);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord {
                    t: root,
                    point: p,
                    normal,
                    material: Arc::clone(&self.material),
                });
            }
        }

        None
    }
}
