//! Model struct for TeamMember type

use serde::{Deserialize, Serialize};

use super::{enums::TeamMemberStatus, DateTime, TeamMemberAssignedLocations};

/// A record representing an individual team member for a business.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct TeamMember {
    /// **Read only** The unique ID for the team member.
    pub id: Option<String>,
    /// A second ID used to associate the team member with an entity in another system.
    pub reference_id: Option<String>,
    /// **Read only** Whether the team member is the owner of the Square account.
    pub is_owner: Option<bool>,
    /// Describes the status of the team member.
    pub status: Option<TeamMemberStatus>,
    /// The given name (that is, the first name) associated with the team member.
    pub given_name: Option<String>,
    /// The family name (that is, the last name) associated with the team member.
    pub family_name: Option<String>,
    /// The email address associated with the team member.
    pub email_address: Option<String>,
    /// The team member's phone number, in E.164 format. For example: +14155552671 - the country
    /// code is 1 for US +551155256325 - the country code is 55 for BR
    pub phone_number: Option<String>,
    /// **Read only** The timestamp, in RFC 3339 format, describing when the team member was
    /// created.
    pub created_at: Option<DateTime>,
    /// **Read only** The timestamp, in RFC 3339 format, describing when the team member was last
    /// updated.
    pub updated_at: Option<DateTime>,
    /// Describes the team member's assigned locations.
    pub assigned_locations: Option<TeamMemberAssignedLocations>,
}
