#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    group: [Group; 1],
}
impl RegisterBlock {
    #[doc = "0x00..0x60 - GROUP\\[%s\\]"]
    #[inline(always)]
    pub const fn group(&self, n: usize) -> &Group {
        &self.group[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x60 - GROUP\\[%s\\]"]
    #[inline(always)]
    pub fn group_iter(&self) -> impl Iterator<Item = &Group> {
        self.group.iter()
    }
}
#[doc = "GROUP\\[%s\\]"]
pub use self::group::Group;
#[doc = r"Cluster"]
#[doc = "GROUP\\[%s\\]"]
pub mod group;
