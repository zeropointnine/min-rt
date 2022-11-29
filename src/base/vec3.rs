use std::ops;

/// Represents a 3D point or vector.
/// Mimics openFrameworks `Vec3` class.
///
#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {

    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn new_origin() -> Vec3 {
        Vec3 { x: 0_f32, y: 0_f32, z: 0_f32 }
    }

    pub fn add(&mut self, b: &Vec3) {
        self.x += b.x;
        self.y += b.y;
        self.z += b.z;
    }

    pub fn sub(&mut self, b: &Vec3) {
        self.x -= b.x;
        self.y -= b.y;
        self.z -= b.z;
    }

    pub fn mul(&mut self, b: &Vec3) {
        self.x *= b.x;
        self.y *= b.y;
        self.z *= b.z;
    }

    pub fn normalize(&mut self) {
        let len = self.length();
        if len > 0.0 {
            self.x /= len;
            self.y /= len;
            self.z /= len;
        } else {
            self.x = 0.0;
            self. y = 0.0;
            self.z = 0.0;
        }
    }

    pub fn get_cross(&self, b: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * b.z  -  self.z * b.y,
            self.z * b.x  -  self.x * b.z,
            self.x * b.y  -  self.y * b.x
        )
    }

    pub fn get_normalized(&self) -> Vec3 {
        let len = self.length();
        if len > 0.0 {
            Vec3::new(self.x / len, self.y / len, self.z / len)
        } else {
            Vec3::new_origin()
        }
    }

    pub fn dot(&self, b: &Vec3) -> f32 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }

    pub fn length(&self) -> f32 {
        self.dot(self).sqrt()
    }

    // Untested
    pub fn get_rotated(&self, radians: f32, axis: &Vec3) -> Vec3 {
        let ax = axis.get_normalized();
        let a = radians;
        let sina = a.sin();
        let cosa = a.cos();
        let cosb = 1.0 - cosa;

        let x = self.x;
        let y = self.y;
        let z = self.z;

        let x1 = x*(ax.x*ax.x*cosb + cosa)
            + y*(ax.x*ax.y*cosb - ax.z*sina)
            + z*(ax.x*ax.z*cosb + ax.y*sina);
        let y1 = x*(ax.y*ax.x*cosb + ax.z*sina)
            + y*(ax.y*ax.y*cosb + cosa)
            + z*(ax.y*ax.z*cosb - ax.x*sina);
        let z1 = x*(ax.z*ax.x*cosb - ax.y*sina)
            + y*(ax.z*ax.y*cosb + ax.x*sina)
            + z*(ax.z*ax.z*cosb + cosa);
        Vec3::new(x1, y1, z1)
    }
}

/// Operator overloads
/// todo cover all possible permutations :/

/// &Vec3 &Vec3
impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: &Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Mul<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Vec3 {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

/// &Vec3 f32
impl ops::Mul<f32> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Vec3 {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

/// f32 &Vec
impl ops::Mul<&Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Vec3 {
        Vec3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

/// &Vec3 Vec3
impl ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

/// Vec3 &Vec3
impl ops::Sub<&Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}


/// Vec3 Vec3
impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

/// Vec3 f32
impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Vec3 {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Vec3 {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}



// Util functions

/// Given a ray that starts at `r1` and goes through `r2`,
/// get the point on the ray that is distance `t` from `r1`
pub fn get_point_on_ray(r1: &Vec3, r2: &Vec3, t: f32) -> Vec3 {
    let dir = r2 - r1;
    let ratio = t / dir.length();
    let mut dir = &dir * ratio;
    dir.add(r1);
    dir
}
