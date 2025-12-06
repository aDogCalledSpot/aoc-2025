use std::{fs::read_to_string, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    direction: Direction,
    distance: u32,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            direction: Direction::from_str(&s[0..1])?,
            distance: u32::from_str(&s[1..]).map_err(|_| ())?,
        })
    }
}

struct Safe {
    pos: u32,
}

impl Safe {
    fn new() -> Self {
        Self { pos: 50 }
    }

    fn turn(&mut self, instruction: &Instruction) {
        let distance = instruction.distance % 100;
        if instruction.direction == Direction::Right {
            self.pos = (self.pos + instruction.distance) % 100;
            return;
        }
        // Direction is left
        if distance <= self.pos {
            self.pos -= distance;
            return;
        }

        self.pos = 100 - (distance - self.pos);
    }
}

fn main() {
    let mut safe = Safe::new();
    let mut points = 0;
    for line in read_to_string("inputs/day1.txt").unwrap().lines() {
        let Ok(instruction) = Instruction::from_str(line) else {
            panic!("Failed to parse line: {line}");
        };
        safe.turn(&instruction);
        if safe.pos == 0 {
            points += 1;
        }
    }
    println!("{points}");
}
