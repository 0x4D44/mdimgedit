pub mod cli;
pub mod color;
pub mod error;
pub mod ops;

pub use cli::{Cli, Command, OutputFormat};
pub use color::parse_color;
pub use error::{ImgEditError, Result};
