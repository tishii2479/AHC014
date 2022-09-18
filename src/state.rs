use crate::def::*; // ignore
use crate::grid::*; // ignore

pub struct State {
    pub grid: Grid,
    pub points: Vec<Pos>,
    pub squares: Vec<(Pos, Pos, Pos, Pos)>,

    pub score: f64,
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
            score: 0.,
        };
        for pos in p.iter() {
            state.grid.add_point(pos, Point::new(&pos, false));
            state.points.push(pos.clone());
            state.score += state.weight(&pos);
        }
        state
    }
}

impl State {
    pub fn perform_add(&mut self, new_pos: &Pos, diagonal: &Pos, connect: &[Pos; 2]) -> bool {
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

        // eprintln!(
        //     "Connected: {:?}, {:?}, {:?}, {:?}",
        //     new_pos, &connect[0], diagonal, &connect[1]
        // );

        self.grid.create_square(&new_pos, &diagonal, &connect);

        self.squares.push((
            new_pos.clone(),
            connect[0].clone(),
            diagonal.clone(),
            connect[1].clone(),
        ));
        self.points.push(new_pos.clone());
        self.score += self.weight(new_pos);

        return true;
    }

    pub fn perform_delete(&mut self, new_pos: &Pos, diagonal: &Pos, connect: &[Pos; 2]) -> bool {
        false
    }
}

impl State {
    pub fn weight(&self, pos: &Pos) -> f64 {
        let c = ((self.grid.size - 1) / 2) as f64;
        (pos.y as f64 - c) * (pos.y as f64 - c) + (pos.x as f64 - c) * (pos.x as f64 - c) + 1.
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
    assert!(state.perform_add(&new_pos, &diagonal, &connect));
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
