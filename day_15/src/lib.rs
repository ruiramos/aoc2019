pub mod computer;
pub mod stateless_computer;
pub mod vis;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum PositionType {
    Wall,
    Open,
    Robot,
    Goal,
}

pub enum InputMode {
    Stdin,
    Args,
}

#[derive(Debug, PartialEq)]
pub enum ComputerState {
    Running,
    Terminated,
}
