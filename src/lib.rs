#[doc(hidden)]
pub mod __private_api;

/// The standard logging macro.
///
/// This macro will generically log with the specified `Level` and `format!`
/// based argument list.
///
/// # Examples
///
/// ```
/// use log::{log, Level};
///
/// # fn main() {
/// let data = (42, "Forty-two");
/// let private_data = "private";
///
/// log!(Level::Error, "Received errors: {}, {}", data.0, data.1);
/// log!(target: "app_events", Level::Warn, "App warning: {}, {}, {}",
///     data.0, data.1, private_data);
/// # }
/// ```
#[macro_export]
macro_rules! log {
    // log!(target: "my_target", Level::Info, key1:? = 42, key2 = true; "a {} event", "log");
    (target: $target:expr, $lvl:expr, $($key:tt $(:$capture:tt)? $(= $value:expr)?),+; $($arg:tt)+) => ({
        let lvl = $lvl;
        if lvl <= log::STATIC_MAX_LEVEL && lvl <= log::max_level() {
            $crate::__private_api::log::<&_>(
                log::__private_api::format_args!($($arg)+),
                lvl,
                &($target, log::__private_api::module_path!()),
            );
        }
    });

    // log!(target: "my_target", Level::Info, "a {} event", "log");
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => ({
        let lvl = $lvl;
        if lvl <= log::STATIC_MAX_LEVEL && lvl <= log::max_level() {
            $crate::__private_api::log(
                log::__private_api::format_args!($($arg)+),
                lvl,
                &($target, log::__private_api::module_path!()),
            );
        }
    });

    // log!(Level::Info, "a log event")
    ($lvl:expr, $($arg:tt)+) => ($crate::log!(target: std::module_path!(), $lvl, $($arg)+));
}

/// Logs a message at the error level.
///
/// # Examples
///
/// ```
/// use log::error;
///
/// # fn main() {
/// let (err_info, port) = ("No connection", 22);
///
/// error!("Error: {err_info} on port {port}");
/// error!(target: "app_events", "App Error: {err_info}, Port: {port}");
/// # }
/// ```
#[macro_export]
macro_rules! error {
    // error!(target: "my_target", key1 = 42, key2 = true; "a {} event", "log")
    // error!(target: "my_target", "a {} event", "log")
    (target: $target:expr, $($arg:tt)+) => ($crate::log!(target: $target, log::Level::Error, $($arg)+));

    // error!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log!(log::Level::Error, $($arg)+))
}

/// Logs a message at the warn level.
///
/// # Examples
///
/// ```
/// use log::warn;
///
/// # fn main() {
/// let warn_description = "Invalid Input";
///
/// warn!("Warning! {warn_description}!");
/// warn!(target: "input_events", "App received warning: {warn_description}");
/// # }
/// ```
#[macro_export]
macro_rules! warn {
    // warn!(target: "my_target", key1 = 42, key2 = true; "a {} event", "log")
    // warn!(target: "my_target", "a {} event", "log")
    (target: $target:expr, $($arg:tt)+) => ($crate::log!(target: $target, log::Level::Warn, $($arg)+));

    // warn!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log!(log::Level::Warn, $($arg)+))
}

/// Logs a message at the info level.
///
/// # Examples
///
/// ```
/// use log::info;
///
/// # fn main() {
/// # struct Connection { port: u32, speed: f32 }
/// let conn_info = Connection { port: 40, speed: 3.20 };
///
/// info!("Connected to port {} at {} Mb/s", conn_info.port, conn_info.speed);
/// info!(target: "connection_events", "Successful connection, port: {}, speed: {}",
///       conn_info.port, conn_info.speed);
/// # }
/// ```
#[macro_export]
macro_rules! info {
    // info!(target: "my_target", key1 = 42, key2 = true; "a {} event", "log")
    // info!(target: "my_target", "a {} event", "log")
    (target: $target:expr, $($arg:tt)+) => ($crate::log!(target: $target, log::Level::Info, $($arg)+));

    // info!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log!(log::Level::Info, $($arg)+))
}

/// Logs a message at the debug level.
///
/// # Examples
///
/// ```
/// use log::debug;
///
/// # fn main() {
/// # struct Position { x: f32, y: f32 }
/// let pos = Position { x: 3.234, y: -1.223 };
///
/// debug!("New position: x: {}, y: {}", pos.x, pos.y);
/// debug!(target: "app_events", "New position: x: {}, y: {}", pos.x, pos.y);
/// # }
/// ```
#[macro_export]
macro_rules! debug {
    // debug!(target: "my_target", key1 = 42, key2 = true; "a {} event", "log")
    // debug!(target: "my_target", "a {} event", "log")
    (target: $target:expr, $($arg:tt)+) => ($crate::log!(target: $target, log::Level::Debug, $($arg)+));

    // debug!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log!(log::Level::Debug, $($arg)+))
}

/// Logs a message at the trace level.
///
/// # Examples
///
/// ```
/// use log::trace;
///
/// # fn main() {
/// # struct Position { x: f32, y: f32 }
/// let pos = Position { x: 3.234, y: -1.223 };
///
/// trace!("Position is: x: {}, y: {}", pos.x, pos.y);
/// trace!(target: "app_events", "x is {} and y is {}",
///        if pos.x >= 0.0 { "positive" } else { "negative" },
///        if pos.y >= 0.0 { "positive" } else { "negative" });
/// # }
/// ```
#[macro_export]
macro_rules! trace {
    // trace!(target: "my_target", key1 = 42, key2 = true; "a {} event", "log")
    // trace!(target: "my_target", "a {} event", "log")
    (target: $target:expr, $($arg:tt)+) => ($crate::log!(target: $target, log::Level::Trace, $($arg)+));

    // trace!("a {} event", "log")
    ($($arg:tt)+) => ($crate::log!(log::Level::Trace, $($arg)+))
}
