use proconio::derive_readable;
use std::ops;

pub const DIR_MAX: usize = 8;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Dir {
    Up = 0,
    UpRight = 1,
    Right = 2,
    DownRight = 3,
    Down = 4,
    DownLeft = 5,
    Left = 6,
    UpLeft = 7,
}

impl Dir {
    pub fn from_i64(v: i64) -> Dir {
        match v {
            0 => Dir::Up,
            1 => Dir::UpRight,
            2 => Dir::Right,
            3 => Dir::DownRight,
            4 => Dir::Down,
            5 => Dir::DownLeft,
            6 => Dir::Left,
            7 => Dir::UpLeft,
            _ => panic!("Dir value {} is invalid.", v),
        }
    }

    pub fn rev(self) -> Dir {
        let a = (self as i64 + 4) % 8;
        Dir::from_i64(a)
    }

    pub fn val(self) -> i64 {
        self as i64
    }

    pub fn to_pos(self) -> Pos {
        match self {
            Dir::Up => Pos { x: 0, y: 1 },
            Dir::UpRight => Pos { x: 1, y: 1 },
            Dir::Right => Pos { x: 1, y: 0 },
            Dir::DownRight => Pos { x: 1, y: -1 },
            Dir::Down => Pos { x: 0, y: -1 },
            Dir::DownLeft => Pos { x: -1, y: -1 },
            Dir::Left => Pos { x: -1, y: 0 },
            Dir::UpLeft => Pos { x: -1, y: 1 },
        }
    }

    pub fn next(self) -> Dir {
        Dir::from_i64((self.val() + 1) % DIR_MAX as i64)
    }

    pub fn prev(self) -> Dir {
        Dir::from_i64((self.val() + (DIR_MAX - 1) as i64) % DIR_MAX as i64)
    }
}

#[derive_readable]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pos {
    pub x: i64,
    pub y: i64,
}

impl Pos {
    pub fn is_aligned(a: &Pos, b: &Pos) -> bool {
        if a == b {
            return false;
        }
        if a.y == b.y || a.x == b.x {
            return true;
        }
        if i64::abs(a.x - b.x) == i64::abs(a.y - b.y) {
            return true;
        }
        return false;
    }

    pub fn get_dir(from: &Pos, to: &Pos) -> Dir {
        assert!(Pos::is_aligned(from, to));

        let delta = to - from;
        if delta.y > 0 && delta.x == 0 {
            return Dir::Up;
        } else if delta.y > 0 && delta.x > 0 {
            return Dir::UpRight;
        } else if delta.y == 0 && delta.x > 0 {
            return Dir::Right;
        } else if delta.y < 0 && delta.x > 0 {
            return Dir::DownRight;
        } else if delta.y < 0 && delta.x == 0 {
            return Dir::Down;
        } else if delta.y < 0 && delta.x < 0 {
            return Dir::DownLeft;
        } else if delta.y == 0 && delta.x < 0 {
            return Dir::Left;
        } else {
            return Dir::UpLeft;
        }
    }

    pub fn between(from: &Pos, to: &Pos) -> Vec<Pos> {
        assert!(Pos::is_aligned(from, to));

        let mut cur = from.clone();
        let mut ret: Vec<Pos> = vec![];

        let dir = Pos::get_dir(from, to);

        // is_alignedなら必ず見つかる（はず）
        loop {
            cur += &dir.to_pos();
            if &cur == to {
                break;
            }
            ret.push(cur.clone());
        }

        return ret;
    }

    pub fn dist(a: &Pos, b: &Pos) -> i64 {
        i64::abs(a.x - b.x) + i64::abs(a.y - b.y)
    }
}

#[test]
fn test_between() {
    let from = Pos { x: 1, y: 3 };
    let to = Pos { x: 4, y: 3 };

    assert_eq!(
        Pos::between(&from, &to),
        vec![Pos { x: 2, y: 3 }, Pos { x: 3, y: 3 }]
    );

    let from = Pos { x: 1, y: 3 };
    let to = Pos { x: 4, y: 6 };

    assert_eq!(
        Pos::between(&from, &to),
        vec![Pos { x: 2, y: 4 }, Pos { x: 3, y: 5 }]
    );
}

#[test]
fn test_get_dir() {
    let c = Pos { x: 5, y: 5 };
    assert_eq!(Pos::get_dir(&c, &Pos { x: 5, y: 7 }), Dir::Up);
    assert_eq!(Pos::get_dir(&c, &Pos { x: 7, y: 7 }), Dir::UpRight);
    assert_eq!(Pos::get_dir(&c, &Pos { x: 7, y: 5 }), Dir::Right);
    assert_eq!(Pos::get_dir(&c, &Pos { x: 7, y: 3 }), Dir::DownRight);
    assert_eq!(Pos::get_dir(&c, &Pos { x: 5, y: 3 }), Dir::Down);
    assert_eq!(Pos::get_dir(&c, &Pos { x: 3, y: 3 }), Dir::DownLeft);
    assert_eq!(Pos::get_dir(&c, &Pos { x: 3, y: 5 }), Dir::Left);
    assert_eq!(Pos::get_dir(&c, &Pos { x: 3, y: 7 }), Dir::UpLeft);
}

