mod error;
mod spec;
#[allow(clippy::module_inception)]
mod toc;

pub use error::{PageOutbound, TocError};
pub use toc::{Toc, UserOutline};
