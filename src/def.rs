use proconio::derive_readable;
use std::ops;

pub const DIR_MAX: usize = 8;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Score {
    pub base: i32,
}

impl Score {
    pub fn new() -> Score {
        Score { base: 0 }
    }
}

impl ops::AddAssign<&Score> for Score {
    fn add_assign(&mut self, rhs: &Score) {
        self.base += rhs.base;
    }
}

impl ops::SubAssign<&Score> for Score {
    fn sub_assign(&mut self, rhs: &Score) {
        self.base -= rhs.base;
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub struct Square {
    pub id: i32,
    pub new_pos: Pos,
    pub diagonal: Pos,
    pub connect: [Pos; 2],
}

impl Square {
    pub fn new(new_pos: Pos, diagonal: Pos, connect: [Pos; 2]) -> Square {
        static mut SQUARE_COUNTER: i32 = 0;
        unsafe {
            SQUARE_COUNTER += 1;
        }
        Square {
            id: unsafe { SQUARE_COUNTER },
            new_pos,
            diagonal,
            connect: [
                std::cmp::min(connect[0], connect[1]),
                std::cmp::max(connect[0], connect[1]),
            ],
        }
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    Add { square: Square },
    Delete { square: Square },
}

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
    pub fn from_i32(v: i32) -> Dir {
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
        let a = (self as i32 + 4) % 8;
        Dir::from_i32(a)
    }

    pub fn val(self) -> i32 {
        self as i32
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

    pub fn is_diagonal(self) -> bool {
        if self == Dir::Up || self == Dir::Right || self == Dir::Down || self == Dir::Left {
            return false;
        }
        true
    }

    pub fn next(self) -> Dir {
        Dir::from_i32((self.val() + 1) % DIR_MAX as i32)
    }

    pub fn prev(self) -> Dir {
        Dir::from_i32((self.val() + (DIR_MAX - 1) as i32) % DIR_MAX as i32)
    }
}

#[derive_readable]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn is_aligned(a: &Pos, b: &Pos) -> bool {
        if a == b {
            return false;
        }
        if a.y == b.y || a.x == b.x {
            return true;
        }
        if i32::abs(a.x - b.x) == i32::abs(a.y - b.y) {
            return true;
        }
        return false;
    }

    pub fn get_dir(from: &Pos, to: &Pos) -> Dir {
        debug_assert!(Pos::is_aligned(from, to));

        let delta = to - from;
        if delta.y > 0 {
            if delta.x > 0 {
                Dir::UpRight
            } else if delta.x == 0 {
                Dir::Up
            } else {
                Dir::UpLeft
            }
        } else if delta.y == 0 {
            if delta.x > 0 {
                Dir::Right
            } else {
                Dir::Left
            }
        } else {
            if delta.x > 0 {
                Dir::DownRight
            } else if delta.x == 0 {
                Dir::Down
            } else {
                Dir::DownLeft
            }
        }
    }

    pub fn between(from: &Pos, to: &Pos) -> Vec<Pos> {
        debug_assert!(Pos::is_aligned(from, to));

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

    pub fn dist(a: &Pos, b: &Pos) -> i32 {
        i32::max(i32::abs(a.x - b.x), i32::abs(a.y - b.y))
    }

    pub fn weight(a: &Pos, b: &Pos) -> i32 {
        (a.x - b.x) * (a.x - b.x) + (a.y - b.y) * (a.y - b.y)
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Point {
    pub pos: Pos,
    pub exists: bool,
    // 各方向にある最も近い点
    pub nearest_points: [Option<Pos>; DIR_MAX],
    // その点を使って作った点
    pub created_points: Vec<Pos>,
    // 各方向が長方形の辺に使われているか
    pub used_dir: [bool; DIR_MAX],
    // 追加されたときに使われた点の情報
    pub added_info: Option<Square>,
}

impl Point {
    pub fn new(pos: &Pos) -> Point {
        Point {
            pos: pos.clone(),
            exists: false,
            nearest_points: [None; DIR_MAX],
            created_points: vec![],
            used_dir: [false; DIR_MAX],
            added_info: None,
        }
    }
}
