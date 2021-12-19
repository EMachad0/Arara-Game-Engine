pub use log::*;
pub use pretty_env_logger::{self as logger, env_logger::*};

pub mod prelude {
    pub use log::*;
    pub use pretty_env_logger::{self as logger, env_logger::*};
}

pub fn debug_logging_level() {
    trace!("a trace example");
    debug!("deboogging");
    info!("such information");
    warn!("o_O");
    error!("boom");
}
