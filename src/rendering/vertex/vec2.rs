use std::ops;

#[repr(C)]
#[derive(
    Clone, Copy, PartialEq, Debug, Default,
    // allow bitwise casts with bytemuck
    bytemuck::Zeroable, bytemuck::Pod,
)]
/// 2D vector with `x` and `y` in \[-1, 1\]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// apply `f` to both fields
    pub fn map<F: Fn(f32) -> f32>(self, f: F) -> Self {
        Self::new(f(self.x), f(self.y))
    }

    pub fn len(self) -> f32 {
        self.x.hypot(self.y)
    }

    pub fn set_len(self, len: f32) -> Self {
        let current_len= self.len();

        #[allow(clippy::float_cmp)]
        if current_len == len {
            return self;
        }

        let normalized = self.map(|x| x / current_len);

        normalized * len
    }

    /// clockwise/+90° orthogonal vector of self with same length
    pub const fn clockwise_orthogonal(self) -> Self {
        Self::new(self.y, -self.x)
    }

    /// clockwise, angle in radians
    pub fn rotate_cw(self, angle: f32) -> Self {
        Self::new(
              self.x .mul_add(angle.cos(), self.y * angle.sin()),
            (-self.x).mul_add(angle.sin(), self.y * angle.cos()),
        )
    }

    /// counterclockwise, angle in radians
    pub fn rotate_ccw(self, angle: f32) -> Self {
        Self::new(
            self.x.mul_add(angle.cos(), -(self.y * angle.sin())),
            self.x.mul_add(angle.sin(),   self.y * angle.cos() ),
        )
    }
}

impl ops::Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Add<f32> for Vec2 {
    type Output = Self;
    fn add(self, rhs: f32) -> Self::Output {
        self.map(|x| x + rhs)
    }
}

impl ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl ops::Sub<f32> for Vec2 {
    type Output = Self;
    fn sub(self, rhs: f32) -> Self::Output {
        self.map(|x| x - rhs)
    }
}

impl ops::Mul for Vec2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl ops::Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        self.map(|x| x * rhs)
    }
}

impl ops::Div for Vec2 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl ops::Div<f32> for Vec2 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        self.map(|x| x / rhs)
    }
}

impl ops::Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.map(|x| -x)
    }
}
