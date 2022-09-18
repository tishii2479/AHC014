use crate::def::*;

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
