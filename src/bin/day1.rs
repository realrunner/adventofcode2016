use std::fmt;
use std::collections::HashSet;

#[derive(Copy, Clone, PartialEq, Debug, Eq, Hash)]
struct Vector2 {
    x: i32,
    y: i32
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
    }
    println!("Final position {} is {} blocks away", position, position.x.abs() + position.y.abs());
    if first_intersection.is_some() {
        let f = first_intersection.unwrap();
        println!("First intersection at {} is {} blocks away", f, f.x.abs() + f.y.abs());
    }
}
