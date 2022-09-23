const TIME_LIMIT: f64 = 4.95;

const DEFAULT_DIST: i64 = 5;
const POINT_PENALTY: i64 = 100;
const DELETION_RECURSION_LIMIT: usize = 10;
const START_TEMP: f64 = 500.;
const END_TEMP: f64 = 0.;

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
            return Neighborhood::Delete;
        }
        // if p < 0.15 {
        //     return Neighborhood::SplitSquare;
        // }
        // if p < 0.20 {
        //     return Neighborhood::ChangeSquare;
        // }
        return Neighborhood::Add;
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
        // let point_closeness_score = self.score.point_closeness as f64;
        // let threshold = progress * 3. * DEFAULT_DIST as f64 * self.points.len() as f64;
        let point_penalty_score = (self.score.point_penalty * POINT_PENALTY) as f64;
        // let edge_length_score = self.score.edge_length as f64 * 100.;
        // base_score + point_closeness_score - threshold
        // eprintln!(
        //     "{} {}",
        //     base_score,
        //     self.score.point_penalty * POINT_PENALTY
        // );
        // base_score + point_penalty_score * (1. - progress)
        // base_score + point_penalty_score
        point_penalty_score
        // base_score
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
        while time::elapsed_seconds() < time_limit {
            let progress = time::elapsed_seconds() / time_limit;
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

            if cfg!(debug_assertions) {
                if loop_count % 100 == 0 {
                    self.score_history.push(self.state.score.base as f64);
                }
            }
            loop_count += 1;
        }

        // if cfg!(debug_assertions) {
        eprintln!("loop_count: {}", loop_count);
        // }
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

        if cfg!(debug_assertions) {
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

    let state = State::new(n, p);
    let mut solver = Solver {
        state,
        neighborhood_selector: NeighborhoodSelector::new(Neighborhood::all().len()),
        optimizer: Optimizer {
            start_temp: START_TEMP,
            end_temp: END_TEMP,
        },
        score_history: vec![],
    };

    solver.solve(TIME_LIMIT);
    solver.output();

    solver.output_statistics(n, m);
    eprintln!("run_time: {}", time::elapsed_seconds());
}
