use std::ops::{Add, Mul};

#[derive(PartialEq, Eq, Debug)]
pub enum Value {
    Number(i32),
    Inf,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Point<const A: i32, const B: i32> {
    x: Value,
    y: Value,
}

impl<const A: i32, const B: i32> Point<A, B> {
    pub fn new(x: Value, y: Value) -> Result<Point<A, B>, String> {
        match (x, y) {
            (Value::Inf, Value::Inf) => Ok(Point {
                x: Value::Inf,
                y: Value::Inf,
            }),
            (Value::Number(x), Value::Number(y)) if y * y != x * x * x + A * x + B => {
                Err("point is not in the curve".to_string())
            }
            (x, y) => Ok(Point { x, y }),
        }
    }
}

impl<const A: i32, const B: i32> Add for Point<A, B> {
    type Output = Point<A, B>;
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
                    x: Value::Number(_),
                    ..
                },
                Point {
                    x: Value::Number(_),
                    ..
                },
            ) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        let p1: Point<5, 7> = Point {
            x: Value::Number(2),
            y: Value::Number(5),
        };
        let p2: Point<5, 7> = Point {
            x: Value::Number(-1),
            y: Value::Number(-1),
        };
        let s = p1 + p2;
        let expt: Point<5, 7> = Point {
            x: Value::Number(3),
            y: Value::Number(-7),
        };
        assert_eq!(s, expt);
    }
}
