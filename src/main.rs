mod def; // expand
mod lib; // expand

use def::*;
use lib::*;
use proconio::input;

impl State {
    fn perform_command(&mut self, command: &Command) -> bool {
        match command {
            Command::Add {
                new_pos,
                diagonal,
                connect,
            } => self.add_point(new_pos, diagonal, connect),
            Command::Delete { pos: _ } => panic!("Not implemented"),
        }
    }

    fn add_point(&mut self, new_pos: &Pos, diagonal: &Pos, connect: &[Pos; 2]) -> bool {
        assert!(Pos::is_aligned(diagonal, &connect[0]));
        assert!(Pos::is_aligned(diagonal, &connect[1]));
        assert!(Pos::is_aligned(new_pos, &connect[0]));
        assert!(Pos::is_aligned(new_pos, &connect[1]));

        // new_posに既に点がないか確認
        if self.grid.has_point(&new_pos) {
            return false;
        }

        // 作ろうとしてる四角の辺に既に点、辺がないか確認する
        if !self.grid.can_connect(&connect[0], new_pos)
            || !self.grid.can_connect(&connect[1], new_pos)
            || !self.grid.can_connect(&connect[0], diagonal)
            || !self.grid.can_connect(&connect[1], diagonal)
        {
            return false;
        }

        // 点を追加する
        self.grid.set_point(new_pos, Point::new(new_pos, true));

        // 辺を追加する
        self.grid.connect(&connect[0], new_pos);
        self.grid.connect(&connect[1], new_pos);
        self.grid.connect(&connect[0], diagonal);
        self.grid.connect(&connect[1], diagonal);

        // nearest_pointsを付け替える
        self.grid.recognize(&connect[0], new_pos);
        self.grid.recognize(&connect[1], new_pos);

        // eprintln!(
        //     "Connected: {:?}, {:?}, {:?}, {:?}",
        //     new_pos, &connect[0], diagonal, &connect[1]
        // );

        self.squares.push((
            new_pos.clone(),
            connect[0].clone(),
            diagonal.clone(),
            connect[1].clone(),
        ));

        return true;
    }
}

#[test]
fn test_add_point() {
    let diagonal = Pos { x: 0, y: 0 };
    let connect: [Pos; 2] = [Pos { x: 2, y: 0 }, Pos { x: 0, y: 2 }];
    let other = Pos { x: 2, y: 4 };
    let new_pos = Pos { x: 2, y: 2 };
    let n: usize = 5;
    let p = vec![
        diagonal.clone(),
        connect[0].clone(),
        connect[1].clone(),
        other.clone(),
    ];

    let mut state = State::new(n, p);
    assert!(state.add_point(&new_pos, &diagonal, &connect));
    assert!(state.grid.point(&new_pos).is_some());

    assert!(state.grid.has_edge(&Pos { x: 1, y: 2 }, &Dir::Left));
    assert!(state.grid.has_edge(&Pos { x: 1, y: 2 }, &Dir::Right));

    match state.grid.point(&connect[0]) {
        Some(point_other) => {
            assert_eq!(
                point_other.nearest_points[Dir::Down.val() as usize],
                Some(Pos { x: 2, y: 2 })
            );
        }
        None => assert!(false),
    }

    match state.grid.point(&new_pos) {
        Some(point_new_pos) => {
            assert_eq!(
                point_new_pos.nearest_points[Dir::Left.val() as usize],
                Some(Pos { x: 0, y: 2 })
            );
            assert_eq!(
                point_new_pos.nearest_points[Dir::Down.val() as usize],
                Some(Pos { x: 2, y: 4 })
            );
            assert_eq!(
                point_new_pos.nearest_points[Dir::Up.val() as usize],
                Some(Pos { x: 2, y: 0 })
            );
        }
        None => assert!(false),
    }

    match state.grid.point(&other) {
        Some(point_other) => {
            assert_eq!(
                point_other.nearest_points[Dir::Up.val() as usize],
                Some(Pos { x: 2, y: 2 })
            );
        }
        None => assert!(false),
    }
}

fn main() {
    time::start_clock();

    input! {
        n: usize,
        m: usize,
        p: [Pos; m],
    }

    let mut state = State::new(n, p);

    while time::elapsed_seconds() < 1. {
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

                if state.grid.has_point(&new_pos) {
                    continue;
                }

                let connect: [Pos; 2] = [pos_prev.clone(), pos_next.clone()];
                let add = Command::Add {
                    new_pos,
                    diagonal: selected_p,
                    connect,
                };
                state.perform_command(&add);
                break;
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
