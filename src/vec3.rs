use std::ops::{Add, AddAssign, DivAssign, MulAssign};

struct Vec3([f64; 3]);

impl Vec3 {
    pub fn from_array(a: [f64; 3]) -> Self {
        Self(a)
    }

    pub fn from_coords(x: f64, y: f64, z: f64) -> Self {
        Self([x, y, z])
    }

    pub fn x(&self) -> f64 {
        self.0[0]
    }

    pub fn y(&self) -> f64 {
        self.0[1]
    }

    pub fn z(&self) -> f64 {
        self.0[2]
    }

    pub fn norm2(&self) -> f64 {
        f64::sqrt(self.norm2_sq())
    }

    pub fn norm2_sq(&self) -> f64 {
        self.0[0].powf(2.) + self.0[1].powf(2.) + self.0[2].powf(2.)
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, other: Vec3) -> Self {
        Self::from_coords(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.0[0] += rhs.0[0];
        self.0[1] += rhs.0[1];
        self.0[2] += rhs.0[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0[0] *= rhs;
        self.0[1] *= rhs;
        self.0[2] *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.0[0] /= rhs;
        self.0[1] /= rhs;
        self.0[2] /= rhs;
    }
}
