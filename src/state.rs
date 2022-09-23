use crate::def::*;
#[allow(unused_imports)]
use crate::framework::IState;
use crate::grid::*;

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
            score: Score::new(),
        };
        for pos in p.iter() {
            state.score += &state.grid.add_point(pos, Point::new(&pos, false), None);
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

        self.score += &self.grid.create_square(&square, is_reverse);

        self.squares.push(square.clone());
        self.points.push(square.new_pos.clone());

        // スコアの更新
        self.score.base += self.weight(&square.new_pos);
        self.score.edge_length += square.size();

        vec![Command::Add {
            square: square.clone(),
        }]
    }

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

        self.score += &self.grid.delete_square(&square);

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
        self.score.edge_length -= square.size();
        performed_commands.push(Command::Delete {
            square: square.clone(),
        });
    }

    pub fn calc_deletion_size(
        &mut self,
        new_pos: &Pos,
        recursion_limit: usize,
        parent_dep_size: usize,
    ) -> usize {
        let mut dep_size: usize = 1 + parent_dep_size;

        let point = self.grid.point(&new_pos).as_ref().unwrap().clone();
        for created_point in &point.created_points {
            if dep_size >= recursion_limit {
                return dep_size - parent_dep_size;
            }

            // 再帰的に処理する場合、既に削除されている時があるので、その時は何もしない
            // TODO: 正当性の確認
            if !self.grid.has_point(&created_point) {
                continue;
            }
            dep_size += self.calc_deletion_size(&created_point, recursion_limit, dep_size);
        }

        dep_size - parent_dep_size
    }
}

