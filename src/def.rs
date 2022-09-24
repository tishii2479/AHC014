use proconio::derive_readable;
use std::ops;

pub const DIR_MAX: usize = 8;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Score {
    pub base: i64,
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
    pub id: i64,
    pub new_pos: Pos,
    pub diagonal: Pos,
    pub connect: [Pos; 2],
}

pub static mut SQUARE_COUNTER: i64 = 0;

impl Square {
    pub fn new(new_pos: Pos, diagonal: Pos, connect: [Pos; 2]) -> Square {
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

    #[allow(dead_code)]
    pub fn size(&self) -> i64 {
        Pos::dist(&self.new_pos, &self.connect[0]) + Pos::dist(&self.new_pos, &self.connect[1])
    }

    #[allow(dead_code)]
    pub fn all_pos(&self) -> [&Pos; 4] {
        [
            &self.new_pos,
            &self.connect[0],
            &self.diagonal,
            &self.connect[1],
        ]
    }

    #[allow(dead_code)]
    pub fn get_corners(&self) -> (i64, i64, i64, i64) {
        let mut min_x = 100;
        let mut max_x = -1;
        let mut min_y = 100;
        let mut max_y = -1;
        for pos in &self.all_pos() {
            min_x = i64::min(min_x, pos.x);
            max_x = i64::max(max_x, pos.x);
            min_y = i64::min(min_y, pos.y);
            max_y = i64::max(max_y, pos.y);
        }
        (min_x, max_x, min_y, max_y)
    }
}

#[derive(Debug)]
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

    pub fn is_diagonal(self) -> bool {
        if self == Dir::Up || self == Dir::Right || self == Dir::Down || self == Dir::Left {
            return false;
        }
        true
    }

    pub fn next(self) -> Dir {
        Dir::from_i64((self.val() + 1) % DIR_MAX as i64)
    }

    pub fn prev(self) -> Dir {
        Dir::from_i64((self.val() + (DIR_MAX - 1) as i64) % DIR_MAX as i64)
    }
}

#[derive_readable]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

    pub fn weight(a: &Pos, b: &Pos) -> i64 {
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
    // 追加された点かどうか
    pub is_added: bool,
    // 各方向にある最も近い点
    // 方向を0~7の値に決める
    pub nearest_points: Vec<Option<Pos>>,
    // その点を使って作った点
    pub created_points: Vec<Pos>,
    // 各方向が長方形の辺に使われているか
    pub used_dir: Vec<bool>,
    // 追加されたときに使われた点の情報
    pub added_info: Option<Square>,
}

impl Point {
    pub fn new(pos: &Pos, is_added: bool) -> Point {
        Point {
            pos: pos.clone(),
            is_added,
            nearest_points: vec![None; DIR_MAX],
            created_points: vec![],
            used_dir: vec![false; DIR_MAX],
            added_info: None,
        }
    }
}
