use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::Rng;

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        rng: &mut dyn rand::RngCore,
    ) -> Option<(Vec3, Ray)>;
}

#[inline]
pub fn random_in_unit_sphere(rng: &mut dyn rand::RngCore) -> Vec3 {
    loop {
        let p = Vec3::new(
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
        );
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

#[inline]
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * Vec3::dot(v, n) * n
}

#[inline]
pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = Vec3::unit_vector(v);
    let dt = Vec3::dot(uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
    } else {
        None
    }
}

#[inline]
pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    #[inline]
    fn scatter(
        &self,
        _ray_in: &Ray,
        rec: &HitRecord,
        rng: &mut dyn rand::RngCore,
    ) -> Option<(Vec3, Ray)> {
        let target = rec.point + rec.normal + random_in_unit_sphere(rng);
        let scattered = Ray::new(rec.point, target - rec.point);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzziness: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzziness < 1.0 {
                fuzziness.max(0.0)
            } else {
                1.0
            },
        }
    }
}

impl Material for Metal {
    #[inline]
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        rng: &mut dyn rand::RngCore,
    ) -> Option<(Vec3, Ray)> {
        let reflected = reflect(Vec3::unit_vector(ray_in.direction()), rec.normal);
        let scattered = Ray::new(
            rec.point,
            reflected + self.fuzz * random_in_unit_sphere(rng),
        );
        let attenuation = self.albedo;
        if Vec3::dot(scattered.direction(), rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }
}

impl Material for Dielectric {
    #[inline]
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        rng: &mut dyn rand::RngCore,
    ) -> Option<(Vec3, Ray)> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);

        let reflected = reflect(ray_in.direction(), rec.normal);

        let (outward_normal, ni_over_nt, cosine) = if Vec3::dot(ray_in.direction(), rec.normal)
            > 0.0
        {
            let outward_normal = -rec.normal;
            let ni_over_nt = self.ref_idx;
            let cosine = self.ref_idx * Vec3::dot(ray_in.direction(), rec.normal)
                / ray_in.direction().length();
            (outward_normal, ni_over_nt, cosine)
        } else {
            let outward_normal = rec.normal;
            let ni_over_nt = 1.0 / self.ref_idx;
            let cosine = -Vec3::dot(ray_in.direction(), rec.normal) / ray_in.direction().length();
            (outward_normal, ni_over_nt, cosine)
        };

        let reflect_prob = match refract(ray_in.direction(), outward_normal, ni_over_nt) {
            Some(_) => schlick(cosine, self.ref_idx).min(1.0).max(0.0),
            None => 1.0,
        };

        if rng.random::<f64>() < reflect_prob {
            Some((attenuation, Ray::new(rec.point, reflected)))
        } else {
            let refracted = refract(ray_in.direction(), outward_normal, ni_over_nt).unwrap();
            Some((attenuation, Ray::new(rec.point, refracted)))
        }
    }
}
