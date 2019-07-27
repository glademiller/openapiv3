#[allow(clippy::trivially_copy_pass_by_ref)] // needs to match signature for use in serde attribute
#[inline]
pub const fn is_false(v: &bool) -> bool {
    !(*v)
}
