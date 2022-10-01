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
