use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Neighborhood {
    Add = 0,
    Delete = 1,
    ChangeSquare = 2,
    SplitSquare = 3,
    MultipleAdd = 4,
}

impl Neighborhood {
    pub fn perform(&self, state: &mut State) -> Vec<Command> {
        match self {
            Neighborhood::Add => Neighborhood::perform_add(state),
            Neighborhood::Delete => Neighborhood::perform_delete(state),
            Neighborhood::ChangeSquare => Neighborhood::perform_change_square(state),
            Neighborhood::SplitSquare => Neighborhood::perform_split_square(state),
            Neighborhood::MultipleAdd => Neighborhood::perform_multiple_add(state),
        }
    }

    fn perform_multiple_add(state: &mut State) -> Vec<Command> {
        let selected_p = state.points[rnd::gen_range(0, state.points.len()) as usize].clone();
        let mut performed_commands = vec![];
        let mut recursion_count = 0;
        Neighborhood::attempt_multiple_add(
            state,
            &selected_p,
            &mut recursion_count,
            &MULTIPLE_ADD_RECURSION_LIMIT,
            &mut performed_commands,
            state.get_score(1.),
        );
        performed_commands
    }

    fn attempt_multiple_add(
        state: &mut State,
        pos: &Pos,
        recursion_count: &mut usize,
        recursion_limit: &usize,
        performed_commands: &mut Vec<Command>,
        start_score: f64,
    ) {
        if state.get_score(1.) > start_score {
            return;
        }
        if *recursion_count >= *recursion_limit {
            return;
        }
        *recursion_count += 1;
        for _ in 0..DIR_MAX {
            let i = rnd::gen_range(0, DIR_MAX);
            let dir = Dir::from_i64(i as i64);
            if let Some(nearest_pos) =
                state.grid.point(&pos).as_ref().unwrap().nearest_points[dir.val() as usize]
            {
                let mut add = Neighborhood::attempt_add(state, &nearest_pos, None);
                performed_commands.append(&mut add);
                Neighborhood::attempt_multiple_add(
                    state,
                    &nearest_pos,
                    recursion_count,
                    recursion_limit,
                    performed_commands,
                    start_score,
                );
            }
        }
    }

    fn perform_add(state: &mut State) -> Vec<Command> {
        let selected_p = state.points[rnd::gen_range(0, state.points.len()) as usize].clone();
        Neighborhood::attempt_add(state, &selected_p, None)
    }

    fn attempt_add(state: &mut State, pos: &Pos, ignore_dir: Option<&Dir>) -> Vec<Command> {
        debug_assert!(state.grid.has_point(&pos));
        let nearest_points = state
            .grid
            .point(&pos)
            .as_ref()
            .unwrap()
            .nearest_points
            .clone();
        for _ in 0..DIR_MAX {
            let i = rnd::gen_range(0, DIR_MAX);
            let diagonal_dir = Dir::from_i64(i as i64);
            if let Some(ignore_dir) = ignore_dir {
                if ignore_dir == &diagonal_dir {
                    continue;
                }
            }
            let performed_commands =
                Neighborhood::attempt_add_dir(state, &pos, &nearest_points, &diagonal_dir);
            if performed_commands.len() > 0 {
                return performed_commands;
            }
        }
        return vec![];
    }

    fn attempt_add_dir(
        state: &mut State,
        pos: &Pos,
        nearest_points: &Vec<Option<Pos>>,
        dir: &Dir,
    ) -> Vec<Command> {
        let dir_next = dir.next();
        let dir_prev = dir.prev();

        if let (Some(pos_prev), Some(pos_next)) = (
            &nearest_points[dir_prev.val() as usize],
            &nearest_points[dir_next.val() as usize],
        ) {
            let new_pos = pos_next + &(pos_prev - &pos);

            if !state.grid.is_valid(&new_pos) {
                return vec![];
            }
            if !state.grid.has_point(&pos)
                || !state.grid.has_point(&pos_prev)
                || !state.grid.has_point(&pos_next)
                || state.grid.has_point(&new_pos)
            {
                return vec![];
            }

            let connect: [Pos; 2] = [pos_prev.clone(), pos_next.clone()];
            let square = Square::new(new_pos, pos.clone(), connect);

            let performed_commands = state.perform_command(&Command::Add { square });
            if performed_commands.len() > 0 {
                return performed_commands;
            }
        }

        vec![]
    }

    fn perform_delete(state: &mut State) -> Vec<Command> {
        if state.squares.len() == 0 {
            return vec![];
        }
        let square = state.squares[rnd::gen_range(0, state.squares.len())];
        Neighborhood::attempt_delete(state, &square)
    }

    fn attempt_delete(state: &mut State, square: &Square) -> Vec<Command> {
        state.perform_command(&Command::Delete {
            square: square.clone(),
        })
    }

    fn perform_change_square(state: &mut State) -> Vec<Command> {
        // 四角を作っている点を探す
        if state.squares.len() == 0 {
            return vec![];
        }
        let square = state.squares[rnd::gen_range(0, state.squares.len()) as usize];
        Neighborhood::attempt_change_square(state, &square)
    }

