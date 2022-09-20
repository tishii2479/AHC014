const TIME_LIMIT: f64 = 4.95;

const DELETION_RECURSION_LIMIT: usize = 10;
const START_TEMP: f64 = 500.;
const END_TEMP: f64 = 0.;

mod def; // expand
mod framework; // expand
mod grid; // expand
mod lib; // expand
mod state; // expand

use std::{fs, io::Write};

use def::*;
use framework::*;
use lib::*;
use proconio::input;
use state::*;

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
        // if p < 0.1 {
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
    fn get_score(&self, progress: f64) -> f64 {
        self.score.get_score(progress)
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
            let mut neighborhood = self.neighborhood_selector.select();

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

            self.state.score.temporary = 0;

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

impl Neighborhood {
    fn perform(&mut self, state: &mut State) -> Vec<Command> {
        match self {
            Neighborhood::Add => self.perform_add(state),
            Neighborhood::Delete => self.perform_delete(state),
            Neighborhood::ChangeSquare => self.perform_change_square(state),
        }
    }

    fn perform_add(&mut self, state: &mut State) -> Vec<Command> {
        let selected_p = state.points[rnd::gen_range(0, state.points.len()) as usize].clone();
        self.attempt_add(state, &selected_p, None)
    }

    fn attempt_add(
        &mut self,
        state: &mut State,
        pos: &Pos,
        ignore_dir: Option<&Dir>,
    ) -> Vec<Command> {
        assert!(state.grid.has_point(&pos));
        let point = state.grid.point(&pos).as_ref().unwrap().clone();

        for _ in 0..DIR_MAX {
            let i = rnd::gen_range(0, DIR_MAX);
            let diagonal_dir = Dir::from_i64(i as i64);
            if let Some(ignore_dir) = ignore_dir {
                if ignore_dir == &diagonal_dir {
                    continue;
                }
            }

            let dir_next = diagonal_dir.next();
            let dir_prev = diagonal_dir.prev();

            if let (Some(pos_prev), Some(pos_next)) = (
                &point.nearest_points[dir_prev.val() as usize],
                &point.nearest_points[dir_next.val() as usize],
            ) {
                let new_pos = pos_next + &(pos_prev - &pos);

                if !state.grid.is_valid(&new_pos) {
                    continue;
                }
                if state.grid.has_point(&new_pos) {
                    continue;
                }

                let connect: [Pos; 2] = [pos_prev.clone(), pos_next.clone()];
                let square = Square::new(new_pos, pos.clone(), connect);

                let performed_commands = state.perform_command(&Command::Add { square });
                if performed_commands.len() > 0 {
                    return performed_commands;
                }
            }
        }
        return vec![];
    }

    fn perform_delete(&mut self, state: &mut State) -> Vec<Command> {
        let selected_p = state.points[rnd::gen_range(0, state.points.len()) as usize].clone();
        self.attemp_delete(state, &selected_p)
    }

    fn attemp_delete(&mut self, state: &mut State, pos: &Pos) -> Vec<Command> {
        assert!(state.grid.has_point(&pos));
        let point = state.grid.point(&pos).as_ref().unwrap().clone();
        if let Some(added_info) = point.added_info {
            return state.perform_command(&Command::Delete { square: added_info });
        }
        return vec![];
    }

    fn perform_change_square(&mut self, state: &mut State) -> Vec<Command> {
        // 四角を作っている点を探す
        let selected_p = state.points[rnd::gen_range(0, state.points.len()) as usize].clone();
        self.attempt_change_square(state, &selected_p)
    }

    fn attempt_change_square(&mut self, state: &mut State, pos: &Pos) -> Vec<Command> {
        assert!(state.grid.has_point(&pos));
        let point = state.grid.point(&pos).as_ref().unwrap().clone();

        for _ in 0..DIR_MAX {
            let i = rnd::gen_range(0, DIR_MAX);
            let front = Dir::from_i64(i as i64);
            let left = front.prev().prev();
            if state.grid.has_edge(&pos, &left) && state.grid.has_edge(&pos, &front) {
                let left_pos = point.nearest_points[left.val() as usize].as_ref().unwrap();
                let front_pos = point.nearest_points[front.val() as usize].as_ref().unwrap();
                let left_front_pos = &(&(left_pos + front_pos) - pos);
                if !state.grid.is_valid(left_front_pos) {
                    continue;
                }
                if let Some(left_front_point) = state.grid.point(left_front_pos).clone() {
                    if !left_front_point.is_added {
                        continue;
                    }
                    let added_square = left_front_point.added_info.as_ref().unwrap();
                    let mut performed_commands = state.perform_command(&Command::Delete {
                        square: added_square.clone(),
                    });

                    // 再帰的にposの点も消してしまった時は中止
                    if !state.grid.has_point(&pos) {
                        return performed_commands;
                    }

                    performed_commands.append(&mut self.attempt_add(
                        state,
                        &pos,
                        Some(&front.prev()),
                    ));
                    return performed_commands;
                }
            }
        }

        return vec![];
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

fn calc_weight(n: i64, pos: &Pos) -> i64 {
    let c = ((n - 1) / 2) as i64;
    (pos.y as i64 - c) * (pos.y as i64 - c) + (pos.x as i64 - c) * (pos.x as i64 - c) + 1
}

fn calc_real_score(n: usize, m: usize, score: i64) -> i64 {
    let mut s = 0;
    for i in 0..n {
        for j in 0..n {
            s += calc_weight(
                n as i64,
                &Pos {
                    x: i as i64,
                    y: j as i64,
                },
            );
        }
    }
    let result = 1e6 * (n as f64 * n as f64) * score as f64 / (m as f64 * s as f64);
    result.round() as i64
}
