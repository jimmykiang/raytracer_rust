#[derive(PartialEq, Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        debug_assert!(w == 1.0 || w == 0.0, "w must be 0 or 1; was {}", w);
        debug_assert!(!x.is_nan(), "x cannot be NaN");
        debug_assert!(!y.is_nan(), "x cannot be NaN");
        debug_assert!(!z.is_nan(), "z cannot be NaN");
        Tuple { x, y, z, w }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A tuple with w=1.0 is a point
    #[test]
    fn test_tuple_w1_constructor() {
        let tuple = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(
            tuple,
            Tuple {
                x: 4.3,
                y: -4.2,
                z: 3.1,
                w: 1.0
            }
        );
    }
}
