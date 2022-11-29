//! Pull employee data into accounting and payroll systems with the Team API.
//!
//! The Team API allows applications to retrieve a roster of team members registered in the Square
//! Point of Sale system, which can be useful in payroll and account contexts.
//!
//! The Team API is best used in conjunction with the Labor API, where you provide team member IDs
//! to manage shifts, breaks, and wages.

use crate::{
    config::Configuration,
    http::client::HttpClient,
    models::{
        errors::ApiError, BulkCreateTeamMembersRequest, BulkCreateTeamMembersResponse,
        BulkUpdateTeamMembersRequest, BulkUpdateTeamMembersResponse, CreateTeamMemberRequest,
        CreateTeamMemberResponse, RetrieveTeamMemberResponse, RetrieveWageSettingResponse,
        SearchTeamMembersRequest, SearchTeamMembersResponse, UpdateTeamMemberRequest,
        UpdateTeamMemberResponse, UpdateWageSettingRequest, UpdateWageSettingResponse,
    },
};

const DEFAULT_URI: &str = "/team-members";

pub struct TeamApi {
    /// App config information
    config: Configuration,
    /// HTTP Client for requests to the Team API endpoints
    client: HttpClient,
}

impl TeamApi {
    /// Instantiates a new `TeamApi`
    pub fn new(config: Configuration, client: HttpClient) -> Self {
        Self { config, client }
    }

    /// Creates a single `TeamMember` object.
    ///
    /// The `TeamMember` object is returned on successful creates. You must provide the following
    /// values in your request to this endpoint:
    /// * `given_name`
    /// * `family_name`
    ///
    /// Learn about [Troubleshooting the Team
    /// API](https://developer.squareup.com/docs/team/troubleshooting#createteammember).
    pub async fn create_team_member(
        &self,
        body: &CreateTeamMemberRequest,
    ) -> Result<CreateTeamMemberResponse, ApiError> {
        let response = self.client.post(&self.url(), body).await?;

        response.deserialize().await
    }

    /// Creates multiple `TeamMember` objects.
    ///
    /// The created `TeamMember` objects are returned on successful creates. This process is non-
    /// transactional and processes as much of the request as possible. If one of the creates in the
    /// request cannot be successfully processed, the request is not marked as failed, but the body
    /// of the response contains explicit error information for the failed create.
    ///
    /// Learn about [Troubleshooting the Team
    /// API](https://developer.squareup.com/docs/team/troubleshooting#createteammember).
    pub async fn bulk_create_team_members(
        &self,
        body: &BulkCreateTeamMembersRequest,
    ) -> Result<BulkCreateTeamMembersResponse, ApiError> {
        let url = format!("{}/bulk-create", self.url());
        let response = self.client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Updates multiple `TeamMember` objects.
    ///
    /// The updated `TeamMember` objects are returned on successful updates. This process is non-
    /// transactional and processes as much of the request as possible. If one of the updates in the
    /// request cannot be successfully processed, the request is not marked as failed, but the body
    /// of the response contains explicit error information for the failed update.
    ///
    /// Learn about [Troubleshooting the Team
    /// API](https://developer.squareup.com/docs/team/troubleshooting#createteammember).
    pub async fn bulk_update_team_members(
        &self,
        body: &BulkUpdateTeamMembersRequest,
    ) -> Result<BulkUpdateTeamMembersResponse, ApiError> {
        let url = format!("{}/bulk-update", self.url());
        let response = self.client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Returns a paginated list of `TeamMember` objects for a business.
    ///
    /// The list can be filtered by the following:
    /// * location IDs
    /// * `status`
    pub async fn search_team_members(
        &self,
        body: &SearchTeamMembersRequest,
    ) -> Result<SearchTeamMembersResponse, ApiError> {
        let url = format!("{}/search", self.url());
        let response = self.client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Retrieves a `TeamMember` object for the given `TeamMember.id`.
    ///
    /// Learn about [Troubleshooting the Team
    /// API](https://developer.squareup.com/docs/team/troubleshooting#createteammember).
    pub async fn retrieve_team_member(
        &self,
        team_member_id: &str,
    ) -> Result<RetrieveTeamMemberResponse, ApiError> {
        let url = format!("{}/{}", self.url(), team_member_id);
        let response = self.client.get(&url).await?;

        response.deserialize().await
    }

    /// Updates a single `TeamMember` object.
    ///
    /// The `TeamMember` object is returned on successful updates.
    ///
    /// Learn about [Troubleshooting the Team
    /// API](https://developer.squareup.com/docs/team/troubleshooting#createteammember).
    pub async fn update_team_member(
        &self,
        team_member_id: &str,
        body: &UpdateTeamMemberRequest,
    ) -> Result<UpdateTeamMemberResponse, ApiError> {
        let url = format!("{}/{}", self.url(), team_member_id);
        let response = self.client.put(&url, body).await?;

        response.deserialize().await
    }

    /// Retrieves a `WageSetting` object for a team member specified by `TeamMember.id`.
    ///
    /// Learn about [Troubleshooting the Team
    /// API](https://developer.squareup.com/docs/team/troubleshooting#createteammember).
    pub async fn retrieve_wage_setting(
        &self,
        team_member_id: &str,
    ) -> Result<RetrieveWageSettingResponse, ApiError> {
        let url = format!("{}/{}/wage-setting", self.url(), team_member_id);
        let response = self.client.get(&url).await?;

        response.deserialize().await
    }

    /// Creates or updates a `WageSetting` object.
    ///
    /// The object is created if a `WageSetting` with the specified `team_member_id` does not exist.
    /// Otherwise, it fully replaces the `WageSetting` object for the team member. The `WageSetting`
    /// is returned on a successful update.
    ///
    /// Learn about [Troubleshooting the Team
    /// API](https://developer.squareup.com/docs/team/troubleshooting#createteammember).
    pub async fn update_wage_setting(
        &self,
        team_member_id: &str,
        body: &UpdateWageSettingRequest,
    ) -> Result<UpdateWageSettingResponse, ApiError> {
        let url = format!("{}/{}/wage-setting", self.url(), team_member_id);
        let response = self.client.put(&url, body).await?;

        response.deserialize().await
    }

    /// Constructs the basic entity URL including domain and entity path. Any additional path
    /// elements (e.g. path parameters) will need to be appended to this URL.
    fn url(&self) -> String {
        format!("{}{}", &self.config.get_base_url(), DEFAULT_URI)
    }
}
