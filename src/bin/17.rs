use hashbrown::HashMap;

pub fn part_one(input: &str) -> Option<u64> {
    let mut tower = Tower::new(input);

    tower.run_steps(2022);

    Some(tower.height as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut tower = Tower::new(input);
    Some(simulate_fast(&mut tower, 1000000000000))
}

fn simulate_fast(tower: &mut Tower, total_rocks: u64) -> u64 {
    let mut seen = HashMap::new();

    for rocks in 1..total_rocks {
        let shape = tower.next_shape();
        let mut rock = Rock::new(tower.height, shape);

        loop {
            let m = tower.next_move();
            rock.try_move(m, tower);
            if !rock.try_fall(tower) {
                tower.add_rock(rock);
                break;
            }
        }

        if let Some((old_rocks, old_height)) =
            seen.insert(tower.tower_state(), (rocks as u32, tower.height as u32))
        {
            let rocks_in_period = rocks - old_rocks as u64;
            let height_per_period = (tower.height - old_height as i32) as u64;
            let added_periods = (total_rocks - rocks) / rocks_in_period;

            let added_height = added_periods * height_per_period;
            let added_rocks = added_periods * rocks_in_period;

            tower.run_steps(total_rocks - (rocks + added_rocks));

            return tower.height as u64 + added_height;
        }
    }

    tower.height as u64
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Left,
    Right,
}

impl Move {
    fn from(c: char) -> Self {
        match c {
            '<' => Self::Left,
            '>' => Self::Right,
            _ => panic!("unexpected '{}'", c),
        }
    }

    fn delta_x(self) -> i8 {
        match self {
            Move::Left => -1,
            Move::Right => 1,
        }
    }
}

type Shape = [(i8, i32)];

const SHAPES: [&Shape; 5] = [
    // ####
    &[(0, 0), (1, 0), (2, 0), (3, 0)],
    // .#.
    // ###
    // .#.
    &[(1, 2), (0, 1), (1, 1), (2, 1), (1, 0)],
    // ..#
    // ..#
    // ###
    &[(2, 2), (2, 1), (0, 0), (1, 0), (2, 0)],
    // #
    // #
    // #
    // #
    &[(0, 3), (0, 2), (0, 1), (0, 0)],
    // ##
    // ##
    &[(0, 1), (1, 1), (0, 0), (1, 0)],
];

const VALID_X: std::ops::Range<i8> = 0..7;

struct Rock {
    x: i8,
    y: i32,
    shape: &'static Shape,
}

impl Rock {
    fn new(tower_height: i32, shape: &'static Shape) -> Self {
        Self {
            x: 2,
            y: tower_height + 3,
            shape,
        }
    }

    fn try_move(&mut self, m: Move, tower: &Tower) {
        let new_x = self.x + m.delta_x();
        if tower.fits(new_x, self.y, self.shape) {
            self.x = new_x;
        }
    }

    fn try_fall(&mut self, tower: &Tower) -> bool {
        let new_y = self.y - 1;
        if tower.fits(self.x, new_y, self.shape) {
            self.y = new_y;
            true
        } else {
            false
        }
    }
}

struct Tower {
    // Since width of the tower is 7, we can represent each row as u8
    rows: Vec<u8>,
    height: i32,
    moves: Vec<Move>,
    shape_index: u8,
    move_index: u16,
}

impl Tower {
    fn new(input: &str) -> Self {
        Self {
            rows: vec![0; 10_000],
            height: 0,
            shape_index: 0,
            moves: input.chars().map(Move::from).collect(),
            move_index: 0,
        }
    }

    fn run_steps(&mut self, rocks: u64) {
        for _ in 0..rocks {
            let mut rock = Rock::new(self.height, self.next_shape());

            loop {
                rock.try_move(self.next_move(), self);

                if !rock.try_fall(self) {
                    self.add_rock(rock);
                    break;
                }
            }
        }
    }

    fn next_shape(&mut self) -> &'static Shape {
        let s = SHAPES[self.shape_index as usize];
        self.shape_index = (self.shape_index + 1) % (SHAPES.len() as u8);
        s
    }

    fn next_move(&mut self) -> Move {
        let m = self.moves[self.move_index as usize];
        self.move_index = (self.move_index + 1) % (self.moves.len() as u16);
        m
    }

    fn add_rock(&mut self, rock: Rock) {
        for (dx, dy) in rock.shape {
            self.set(rock.x + dx, rock.y + dy);
        }
    }

    fn fits(&self, x: i8, y: i32, shape: &Shape) -> bool {
        for (dx, dy) in shape {
            if !self.is_free(x + dx, y + dy) {
                return false;
            }
        }
        true
    }

    fn set(&mut self, x: i8, y: i32) {
        let i = y as usize;

        while i >= self.rows.len() {
            self.rows.push(0);
        }

        self.height = self.height.max(y + 1);
        self.rows[i] |= 1 << x;
    }

    fn is_free(&self, x: i8, y: i32) -> bool {
        if !VALID_X.contains(&x) || y < 0 {
            return false;
        }

        let i = y as usize;
        if i >= self.rows.len() {
            return true;
        }

        let row = self.rows[i];
        (row & (1 << x)) == 0
    }

    fn tower_state(&self) -> u32 {
        if self.height < 3 {
            return 0;
        }
        let h = self.height as usize;

        // 16 bits for move index, 3 bits for shape index and 7 bits for floor
        ((self.move_index as u32) << 10)
            | ((self.shape_index as u32) << 7)
            | (self.rows[h - 3] as u32)
    }
}

fn main() {
    let input = &aoc::read_file("inputs", 17);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 17);
        assert_eq!(part_two(&input), Some(1514285714288));
    }
}
