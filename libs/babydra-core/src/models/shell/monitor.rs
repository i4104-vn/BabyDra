//! System monitor metrics data models.

use serde::{Deserialize, Serialize};

/// Raw CPU time values used to calculate delta load values.
#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CpuTime {
    pub total: u64,
    pub idle: u64,
}
