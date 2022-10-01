use crate::def::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grid {
    pub size: usize,
    pub points: Vec<Vec<Point>>,
    pub edges: Vec<Vec<Vec<bool>>>,
}

impl Grid {
    pub fn new(n: usize) -> Grid {
        let mut points = vec![vec![Point::new(&Pos { x: 0, y: 0 }); n]; n];
        for i in 0..n {
            for j in 0..n {
                points[i][j].pos = Pos {
                    x: j as i32,
                    y: i as i32,
                };
            }
        }
        Grid {
            size: n,
            points,
            edges: vec![vec![vec![false; DIR_MAX]; n]; n],
        }
    }

    pub fn point(&mut self, pos: &Pos) -> &mut Point {
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
        self.add_point(&square.new_pos, Some(square.clone()));

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
        let nearest_points = self.point(&pos).nearest_points.clone();
        for i in 0..DIR_MAX {
            let dir = Dir::from_i32(i as i32);
            if let Some(nearest_pos) = &nearest_points[dir.val() as usize] {
                debug_assert!(self.has_point(&nearest_pos));

                if let Some(opposite_nearest_pos) = &nearest_points[dir.rev().val() as usize] {
                    self.point(nearest_pos).nearest_points[dir.rev().val() as usize] =
                        Some(opposite_nearest_pos.clone());
                } else {
                    self.point(nearest_pos).nearest_points[dir.rev().val() as usize] = None;
                }
            }
        }
        for i in 0..DIR_MAX {
            self.point(&pos).nearest_points[i] = None;
            self.point(&pos).used_dir[i] = false;
        }
        self.point(&pos).created_points.clear();
        self.point(&pos).added_info = None;
        self.point(&pos).exists = false;
    }

    pub fn add_point(&mut self, pos: &Pos, square: Option<Square>) {
        debug_assert!(!self.has_point(&pos));
        self.point(pos).added_info = square;
        self.point(pos).exists = true;

        for i in 0..DIR_MAX {
            let dir = Dir::from_i32(i as i32);
            if let Some(nearest_pos) = self.nearest_point_pos(&pos, &dir) {
                let nearest_point = self.point(&nearest_pos);

                nearest_point.nearest_points[dir.rev().val() as usize] = Some(pos.clone());
                self.point(pos).nearest_points[dir.val() as usize] = Some(nearest_pos.clone());
            }
        }
    }

    fn add_edge(&mut self, pos: &Pos, dir: &Dir) {
        self.edges[pos.y as usize][pos.x as usize][dir.val() as usize] = true;
    }

    fn remove_edge(&mut self, pos: &Pos, dir: &Dir) {
        self.edges[pos.y as usize][pos.x as usize][dir.val() as usize] = false;
    }

    pub fn has_point(&self, pos: &Pos) -> bool {
        self.points[pos.y as usize][pos.x as usize].exists
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
        let created_points = &self.point(&a).created_points;
        let target_index = created_points.iter().position(|x| *x == *target).unwrap();
        self.point(&a).created_points.remove(target_index);
    }

    fn register_created_points(&mut self, a: &Pos, target: &Pos) {
        self.point(&a).created_points.push(target.clone());
    }

    pub fn is_valid(&self, pos: &Pos) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.size as i32 && pos.y < self.size as i32
    }
}
