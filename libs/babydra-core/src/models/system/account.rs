//! User account, identity, and system user data models.

use serde::{Deserialize, Serialize};

/// Information about a user account.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserAccountInfo {
    pub username: String,
    pub display_name: String,
    pub uid: u32,
    pub gid: u32,
    pub home_dir: String,
    pub shell: String,
}
