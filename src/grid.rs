use crate::def::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grid {
    pub size: usize,
    pub points: Vec<Vec<Option<Point>>>,
    pub edges: Vec<Vec<Vec<usize>>>,
}

impl Grid {
    pub fn new(n: usize) -> Grid {
        Grid {
            size: n,
            points: vec![vec![None; n]; n],
            edges: vec![vec![vec![0; DIR_MAX]; n]; n],
        }
    }

    pub fn point(&mut self, pos: &Pos) -> &mut Option<Point> {
        &mut self.points[pos.y as usize][pos.x as usize]
    }

    pub fn can_connect(&self, a: &Pos, b: &Pos) -> bool {
        debug_assert!(Pos::is_aligned(a, b));
        for p in Pos::between(a, b) {
            if self.has_point(&p) {
                return false;
            }
        }
        return true;
    }

    fn connect(&mut self, a: &Pos, b: &Pos, is_reverse: bool) -> Score {
        let mut score = Score::new();
        let dir = Pos::get_dir(a, b);
        if self.edge_count(&a, &dir) >= 1 {
            score.edge_penalty += 1;
        }
        self.add_edge(&a, &dir);
        for p in Pos::between(&a, &b) {
            if !is_reverse {
                debug_assert!(!self.has_point(&p));
            }

            if self.edge_count(&p, &dir) >= 1 {
                score.edge_penalty += 1;
            }
            self.add_edge(&p, &dir);

            if self.edge_count(&p, &dir.rev()) >= 1 {
                score.edge_penalty += 1;
            }
            self.add_edge(&p, &dir.rev());
        }
        if self.edge_count(&b, &dir.rev()) >= 1 {
            score.edge_penalty += 1;
        }
        self.add_edge(&b, &dir.rev());
        score
    }

    fn disconnect(&mut self, a: &Pos, b: &Pos) -> Score {
        let mut score = Score::new();
        let dir = Pos::get_dir(&a, &b);
        if self.edge_count(&a, &dir) >= 2 {
            score.edge_penalty -= 1;
        }
        self.remove_edge(&a, &dir);
        for p in Pos::between(a, b) {
            debug_assert!(self.edge_count(&p, &dir) > 0);

            if self.edge_count(&p, &dir) >= 2 {
                score.edge_penalty -= 1;
            }
            self.remove_edge(&p, &dir);
            if self.edge_count(&p, &dir.rev()) >= 2 {
                score.edge_penalty -= 1;
            }
            self.remove_edge(&p, &dir.rev());
        }
        if self.edge_count(&b, &dir.rev()) >= 2 {
            score.edge_penalty -= 1;
        }
        self.remove_edge(&b, &dir.rev());
        score
    }

    pub fn create_square(&mut self, square: &Square, is_reverse: bool) -> Score {
        // 点を追加する
        let mut score = self.add_point(
            &square.new_pos,
            Point::new(&square.new_pos),
            Some(square.clone()),
        );

        // 辺を追加する
        score += &self.connect(&square.connect[0], &square.new_pos, is_reverse);
        score += &self.connect(&square.connect[1], &square.new_pos, is_reverse);
        score += &self.connect(&square.connect[0], &square.diagonal, is_reverse);
        score += &self.connect(&square.connect[1], &square.diagonal, is_reverse);

        // 使った点を登録する
        self.register_created_points(&square.connect[0], &square.new_pos);
        self.register_created_points(&square.connect[1], &square.new_pos);
        self.register_created_points(&square.diagonal, &square.new_pos);

        score
    }

    pub fn delete_square(&mut self, square: &Square) -> Score {
        // 点を削除する
        let mut score = self.remove_point(&square.new_pos);

        // 辺を削除する
        score += &self.disconnect(&square.connect[0], &square.new_pos);
        score += &self.disconnect(&square.connect[1], &square.new_pos);
        score += &self.disconnect(&square.connect[0], &square.diagonal);
        score += &self.disconnect(&square.connect[1], &square.diagonal);

        // 使った点の登録を消す
        self.unregister_created_points(&square.connect[0], &square.new_pos);
        self.unregister_created_points(&square.connect[1], &square.new_pos);
        self.unregister_created_points(&square.diagonal, &square.new_pos);

        score
    }

    pub fn remove_point(&mut self, pos: &Pos) -> Score {
        debug_assert!(self.has_point(&pos));

        let score = Score::new();

        let nearest_points = self.point(&pos).as_ref().unwrap().nearest_points.clone();
        for i in 0..DIR_MAX {
            let dir = Dir::from_i64(i as i64);
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
        score
    }

    pub fn add_point(&mut self, pos: &Pos, mut point: Point, square: Option<Square>) -> Score {
        debug_assert!(!self.has_point(&pos));

        let score = Score::new();

        for i in 0..DIR_MAX {
            let dir = Dir::from_i64(i as i64);
            if let Some(nearest_pos) = self.nearest_point_pos(&pos, &dir) {
                let nearest_point = self.point(&nearest_pos).as_mut().unwrap();

                nearest_point.nearest_points[dir.rev().val() as usize] = Some(pos.clone());
                point.nearest_points[dir.val() as usize] = Some(nearest_pos.clone());
            }
        }
        point.added_info = square;
        self.points[pos.y as usize][pos.x as usize] = Some(point);

        score
    }

    fn add_edge(&mut self, pos: &Pos, dir: &Dir) {
        self.edges[pos.y as usize][pos.x as usize][dir.val() as usize] += 1;
    }

    fn remove_edge(&mut self, pos: &Pos, dir: &Dir) {
        debug_assert!(self.edges[pos.y as usize][pos.x as usize][dir.val() as usize] > 0);
        self.edges[pos.y as usize][pos.x as usize][dir.val() as usize] -= 1;
    }

    pub fn has_point(&self, pos: &Pos) -> bool {
        self.points[pos.y as usize][pos.x as usize].is_some()
    }

    pub fn edge_count(&self, pos: &Pos, dir: &Dir) -> usize {
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
        pos.x >= 0 && pos.y >= 0 && pos.x < self.size as i64 && pos.y < self.size as i64
    }
}
