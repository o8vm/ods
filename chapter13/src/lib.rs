pub mod binarytrie;
pub mod xfasttrie;
pub mod yfasttrie;

pub trait USizeV {
    fn usize_value(&self) -> usize;
}
impl USizeV for i32 {
    fn usize_value(&self) -> usize {
        *self as usize
    }
}
