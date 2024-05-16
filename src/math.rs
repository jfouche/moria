use bevy::{
    math::{Vec2, Vec3Swizzles},
    transform::components::Transform,
};

pub trait SignedAngle {
    /// get the signed angle between `self` and `rhs`
    fn signed_angle_with(&self, rhs: Self) -> f32;
}

impl SignedAngle for Vec2 {
    fn signed_angle_with(&self, rhs: Self) -> f32 {
        rhs.x.atan2(rhs.y) - self.x.atan2(self.y)
    }
}

impl SignedAngle for Transform {
    fn signed_angle_with(&self, rhs: Self) -> f32 {
        let self_direction = *self.forward();
        let direction_to_rhs = rhs.translation - self.translation;
        self_direction.xz().signed_angle_with(direction_to_rhs.xz())
    }
}

#[cfg(test)]
mod test {
    use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};

    use super::*;

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
    fn test_angle() {
        assert_eq_angle!(Vec2::X.signed_angle_with(Vec2::X), 0.0);
        assert_eq_angle!(Vec2::X.signed_angle_with(Vec2::NEG_X), PI);
        assert_eq_angle!(Vec2::X.signed_angle_with(Vec2::Y), -FRAC_PI_2);
        assert_eq_angle!(Vec2::X.signed_angle_with(Vec2::new(2.0, -2.0)), FRAC_PI_4);
    }
}
