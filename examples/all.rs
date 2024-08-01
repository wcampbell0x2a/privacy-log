use privacy_log::{error, info, trace, warn};

fn main() {
    env_logger::init();

    error!("error");
    info!("error");
    trace!("error");
    warn!("error");
}
