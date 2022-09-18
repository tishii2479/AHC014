use crate::def::*; // ignore
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
            state.grid.add_point(pos, Point::new(&pos, false));
            state.points.push(pos.clone());
            state.score.base += state.weight(&pos);
        }
        state
    }
}

impl State {
    pub fn perform_add(&mut self, square: &Square) -> bool {
        assert!(Pos::is_aligned(&square.diagonal, &square.connect[0]));
        assert!(Pos::is_aligned(&square.diagonal, &square.connect[1]));
        assert!(Pos::is_aligned(&square.new_pos, &square.connect[0]));
        assert!(Pos::is_aligned(&square.new_pos, &square.connect[1]));

        // new_posに既に点がないか確認
        if self.grid.has_point(&square.new_pos) {
            return false;
        }

        // 作ろうとしてる四角の辺に既に点、辺がないか確認する
        if !self.grid.can_connect(&square.connect[0], &square.new_pos)
            || !self.grid.can_connect(&square.connect[1], &square.new_pos)
            || !self.grid.can_connect(&square.connect[0], &square.diagonal)
            || !self.grid.can_connect(&square.connect[1], &square.diagonal)
        {
            return false;
        }

        // eprintln!(
        //     "Connected: {:?}, {:?}, {:?}, {:?}",
        //     new_pos, &square.connect[0], square.diagonal, &square.connect[1]
        // );

        self.grid.create_square(&square);

        self.squares.push(square.clone());
        self.points.push(square.new_pos.clone());
        self.score.base += self.weight(&square.new_pos);

        true
    }

    pub fn perform_delete(&mut self, square: &Square) -> bool {
        assert!(Pos::is_aligned(&square.diagonal, &square.connect[0]));
        assert!(Pos::is_aligned(&square.diagonal, &square.connect[1]));
        assert!(Pos::is_aligned(&square.new_pos, &square.connect[0]));
        assert!(Pos::is_aligned(&square.new_pos, &square.connect[1]));

        assert!(self.grid.has_point(&square.new_pos));

        // TODO: delete square recursively

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
        // FIXME: O(n)
        self.score.base -= self.weight(&square.new_pos);

        true
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
    let copied_state = state.clone();
    let square = Square {
        new_pos: new_pos.clone(),
        diagonal: diagonal.clone(),
        connect: connect.clone(),
    };
    state.perform_add(&square);
    state.perform_delete(&square);
    assert_eq!(copied_state, state);
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
    let square = Square {
        new_pos: new_pos.clone(),
        diagonal: diagonal.clone(),
        connect: connect.clone(),
    };
    assert!(state.perform_add(&square));
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
