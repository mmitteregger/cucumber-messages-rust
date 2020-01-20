//! Private serde helper functions.

#[allow(clippy::trivially_copy_pass_by_ref)] // serde requires function to take argument by ref
#[inline]
pub fn is_zero(num: &u32) -> bool {
    *num == 0
}
