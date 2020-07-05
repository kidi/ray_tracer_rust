

pub struct ArrayVect {
    pub elts: Vec<i32>
}

impl ArrayVect {
    pub fn array3(_x: i32, _y: i32, _z: i32) -> ArrayVect {
        ArrayVect {
            elts: vec![_x, _y, _z]
        }
    }
    pub fn array6(_x1: i32, _y1: i32, _z1: i32, _x2: i32, _y2: i32, _z2: i32) -> ArrayVect {
        ArrayVect {
            elts: vec![_x1, _y1, _z1, _x2, _y2, _z2]
        }
    }
    pub fn concat(&self, other: ArrayVect) -> ArrayVect {
        let mut vec = self.elts.clone();
        let vec2 = &mut other.elts.clone();
        vec.append(vec2);
        ArrayVect {
            elts: vec
        }
    }
    pub fn concatRef(&self, other: &ArrayVect) -> ArrayVect {
        let mut vec = self.elts.clone();
        let vec2 = &mut other.elts.clone();
        vec.append(vec2);
        ArrayVect {
            elts: vec
        }
    }

}

#[cfg(test)]
mod ray_tests {
    use super::*;

    #[test]
    fn ray_can_create_array_from_new() {
        let a = ArrayVect::array3(1, 2, 3);
        assert_eq!(
            a.elts,
            vec![1, 2, 3]
        )
    }

    #[test]
    fn ray_can_concatenate_array() {
        let a = ArrayVect::array3(1, 2, 3);
        let b = ArrayVect::array3(3, 4, 5);
        assert_eq!(
            a.concat(b).elts,
            vec![1, 2, 3, 3, 4, 5]
        )
    }

}
 