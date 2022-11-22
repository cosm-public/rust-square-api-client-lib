//! Query String parameters for the Retrieve Subscription API

/// This is a model struct for RetrieveSubscriptionParameters (query parameters)
#[derive(Clone, Debug, Default)]
pub struct RetrieveSubscriptionParameters {
    /// A query parameter to specify related information to be included in the response.
    ///
    /// The supported query parameter values are:
    ///
    /// * actions: to include scheduled actions on the targeted subscription.
    pub include: Option<String>,
}

impl RetrieveSubscriptionParameters {
    pub fn to_query_string(&self) -> String {
        self.to_string()
    }
}

impl From<RetrieveSubscriptionParameters> for String {
    fn from(retrieve_subscription_parameters: RetrieveSubscriptionParameters) -> Self {
        retrieve_subscription_parameters.to_string()
    }
}

impl ToString for RetrieveSubscriptionParameters {
    fn to_string(&self) -> String {
        let mut params = Vec::new();

        if let Some(include) = &self.include {
            params.push(format!("include={}", include));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}
