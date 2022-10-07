//! Query parameters for the List Gift Card Activities API

use super::{
    enums::{GiftCardActivityType, SortOrder},
    DateTime,
};

/// This is a model struct for ListGiftCardActivitiesParameters (query parameters)
#[derive(Clone, Debug, Default)]
pub struct ListGiftCardActivitiesParameters {
    /// If a gift card ID is provided, the endpoint returns activities related to the specified gift
    /// card. Otherwise, the endpoint returns all gift card activities for the seller.
    pub gift_card_id: Option<String>,
    /// If a [type](GiftCardActivityType) is provided, the endpoint returns gift card activities of
    /// the specified type. Otherwise, the endpoint returns all types of gift card activities.
    pub r#type: Option<GiftCardActivityType>,
    /// If a location ID is provided, the endpoint returns gift card activities for the specified
    /// location. Otherwise, the endpoint returns gift card activities for all locations.
    pub location_id: Option<String>,
    /// The timestamp for the beginning of the reporting period, in RFC 3339 format. This start time
    /// is inclusive. The default value is the current time minus one year.
    pub begin_time: Option<DateTime>,
    /// The timestamp for the end of the reporting period, in RFC 3339 format. This end time is
    /// inclusive. The default value is the current time.
    pub end_time: Option<DateTime>,
    /// If a limit is provided, the endpoint returns the specified number of results (or fewer) per
    /// page. The maximum value is 100. The default value is 50. For more information, see
    /// [Pagination](https://developer.squareup.com/docs/working-with-apis/pagination).
    pub limit: Option<i32>,
    /// A pagination cursor returned by a previous call to this endpoint. Provide this cursor to
    /// retrieve the next set of results for the original query. If a cursor is not provided, the
    /// endpoint returns the first page of the results. For more information, see
    /// [Pagination](https://developer.squareup.com/docs/basics/api101/pagination).
    pub cursor: Option<String>,
    /// The order in which the endpoint returns the activities, based on `created_at`.
    ///
    /// - ASC - Oldest to newest.
    /// - DESC - Newest to oldest (default).
    pub sort_order: Option<SortOrder>,
}

impl ListGiftCardActivitiesParameters {
    pub fn to_query_string(&self) -> String {
        self.to_string()
    }
}

impl From<ListGiftCardActivitiesParameters> for String {
    fn from(list_gift_card_activities_parameters: ListGiftCardActivitiesParameters) -> Self {
        list_gift_card_activities_parameters.to_string()
    }
}

impl ToString for ListGiftCardActivitiesParameters {
    fn to_string(&self) -> String {
        let mut params = Vec::new();

        if let Some(gift_card_id) = &self.gift_card_id {
            params.push(format!("gift_card_id={}", gift_card_id));
        }

        if let Some(gift_card_activity_type) = &self.r#type {
            params
                .push(format!("type={}", serde_json::to_string(gift_card_activity_type).unwrap()));
        }

        if let Some(location_id) = &self.location_id {
            params.push(format!("location_id={}", location_id));
        }

        if let Some(begin_time) = &self.begin_time {
            params.push(format!("begin_time={}", serde_json::to_string(begin_time).unwrap()));
        }

        if let Some(end_time) = &self.end_time {
            params.push(format!("end_time={}", serde_json::to_string(end_time).unwrap()));
        }

        if let Some(limit) = self.limit {
            params.push(format!("limit={}", limit));
        }

        if let Some(cursor) = &self.cursor {
            params.push(format!("cursor={}", cursor));
        }

        if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        }
    }
}
