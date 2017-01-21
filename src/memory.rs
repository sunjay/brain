use std::collections::{HashMap, VecDeque};

pub struct MemoryLayout {
    current: usize,
    available_cells: VecDeque<bool>,
    named_cells: HashMap<String, usize>,
}

impl MemoryLayout {
    pub fn new() -> MemoryLayout {
        MemoryLayout {
            current: 0,
            available_cells: VecDeque::new(),
            named_cells: HashMap::new(),
        }
    }

    pub fn current_cell(&self) -> usize {
        self.current
    }

    pub fn next_available_cell(&mut self) -> usize {
        let mut current = 0;
        loop {
            while current >= self.available_cells.len() {
                self.available_cells.push_back(true);
            }

            if self.available_cells[current] {
                break;
            }

            current += 1;
        }

        current
    }
}