    fn attempt_change_square(state: &mut State, square: &Square) -> Vec<Command> {
        let start_score = state.get_score(1.);
        let mut performed_commands = state.perform_command(&Command::Delete {
            square: square.clone(),
        });

        // 四角を消せなかったら中止
        if performed_commands.len() == 0 {
            return performed_commands;
        }

        // 再帰的にposの点も消してしまった時は中止
        if !state.grid.has_point(&square.diagonal) {
            return performed_commands;
        }

        let mut recursion_count: usize = 0;
        Neighborhood::attempt_multiple_add(
            state,
            &square.diagonal,
            &mut recursion_count,
            &MULTIPLE_ADD_RECURSION_LIMIT,
            &mut performed_commands,
            start_score,
        );
        return performed_commands;
    }

    fn perform_split_square(state: &mut State) -> Vec<Command> {
        if state.squares.len() == 0 {
            return vec![];
        }
        let selected_square = state.squares[rnd::gen_range(0, state.squares.len()) as usize];
        Neighborhood::attempt_split_square(state, &selected_square)
    }

    fn attempt_split_square(state: &mut State, square: &Square) -> Vec<Command> {
        let nearest_points = state
            .grid
            .point(&square.diagonal)
            .as_ref()
            .unwrap()
            .nearest_points
            .clone();
        let dir1 = Pos::get_dir(&square.diagonal, &square.connect[0]);
        let dir2 = Pos::get_dir(&square.diagonal, &square.connect[1]);

        if nearest_points[dir1.val() as usize] != Some(square.connect[0])
            || nearest_points[dir2.val() as usize] != Some(square.connect[1])
        {
            let start_score = state.get_score(1.);
            let mut performed_commands = state.perform_command(&Command::Delete {
                square: square.clone(),
            });

            if performed_commands.len() == 0 {
                return vec![];
            }

            let mut recursion_count: usize = 0;
            Neighborhood::attempt_multiple_add(
                state,
                &square.diagonal,
                &mut recursion_count,
                &MULTIPLE_ADD_RECURSION_LIMIT,
                &mut performed_commands,
                start_score,
            );
            return performed_commands;
        }

        vec![]
    }
}

impl Neighborhood {
    pub fn all() -> [Neighborhood; 5] {
        [
            Neighborhood::Add,
            Neighborhood::Delete,
            Neighborhood::ChangeSquare,
            Neighborhood::SplitSquare,
            Neighborhood::MultipleAdd,
        ]
    }

    pub fn from_i64(v: i64) -> Neighborhood {
        match v {
            0 => Neighborhood::Add,
            1 => Neighborhood::Delete,
            2 => Neighborhood::ChangeSquare,
            3 => Neighborhood::SplitSquare,
            4 => Neighborhood::MultipleAdd,
            _ => panic!("Neighborhood value {} is invalid.", v),
        }
    }
}

#[test]
fn test_split_square() {
    let diagonal = Pos { x: 0, y: 0 };
    let connect: [Pos; 2] = [Pos { x: 2, y: 0 }, Pos { x: 0, y: 2 }];
    let new_pos = Pos { x: 2, y: 2 };
    let add_pos = Pos { x: 1, y: 0 };
    let connect2: [Pos; 2] = [Pos { x: 1, y: 0 }, Pos { x: 0, y: 2 }];
    let new_pos2 = Pos { x: 1, y: 2 };
    let n: usize = 5;
    let p = vec![diagonal.clone(), connect[0].clone(), connect[1].clone()];

    let square = Square::new(new_pos.clone(), diagonal.clone(), connect.clone());
    let mut state = State::new(n, p);
    state.perform_add(&square, false);
    state
        .grid
        .add_point(&add_pos, Point::new(&add_pos, true), None);

    Neighborhood::attempt_split_square(&mut state, &square);

    let mut square = Square::new(new_pos2.clone(), diagonal.clone(), connect2.clone());
    square.id = state.squares[0].id;
    assert_eq!(state.squares[0], square);
}

#[test]
fn test_change_square() {
    let selected_p = Pos { x: 2, y: 2 };
    let connect: [Pos; 2] = [Pos { x: 2, y: 0 }, Pos { x: 0, y: 2 }];
    let connect2: [Pos; 2] = [Pos { x: 2, y: 0 }, Pos { x: 4, y: 2 }];
    let old_pos = Pos { x: 0, y: 0 };
    let new_pos = Pos { x: 4, y: 0 };
    let n: usize = 5;
    let p = vec![
        selected_p.clone(),
        connect[0].clone(),
        connect[1].clone(),
        connect2[1].clone(),
    ];

    let mut state = State::new(n, p);
    let mut other_state = state.clone();
    let square = Square::new(old_pos.clone(), selected_p.clone(), connect);
    other_state.perform_command(&Command::Add {
        square: Square::new(new_pos.clone(), selected_p.clone(), connect2),
    });
    state.perform_command(&Command::Add {
        square: square.clone(),
    });
    let copied_state = state.clone();
    let performed_commands = Neighborhood::attempt_change_square(&mut state, &square);

    // multiple_addが不定なので消す
    // Squareのidは異なってしまう
    // assert_eq!(performed_commands.len(), 4);
    // assert_eq!(state, other_state);
    // assert!(!state.grid.has_point(&old_pos));
    // assert!(state.grid.has_point(&new_pos));

    for command in performed_commands.iter().rev() {
        state.reverse_command(command);
    }

    assert_eq!(state, copied_state);
}
