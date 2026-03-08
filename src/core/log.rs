pub mod prelude {
    pub use super::init::subscriber_init;
    pub use tracing::{debug, error, info, trace, warn};
    pub use tracing::{debug_span, error_span, event_enabled, info_span, trace_span, warn_span};
}

pub mod init {
    pub fn subscriber_init() {
        let timer =
            tracing_subscriber::fmt::time::LocalTime::new(time::macros::format_description!(
                "[year repr:last_two]/[month]/[day] [hour]:[minute]:[second].[subsecond digits:6]"
            ));

        let format = tracing_subscriber::fmt::format()
            .with_timer(timer)
            .with_ansi(true)
            .compact();

        tracing_subscriber::fmt()
            .event_format(format)
            .with_max_level(tracing::Level::INFO)
            .init();
    }
}
