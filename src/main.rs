mod def; // expand
mod lib; // expand

use def::*;
use lib::*;
use proconio::input;

fn main() {
    time::start_clock();
    input! {
        n: usize,
        m: usize,
        p: [Pos; m]
    }

    let mut state = State::new(n, p);

    while time::elapsed_seconds() < 2. {
        let selected_p = state.points[rnd::gen_range(0, state.points.len()) as usize].clone();
        let point = state.grid.point(&selected_p).as_ref().unwrap().clone();

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

                if !state.grid.is_valid(&new_pos) {
                    continue;
                }
                if state.grid.has_point(&new_pos) {
                    continue;
                }

                let connect: [Pos; 2] = [pos_prev.clone(), pos_next.clone()];
                let add = Command::Add {
                    new_pos,
                    diagonal: selected_p.clone(),
                    connect,
                };

                if state.perform_command(&add) {
                    break;
                }
            }
        }
    }

    println!("{}", state.squares.len());
    for (p1, p2, p3, p4) in state.squares {
        println!(
            "{} {} {} {} {} {} {} {}",
            p1.x, p1.y, p2.x, p2.y, p3.x, p3.y, p4.x, p4.y
        );
    }
    eprintln!("run_time: {}", time::elapsed_seconds());
}
