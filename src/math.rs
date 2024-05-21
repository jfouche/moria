use bevy::{
    math::{primitives::Direction3d, Vec3},
    transform::components::Transform,
};
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct HorizontalVec(f32, f32);

impl HorizontalVec {
    pub fn from_angle(angle: f32) -> Self {
        let (sin, cos) = angle.sin_cos();
        HorizontalVec(sin, cos)
    }

    pub fn signed_angle_with(&self, rhs: HorizontalVec) -> f32 {
        rhs.0.atan2(rhs.1) - self.0.atan2(self.1)
    }
}

impl From<Vec3> for HorizontalVec {
    fn from(v: Vec3) -> Self {
        HorizontalVec(v.x, v.z)
    }
}

impl From<Direction3d> for HorizontalVec {
    fn from(v: Direction3d) -> Self {
        HorizontalVec(v.x, v.z)
    }
}

impl From<HorizontalVec> for Vec3 {
    fn from(v: HorizontalVec) -> Self {
        Vec3 {
            x: v.0,
            y: 0.0,
            z: v.1,
        }
    }
}

/// Trait to transform an object to a horizontal vec (no "y" value)
pub trait IntoHorizontalVec {
    fn horizontal(self) -> HorizontalVec;
}

/// Generic implementation of IntoHorizontalVec to use the core [From] trait
impl<T> IntoHorizontalVec for T
where
    T: Into<HorizontalVec>,
{
    fn horizontal(self) -> HorizontalVec {
        self.into()
    }
}

pub trait SignedAngle<T> {
    /// get the signed angle between `self` and `rhs`
    fn signed_angle_with(&self, rhs: T) -> f32;
}

impl SignedAngle<Transform> for Transform {
    fn signed_angle_with(&self, rhs: Transform) -> f32 {
        let self_direction = self.forward().horizontal();
        let direction_to_rhs: HorizontalVec = (rhs.translation - self.translation).horizontal();
        self_direction.signed_angle_with(direction_to_rhs)
    }
}

impl SignedAngle<HorizontalVec> for Transform {
    fn signed_angle_with(&self, rhs: HorizontalVec) -> f32 {
        let self_direction = self.forward().horizontal();
        self_direction.signed_angle_with(rhs)
    }
}

pub trait Angle {
    /// return the number in [0; 2*PI [
    fn angle(&self) -> Self;
}

impl Angle for f32 {
    fn angle(&self) -> Self {
        let mut angle = *self;
        while angle < 0.0 {
            angle += 2.0 * PI;
        }
        while angle >= 2.0 * PI {
            angle -= 2.0 * PI
        }
        angle
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

    const X: HorizontalVec = HorizontalVec(1.0, 0.0);
    const NEG_X: HorizontalVec = HorizontalVec(-1.0, 0.0);
    const Y: HorizontalVec = HorizontalVec(0.0, 1.0);

    fn eq_f32(x: f32, y: f32) -> bool {
        (x - y).abs() < f32::EPSILON
    }

    fn eq_angle(x: f32, y: f32) -> bool {
        eq_f32(x, y) || eq_f32(x, y + 2.0 * PI) || eq_f32(x, y - 2.0 * PI)
    }

    macro_rules! assert_eq_angle {
        ($x:expr, $y:expr) => {
            assert!(eq_angle($x, $y), "{} != {}", $x, $y);
        };
    }

    #[test]
    fn test_horizontal_vec() {
        assert_eq_angle!(X.signed_angle_with(X), 0.0);
        assert_eq_angle!(X.signed_angle_with(NEG_X), PI);
        assert_eq_angle!(X.signed_angle_with(Y), -FRAC_PI_2);
        assert_eq_angle!(X.signed_angle_with(HorizontalVec(2.0, -2.0)), FRAC_PI_4);
    }

    #[test]
    fn test_horizontal_trait() {
        let x_dir = Direction3d::new(Vec3::X).unwrap();
        let x_dir = x_dir.horizontal();
        assert!(eq_f32(x_dir.0, Vec3::X.x));
        assert!(eq_f32(x_dir.1, Vec3::X.z));
    }
}
