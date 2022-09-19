use crate::def::*; // ignore
#[allow(unused_imports)] // ignore
use crate::framework::IState; // ignore
use crate::grid::*; // ignore

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct State {
    pub grid: Grid,
    pub points: Vec<Pos>,
    pub squares: Vec<Square>,
    pub score: Score,
}

impl State {
    pub fn new(n: usize, p: Vec<Pos>) -> State {
        let mut state = State {
            grid: Grid {
                size: n,
                points: vec![vec![None; n]; n],
                edges: vec![vec![vec![false; DIR_MAX]; n]; n],
            },
            points: p.clone(),
            squares: vec![],
            score: Score { base: 0 },
        };
        for pos in p.iter() {
            state.grid.add_point(pos, Point::new(&pos, false), None);
            state.points.push(pos.clone());
            state.score.base += state.weight(&pos);
        }
        state
    }
}

impl State {
    pub fn perform_add(&mut self, square: &Square, is_reverse: bool) -> Vec<Command> {
        assert!(Pos::is_aligned(&square.diagonal, &square.connect[0]));
        assert!(Pos::is_aligned(&square.diagonal, &square.connect[1]));
        assert!(Pos::is_aligned(&square.new_pos, &square.connect[0]));
        assert!(Pos::is_aligned(&square.new_pos, &square.connect[1]));

        // new_posに既に点がないか確認
        if self.grid.has_point(&square.new_pos) {
            return vec![];
        }

        // 作ろうとしてる四角の辺に既に点、辺がないか確認する
        if !is_reverse
            && (!self.grid.can_connect(&square.connect[0], &square.new_pos)
                || !self.grid.can_connect(&square.connect[1], &square.new_pos)
                || !self.grid.can_connect(&square.connect[0], &square.diagonal)
                || !self.grid.can_connect(&square.connect[1], &square.diagonal))
        {
            return vec![];
        }

        // eprintln!(
        //     "Connected: {:?}, {:?}, {:?}, {:?}",
        //     &square.new_pos, &square.connect[0], square.diagonal, &square.connect[1]
        // );

        self.grid.create_square(&square, is_reverse);

        self.squares.push(square.clone());
        self.points.push(square.new_pos.clone());
        self.score.base += self.weight(&square.new_pos);

        vec![Command::Add {
            square: square.clone(),
        }]
    }

    // TODO: add recursion limit
    pub fn perform_delete(&mut self, square: &Square, performed_commands: &mut Vec<Command>) {
        assert!(Pos::is_aligned(&square.diagonal, &square.connect[0]));
        assert!(Pos::is_aligned(&square.diagonal, &square.connect[1]));
        assert!(Pos::is_aligned(&square.new_pos, &square.connect[0]));
        assert!(Pos::is_aligned(&square.new_pos, &square.connect[1]));

        assert!(self.grid.has_point(&square.new_pos));

        // new_posの点を使って作られた四角を再帰的に消す
        let point = self.grid.point(&square.new_pos).as_ref().unwrap().clone();
        for created_point in &point.created_points {
            // 再帰的に処理する場合、既に削除されている時があるので、その時は何もしない
            // TODO: 正当性の確認
            if !self.grid.has_point(&created_point) {
                continue;
            }
            let created_square = self
                .grid
                .point(created_point)
                .as_ref()
                .unwrap()
                .added_info
                .as_ref()
                .unwrap()
                .clone();
            self.perform_delete(&created_square, performed_commands);
        }

        self.grid.delete_square(&square);

        // FIXME: O(n)
        self.squares
            .remove(self.squares.iter().position(|x| *x == *square).unwrap());
        // FIXME: O(n)
        self.points.remove(
            self.points
                .iter()
                .position(|x| *x == square.new_pos)
                .unwrap(),
        );
        self.score.base -= self.weight(&square.new_pos);
        performed_commands.push(Command::Delete {
            square: square.clone(),
        });
    }
}

impl State {
    pub fn weight(&self, pos: &Pos) -> i64 {
        let c = ((self.grid.size - 1) / 2) as i64;
        (pos.y as i64 - c) * (pos.y as i64 - c) + (pos.x as i64 - c) * (pos.x as i64 - c) + 1
    }
}

