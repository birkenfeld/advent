// Type-specific re-exports of cgmath and added utilities.

pub trait VecExt<T> {
    fn splat(v: T) -> Self;
    fn manhattan(self) -> T;
}

macro_rules! impl_type {
    ($ty:tt, $one:literal, $zero:literal) => {

pub mod $ty {
    pub use super::VecExt;
    pub type Vec2 = cgmath::Vector2<$ty>;

    pub const fn vec2(x: $ty, y: $ty) -> Vec2 {
        Vec2::new(x, y)
    }

    pub const X2: Vec2 = vec2($one, $zero);
    pub const Y2: Vec2 = vec2($zero, $one);

    impl super::VecExt<$ty> for Vec2 {
        fn splat(v: $ty) -> Vec2 { vec2(v, v) }
        fn manhattan(self) -> $ty { self.x.abs() + self.y.abs() }
    }

    impl crate::input::InputItem for Vec2 {
        fn read_part(tok: &mut impl Iterator<Item=&'static str>) -> Option<Self> {
            let x = tok.next()?.parse().ok()?;
            let y = tok.next()?.parse().ok()?;
            Some(vec2(x, y))
        }
    }

    /////////////////////////////////////////////////////////

    pub type Vec3 = cgmath::Vector3<$ty>;

    pub const fn vec3(x: $ty, y: $ty, z: $ty) -> Vec3 {
        Vec3::new(x, y, z)
    }

    pub const X3: Vec3 = vec3($one, $zero, $zero);
    pub const Y3: Vec3 = vec3($zero, $one, $zero);
    pub const Z3: Vec3 = vec3($zero, $zero, $one);

    impl super::VecExt<$ty> for Vec3 {
        fn splat(v: $ty) -> Vec3 { vec3(v, v, v) }
        fn manhattan(self) -> $ty { self.x.abs() + self.y.abs() + self.z.abs() }
    }

    impl crate::input::InputItem for Vec3 {
        fn read_part(tok: &mut impl Iterator<Item=&'static str>) -> Option<Self> {
            let x = tok.next()?.parse().ok()?;
            let y = tok.next()?.parse().ok()?;
            let z = tok.next()?.parse().ok()?;
            Some(vec3(x, y, z))
        }
    }
}


    };
}

impl_type!(i32, 1, 0);
impl_type!(i64, 1, 0);
impl_type!(f64, 1.0, 0.0);
