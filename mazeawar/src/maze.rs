use std::thread::current;
use r::{Rng, thread_rng};
use macroquad::prelude::*;
use crate::map::{Map, WALL};

//each cell has 4 walls. The number is an index in the array of walls
const TOP: usize = 0;
const RIGHT: usize = 1;
const BOTTOM: usize = 2;
const LEFT: usize = 3;

//constants to define difficulty level by percentage of the walls that has to be removed
pub const LOW: i32 = 50;
pub const MEDIUM: i32 = 30;
pub const HIGH: i32 = 10;


#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Cell {
    pub row: i32,
    pub col: i32,
    pub walls: [bool; 4],
    pub visited: bool,
}

impl Cell {
    fn new(row: i32, col: i32) -> Cell {
        Cell {
            row,
            col,
            walls: [true, true, true, true],
            visited: false,
        }
    }
    //removes wall between two cells
    fn remove_wall(&mut self, next: &mut Cell) -> i32 {
        let x = self.col - next.col;
        let y = self.row - next.row;
        if x == 1 && self.walls[LEFT] {
            self.walls[LEFT] = false;
            next.walls[RIGHT] = false;
            return 1;
        }
        if x == -1 && self.walls[RIGHT] {
            self.walls[RIGHT] = false;
            next.walls[LEFT] = false;
            return 1;
        }
        if y == 1 && self.walls[TOP] {
            self.walls[TOP] = false;
            next.walls[BOTTOM] = false;
            return 1;
        }
        if y == -1 && self.walls[BOTTOM] {
            self.walls[BOTTOM] = false;
            next.walls[TOP] = false;
            return 1;
        }
        return 0;
    }
}

#[derive(PartialEq, Clone)]
pub struct Grid {
    width: i32,
    height: i32,
    pub cells: Vec<Cell>,
    backtrace: Vec<usize>,
    current: usize,
    difficulty: i32,
    walls: i32,
}

