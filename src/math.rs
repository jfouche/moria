use bevy::{
    math::{primitives::Direction3d, Vec3},
    transform::components::Transform,
};

pub trait SignedAngle {
    /// get the signed angle between `self` and `rhs`
    fn signed_angle_between(&self, rhs: &Self) -> f32;
}

impl SignedAngle for Direction3d {
    fn signed_angle_between(&self, rhs: &Self) -> f32 {
        assert!(self.y == 0.0 && rhs.y == 0.0);
        (**self).signed_angle_between(rhs)
    }
}

impl SignedAngle for Vec3 {
    fn signed_angle_between(&self, rhs: &Self) -> f32 {
        assert!(self.y == 0.0 && rhs.y == 0.0, "{self}, {rhs}");
        self.cross(*rhs).dot(Vec3::Y).atan2(self.dot(*rhs))
    }
}

impl SignedAngle for Transform {
    fn signed_angle_between(&self, rhs: &Self) -> f32 {
        let direction = rhs.translation.horizontal() - self.translation.horizontal();
        direction.signed_angle_between(&self.forward().horizontal())
    }
}

pub trait Flatten {
    fn horizontal(self) -> Self;
}

impl Flatten for Direction3d {
    fn horizontal(self) -> Self {
        Direction3d::new((*self).horizontal()).unwrap()
    }
}

impl Flatten for Vec3 {
    fn horizontal(self) -> Self {
        Vec3 {
            x: self.x,
            y: 0.0,
            z: self.z,
        }
    }
}
