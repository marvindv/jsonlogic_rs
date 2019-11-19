mod equality;
mod not_equal;
mod strict_equality;
mod strict_not_equal;
mod truthy;

pub use equality::compute_equality;
pub use not_equal::compute_not_equal;
pub use strict_equality::compute_strict_equality;
pub use strict_not_equal::compute_strict_not_equal;
pub use truthy::truthy;
