use std::{cmp::Ordering, fs::read_to_string, str::FromStr};

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

    fn do_turn(pos: &mut u32, instruction: &Instruction) -> u32 {
        assert!(*pos < 100);
        assert_ne!(instruction.distance, 0);

        let ret = instruction.distance / 100;
        let distance = instruction.distance % 100;
        if distance == 0 {
            return ret;
        }
        if instruction.direction == Direction::Right {
            *pos += distance;
            if *pos >= 100 {
                *pos %= 100;
                return ret + 1;
            }
            return ret;
        }
        match (*pos).cmp(&distance) {
            Ordering::Less if *pos == 0 => {
                *pos = 100 - (distance - *pos);
                assert_ne!(*pos, 0);
                ret
            }
            Ordering::Less => {
                *pos = 100 - (distance - *pos);
                assert_ne!(*pos, 0);
                ret + 1
            }
            Ordering::Equal => {
                *pos -= distance;
                assert_eq!(*pos, 0);
                ret + 1
            }
            Ordering::Greater => {
                *pos -= distance;
                assert_ne!(*pos, 0);
                ret
            }
        }
    }

    fn turn(&mut self, instruction: &Instruction) -> u32 {
        Self::do_turn(&mut self.pos, instruction)
    }
}

fn main() {
    let mut safe = Safe::new();
    let mut points = 0;
    for line in read_to_string("inputs/day1.txt").unwrap().lines() {
        let Ok(instruction) = Instruction::from_str(line) else {
            panic!("Failed to parse line: {line}");
        };
        points += safe.turn(&instruction);
    }
    println!("{points}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_everything() {
        assert_eq!(
            Safe::do_turn(
                &mut 50,
                &Instruction {
                    direction: Direction::Right,
                    distance: 1000
                }
            ),
            10
        );
        assert_eq!(
            Safe::do_turn(
                &mut 0,
                &Instruction {
                    direction: Direction::Right,
                    distance: 100
                }
            ),
            1
        );
        let mut x = 50;
        assert_eq!(
            Safe::do_turn(
                &mut x,
                &Instruction {
                    direction: Direction::Left,
                    distance: 68
                }
            ),
            1
        );
        assert_eq!(x, 82);
        assert_eq!(
            Safe::do_turn(
                &mut x,
                &Instruction {
                    direction: Direction::Left,
                    distance: 30
                }
            ),
            0
        );
        assert_eq!(x, 52);
        assert_eq!(
            Safe::do_turn(
                &mut x,
                &Instruction {
                    direction: Direction::Right,
                    distance: 48
                }
            ),
            1
        );
        assert_eq!(x, 0);
        assert_eq!(
            Safe::do_turn(
                &mut x,
                &Instruction {
                    direction: Direction::Left,
                    distance: 5
                }
            ),
            0
        );
        assert_eq!(x, 95);
        assert_eq!(
            Safe::do_turn(
                &mut x,
                &Instruction {
                    direction: Direction::Right,
                    distance: 60
                }
            ),
            1
        );
        assert_eq!(x, 55);
        assert_eq!(
            Safe::do_turn(
                &mut x,
                &Instruction {
                    direction: Direction::Left,
                    distance: 55
                }
            ),
            1
        );
        assert_eq!(x, 0);
        assert_eq!(
            Safe::do_turn(
                &mut x,
                &Instruction {
                    direction: Direction::Left,
                    distance: 1
                }
            ),
            0
        );
        assert_eq!(x, 99);
        assert_eq!(
            Safe::do_turn(
                &mut x,
                &Instruction {
                    direction: Direction::Left,
                    distance: 99
                }
            ),
            1
        );
        assert_eq!(x, 0);
        assert_eq!(
            Safe::do_turn(
                &mut x,
                &Instruction {
                    direction: Direction::Right,
                    distance: 14
                }
            ),
            0
        );
        assert_eq!(x, 14);
        assert_eq!(
            Safe::do_turn(
                &mut x,
                &Instruction {
                    direction: Direction::Left,
                    distance: 82
                }
            ),
            1
        );
        assert_eq!(x, 32);
    }
}
