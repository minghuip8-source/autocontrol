pub mod control;
pub mod error;
pub mod keyboard;
pub mod platform;

pub use control::AutoControl;
pub use error::{AutoError, Result};
pub use keyboard::Key;