impl Grid {
    pub fn new(width: i32, height: i32, difficulty: i32) -> Grid {
        let mut grid = Grid {
            width,
            height,
            cells: Vec::new(),
            backtrace: Vec::new(),
            current: 0,
            difficulty,
            walls: 0,
        };
        for i in 0..height {
            for j in 0..width {
                grid.cells.push(Cell::new(i, j));
            }
        };
        grid.walls = ((width + 1) * height) + ((height + 1) * width) - (width * 2 + height * 2);
        return grid;
    }
    //returns cell's index in the array
    fn calculate_index(&self, row: i32, col: i32) -> i32 {
        if row < 0 || col < 0 || row > self.height - 1 || col > self.width - 1 {
            return -1;
        }
        return col + (row * self.width);
    }
    //returns a list of available "exits" from given cell
    fn get_available_neighbours(&self, ignore_visited: bool) -> Vec<usize> {
        let mut neighbours = Vec::new();
        let current_row = self.cells[self.current].row;
        let current_col = self.cells[self.current].col;
        let neighbor_indices: [i32; 4] = [
            self.calculate_index(current_row - 1, current_col),
            self.calculate_index(current_row, current_col + 1),
            self.calculate_index(current_row + 1, current_col),
            self.calculate_index(current_row, current_col - 1),
        ];
        for i in neighbor_indices.iter() {
            if *i != -1 && (ignore_visited || !self.cells[*i as usize].visited) {
                neighbours.push(*i as usize);
            }
        }
        return neighbours;
    }
    //chooses next destination randomly if there's any
    fn find_next_cell(&mut self, ignore_visited: bool) -> Option<usize> {
        let neighbours = self.get_available_neighbours(ignore_visited);
        if !neighbours.is_empty() {
            return if neighbours.len() == 1 {
                Some(neighbours[0])
            } else {
                Some(neighbours[(thread_rng().gen_range(0..neighbours.len())) as usize])
            };
        }
        return None;
    }
    //main function to generate maze
    pub fn generate_maze(&mut self) {
        loop {
            self.cells[self.current].visited = true;
            let next = self.find_next_cell(false);
            match next {
                Some(next) => {
                    self.cells[next].visited = true;
                    self.backtrace.push(self.current);
                    //an ugly way to get access to two elements in the vec at the same time to avoid double borrowing
                    let (lower_part, higher_part) =
                        self.cells.split_at_mut(std::cmp::max(self.current, next));
                    let cell1 = &mut lower_part[std::cmp::min(self.current, next)];
                    let cell2 = &mut higher_part[0];
                    //this count will be needed to adjust to difficulty level
                    self.walls = self.walls - cell1.remove_wall(cell2);
                    self.current = next;
                }
                //if there's no available "exits" from the current cell algo goes one step back
                None => {
                    if !self.backtrace.is_empty() {
                        self.current = self.backtrace[0];
                        self.backtrace.remove(0);
                    } else {
                        self.adjust_difficulty_level();
                        break;
                    }
                }
            }
        }
    }
    fn adjust_difficulty_level(&mut self) {
        //count number of walls that has to be deleted
        let mut has_to_remove = self.walls * self.difficulty / 100;

        while has_to_remove != 0 {
            let random_index = thread_rng().gen_range(0..self.cells.len());
            self.current = random_index;
            let next = self.find_next_cell(true);

            if let Some(next) = next {
                let (lower_part, higher_part) =
                    self.cells.split_at_mut(std::cmp::max(self.current, next));
                let cell1 = &mut lower_part[std::cmp::min(self.current, next)];
                let cell2 = &mut higher_part[0];
                if cell1.remove_wall(cell2) == 1 {
                    self.walls = self.walls - 1;
                    has_to_remove = has_to_remove - 1;
                }
            }
        }
    }
    //converts array of cells to matrix
    pub fn convert_to_map(&self) -> Map {
        let mut map = Map::new((self.width * 2 + 1) as usize, (self.height * 2 + 1) as usize);
        for i in 0..self.cells.len() {
            let cell = self.cells[i].walls;
            let row = self.cells[i].row as usize;
            let col = self.cells[i].col as usize;
            if cell[TOP] {
                map.0[row * 2][col * 2] = WALL;
                map.0[row * 2][col * 2 + 1] = WALL;
                map.0[row * 2][col * 2 + 2] = WALL;
            }
            if cell[RIGHT] {
                map.0[row * 2][col * 2 + 2] = WALL;
                map.0[row * 2+ 1][col * 2 + 2] = WALL;
                map.0[row * 2 + 2][col * 2 + 2] = WALL;
            }
            if cell[BOTTOM] {
                map.0[row * 2 + 2][col * 2] = WALL;
                map.0[row * 2 + 2][col * 2 + 1] = WALL;
                map.0[row * 2 + 2][col * 2 + 2] = WALL;
            }
            if cell[LEFT] {
                map.0[row * 2][col * 2] = WALL;
                map.0[row * 2+ 1][col * 2] = WALL;
                map.0[row * 2 + 2][col * 2] = WALL;
            }
        };
        return map;
    }

    pub fn draw(&self) {
        let size: f32 = 10.0;
        let offset: f32 = 100.0;
        for ind in 0..self.cells.len() {
            let row = self.cells[ind].row as f32;
            let col = self.cells[ind].col as f32;
            if self.cells[ind].walls[TOP] {
                draw_line(col * size + offset, row * size + offset, (col + 1.0) * size + offset, row * size + offset, 1.0, WHITE);
            }
            if self.cells[ind].walls[RIGHT] {
                draw_line((col + 1.0) * size + offset, row * size + offset, (col + 1.0) * size + offset, (row + 1.0) * size + offset, 1.0, WHITE);
            }
            if self.cells[ind].walls[BOTTOM] {
                draw_line(col * size + offset, (row + 1.0) * size + offset, (col + 1.0) * size + offset, (row + 1.0) * size + offset, 1.0, WHITE);
            }
            if self.cells[ind].walls[LEFT] {
                draw_line(col * size + offset, row * size + offset, col * size + offset, (row + 1.0) * size + offset, 1.0, WHITE);
            }
        }
    }
}
