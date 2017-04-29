#[derive(Debug, Clone, Copy)]
pub enum OptimizationLevel {
    // The higher the optimization level, the more aggressive the optimizations
    Off,
    L1,
    L2,
}

pub trait Optimize {
    fn optimize(self, level: OptimizationLevel) -> Self;
}
