use std::convert::{From, Into};
use std::ops::{Add, Div, Mul, Sub};

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Value<T: Add> {
    Number(T),
    Inf,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Point<const A: i32, const B: i32, T: Add<Output = T>> {
    x: Value<T>,
    y: Value<T>,
}

impl<const A: i32, const B: i32, T: Add<Output = T> + Mul<Output = T>> Point<A, B, T>
where
    T: From<i32> + PartialEq + Copy,
{
    pub fn new(x: Value<T>, y: Value<T>) -> Result<Point<A, B, T>, String> {
        let a = T::from(A);
        let b = T::from(B);
        match (x, y) {
            (Value::Inf, Value::Inf) => Ok(Point {
                x: Value::Inf,
                y: Value::Inf,
            }),
            (Value::Number(x), Value::Number(y)) if y * y != x * x * x + a * x + b => {
                Err("point is not in the curve".to_string())
            }
            (x, y) => Ok(Point { x, y }),
        }
    }
}

impl<
        const A: i32,
        const B: i32,
        T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Div<Output = T>,
    > Add for Point<A, B, T>
where
    T: From<i32> + PartialEq + Copy,
{
    type Output = Point<A, B, T>;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Point { x: Value::Inf, .. }, rhs) => rhs,
            (left, Point { x: Value::Inf, .. }) => left,
            (
                Point {
                    x: Value::Number(x1),
                    y: Value::Number(y1),
                },
                Point {
                    x: Value::Number(x2),
                    y: Value::Number(y2),
                },
            ) if x1 == x2 && y1 != y2 => Point {
                x: Value::Inf,
                y: Value::Inf,
            },
            (
                Point {
                    x: Value::Number(x1),
                    y: Value::Number(y1),
                },
                Point {
                    x: Value::Number(x2),
                    y: Value::Number(y2),
                },
            ) if x1 != x2 => {
                let s = (y2 - y1) / (x2 - x1);
                let x3 = s * s - x1 - x2;
                let y3 = s * (x1 - x3) - y1;
                Point {
                    x: Value::Number(x3),
                    y: Value::Number(y3),
                }
            }
            (
                Point {
                    x: Value::Number(_x1),
                    y: Value::Number(y1),
                },
                Point {
                    x: Value::Number(_x2),
                    y: Value::Number(_y2),
                },
            ) if y1 == 0.into() => Point {
                x: Value::Inf,
                y: Value::Inf,
            },
            (
                Point {
                    x: Value::Number(x1),
                    y: Value::Number(y1),
                },
                Point {
                    x: Value::Number(_x2),
                    y: Value::Number(_y2),
                },
            ) => {
                let s: T = (T::from(3) * x1 * x1 + A.into()) / (T::from(2) * y1);
                let x3 = s * s - T::from(2) * x1;
                let y3 = s * (x1 - x3) - y1;
                Point {
                    x: Value::Number(x3),
                    y: Value::Number(y3),
                }
            }
            (_, _) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    type Point_i32 = Point<5, 7, i32>;

    #[test]
    fn test_add_case_x1_dif_x2() {
        let p1: Point_i32 = Point {
            x: Value::Number(2),
            y: Value::Number(5),
        };
        let p2: Point_i32 = Point {
            x: Value::Number(-1),
            y: Value::Number(-1),
        };
        let s = p1 + p2;
        let expt: Point_i32 = Point {
            x: Value::Number(3),
            y: Value::Number(-7),
        };
        assert_eq!(s, expt);
    }

    #[test]
    fn test_add_case_p1_eq_p2() {
        let p1: Point_i32 = Point {
            x: Value::Number(-1),
            y: Value::Number(-1),
        };
        let p2: Point_i32 = Point {
            x: Value::Number(-1),
            y: Value::Number(-1),
        };
        let s = p1 + p2;
        let expt = Point_i32::new(Value::Number(18), Value::Number(77)).unwrap();
        assert_eq!(s, expt);
    }
}
