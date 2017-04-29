#[derive(Debug, Clone, Copy)]
pub enum OptimizationLevel {
    // May be room for more optimization levels later
    Off,
    On,
}

pub trait Optimize {
    fn optimize(self, level: OptimizationLevel) -> Self;
}
