use crate::def::*;

#[derive(Clone, Debug, PartialEq, Eq)]
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
        self.add_edge(a, &dir);
        for p in Pos::between(a, b) {
            assert!(!self.has_edge(&p, &dir));
            assert!(!self.has_point(&p));

            self.add_edge(&p, &dir);
            self.add_edge(&p, &dir.rev());
        }
        self.add_edge(b, &dir.rev());
    }

    pub fn disconnect(&mut self, a: &Pos, b: &Pos) {
        let dir = Pos::get_dir(a, b);
        self.remove_edge(a, &dir);
        for p in Pos::between(a, b) {
            assert!(self.has_edge(&p, &dir));

            self.remove_edge(&p, &dir);
            self.remove_edge(&p, &dir.rev());
        }
        self.remove_edge(b, &dir.rev());
    }

    pub fn create_square(&mut self, square: &Square) {
        // 点を追加する
        self.add_point(
            &square.new_pos,
            Point::new(&square.new_pos, true),
            Some(square.clone()),
        );

        // 辺を追加する
        self.connect(&square.connect[0], &square.new_pos);
        self.connect(&square.connect[1], &square.new_pos);
        self.connect(&square.connect[0], &square.diagonal);
        self.connect(&square.connect[1], &square.diagonal);

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
        assert!(self.has_point(&pos));
        let point = self.point(&pos).as_ref().unwrap().clone();
        for i in 0..DIR_MAX {
            let dir = Dir::from_i64(i as i64);
            if let Some(nearest_pos) = &point.nearest_points[dir.val() as usize] {
                assert!(self.has_point(&nearest_pos));
                if let Some(opposite_nearest_pos) = &point.nearest_points[dir.rev().val() as usize]
                {
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
        assert!(!self.has_point(&pos));
        for i in 0..DIR_MAX {
            let dir = Dir::from_i64(i as i64);
            if let Some(nearest_pos) = self.nearest_point_pos(&pos, &dir) {
                self.point(&nearest_pos).as_mut().unwrap().nearest_points
                    [dir.rev().val() as usize] = Some(pos.clone());
                point.nearest_points[dir.val() as usize] = Some(nearest_pos.clone());
            }
        }
        point.added_info = square.clone();
        self.points[pos.y as usize][pos.x as usize] = Some(point);
    }

    pub fn add_edge(&mut self, pos: &Pos, dir: &Dir) {
        self.edges[pos.y as usize][pos.x as usize][dir.val() as usize] = true;
    }

    pub fn remove_edge(&mut self, pos: &Pos, dir: &Dir) {
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
        // FIXME: O(n)
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
        pos.x >= 0 && pos.y >= 0 && pos.x < self.size as i64 && pos.y < self.size as i64
    }
}
