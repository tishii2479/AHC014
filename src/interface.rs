use crate::def::*; // ignore

pub trait IState {
    fn get_score(&self) -> f64;
    fn perform_command(&mut self, command: &Command) -> bool;
    fn reverse_command(&mut self, command: &Command);
}

pub trait INeighborhoodSelector {
    fn select(&self) -> Neighborhood;
}

pub trait IOptimizer {
    fn should_adopt_new_state(&self, score_diff: f64, progress: f64) -> bool;
}

pub trait ISolver {
    fn solve(&mut self, time_limit: f64);
    fn perform_neighborhood(&mut self, neighborhood: Neighborhood) -> Vec<Command>;
}
