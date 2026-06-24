// mod commands;
mod error;
mod events;
mod sparkle;

pub use error::{Error, Result};
pub use events::NotificationKind;
pub use sparkle::{Sparkle, SparkleConfig};
