const TIME_LIMIT: f32 = 4.97;
const LOOP_INTERVAL: usize = 100;
const WRITE_SCORE_LOG: bool = false;

const MULTIPLE_ADD_RECURSION_LIMIT: usize = 20;
const DELETION_RECURSION_LIMIT: usize = 10;

mod def; // expand
mod framework; // expand
mod grid; // expand
mod neighborhood; // expand
mod state; // expand
mod util; // expand

use std::{fs, io::Write};

use def::*;
use framework::*;
use neighborhood::*;
use proconio::input;
use state::*;
use util::*;

#[allow(unused_variables)]
fn calc_start_temp(n: usize, m: usize) -> f32 {
    500. * (n as f32 / 30.).powf(2.)
}

#[allow(unused_variables)]
fn calc_end_temp(n: usize, m: usize) -> f32 {
    25. * (n as f32 / 30.).powf(2.)
}

struct NeighborhoodSelector {
    total_cnt: Vec<i32>,
    adopted_cnt: Vec<i32>,
}

impl NeighborhoodSelector {
    fn new(neighborhood_count: usize) -> NeighborhoodSelector {
        NeighborhoodSelector {
            total_cnt: vec![0; neighborhood_count],
            adopted_cnt: vec![0; neighborhood_count],
        }
    }

    fn output_statistics(&self) {
        for i in 0..self.total_cnt.len() {
            eprintln!(
                "{:?}: (total_cnt: {}, adopted_cnt: {}, adopted_ratio: {:.4})",
                Neighborhood::from_i32(i as i32),
                self.total_cnt[i],
                self.adopted_cnt[i],
                self.adopted_cnt[i] as f32 / self.total_cnt[i] as f32,
            );
        }
    }
}

impl INeighborhoodSelector for NeighborhoodSelector {
    fn select(&self) -> Neighborhood {
        let p = rnd::nextf();
        if p < 0.05 {
            Neighborhood::Delete
        } else if p < 0.15 {
            Neighborhood::ChangeSquare
        } else if p < 0.25 {
            Neighborhood::SplitSquare
        } else {
            Neighborhood::Add
        }
    }

    fn step(&mut self, neighborhood: &Neighborhood, adopted: bool) {
        self.total_cnt[*neighborhood as usize] += 1;
        if adopted {
            self.adopted_cnt[*neighborhood as usize] += 1;
        }
    }
}

struct Optimizer {
    start_temp: f32,
    end_temp: f32,
    current_temp: f32,
}

impl IOptimizer for Optimizer {
    fn update_temp(&mut self, progress: f32) {
        self.current_temp = self.start_temp + (self.end_temp - self.start_temp) * progress;
    }

    fn should_adopt_new_state(&self, score_diff: f32) -> bool {
        let prob = (score_diff / self.current_temp).exp();
        return prob > rnd::nextf();
    }
}

impl Optimizer {
    fn new(start_temp: f32, end_temp: f32) -> Optimizer {
        let mut optimizer = Optimizer {
            start_temp,
            end_temp,
            current_temp: 0.,
        };
        optimizer.update_temp(0.);
        optimizer
    }
}

impl IState for State {
    #[allow(unused_variables)]
    fn get_score(&self, progress: f32) -> f32 {
        let base_score = self.score.base as f32;
        base_score
    }

    fn perform_command(&mut self, command: &Command) -> Vec<Command> {
        match command {
            Command::Add { square } => self.perform_add(square, false),
            Command::Delete { square } => {
                // ???????????????????????????????????????????????????
                if self.calc_deletion_size(&square.new_pos, DELETION_RECURSION_LIMIT, 0)
                    >= DELETION_RECURSION_LIMIT
                {
                    return vec![];
                }

                let mut performed_commands: Vec<Command> = vec![];
                self.perform_delete(square, &mut performed_commands);
                performed_commands
            }
        }
    }

    fn reverse_command(&mut self, command: &Command) {
        match command {
            Command::Add { square } => {
                let mut performed_commands: Vec<Command> = vec![];
                self.perform_delete(square, &mut performed_commands);
                performed_commands
            }
            Command::Delete { square } => self.perform_add(square, true),
        };
    }
}

struct Solver {
    state: State,
    neighborhood_selector: NeighborhoodSelector,
    optimizer: Optimizer,
    score_history: Vec<f32>,
}

impl ISolver for Solver {
    fn solve(&mut self, time_limit: f32) {
        let mut loop_count = 0;
        let mut best_state = self.state.clone();
        let mut progress = time::elapsed_seconds() as f32 / time_limit;
        while progress < 1. {
            let is_interval = (loop_count % LOOP_INTERVAL) == 0;
            if is_interval {
                progress = time::elapsed_seconds() as f32 / time_limit;
                self.optimizer.update_temp(progress);
            }

            let neighborhood = self.neighborhood_selector.select();

            let current_score = self.state.get_score(progress);

            let performed_commands = neighborhood.perform(&mut self.state);

            let new_score = self.state.get_score(progress);

            let adopt_new_state = self
                .optimizer
                .should_adopt_new_state(new_score - current_score)
                && performed_commands.len() > 0;

            if !adopt_new_state {
                for command in performed_commands.iter().rev() {
                    self.state.reverse_command(command);
                }
            }

            self.neighborhood_selector
                .step(&neighborhood, adopt_new_state);

            if is_interval && self.state.get_score(1.) > best_state.get_score(1.) {
                best_state = self.state.clone();
            }
            loop_count += 1;
        }
        eprintln!("loop_count: {}", loop_count);
        self.state = best_state.clone();
    }
}

impl Solver {
    fn new(
        state: State,
        neighborhood_selector: NeighborhoodSelector,
        optimizer: Optimizer,
    ) -> Solver {
        Solver {
            state: state.clone(),
            neighborhood_selector,
            optimizer,
            score_history: vec![],
        }
    }

    fn output(&mut self) {
        println!("{}", self.state.squares.len());
        self.state.squares.sort_by(|a, b| a.id.cmp(&b.id));
        for Square {
            id: _,
            new_pos,
            diagonal,
            connect,
        } in &self.state.squares
        {
            println!(
                "{} {} {} {} {} {} {} {}",
                new_pos.x,
                new_pos.y,
                connect[0].x,
                connect[0].y,
                diagonal.x,
                diagonal.y,
                connect[1].x,
                connect[1].y
            );
        }
    }

    #[allow(dead_code)]
    fn output_statistics(&self, n: usize, m: usize) {
        eprintln!("state_score: {}", self.state.get_score(1.));
        eprintln!(
            "real_score: {}",
            calc_real_score(n, m, self.state.score.base as i32)
        );
        self.neighborhood_selector.output_statistics();

        if WRITE_SCORE_LOG {
            // ??????????????????????????????
            let mut file = fs::File::create("tools/out/score_log.txt").unwrap();
            for score in &self.score_history {
                let score = calc_real_score(n, m, *score as i32);
                file.write((score.to_string() + "\n").as_bytes()).unwrap();
            }
        }
    }
}

fn main() {
    time::start_clock();

    input! {
        n: usize,
        m: usize,
        p: [Pos; m]
    }

    let start_temp: f32 = calc_start_temp(n, m);
    let end_temp: f32 = calc_end_temp(n, m);

    let state = State::new(n, p);
    let mut solver = Solver::new(
        state,
        NeighborhoodSelector::new(Neighborhood::all().len()),
        Optimizer::new(start_temp, end_temp),
    );

    solver.solve(TIME_LIMIT);
    solver.output();
    solver.output_statistics(n, m);
}
