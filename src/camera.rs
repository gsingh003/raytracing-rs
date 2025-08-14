use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use rand::Rng;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        vertical_fov_degrees: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vertical_fov_degrees.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;

        let origin = look_from;
        let w = Vec3::unit_vector(look_from - look_at);
        let u = Vec3::unit_vector(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);

        let lower_left_corner =
            origin - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w;
        let horizontal = 2.0 * half_width * focus_dist * u;
        let vertical = 2.0 * half_height * focus_dist * v;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius: aperture * 0.5,
        }
    }
}
