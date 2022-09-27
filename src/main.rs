const TIME_LIMIT: f64 = 4.98;
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
fn calc_start_temp(n: usize, m: usize) -> f64 {
    500. * (n as f64 / 30.).powf(2.)
}

#[allow(unused_variables)]
fn calc_end_temp(n: usize, m: usize) -> f64 {
    25. * (n as f64 / 30.).powf(2.)
}

struct NeighborhoodSelector {
    total_cnt: Vec<i64>,
    adopted_cnt: Vec<i64>,
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
                Neighborhood::from_i64(i as i64),
                self.total_cnt[i],
                self.adopted_cnt[i],
                self.adopted_cnt[i] as f64 / self.total_cnt[i] as f64,
            );
        }
    }
}

impl INeighborhoodSelector for NeighborhoodSelector {
    fn select(&self) -> Neighborhood {
        let p = rnd::nextf();
        if p < 0.1 {
            Neighborhood::Delete
        } else if p < 0.2 {
            Neighborhood::ChangeSquare
        } else if p < 0.3 {
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
    start_temp: f64,
    end_temp: f64,
}

impl IOptimizer for Optimizer {
    fn should_adopt_new_state(&self, score_diff: f64, progress: f64) -> bool {
        let temp = self.start_temp + (self.end_temp - self.start_temp) * progress;
        let prob = (score_diff / temp).exp();
        return prob > rnd::nextf();
    }
}

impl IState for State {
    #[allow(unused_variables)]
    fn get_score(&self, progress: f64) -> f64 {
        let base_score = self.score.base as f64;
        let additional_score = self.score.additional as f64;
        base_score
    }

    fn perform_command(&mut self, command: &Command) -> Vec<Command> {
        match command {
            Command::Add { square } => self.perform_add(square, false),
            Command::Delete { square } => {
                // 削除する四角が多すぎるときは不採用
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
    score_history: Vec<f64>,
}

impl ISolver for Solver {
    fn solve(&mut self, time_limit: f64) {
        let mut loop_count = 0;
        let mut progress = time::elapsed_seconds() / time_limit;
        while progress < 1. {
            if loop_count % LOOP_INTERVAL == 0 {
                progress = time::elapsed_seconds() / time_limit;
            }
            let neighborhood = self.neighborhood_selector.select();

            let current_score = self.state.get_score(progress);

            let performed_commands = neighborhood.perform(&mut self.state);

            let new_score = self.state.get_score(progress);

            let adopt_new_state = self
                .optimizer
                .should_adopt_new_state(new_score - current_score, progress)
                && performed_commands.len() > 0;

            if !adopt_new_state {
                for command in performed_commands.iter().rev() {
                    self.state.reverse_command(command);
                }
            }

            self.neighborhood_selector
                .step(&neighborhood, adopt_new_state);

            if WRITE_SCORE_LOG {
                if loop_count % 100 == 0 {
                    self.score_history.push(self.state.score.base as f64);
                }
            }
            loop_count += 1;
        }

        eprintln!("loop_count: {}", loop_count);
    }
}

impl Solver {
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

    fn output_statistics(&self, n: usize, m: usize) {
        eprintln!("state_score: {}", self.state.get_score(1.));
        eprintln!(
            "real_score: {}",
            calc_real_score(n, m, self.state.score.base as i64)
        );
        self.neighborhood_selector.output_statistics();

        if WRITE_SCORE_LOG {
            // スコア遷移の書き出し
            let mut file = fs::File::create("tools/out/score_log.txt").unwrap();
            for score in &self.score_history {
                let score = calc_real_score(n, m, *score as i64);
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

    let start_temp: f64 = calc_start_temp(n, m);
    let end_temp: f64 = calc_end_temp(n, m);

    let state = State::new(n, p);
    let mut solver = Solver {
        state,
        neighborhood_selector: NeighborhoodSelector::new(Neighborhood::all().len()),
        optimizer: Optimizer {
            start_temp,
            end_temp,
        },
        score_history: vec![],
    };

    solver.solve(TIME_LIMIT);
    solver.output();

    solver.output_statistics(n, m);
    eprintln!("run_time: {}", time::elapsed_seconds());
}
