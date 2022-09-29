//! Model struct for DeviceDetails type.

use serde::{Deserialize, Serialize};

/// Details about the device that took the payment.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DeviceDetails {
    /// The Square-issued ID of the device.
    pub device_id: Option<String>,
    /// The Square-issued installation ID for the device.
    pub device_installation_id: Option<String>,
    /// The name of the device set by the seller.
    pub device_name: Option<String>,
}
