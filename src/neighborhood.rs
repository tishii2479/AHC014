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
        let mut performed_commands = Neighborhood::attempt_add(state, &selected_p, None);
        if performed_commands.len() == 0 {
            return vec![];
        }
        for _ in 0..DIR_MAX {
            let i = rnd::gen_range(0, DIR_MAX);
            let dir = Dir::from_i64(i as i64);
            if let Some(nearest_pos) = state
                .grid
                .point(&selected_p)
                .as_ref()
                .unwrap()
                .nearest_points[dir.val() as usize]
            {
                if rnd::nextf() < 0.5 {
                    continue;
                }
                let mut second_add = Neighborhood::attempt_add(state, &nearest_pos, None);
                performed_commands.append(&mut second_add);
            }
        }
        performed_commands
    }

    fn perform_add(state: &mut State) -> Vec<Command> {
        let selected_p = state.points[rnd::gen_range(0, state.points.len()) as usize].clone();
        Neighborhood::attempt_add(state, &selected_p, None)
    }

    fn attempt_add(state: &mut State, pos: &Pos, ignore_dir: Option<&Dir>) -> Vec<Command> {
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
            let performed_commands = Neighborhood::attempt_add_dir(state, &point, &diagonal_dir);
            if performed_commands.len() > 0 {
                return performed_commands;
            }
        }
        return vec![];
    }

    fn attempt_add_dir(state: &mut State, point: &Point, dir: &Dir) -> Vec<Command> {
        let dir_next = dir.next();
        let dir_prev = dir.prev();

        if let (Some(pos_prev), Some(pos_next)) = (
            &point.nearest_points[dir_prev.val() as usize],
            &point.nearest_points[dir_next.val() as usize],
        ) {
            let new_pos = pos_next + &(pos_prev - &point.pos);

            if !state.grid.is_valid(&new_pos) {
                return vec![];
            }
            if !state.grid.has_point(&point.pos)
                || !state.grid.has_point(&pos_prev)
                || !state.grid.has_point(&pos_next)
            {
                return vec![];
            }

            let connect: [Pos; 2] = [pos_prev.clone(), pos_next.clone()];
            let square = Square::new(new_pos, point.pos.clone(), connect);

            let performed_commands = state.perform_command(&Command::Add { square });
            if performed_commands.len() > 0 {
                return performed_commands;
            }
        }

        vec![]
    }

    fn perform_delete(state: &mut State) -> Vec<Command> {
        let selected_p = state.points[rnd::gen_range(0, state.points.len()) as usize].clone();
        Neighborhood::attemp_delete(state, &selected_p)
    }

    fn attemp_delete(state: &mut State, pos: &Pos) -> Vec<Command> {
        assert!(state.grid.has_point(&pos));
        let point = state.grid.point(&pos).as_ref().unwrap().clone();
        if let Some(added_info) = point.added_info {
            return state.perform_command(&Command::Delete { square: added_info });
        }
        return vec![];
    }

    fn perform_change_square(state: &mut State) -> Vec<Command> {
        // 四角を作っている点を探す
        let selected_p = state.points[rnd::gen_range(0, state.points.len()) as usize].clone();
        Neighborhood::attempt_change_square(state, &selected_p)
    }

    fn attempt_change_square(state: &mut State, pos: &Pos) -> Vec<Command> {
        assert!(state.grid.has_point(&pos));
        let nearest_points = state
            .grid
            .point(&pos)
            .as_ref()
            .unwrap()
            .nearest_points
            .clone();

        for _ in 0..DIR_MAX {
            let i = rnd::gen_range(0, DIR_MAX);
            let front = Dir::from_i64(i as i64);
            let left = front.prev().prev();
            if state.grid.has_edge(&pos, &left) && state.grid.has_edge(&pos, &front) {
                let left_pos = nearest_points[left.val() as usize].as_ref().unwrap();
                let front_pos = nearest_points[front.val() as usize].as_ref().unwrap();
                let left_front_pos = &(&(left_pos + front_pos) - pos);
                if !state.grid.is_valid(left_front_pos) {
                    continue;
                }
                if let Some(left_front_point) = state.grid.point(left_front_pos) {
                    if !left_front_point.is_added {
                        continue;
                    }
                    let added_square = left_front_point.added_info.as_ref().unwrap().clone();
                    let mut performed_commands = state.perform_command(&Command::Delete {
                        square: added_square,
                    });

                    // 四角を消せなかったら中止
                    if performed_commands.len() == 0 {
                        return performed_commands;
                    }

                    // 再帰的にposの点も消してしまった時は中止
                    if !state.grid.has_point(&pos) {
                        return performed_commands;
                    }

                    performed_commands.append(&mut Neighborhood::attempt_add(
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

    fn perform_split_square(state: &mut State) -> Vec<Command> {
        if state.squares.len() == 0 {
            return vec![];
        }
        let selected_square = state.squares[rnd::gen_range(0, state.squares.len()) as usize];
        Neighborhood::attempt_split_square(state, &selected_square)
    }

    fn attempt_split_square(state: &mut State, square: &Square) -> Vec<Command> {
        let diagonal_point = state.grid.point(&square.diagonal).as_ref().unwrap().clone();
        let dir1 = Pos::get_dir(&square.diagonal, &square.connect[0]);
        let dir2 = Pos::get_dir(&square.diagonal, &square.connect[1]);

        if diagonal_point.nearest_points[dir1.val() as usize] != Some(square.connect[0])
            || diagonal_point.nearest_points[dir2.val() as usize] != Some(square.connect[1])
        {
            let mut performed_commands = state.perform_command(&Command::Delete {
                square: square.clone(),
            });

            if performed_commands.len() == 0 {
                return vec![];
            }
            let dir = Dir::from_i64(
                (Pos::get_dir(&square.diagonal, &square.connect[0]).val()
                    + Pos::get_dir(&square.diagonal, &square.connect[1]).val())
                    / 2,
            );
            performed_commands.append(&mut Neighborhood::attempt_add_dir(
                state,
                &diagonal_point,
                &dir,
            ));
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
    other_state.perform_command(&Command::Add {
        square: Square::new(new_pos.clone(), selected_p.clone(), connect2),
    });
    state.perform_command(&Command::Add {
        square: Square::new(old_pos.clone(), selected_p.clone(), connect),
    });
    let copied_state = state.clone();
    let performed_commands = Neighborhood::attempt_change_square(&mut state, &selected_p);

    assert_eq!(performed_commands.len(), 2);
    // FXIME: Squareのidは異なってしまう
    // assert_eq!(state, other_state);
    assert!(!state.grid.has_point(&old_pos));
    assert!(state.grid.has_point(&new_pos));

    for command in performed_commands.iter().rev() {
        state.reverse_command(command);
    }

    assert_eq!(state, copied_state);
}
