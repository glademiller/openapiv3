#[inline]
pub const fn is_false(v: &bool) -> bool {
    !(*v)
}
