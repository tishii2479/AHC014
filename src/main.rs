mod def; // expand
mod framework; // expand
mod grid; // expand
mod lib; // expand
mod state; // expand

use def::*;
use framework::*;
use lib::*;
use proconio::input;
use state::*;

struct NeighborhoodSelector;

impl INeighborhoodSelector for NeighborhoodSelector {
    fn select(&self) -> Neighborhood {
        return Neighborhood::Add;
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
    fn get_score(&self) -> f64 {
        self.score.get_score()
    }

    fn perform_command(&mut self, command: &Command) -> Vec<Command> {
        match command {
            Command::Add { square } => self.perform_add(square),
            Command::Delete { square } => {
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
            Command::Delete { square } => self.perform_add(square),
        };
    }
}

struct Solver {
    state: State,
    neighborhood_selector: NeighborhoodSelector,
    optimizer: Optimizer,
}

impl ISolver for Solver {
    fn solve(&mut self, time_limit: f64) {
        let mut loop_count = 0;
        while time::elapsed_seconds() < time_limit {
            let progress = time::elapsed_seconds() / time_limit;
            let neighborhood = self.neighborhood_selector.select();

            let current_score = self.state.get_score();

            let performed_commands = self.perform_neighborhood(neighborhood);

            let new_score = self.state.get_score();

            let adopt_new_state = self
                .optimizer
                .should_adopt_new_state(new_score - current_score, progress);

            if !adopt_new_state {
                for command in performed_commands.iter().rev() {
                    self.state.reverse_command(command);
                }
            }
            loop_count += 1;
        }
        eprintln!("loop_count: {}", loop_count);
    }

    fn perform_neighborhood(&mut self, neighborhood: Neighborhood) -> Vec<Command> {
        let mut performed_commands: Vec<Command> = vec![];
        match neighborhood {
            Neighborhood::Add => {
                let selected_p =
                    self.state.points[rnd::gen_range(0, self.state.points.len()) as usize].clone();
                let point = self.state.grid.point(&selected_p).as_ref().unwrap().clone();

                // TODO: randomize
                for i in 0..DIR_MAX {
                    let diagonal_dir = Dir::from_i64(i as i64);
                    let dir_next = diagonal_dir.next();
                    let dir_prev = diagonal_dir.prev();

                    if let (Some(pos_prev), Some(pos_next)) = (
                        &point.nearest_points[dir_prev.val() as usize],
                        &point.nearest_points[dir_next.val() as usize],
                    ) {
                        let new_pos = pos_next + &(pos_prev - &selected_p);

                        if !self.state.grid.is_valid(&new_pos) {
                            continue;
                        }
                        if self.state.grid.has_point(&new_pos) {
                            continue;
                        }

                        let connect: [Pos; 2] = [pos_prev.clone(), pos_next.clone()];
                        let add = Command::Add {
                            square: Square {
                                new_pos,
                                diagonal: selected_p.clone(),
                                connect,
                            },
                        };

                        performed_commands = self.state.perform_command(&add);
                        if performed_commands.len() > 0 {
                            break;
                        }
                    }
                }
            }
            Neighborhood::Delete => {
                panic!("Not implemented");
            }
        }
        return performed_commands;
    }
}

impl Solver {
    fn output(&self) {
        println!("{}", self.state.squares.len());
        for Square {
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
}

fn main() {
    time::start_clock();

    const TIME_LIMIT: f64 = 1.9;
    input! {
        n: usize,
        m: usize,
        p: [Pos; m]
    }

    let state = State::new(n, p);
    let mut solver = Solver {
        state,
        neighborhood_selector: NeighborhoodSelector {},
        optimizer: Optimizer {
            start_temp: 5000.,
            end_temp: 0.,
        },
    };

    solver.solve(TIME_LIMIT);
    solver.output();

    if cfg!(debug_assertions) {
        eprintln!("state_score: {}", solver.state.get_score());
        eprintln!(
            "real_score: {}",
            calc_real_score(n, m, solver.state.get_score() as i64)
        );
        eprintln!("run_time: {}", time::elapsed_seconds());
    }
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
