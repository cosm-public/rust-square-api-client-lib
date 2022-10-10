//! Response body struct for the Retrieve Customer Segment API

use serde::{Deserialize, Serialize};

use super::{errors::Error, CustomerSegment};

/// This is a model struct for RetrieveCustomerSegmentResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct RetrieveCustomerSegmentResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
    /// The retrieved customer group.
    pub segment: CustomerSegment,
}
