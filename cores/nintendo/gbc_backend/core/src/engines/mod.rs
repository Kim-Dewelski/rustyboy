pub mod interpreter;
pub mod jit;

pub trait Engine {
    type EngineData: Default;
}