impl State {
    pub fn weight(&self, pos: &Pos) -> i64 {
        let c = ((self.grid.size - 1) / 2) as i64;
        (pos.y as i64 - c) * (pos.y as i64 - c) + (pos.x as i64 - c) * (pos.x as i64 - c) + 1
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

    let mut split_square = Neighborhood::SplitSquare;
    split_square.attempt_split_square(&mut state, &square);

    let mut square = Square::new(new_pos2.clone(), diagonal.clone(), connect2.clone());
    square.id = state.squares[0].id;
    assert_eq!(state.squares[0], square);
}

#[allow(dead_code)]
// #[test]
fn test_calc_point_penalty() {
    let n = 31;
    let state = State::new(n, vec![]);

    let square = Square::new(
        Pos { x: 20, y: 15 },
        Pos { x: 21, y: 16 },
        [Pos { x: 20, y: 16 }, Pos { x: 21, y: 15 }],
    );
    let square2 = Square::new(
        Pos { x: 20, y: 16 },
        Pos { x: 21, y: 17 },
        [Pos { x: 20, y: 17 }, Pos { x: 21, y: 16 }],
    );

    assert_eq!(state.grid.calc_point_penalty(&square).point_penalty, 0);
    assert_eq!(state.grid.calc_point_penalty(&square2).point_penalty, 4);

    let square = Square::new(
        Pos { x: 10, y: 15 },
        Pos { x: 12, y: 15 },
        [Pos { x: 11, y: 16 }, Pos { x: 11, y: 14 }],
    );
    let square2 = Square::new(
        Pos { x: 10, y: 16 },
        Pos { x: 12, y: 16 },
        [Pos { x: 11, y: 17 }, Pos { x: 11, y: 15 }],
    );
    let square3 = Square::new(
        Pos { x: 9, y: 15 },
        Pos { x: 11, y: 15 },
        [Pos { x: 10, y: 16 }, Pos { x: 10, y: 14 }],
    );

    assert_eq!(state.grid.calc_point_penalty(&square).point_penalty, 0);
    assert_eq!(state.grid.calc_point_penalty(&square2).point_penalty, 4);
    assert_eq!(state.grid.calc_point_penalty(&square3).point_penalty, 0);

    let square = Square::new(
        Pos { x: 15, y: 10 },
        Pos { x: 15, y: 8 },
        [Pos { x: 14, y: 9 }, Pos { x: 16, y: 9 }],
    );
    let square2 = Square::new(
        Pos { x: 16, y: 10 },
        Pos { x: 16, y: 8 },
        [Pos { x: 15, y: 9 }, Pos { x: 17, y: 9 }],
    );
    let square3 = Square::new(
        Pos { x: 15, y: 9 },
        Pos { x: 15, y: 7 },
        [Pos { x: 14, y: 8 }, Pos { x: 16, y: 8 }],
    );

    assert_eq!(state.grid.calc_point_penalty(&square).point_penalty, 4);
    assert_eq!(state.grid.calc_point_penalty(&square2).point_penalty, 0);
    assert_eq!(state.grid.calc_point_penalty(&square3).point_penalty, 4);

    let square = Square::new(
        Pos { x: 10, y: 15 },
        Pos { x: 13, y: 16 },
        [Pos { x: 11, y: 14 }, Pos { x: 12, y: 17 }],
    );
    assert_eq!(state.grid.calc_point_penalty(&square).point_penalty, 2);

    let square = Square::new(
        Pos { x: 15, y: 6 },
        Pos { x: 15, y: 8 },
        [Pos { x: 14, y: 7 }, Pos { x: 16, y: 7 }],
    );
    let square2 = Square::new(
        Pos { x: 16, y: 6 },
        Pos { x: 16, y: 8 },
        [Pos { x: 15, y: 7 }, Pos { x: 17, y: 7 }],
    );
    let square3 = Square::new(
        Pos { x: 15, y: 5 },
        Pos { x: 15, y: 7 },
        [Pos { x: 14, y: 6 }, Pos { x: 16, y: 6 }],
    );

    assert_eq!(state.grid.calc_point_penalty(&square).point_penalty, 4);
    assert_eq!(state.grid.calc_point_penalty(&square2).point_penalty, 0);
    assert_eq!(state.grid.calc_point_penalty(&square3).point_penalty, 4);
}

#[test]
fn test_calc_point_closeness() {
    let n: usize = 5;
    let p = vec![Pos { x: 2, y: 2 }];
    let mut state = State::new(n, p);
    let new_pos = Pos { x: 0, y: 0 };
    let new_pos2 = Pos { x: 1, y: 1 };

    let copied_state = state.clone();
    state.score += &state
        .grid
        .add_point(&new_pos, Point::new(&new_pos, true), None);
    state.score += &state
        .grid
        .add_point(&new_pos2, Point::new(&new_pos2, true), None);
    state.score += &state.grid.remove_point(&new_pos2);
    state.score += &state.grid.remove_point(&new_pos);

    assert_eq!(state, copied_state);
}

#[test]
fn test_add_point_on_square_edge() {
    let diagonal = Pos { x: 0, y: 0 };
    let connect: [Pos; 2] = [Pos { x: 2, y: 0 }, Pos { x: 0, y: 2 }];
    let new_pos = Pos { x: 2, y: 2 };
    let diagonal2 = Pos { x: 4, y: 1 };
    let connect2: [Pos; 2] = [Pos { x: 3, y: 0 }, Pos { x: 3, y: 2 }];
    let new_pos2 = Pos { x: 2, y: 1 };
    let n: usize = 5;
    let p = vec![
        diagonal.clone(),
        connect[0].clone(),
        connect[1].clone(),
        diagonal2.clone(),
        connect2[0].clone(),
        connect2[1].clone(),
    ];
    let mut state = State::new(n, p);
    let square = Square::new(new_pos.clone(), diagonal.clone(), connect.clone());
    let square2 = Square::new(new_pos2.clone(), diagonal2.clone(), connect2.clone());

    state.perform_add(&square, false);
    assert_eq!(state.perform_add(&square2, false).len(), 1);
}

#[test]
fn test_perform_single_delete() {
    let diagonal = Pos { x: 0, y: 0 };
    let connect: [Pos; 2] = [Pos { x: 2, y: 0 }, Pos { x: 0, y: 2 }];
    let new_pos = Pos { x: 2, y: 2 };
    let n: usize = 5;
    let p = vec![diagonal.clone(), connect[0].clone(), connect[1].clone()];
    let mut state = State::new(n, p);
    let copied_state = state.clone();
    let square = Square::new(new_pos.clone(), diagonal.clone(), connect.clone());
    state.perform_add(&square, false);
    state.perform_delete(&square, &mut vec![]);

    assert_eq!(state, copied_state);
}

#[test]
fn test_perform_recursive_delete() {
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
    eprintln!("{}", state.score.point_closeness);
    state.perform_add(&square, false);
    eprintln!("{}", state.score.point_closeness);
    state.perform_add(&square2, false);
    eprintln!("{}", state.score.point_closeness);
    state.perform_delete(&square, &mut vec![]);
    eprintln!(
        "{} {}",
        state.score.point_closeness, copied_state.score.point_closeness
    );

    assert_eq!(state, copied_state);
}

#[test]
fn test_perform_add() {
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
    other_state.perform_command(&Command::Add {
        square: Square::new(new_pos.clone(), selected_p.clone(), connect2),
    });
    state.perform_command(&Command::Add {
        square: Square::new(old_pos.clone(), selected_p.clone(), connect),
    });
    let copied_state = state.clone();
    let mut neighborhood = Neighborhood::ChangeSquare;
    let performed_commands = neighborhood.attempt_change_square(&mut state, &selected_p);

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
