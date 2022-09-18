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

    pub fn set_point(&mut self, pos: &Pos, point: Point) {
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

    pub fn recognize(&mut self, a: &Pos, target: &Pos) {
        if !Pos::is_aligned(a, target) {
            return;
        }
        let dir = Pos::get_dir(a, target);
        let point_a = self.point(a).as_mut().unwrap();
        match &point_a.nearest_points[dir.val() as usize].clone() {
            Some(current_nearest_pos) => {
                // from
                // a <-> current_nearest_pos
                // to
                // a <-> target <-> current_nearest_pos
                // -> dir
                // <- dir.rev()
                if Pos::dist(a, target) < Pos::dist(a, &current_nearest_pos) {
                    point_a.nearest_points[dir.val() as usize] = Some(target.clone());
                    self.point(current_nearest_pos)
                        .as_mut()
                        .unwrap()
                        .nearest_points[dir.rev() as usize] = Some(target.clone());
                    self.point(target).as_mut().unwrap().nearest_points[dir.rev() as usize] =
                        Some(a.clone());
                    self.point(target).as_mut().unwrap().nearest_points[dir.val() as usize] =
                        Some(current_nearest_pos.clone());
                }
            }
            None => {
                // a <-> target
                point_a.nearest_points[dir.val() as usize] = Some(target.clone());
                self.point(target).as_mut().unwrap().nearest_points[dir.rev() as usize] =
                    Some(a.clone());
            }
        }
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
        let mut data: Vec<Vec<Option<Point>>> = vec![vec![None; n]; n];
        for pos in p.iter() {
            let point = Point::new(pos, false);
            data[pos.y as usize][pos.x as usize] = Some(point);
        }

        let mut state = State {
            grid: Grid {
                points: data,
                edges: vec![vec![vec![false; DIR_MAX]; n]; n],
            },
            points: p.clone(),
            squares: vec![],
        };
        for pos1 in p.iter() {
            for pos2 in p.iter() {
                if !Pos::is_aligned(pos1, pos2) {
                    continue;
                }
                state.grid.recognize(pos1, pos2);
                state.grid.recognize(pos2, pos1);
            }
        }

        state
    }
}

#[test]
fn test_recognize() {}
