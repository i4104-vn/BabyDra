//! Power action model for the power island feature.

/// Supported power management actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerAction {
    Shutdown = 0,
    Reboot = 1,
    Sleep = 2,
    Logout = 3,
}

impl PowerAction {
    /// Resolves a power action from a zero-based menu index (0..4).
    pub fn from_index(idx: usize) -> Option<Self> {
        match idx {
            0 => Some(Self::Shutdown),
            1 => Some(Self::Reboot),
            2 => Some(Self::Sleep),
            3 => Some(Self::Logout),
            _ => None,
        }
    }

    /// Returns the zero-based index of this action.
    pub fn to_index(self) -> usize {
        self as usize
    }

    /// Executes the corresponding system power command.
    pub fn execute(self) {
        match self {
            Self::Shutdown => {
                babydra_core::poweroff();
            }
            Self::Reboot => {
                babydra_core::reboot();
            }
            Self::Sleep => {
                babydra_core::suspend();
            }
            Self::Logout => {
                babydra_core::services::actions::execute_exit_shell();
            }
        }
    }
}
