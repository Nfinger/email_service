mod dashboard;
mod logout;
mod password;
mod newsletters;

pub use logout::log_out;
pub use password::*;
pub use dashboard::admin_dashboard;
pub use newsletters::*;