pub mod scapegoattree;

pub fn log32(q: usize) -> i64 {
    (q as f64).log(3.0/2.0).floor() as i64
}