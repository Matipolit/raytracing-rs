use vector3d::Vector3d;
use std::ops;

impl ops::Mul<f64> for Vector3d<f64> {
    type Output = Vector3d<f64>;
    
    fn mul(self, num: f64) -> Vector3d<f64> {
        Vector3d{x: self.x * num, y: self.y * num, z: self.z * num }
    }
}

struct Ray {
    origin: Vector3d<f64>,
    direction: Vector3d<f64>,
}

impl Ray {
    fn at(&self, t: f64) -> f64 {
        self.origin + t*self.direction
    }
}
