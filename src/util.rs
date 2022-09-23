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
    pub fn nextf() -> f64 {
        (next() & 4294967295) as f64 / 4294967296.
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

pub fn calc_weight(n: i64, pos: &Pos) -> i64 {
    let c = ((n - 1) / 2) as i64;
    (pos.y as i64 - c) * (pos.y as i64 - c) + (pos.x as i64 - c) * (pos.x as i64 - c) + 1
}

pub fn calc_real_score(n: usize, m: usize, score: i64) -> i64 {
    let mut s = 0;
    for i in 0..n {
        for j in 0..n {
            s += calc_weight(
                n as i64,
                &Pos {
                    x: i as i64,
                    y: j as i64,
                },
            );
        }
    }
    let result = 1e6 * (n as f64 * n as f64) * score as f64 / (m as f64 * s as f64);
    result.round() as i64
}
