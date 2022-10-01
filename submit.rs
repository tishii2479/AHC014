const TIME_LIMIT: f32 = 4.97;
const LOOP_INTERVAL: usize = 100;
const WRITE_SCORE_LOG: bool = false;

const MULTIPLE_ADD_RECURSION_LIMIT: usize = 20;
const DELETION_RECURSION_LIMIT: usize = 10;

pub mod def {
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
                nearest_points: [None; DIR_MAX],
                created_points: vec![],
                used_dir: [false; DIR_MAX],
                added_info: None,
            }
        }
    }
}
pub mod framework {
    use crate::def::Command;
    use crate::neighborhood::Neighborhood;

    pub trait IState {
        fn get_score(&self, progress: f32) -> f32;
        fn perform_command(&mut self, command: &Command) -> Vec<Command>;
        fn reverse_command(&mut self, command: &Command);
    }

    pub trait INeighborhoodSelector {
        fn select(&self) -> Neighborhood;
        fn step(&mut self, neighborhood: &Neighborhood, adopted: bool);
    }

    pub trait IOptimizer {
        fn update_temp(&mut self, progress: f32);
        fn should_adopt_new_state(&self, score_diff: f32) -> bool;
    }

    pub trait ISolver {
        fn solve(&mut self, time_limit: f32);
    }
}
pub mod grid {
    use crate::def::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Grid {
        pub size: usize,
        pub points: Vec<Vec<Option<Point>>>,
        pub edges: Vec<Vec<Vec<bool>>>,
    }

    impl Grid {
        pub fn new(n: usize) -> Grid {
            Grid {
                size: n,
                points: vec![vec![None; n]; n],
                edges: vec![vec![vec![false; DIR_MAX]; n]; n],
            }
        }

        pub fn point(&mut self, pos: &Pos) -> &mut Option<Point> {
            &mut self.points[pos.y as usize][pos.x as usize]
        }

