use std::iter;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq, Clone)]
pub struct MemoryLayout {
    available_cells: VecDeque<bool>,
    named_cells: HashMap<String, Cell>,
}

#[derive(Debug, PartialEq, Clone)]
struct Cell {
    position: usize,
    size: usize,
}

impl MemoryLayout {
    /// Creates a new blank memory layout
    pub fn new() -> MemoryLayout {
        MemoryLayout {
            available_cells: VecDeque::new(),
            named_cells: HashMap::new(),
        }
    }

    /// Returns the (position, size) of a cell with the given name
    pub fn get_cell_contents(&self, name: &str) -> Option<(usize, usize)> {
        self.named_cells.get(name).map(|c| (c.position, c.size))
    }

    pub fn is_declared(&self, name: &str) -> bool {
        self.named_cells.contains_key(name)
    }

    pub fn declare(&mut self, name: &str, size: usize) -> usize {
        let mut current = 0;
        'search: loop {
            self.ensure_available_cells(current + size);

            if self.available_cells[current] {
                for i in 1..size {
                    if !self.available_cells[current + i] {
                        current += i;
                        continue 'search;
                    }
                }

                self.reserve(current, size);
                break;
            }
            else {
                current += 1;
            }
        }

        self.named_cells.insert(name.to_owned(), Cell {
            position: current,
            size: size,
        });

        current
    }

    fn reserve(&mut self, start: usize, size: usize) {
        for i in 0..size {
            self.available_cells[start + i] = false;
        }
    }

    pub fn next_available_cell(&mut self) -> usize {
        let mut current = 0;
        loop {
            self.ensure_available_cells(current);

            if self.available_cells[current] {
                break;
            }

            current += 1;
        }

        current
    }

    /// Ensures the available_cells collection is at least
    /// the given size + 1
    pub fn ensure_available_cells(&mut self, size: usize) {
        if size >= self.available_cells.len() {
            let needed = size - self.available_cells.len() + 1;
            self.available_cells.extend(iter::repeat(true).take(needed));
        }
    }
}
