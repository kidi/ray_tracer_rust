pub struct ArrayVect {
    pub elts: Vec<i32>,
}

impl ArrayVect {
    pub fn array3(_x: i32, _y: i32, _z: i32) -> ArrayVect {
        ArrayVect {
            elts: vec![_x, _y, _z],
        }
    }
    pub fn array6(_x1: i32, _y1: i32, _z1: i32, _x2: i32, _y2: i32, _z2: i32) -> ArrayVect {
        ArrayVect {
            elts: vec![_x1, _y1, _z1, _x2, _y2, _z2],
        }
    }
    pub fn concat(&self, other: ArrayVect) -> ArrayVect {
        let mut vec = self.elts.clone();
        let vec2 = &mut other.elts.clone();
        vec.append(vec2);
        ArrayVect { elts: vec }
    }
    pub fn concatRef(&self, other: &ArrayVect) -> ArrayVect {
        let mut vec = self.elts.clone();
        let vec2 = &mut other.elts.clone();
        vec.append(vec2);
        ArrayVect { elts: vec }
    }
}

// A tuple struct
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Tuple(pub f32, pub f32, pub f32, pub f32);

impl Tuple {
    pub fn x(&self) -> f32 {
        self.0
    }
    pub fn y(&self) -> f32 {
        self.1
    }
    pub fn z(&self) -> f32 {
        self.2
    }
    pub fn w(&self) -> f32 {
        self.3
    }
    pub fn isPoint(&self) -> bool {
        self.w() == 1.0
    }
    pub fn isVector(&self) -> bool {
        self.w() == 0.0
    }
    pub fn point3(x: f32, y: f32, z: f32) -> Tuple {
        Tuple(x, y, z, 1.0)
    }
    pub fn vector3(x: f32, y: f32, z: f32) -> Tuple {
        Tuple(x, y, z, 0.0)
    }
    pub fn add(&self, other: &Tuple) -> Tuple {
        let x = self.x() + other.x();
        let y = self.y() + other.y();
        let z = self.z() + other.z();
        let w = self.w() + other.w();
        Tuple(x, y, z, w)
    }
    pub fn sub(&self, other: &Tuple) -> Tuple {
        let x = self.x() - other.x();
        let y = self.y() - other.y();
        let z = self.z() - other.z();
        let w = self.w() - other.w();
        Tuple(x, y, z, w)
    }
    pub fn neg(&self) -> Tuple {
        Tuple(-self.x(), -self.y(), -self.z(), -self.w())
    }
}

fn eqvFloat(x: f32, y: f32) -> bool {
    let epsilon = 0.00001;
    (x - y).abs() < epsilon
}

#[cfg(test)]
mod ray_tests {
    use super::*;

    #[test]
    fn ray_can_create_array_from_new() {
        let a = ArrayVect::array3(1, 2, 3);
        assert_eq!(a.elts, vec![1, 2, 3])
    }

    #[test]
    fn ray_can_concatenate_array() {
        let a = ArrayVect::array3(1, 2, 3);
        let b = ArrayVect::array3(3, 4, 5);
        assert_eq!(a.concat(b).elts, vec![1, 2, 3, 3, 4, 5])
    }
}
