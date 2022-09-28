use crate::def::Command;
use crate::neighborhood::Neighborhood;

pub trait IState {
    fn get_score(&self, progress: f64) -> f64;
    fn perform_command(&mut self, command: &Command) -> Vec<Command>;
    fn reverse_command(&mut self, command: &Command);
}

pub trait INeighborhoodSelector {
    fn select(&self) -> Neighborhood;
    fn step(&mut self, neighborhood: &Neighborhood, adopted: bool);
}

pub trait IOptimizer {
    fn update_temp(&mut self, progress: f64);
    fn should_adopt_new_state(&self, score_diff: f64) -> bool;
}

pub trait ISolver {
    fn solve(&mut self, time_limit: f64);
}