impl ops::AddAssign<&Pos> for Pos {
    fn add_assign(&mut self, rhs: &Pos) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Add<&Pos> for &Pos {
    type Output = Pos;
    fn add(self, rhs: &Pos) -> Pos {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<&Pos> for &Pos {
    type Output = Pos;
    fn sub(self, rhs: &Pos) -> Pos {
        Pos {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Clone)]
pub struct Point {
    pub pos: Pos,
    // 追加された点かどうか
    pub is_added: bool,
    // 各方向にある最も近い点
    // 方向を0~7の値に決める
    pub nearest_points: Vec<Option<Pos>>,
    // その点を使って作った点
    pub used_points: Vec<Pos>,
    // 各方向が長方形の辺に使われているか
    pub used_dir: Vec<bool>,
}

impl Point {
    pub fn new(pos: &Pos, is_added: bool) -> Point {
        Point {
            pos: pos.clone(),
            is_added: is_added,
            nearest_points: vec![None; DIR_MAX],
            used_points: vec![],
            used_dir: vec![false; DIR_MAX],
        }
    }
}

pub struct State {
    pub grid: Grid,
    pub points: Vec<Pos>,
    pub squares: Vec<(Pos, Pos, Pos, Pos)>,
}

pub struct Grid {
    pub size: usize,
    pub points: Vec<Vec<Option<Point>>>,
    pub edges: Vec<Vec<Vec<bool>>>,
}

impl Grid {
    pub fn point(&mut self, pos: &Pos) -> &mut Option<Point> {
        &mut self.points[pos.y as usize][pos.x as usize]
    }

    pub fn can_connect(&self, a: &Pos, b: &Pos) -> bool {
        assert!(Pos::is_aligned(a, b));
        let dir = Pos::get_dir(a, b);
        if self.has_edge(a, &dir) {
            return false;
        }
        for p in Pos::between(a, b) {
            if self.has_point(&p) {
                return false;
            }
            if self.has_edge(&p, &dir) || self.has_edge(&p, &dir.rev()) {
                return false;
            }
        }
        if self.has_edge(b, &dir.rev()) {
            return false;
        }
        return true;
    }

    pub fn connect(&mut self, a: &Pos, b: &Pos) {
        let dir = Pos::get_dir(a, b);
        self.set_edge(a, &dir);
        for p in Pos::between(a, b) {
            assert!(!self.has_edge(&p, &dir));
            assert!(!self.has_point(&p));

            self.set_edge(&p, &dir);
            self.set_edge(&p, &dir.rev());
        }
        self.set_edge(b, &dir.rev());
    }

    pub fn set_point(&mut self, pos: &Pos, mut point: Point) {
        assert!(!self.has_point(&pos));
        for i in 0..DIR_MAX {
            let dir = Dir::from_i64(i as i64);
            if let Some(nearest_pos) = self.nearest_point_pos(&pos, &dir) {
                self.point(&nearest_pos).as_mut().unwrap().nearest_points
                    [dir.rev().val() as usize] = Some(pos.clone());
                point.nearest_points[dir.val() as usize] = Some(nearest_pos.clone());
            }
        }
        self.points[pos.y as usize][pos.x as usize] = Some(point);
    }

    pub fn set_edge(&mut self, pos: &Pos, dir: &Dir) {
        self.edges[pos.y as usize][pos.x as usize][dir.val() as usize] = true;
    }

    pub fn has_point(&self, pos: &Pos) -> bool {
        self.points[pos.y as usize][pos.x as usize].is_some()
    }

    pub fn has_edge(&self, pos: &Pos, dir: &Dir) -> bool {
        self.edges[pos.y as usize][pos.x as usize][dir.val() as usize].clone()
    }

    pub fn nearest_point_pos(&self, from: &Pos, dir: &Dir) -> Option<Pos> {
        let mut cur = from.clone();

        // 最大でもself.size回loopを回せばいい
        for _ in 0..self.size {
            cur += &dir.to_pos();
            if !self.is_valid(&cur) {
                break;
            }
            if self.has_point(&cur) {
                return Some(cur);
            }
        }
        return None;
    }

    pub fn is_valid(&self, pos: &Pos) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.size as i64 && pos.y < self.size as i64
    }
}

pub enum Command {
    Add {
        new_pos: Pos,
        diagonal: Pos,
        connect: [Pos; 2],
    },
    Delete {
        pos: Pos,
    },
}

pub enum Neighborhood {
    Add,
    Delete,
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
        };
        for pos in p.iter() {
            state.grid.set_point(pos, Point::new(&pos, false));
            state.points.push(pos.clone());
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

        // 点を追加する
        self.grid.set_point(new_pos, Point::new(new_pos, true));

        // 辺を追加する
        self.grid.connect(&connect[0], new_pos);
        self.grid.connect(&connect[1], new_pos);
        self.grid.connect(&connect[0], diagonal);
        self.grid.connect(&connect[1], diagonal);

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
        self.points.push(new_pos.clone());

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