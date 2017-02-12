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

    /// Returns whether a given name has been declared yet
    pub fn is_declared(&self, name: &str) -> bool {
        self.named_cells.contains_key(name)
    }

    /// Declares a variable with the given name and size
    /// Returns the position of that variable in the memory
    pub fn declare(&mut self, name: &str, size: usize) -> usize {
        let position = self.allocate(size);

        self.named_cells.insert(name.to_owned(), Cell {
            position: position,
            size: size,
        });

        position
    }

    /// Allocates a set of consecutive cells of the given size
    pub fn allocate(&mut self, size: usize) -> usize {
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

        current
    }

    fn reserve(&mut self, start: usize, size: usize) {
        for i in 0..size {
            self.available_cells[start + i] = false;
        }
    }

    /// Undeclares a name and frees the used space
    /// If found, returns the (position, size) of the name
    /// Otherwise, returns None
    pub fn undeclare(&mut self, name: &str) -> Option<(usize, usize)> {
        self.named_cells.remove(name).and_then(|Cell {position, size}| {
            self.free(position, size);
            Some((position, size))
        })
    }

    pub fn free(&mut self, start: usize, size: usize) {
        for i in 0..size {
            self.available_cells[start + i] = true;
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
