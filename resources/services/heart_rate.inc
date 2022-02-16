/// Service name
const SERVICE_NAME: &str = "Heart Rate Service";

/// Service UUID
const SERVICE: bluer::id::Service = bluer::id::Service::HeartRate;

/// Characteristic UUID
const HEART_RATE_MEASUREMENT_CHARACTERISTIC: bluer::id::Characteristic = bluer::id::Characteristic::HeartRateMeasurement;