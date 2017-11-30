use std::ops;
use std::fmt;
use std::collections::HashSet;

#[derive(Copy, Clone, PartialEq, Debug, Eq, Hash)]
struct Vector2 {
    x: i32,
    y: i32
}

impl Vector2 {
    pub fn cross(&self, other: &Vector2) -> i32 {
        return self.x * other.y - self.y * other.x;
    }
}

impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;
    fn add(self, _rhs: Vector2) -> Vector2 {
        return Vector2 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        };
    }
}

impl ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;
    fn sub(self, _rhs: Vector2) -> Vector2 {
        return Vector2 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        };
    }
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let input = "R3, L5, R1, R2, L5, R2, R3, L2, L5, R5, L4, L3, R5, L1, R3, R4, R1, L3, R3, L2, L5, L2, R4, R5, R5, L4, L3, L3, R4, R4, R5, L5, L3, R2, R2, L3, L4, L5, R1, R3, L3, R2, L3, R5, L194, L2, L5, R2, R1, R1, L1, L5, L4, R4, R2, R2, L4, L1, R2, R53, R3, L5, R72, R2, L5, R3, L4, R187, L4, L5, L2, R1, R3, R5, L4, L4, R2, R5, L5, L4, L3, R5, L2, R1, R1, R4, L1, R2, L3, R5, L4, R2, L3, R1, L4, R4, L1, L2, R3, L1, L1, R4, R3, L4, R2, R5, L2, L3, L3, L1, R3, R5, R2, R3, R1, R2, L1, L4, L5, L2, R4, R5, L2, R4, R4, L3, R2, R1, L4, R3, L3, L4, L3, L1, R3, L2, R2, L4, L4, L5, R3, R5, R3, L2, R5, L2, L1, L5, L1, R2, R4, L5, R2, L4, L5, L4, L5, L2, L5, L4, R5, R3, R2, R2, L3, R3, L2, L5";
    let mut position = Vector2 {x: 0, y: 0};
    let directions = vec![
        Vector2 {x:  0, y:  1}, // North
        Vector2 {x:  1, y:  0}, // East
        Vector2 {x:  0, y: -1}, // South
        Vector2 {x: -1, y:  0}, // West
    ];
    let mut history = HashSet::new();
    history.insert(position.clone());
    let mut direction: usize = 0; //index into directions
    let mut first_intersection: Option<Vector2> = None;
    let max = (directions.len() - 1) as i32;
    
    for movement in input.split(",") {
        let mut pieces = movement.trim().to_string();
        let moves = pieces.split_off(1).parse::<i32>().unwrap();

        let mut current_direction = direction as i32;
        match pieces.as_ref() {
            "R" => current_direction += 1,
            "L" => current_direction -= 1,
            _ => (),
        };
        if current_direction < 0 {
            current_direction = max;
        }
        if current_direction > max {
            current_direction = 0;
        }
        direction = current_direction as usize;

        for _ in 0..moves {
            position = Vector2 {
                x: position.x + directions[direction].x,
                y: position.y + directions[direction].y,
            };
            if first_intersection.is_none() && history.contains(&position) {
                first_intersection = Some(position);
            }
            history.insert(position);
        }

        

        // println!("Next position ({}, {})", position.x, position.y);

        
        

        // let l = history.len();
        // if history.len() >= 4 && first_intersection.is_none() {
        //     let b1 = (&history[l - 2]).clone();
        //     let b2 = (&history[l - 1]).clone();
        //     let tail = &history[0..l - 3];
        //     for iter in tail.windows(2) {
        //         let a1 = (&iter[0]).clone();
        //         let a2 = (&iter[1]).clone();
        //         match intersection(b1, b2, a1, a2) {
        //             Some(i) => {
        //                 println!("Intersection between ({}, {}) and ({}, {}) is {}", b1, b2, a1, a2, i);
        //                 first_intersection = Some(i);
        //                 break;
        //             },
        //             None => ()
        //         }
        //     }
        // }
    }
    println!("Final position {} is {} blocks away", position, position.x.abs() + position.y.abs());
    if first_intersection.is_some() {
        let f = first_intersection.unwrap();
        println!("First intersection at {} is {} blocks away", f, f.x.abs() + f.y.abs());
    }
}

// /**
//  * s10 = r
//  * s32 = s
//  * s02 = c
//  */
// fn intersection(p: Vector2, p2: Vector2, q: Vector2, q2: Vector2) -> Option<Vector2> {
//     let c = q - p;
//     let r = p2 - p;
//     let s = q2 - q;

//     let denominator = r.cross(&s);
    
//     if denominator == 0 {
//         return None
//     }

//     let denom_is_positive = denominator > 0;
    
//     let s_numerator = c.cross(&r);
//     let t_numerator = c.cross(&s);

//     if (s_numerator < 0) == denom_is_positive {
//         return None;
//     }

//     if (t_numerator < 0) == denom_is_positive {
//         return None;
//     }

//     if (s_numerator > denominator) == denom_is_positive ||
//        (t_numerator > denominator) == denom_is_positive {
//            return None;
//        }

//     let t = t_numerator / denominator;
//     let i = Vector2 {x: p.x + (t * r.x), y: p.y + (t * r.y)};

//     return Some(i);
// }

// fn is_point_on_line(p1: Vector2, p2: Vector2, b: Vector2) -> bool {
//     // let p1_origin = Vector2 {x: 0, y: 0};
//     let p2_origin = Vector2 {x: p2.x - p1.x, y: p2.y - p1.y};
//     let b_offset = Vector2 {x: b.x - p1.x, y: b.y - p1.y};
//     let r = p2_origin.cross(&b_offset);
    
//     return r.abs() < 0;
// }

// fn intersection2(x1: Vector2, y1: Vector2, x2: Vector2, y2: Vector2) -> Option<Vector2> {
//     let a1 = y1.y - x1.y;
//     let b1 = x1.x - y1.x;
//     let c1 = a1 * (x1.x) + b1 * (x1.y);

//     let a2 = y2.y - x2.y;
//     let b2 = x2.x - y2.x;
//     let c2 = a2 * (x2.x) + b2 * (x2.y);

//     let determinant = a1 * b2 - a2 * b1;

//     if determinant == 0 {
//         return None;
//     }

//     let ix = (b2 * c1 - b1 * c2) / determinant;
//     let iy = (a1 * c2 - a2 * c1) / determinant;
//     let i = Vector2 {x: ix, y: iy};
//     println!("L{},{} and L{},{} -> {}", x1, y1, x2, y2, i);
//     if is_point_on_line(x1, y1, i) && is_point_on_line(x2, y2, i) {
//         return Some(i);
//     }

//     return None;
// }