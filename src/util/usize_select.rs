use libc::size_t;

#[cfg(feature = "fix_usize_size_t")]
pub type usize_fixed = u64;

#[cfg(not(feature = "fix_usize_size_t"))]
pub type usize_fixed = usize;

#[cfg(feature = "fix_usize_size_t")]
pub type size_t_fixed = u64;

#[cfg(not(feature = "fix_usize_size_t"))]
pub type size_t_fixed = size_t;