#[test]
fn test_delete_point() {
    let diagonal = Pos { x: 0, y: 0 };
    let connect: [Pos; 2] = [Pos { x: 2, y: 0 }, Pos { x: 0, y: 2 }];
    let connect2: [Pos; 2] = [Pos { x: 2, y: 4 }, Pos { x: 4, y: 2 }];
    let new_pos = Pos { x: 2, y: 2 };
    let new_pos2 = Pos { x: 4, y: 4 };
    let n: usize = 5;
    let p = vec![
        diagonal.clone(),
        connect[0].clone(),
        connect[1].clone(),
        connect2[0].clone(),
        connect2[1].clone(),
    ];
    let mut state = State::new(n, p);
    let copied_state = state.clone();
    let square = Square::new(new_pos.clone(), diagonal.clone(), connect.clone());
    let square2 = Square::new(new_pos2.clone(), new_pos.clone(), connect2.clone());
    state.perform_add(&square, false);
    state.perform_add(&square2, false);
    state.perform_delete(&square, &mut vec![]);
    assert_eq!(state, copied_state);
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
    let square = Square::new(new_pos.clone(), diagonal.clone(), connect.clone());
    assert_eq!(state.perform_add(&square, false).len(), 1);
    assert!(state.grid.point(&new_pos).is_some());

    assert!(state.grid.has_edge(&Pos { x: 1, y: 2 }, &Dir::Left));
    assert!(state.grid.has_edge(&Pos { x: 1, y: 2 }, &Dir::Right));

    match state.grid.point(&connect[0]) {
        Some(point_other) => {
            assert_eq!(
                point_other.nearest_points[Dir::Up.val() as usize],
                Some(Pos { x: 2, y: 2 })
            );

            assert!(point_other.created_points[0] == Pos { x: 2, y: 2 });
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
                point_new_pos.nearest_points[Dir::Up.val() as usize],
                Some(Pos { x: 2, y: 4 })
            );
            assert_eq!(
                point_new_pos.nearest_points[Dir::Down.val() as usize],
                Some(Pos { x: 2, y: 0 })
            );
        }
        None => assert!(false),
    }

    match state.grid.point(&other) {
        Some(point_other) => {
            assert_eq!(
                point_other.nearest_points[Dir::Down.val() as usize],
                Some(Pos { x: 2, y: 2 })
            );
        }
        None => assert!(false),
    }
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
    state.perform_command(&Command::Add {
        square: Square::new(old_pos.clone(), selected_p.clone(), connect),
    });
    other_state.perform_command(&Command::Add {
        square: Square::new(new_pos.clone(), selected_p.clone(), connect2),
    });

    let copied_state = state.clone();
    let mut neighborhood = Neighborhood::ChangeSquare;
    let performed_commands = neighborhood.attempt_change_square(&mut state, &selected_p);

    assert_eq!(performed_commands.len(), 2);
    assert!(!state.grid.has_point(&old_pos));
    assert!(state.grid.has_point(&new_pos));

    // Squareのidは異なってしまうので、それ以外で比較する
    assert_eq!(state.points, other_state.points);
    assert_eq!(state.score, other_state.score);

    for command in performed_commands.iter().rev() {
        state.reverse_command(command);
    }
    assert_eq!(state, copied_state);
}

#[test]
fn test_reverse_command() {
    let diagonal = Pos { x: 0, y: 0 };
    let connect: [Pos; 2] = [Pos { x: 2, y: 0 }, Pos { x: 0, y: 2 }];
    let new_pos = Pos { x: 2, y: 2 };
    let n: usize = 5;
    let p = vec![diagonal.clone(), connect[0].clone(), connect[1].clone()];
    let mut state = State::new(n, p);
    let copied_state = state.clone();
    let square = Square::new(new_pos.clone(), diagonal.clone(), connect.clone());

    state.perform_command(&Command::Add {
        square: square.clone(),
    });
    state.reverse_command(&Command::Add {
        square: square.clone(),
    });
    assert_eq!(state, copied_state);

    state.perform_command(&Command::Add {
        square: square.clone(),
    });

    let copied_state = state.clone();
    state.perform_command(&Command::Delete {
        square: square.clone(),
    });
    state.reverse_command(&Command::Delete {
        square: square.clone(),
    });
    assert_eq!(state, copied_state);
}

#[test]
fn test_reverse_recursive_delete_command() {
    let diagonal = Pos { x: 0, y: 0 };
    let connect: [Pos; 2] = [Pos { x: 2, y: 0 }, Pos { x: 0, y: 2 }];
    let connect2: [Pos; 2] = [Pos { x: 2, y: 4 }, Pos { x: 4, y: 2 }];
    let connect3: [Pos; 2] = [Pos { x: 0, y: 4 }, Pos { x: 4, y: 4 }];
    let new_pos = Pos { x: 2, y: 2 };
    let new_pos2 = Pos { x: 4, y: 4 };
    let new_pos3 = Pos { x: 2, y: 6 };
    let n: usize = 10;
    let p = vec![
        diagonal.clone(),
        connect[0].clone(),
        connect[1].clone(),
        connect2[0].clone(),
        connect2[1].clone(),
        connect3[0].clone(),
    ];
    let mut state = State::new(n, p);
    let square = Square::new(new_pos.clone(), diagonal.clone(), connect.clone());
    let square2 = Square::new(new_pos2.clone(), new_pos.clone(), connect2.clone());
    let square3 = Square::new(new_pos3.clone(), new_pos.clone(), connect3.clone());

    state.perform_command(&Command::Add {
        square: square.clone(),
    });
    state.perform_command(&Command::Add {
        square: square2.clone(),
    });
    state.perform_command(&Command::Add {
        square: square3.clone(),
    });

    let copied_state = state.clone();
    let performed_commands = state.perform_command(&Command::Delete {
        square: square.clone(),
    });
    for command in performed_commands.iter().rev() {
        state.reverse_command(command);
    }
    assert_eq!(performed_commands.len(), 3);
    assert_eq!(state, copied_state);
}
