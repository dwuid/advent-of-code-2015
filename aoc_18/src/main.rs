
use std::mem::swap;
use std::str::from_utf8;

struct GameOfLife {
    grid:   Vec<Vec<bool>>,
    shadow: Vec<Vec<bool>>,

    x: usize,
    y: usize
}

impl GameOfLife {
    fn new(grid: &Vec<Vec<bool>>) -> Self {
        let x = if grid.len() > 0 { grid[0].len() } else { 0 };
        let y = grid.len();

        GameOfLife {
            grid: grid.clone(), shadow: vec![vec![false; y]; x],
            x: x, y: y
        }
    }

    fn neighbors(&self, i: i32, j: i32) -> usize {
        let mut alive = 0;

        let x = self.x as i32;
        let y = self.y as i32;

        let neighbors: [(i32, i32); 8] = [
            (i - 1, j - 1), (i, j - 1), (i + 1, j - 1),
            (i - 1, j    ),             (i + 1, j    ),
            (i - 1, j + 1), (i, j + 1), (i + 1, j + 1),
        ];

        for &(i, j) in &neighbors {
            if (0 <= i && i < x) && (0 <= j && j < y) {
                let current = self.grid[i as usize][j as usize] as usize;
                alive += current;
            }
        }

        alive
    }

    fn broken_lights(&mut self) {
        self.grid[0][0]                   = true;
        self.grid[0][self.y - 1]          = true;
        self.grid[self.x - 1][0]          = true;
        self.grid[self.x - 1][self.y - 1] = true;
    }

    fn step(&mut self) {
        for i in 0..self.x {
            for j in 0..self.y {
                let n = self.neighbors(i as i32, j as i32);
                self.shadow[i][j] = match self.grid[i][j] {
                    true  => n == 2 || n == 3,
                    false => n == 3
                };
            }
        }

        swap(&mut self.grid, &mut self.shadow);
    }

    fn population(&self) -> usize {
        let mut result = 0;
        for row in &self.grid {
            for element in row {
                result += *element as usize;
            }
        }

        result
    }
}

static PARSE_ERROR: &'static str = "Invalid input format.";

fn main() {
    let input = from_utf8(include_bytes!("../input.txt")).unwrap();
    let lines: Vec<_> = input.split('\n').filter(|l| !l.is_empty()).collect();

    let y = lines.len();
    assert!(y > 0, PARSE_ERROR);

    let x = lines[0].len();
    assert!(x == y, PARSE_ERROR);

    let mut grid = vec![vec![false; y]; x];
    for (j, &line) in lines.iter().enumerate() {
        for (i, state) in line.chars().enumerate() {
            grid[i][j] = match state {
                '.' => false,
                '#' => true,
                _   => panic!(PARSE_ERROR)
            }
        }
    }

    let mut game = GameOfLife::new(&grid);
    for _ in 0..100 {
        game.step();
    }

    let mut broken = GameOfLife::new(&grid);
    broken.broken_lights();

    for _ in 0..100 {
        broken.step();
        broken.broken_lights();
    }

    println!("{} lights are lit.", game.population());
    println!("For the broken game, {} lights are lit.", broken.population());
}

