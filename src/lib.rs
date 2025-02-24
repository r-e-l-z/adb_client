#![crate_type = "lib"]
#![forbid(unsafe_code)]
#![forbid(missing_docs)]
#![doc = include_str!("../README.md")]

mod adb_tcp_connexion;
mod adb_termios;
mod error;
mod models;
mod traits;
pub use adb_tcp_connexion::AdbTcpConnexion;
pub use error::{Result, RustADBError};
pub use models::{AdbVersion, Device, DeviceLong, DeviceState};
pub use traits::AdbCommandProvider;
