use typespec_client_core::setters;

/// Telemetry options.
#[derive(Clone, Debug, Default)]
pub struct TelemetryOptions {
    /// Optional application ID to telemetry.
    pub(crate) application_id: Option<String>,
}

impl TelemetryOptions {
    setters! {
        #[doc = "Set the application ID to telemetry."]
        application_id: String => Some(application_id),
    }
}