        pub fn can_connect(&self, a: &Pos, b: &Pos) -> bool {
            debug_assert!(Pos::is_aligned(a, b));
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

        fn connect(&mut self, a: &Pos, b: &Pos, is_reverse: bool) {
            let dir = Pos::get_dir(a, b);
            self.add_edge(a, &dir);
            for p in Pos::between(a, b) {
                debug_assert!(!self.has_edge(&p, &dir));
                if !is_reverse {
                    debug_assert!(!self.has_point(&p));
                }
                self.add_edge(&p, &dir);
                self.add_edge(&p, &dir.rev());
            }
            self.add_edge(b, &dir.rev());
        }

        fn disconnect(&mut self, a: &Pos, b: &Pos) {
            let dir = Pos::get_dir(a, b);
            self.remove_edge(a, &dir);
            for p in Pos::between(a, b) {
                debug_assert!(self.has_edge(&p, &dir));

                self.remove_edge(&p, &dir);
                self.remove_edge(&p, &dir.rev());
            }
            self.remove_edge(b, &dir.rev());
        }

        pub fn create_square(&mut self, square: &Square, is_reverse: bool) {
            // 点を追加する
            self.add_point(
                &square.new_pos,
                Point::new(&square.new_pos),
                Some(square.clone()),
            );

            // 辺を追加する
            self.connect(&square.connect[0], &square.new_pos, is_reverse);
            self.connect(&square.connect[1], &square.new_pos, is_reverse);
            self.connect(&square.connect[0], &square.diagonal, is_reverse);
            self.connect(&square.connect[1], &square.diagonal, is_reverse);

            // 使った点を登録する
            self.register_created_points(&square.connect[0], &square.new_pos);
            self.register_created_points(&square.connect[1], &square.new_pos);
            self.register_created_points(&square.diagonal, &square.new_pos);
        }

        pub fn delete_square(&mut self, square: &Square) {
            // 点を削除する
            self.remove_point(&square.new_pos);

            // 辺を削除する
            self.disconnect(&square.connect[0], &square.new_pos);
            self.disconnect(&square.connect[1], &square.new_pos);
            self.disconnect(&square.connect[0], &square.diagonal);
            self.disconnect(&square.connect[1], &square.diagonal);

            // 使った点の登録を消す
            self.unregister_created_points(&square.connect[0], &square.new_pos);
            self.unregister_created_points(&square.connect[1], &square.new_pos);
            self.unregister_created_points(&square.diagonal, &square.new_pos);
        }

        pub fn remove_point(&mut self, pos: &Pos) {
            debug_assert!(self.has_point(&pos));
            let nearest_points = self.point(&pos).as_ref().unwrap().nearest_points.clone();
            for i in 0..DIR_MAX {
                let dir = Dir::from_i32(i as i32);
                if let Some(nearest_pos) = &nearest_points[dir.val() as usize] {
                    debug_assert!(self.has_point(&nearest_pos));

                    if let Some(opposite_nearest_pos) = &nearest_points[dir.rev().val() as usize] {
                        self.point(nearest_pos).as_mut().unwrap().nearest_points
                            [dir.rev().val() as usize] = Some(opposite_nearest_pos.clone());
                    } else {
                        self.point(nearest_pos).as_mut().unwrap().nearest_points
                            [dir.rev().val() as usize] = None;
                    }
                }
            }
            self.points[pos.y as usize][pos.x as usize] = None;
        }

        pub fn add_point(&mut self, pos: &Pos, mut point: Point, square: Option<Square>) {
            debug_assert!(!self.has_point(&pos));

            for i in 0..DIR_MAX {
                let dir = Dir::from_i32(i as i32);
                if let Some(nearest_pos) = self.nearest_point_pos(&pos, &dir) {
                    let nearest_point = self.point(&nearest_pos).as_mut().unwrap();

                    nearest_point.nearest_points[dir.rev().val() as usize] = Some(pos.clone());
                    point.nearest_points[dir.val() as usize] = Some(nearest_pos.clone());
                }
            }
            point.added_info = square;
            self.points[pos.y as usize][pos.x as usize] = Some(point);
        }

        fn add_edge(&mut self, pos: &Pos, dir: &Dir) {
            self.edges[pos.y as usize][pos.x as usize][dir.val() as usize] = true;
        }

        fn remove_edge(&mut self, pos: &Pos, dir: &Dir) {
            self.edges[pos.y as usize][pos.x as usize][dir.val() as usize] = false;
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

        fn unregister_created_points(&mut self, a: &Pos, target: &Pos) {
            let created_points = &self.point(&a).as_mut().unwrap().created_points;
            let target_index = created_points.iter().position(|x| *x == *target).unwrap();
            self.point(&a)
                .as_mut()
                .unwrap()
                .created_points
                .remove(target_index);
        }

        fn register_created_points(&mut self, a: &Pos, target: &Pos) {
            self.point(&a)
                .as_mut()
                .unwrap()
                .created_points
                .push(target.clone());
        }

        pub fn is_valid(&self, pos: &Pos) -> bool {
            pos.x >= 0 && pos.y >= 0 && pos.x < self.size as i32 && pos.y < self.size as i32
        }
    }
}
pub mod neighborhood {
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
            let selected_p = state.sample_point_pos();
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
            start_score: f32,
        ) {
            if state.get_score(1.) > start_score {
                return;
            }
            if *recursion_count >= *recursion_limit {
                return;
            }
            *recursion_count += 1;
            let mut used_bits: usize = 0;
            for _ in 0..DIR_MAX {
                let i = rnd::gen_range(0, DIR_MAX);
                if used_bits & (1 << i) > 0 {
                    continue;
                }
                used_bits |= 1 << i;
                let dir = Dir::from_i32(i as i32);
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
            let selected_p = state.sample_point_pos();
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
            let mut used_bits: usize = 0;
            for _ in 0..DIR_MAX {
                let i = rnd::gen_range(0, DIR_MAX);
                if used_bits & (1 << i) > 0 {
                    continue;
                }
                used_bits |= 1 << i;
                let diagonal_dir = Dir::from_i32(i as i32);
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
            nearest_points: &[Option<Pos>; DIR_MAX],
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
            let square = state.sample_square();
            Neighborhood::attempt_delete(state, &square)
        }

        fn attempt_delete(state: &mut State, square: &Square) -> Vec<Command> {
            state.perform_command(&Command::Delete { square: *square })
        }

        fn perform_change_square(state: &mut State) -> Vec<Command> {
            // 四角を作っている点を探す
            if state.squares.len() == 0 {
                return vec![];
            }
            let square = state.sample_square();
            Neighborhood::attempt_change_square(state, &square)
        }

        fn attempt_change_square(state: &mut State, square: &Square) -> Vec<Command> {
            let start_score = state.get_score(1.);
            let mut performed_commands =
                state.perform_command(&Command::Delete { square: *square });

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
            let selected_square = state.sample_square();
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

        pub fn from_i32(v: i32) -> Neighborhood {
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
}
pub mod state {
    use crate::grid::*;
    use crate::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct State {
        pub grid: Grid,
        pub squares: Vec<Square>,
        pub score: Score,
    }

    impl State {
        pub fn new(n: usize, p: Vec<Pos>) -> State {
            let mut state = State {
                grid: Grid::new(n),
                squares: vec![],
                score: Score::new(),
            };
            for pos in p.iter() {
                state.grid.add_point(pos, Point::new(&pos), None);
                state.score.base += state.weight(&pos);
            }
            state
        }
    }

    impl State {
        pub fn can_perform_add(&mut self, square: &Square, is_reverse: bool) -> bool {
            debug_assert!(Pos::is_aligned(&square.diagonal, &square.connect[0]));
            debug_assert!(Pos::is_aligned(&square.diagonal, &square.connect[1]));
            debug_assert!(Pos::is_aligned(&square.new_pos, &square.connect[0]));
            debug_assert!(Pos::is_aligned(&square.new_pos, &square.connect[1]));

            // new_posに既に点がないか確認
            if self.grid.has_point(&square.new_pos) {
                return false;
            }

            // 作ろうとしてる四角の辺に既に点、辺がないか確認する
            if !is_reverse
                && (!self.grid.can_connect(&square.connect[0], &square.new_pos)
                    || !self.grid.can_connect(&square.connect[1], &square.new_pos)
                    || !self.grid.can_connect(&square.connect[0], &square.diagonal)
                    || !self.grid.can_connect(&square.connect[1], &square.diagonal))
            {
                return false;
            }

            true
        }

        pub fn perform_add(&mut self, square: &Square, is_reverse: bool) -> Vec<Command> {
            if !self.can_perform_add(square, is_reverse) {
                return vec![];
            }

            self.grid.create_square(&square, is_reverse);

            self.squares.push(square.clone());

            // スコアの更新
            self.score.base += self.weight(&square.new_pos);

            vec![Command::Add { square: *square }]
        }

        pub fn perform_delete(&mut self, square: &Square, performed_commands: &mut Vec<Command>) {
            debug_assert!(Pos::is_aligned(&square.diagonal, &square.connect[0]));
            debug_assert!(Pos::is_aligned(&square.diagonal, &square.connect[1]));
            debug_assert!(Pos::is_aligned(&square.new_pos, &square.connect[0]));
            debug_assert!(Pos::is_aligned(&square.new_pos, &square.connect[1]));

            debug_assert!(self.grid.has_point(&square.new_pos));

            // new_posの点を使って作られた四角を再帰的に消す
            let created_points = self
                .grid
                .point(&square.new_pos)
                .as_ref()
                .unwrap()
                .created_points
                .clone();
            for created_point in &created_points {
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
            self.score.base -= self.weight(&square.new_pos);
            performed_commands.push(Command::Delete { square: *square });
        }

        pub fn sample_point_pos(&self) -> Pos {
            loop {
                let pos = Pos {
                    x: rnd::gen_range(0, self.grid.size) as i32,
                    y: rnd::gen_range(0, self.grid.size) as i32,
                };
                if self.grid.has_point(&pos) {
                    return pos;
                }
            }
        }

        pub fn sample_square(&self) -> Square {
            self.squares[rnd::gen_range(0, self.squares.len()) as usize]
        }

        pub fn calc_deletion_size(
            &mut self,
            new_pos: &Pos,
            recursion_limit: usize,
            parent_dep_size: usize,
        ) -> usize {
            let mut dep_size: usize = 1 + parent_dep_size;

            let created_points = self
                .grid
                .point(&new_pos)
                .as_ref()
                .unwrap()
                .created_points
                .clone();
            for created_point in &created_points {
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
        pub fn weight(&self, pos: &Pos) -> i32 {
            let c = ((self.grid.size - 1) / 2) as i32;
            (pos.y as i32 - c) * (pos.y as i32 - c) + (pos.x as i32 - c) * (pos.x as i32 - c) + 1
        }
    }
}
pub mod util {
    use crate::Pos;

    pub mod rnd {
        #[allow(unused)]
        static mut S: usize = 88172645463325252;

        #[allow(unused)]
        #[inline]
        pub fn next() -> usize {
            unsafe {
                S = S ^ S << 7;
                S = S ^ S >> 9;
                S
            }
        }

        #[allow(unused)]
        #[inline]
        pub fn nextf() -> f32 {
            (next() & 4294967295) as f32 / 4294967296.
        }

        #[allow(unused)]
        #[inline]
        pub fn gen_range(low: usize, high: usize) -> usize {
            (next() % (high - low)) + low
        }
    }

    pub mod time {
        static mut START: f64 = -1.;
        #[allow(unused)]
        pub fn start_clock() {
            let _ = elapsed_seconds();
        }

        #[allow(unused)]
        #[inline]
        pub fn elapsed_seconds() -> f64 {
            let t = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64();
            unsafe {
                if START < 0. {
                    START = t;
                }
                t - START
            }
        }
    }

    pub fn calc_weight(n: i32, pos: &Pos) -> i32 {
        let c = ((n - 1) / 2) as i32;
        (pos.y as i32 - c) * (pos.y as i32 - c) + (pos.x as i32 - c) * (pos.x as i32 - c) + 1
    }

    pub fn calc_real_score(n: usize, m: usize, score: i32) -> i32 {
        let mut s = 0;
        for i in 0..n {
            for j in 0..n {
                s += calc_weight(
                    n as i32,
                    &Pos {
                        x: i as i32,
                        y: j as i32,
                    },
                );
            }
        }
        let result = 1e6 * (n as f32 * n as f32) * score as f32 / (m as f32 * s as f32);
        result.round() as i32
    }
}

use std::{fs, io::Write};

use def::*;
use framework::*;
use neighborhood::*;
use proconio::input;
use state::*;
use util::*;

#[allow(unused_variables)]
fn calc_start_temp(n: usize, m: usize) -> f32 {
    500. * (n as f32 / 30.).powf(2.)
}

#[allow(unused_variables)]
fn calc_end_temp(n: usize, m: usize) -> f32 {
    25. * (n as f32 / 30.).powf(2.)
}

struct NeighborhoodSelector {
    total_cnt: Vec<i32>,
    adopted_cnt: Vec<i32>,
}

impl NeighborhoodSelector {
    fn new(neighborhood_count: usize) -> NeighborhoodSelector {
        NeighborhoodSelector {
            total_cnt: vec![0; neighborhood_count],
            adopted_cnt: vec![0; neighborhood_count],
        }
    }

    fn output_statistics(&self) {
        for i in 0..self.total_cnt.len() {
            eprintln!(
                "{:?}: (total_cnt: {}, adopted_cnt: {}, adopted_ratio: {:.4})",
                Neighborhood::from_i32(i as i32),
                self.total_cnt[i],
                self.adopted_cnt[i],
                self.adopted_cnt[i] as f32 / self.total_cnt[i] as f32,
            );
        }
    }
}

impl INeighborhoodSelector for NeighborhoodSelector {
    fn select(&self) -> Neighborhood {
        let p = rnd::nextf();
        if p < 0.05 {
            Neighborhood::Delete
        } else if p < 0.15 {
            Neighborhood::ChangeSquare
        } else if p < 0.25 {
            Neighborhood::SplitSquare
        } else {
            Neighborhood::Add
        }
    }

    fn step(&mut self, neighborhood: &Neighborhood, adopted: bool) {
        self.total_cnt[*neighborhood as usize] += 1;
        if adopted {
            self.adopted_cnt[*neighborhood as usize] += 1;
        }
    }
}

struct Optimizer {
    start_temp: f32,
    end_temp: f32,
    current_temp: f32,
}

impl IOptimizer for Optimizer {
    fn update_temp(&mut self, progress: f32) {
        self.current_temp = self.start_temp + (self.end_temp - self.start_temp) * progress;
    }

    fn should_adopt_new_state(&self, score_diff: f32) -> bool {
        let prob = (score_diff / self.current_temp).exp();
        return prob > rnd::nextf();
    }
}

impl Optimizer {
    fn new(start_temp: f32, end_temp: f32) -> Optimizer {
        let mut optimizer = Optimizer {
            start_temp,
            end_temp,
            current_temp: 0.,
        };
        optimizer.update_temp(0.);
        optimizer
    }
}

impl IState for State {
    #[allow(unused_variables)]
    fn get_score(&self, progress: f32) -> f32 {
        let base_score = self.score.base as f32;
        base_score
    }

    fn perform_command(&mut self, command: &Command) -> Vec<Command> {
        match command {
            Command::Add { square } => self.perform_add(square, false),
            Command::Delete { square } => {
                // 削除する四角が多すぎるときは不採用
                if self.calc_deletion_size(&square.new_pos, DELETION_RECURSION_LIMIT, 0)
                    >= DELETION_RECURSION_LIMIT
                {
                    return vec![];
                }

                let mut performed_commands: Vec<Command> = vec![];
                self.perform_delete(square, &mut performed_commands);
                performed_commands
            }
        }
    }

    fn reverse_command(&mut self, command: &Command) {
        match command {
            Command::Add { square } => {
                let mut performed_commands: Vec<Command> = vec![];
                self.perform_delete(square, &mut performed_commands);
                performed_commands
            }
            Command::Delete { square } => self.perform_add(square, true),
        };
    }
}

struct Solver {
    state: State,
    neighborhood_selector: NeighborhoodSelector,
    optimizer: Optimizer,
    score_history: Vec<f32>,
}

impl ISolver for Solver {
    fn solve(&mut self, time_limit: f32) {
        let mut loop_count = 0;
        let mut best_state = self.state.clone();
        let mut progress = time::elapsed_seconds() as f32 / time_limit;
        while progress < 1. {
            let is_interval = (loop_count % LOOP_INTERVAL) == 0;
            if is_interval {
                progress = time::elapsed_seconds() as f32 / time_limit;
                self.optimizer.update_temp(progress);
            }

            let neighborhood = self.neighborhood_selector.select();

            let current_score = self.state.get_score(progress);

            let performed_commands = neighborhood.perform(&mut self.state);

            let new_score = self.state.get_score(progress);

            let adopt_new_state = self
                .optimizer
                .should_adopt_new_state(new_score - current_score)
                && performed_commands.len() > 0;

            if !adopt_new_state {
                for command in performed_commands.iter().rev() {
                    self.state.reverse_command(command);
                }
            }

            self.neighborhood_selector
                .step(&neighborhood, adopt_new_state);

            if is_interval && self.state.get_score(1.) > best_state.get_score(1.) {
                best_state = self.state.clone();
            }
            loop_count += 1;
        }
        self.state = best_state.clone();
    }
}

impl Solver {
    fn new(
        state: State,
        neighborhood_selector: NeighborhoodSelector,
        optimizer: Optimizer,
    ) -> Solver {
        Solver {
            state: state.clone(),
            neighborhood_selector,
            optimizer,
            score_history: vec![],
        }
    }

    fn output(&mut self) {
        println!("{}", self.state.squares.len());
        self.state.squares.sort_by(|a, b| a.id.cmp(&b.id));
        for Square {
            id: _,
            new_pos,
            diagonal,
            connect,
        } in &self.state.squares
        {
            println!(
                "{} {} {} {} {} {} {} {}",
                new_pos.x,
                new_pos.y,
                connect[0].x,
                connect[0].y,
                diagonal.x,
                diagonal.y,
                connect[1].x,
                connect[1].y
            );
        }
    }

    #[allow(dead_code)]
    fn output_statistics(&self, n: usize, m: usize) {
        eprintln!("state_score: {}", self.state.get_score(1.));
        eprintln!(
            "real_score: {}",
            calc_real_score(n, m, self.state.score.base as i32)
        );
        self.neighborhood_selector.output_statistics();

        if WRITE_SCORE_LOG {
            // スコア遷移の書き出し
            let mut file = fs::File::create("tools/out/score_log.txt").unwrap();
            for score in &self.score_history {
                let score = calc_real_score(n, m, *score as i32);
                file.write((score.to_string() + "\n").as_bytes()).unwrap();
            }
        }
    }
}

fn main() {
    time::start_clock();

    input! {
        n: usize,
        m: usize,
        p: [Pos; m]
    }

    let start_temp: f32 = calc_start_temp(n, m);
    let end_temp: f32 = calc_end_temp(n, m);

    let state = State::new(n, p);
    let mut solver = Solver::new(
        state,
        NeighborhoodSelector::new(Neighborhood::all().len()),
        Optimizer::new(start_temp, end_temp),
    );

    solver.solve(TIME_LIMIT);
    solver.output();
}
