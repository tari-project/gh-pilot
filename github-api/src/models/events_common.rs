use serde::{Deserialize, Serialize};

use crate::models::{InstallationLite, Organization, Repository2, SimpleUser};

//----------------------------------  Fields common to all events ------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommonEventFields {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository2,
    pub sender: SimpleUser,
}
