//! Volume/Audio device configuration model.

#[derive(Clone, Debug)]
pub struct AudioDevice {
    pub name: String,
    pub description: String,
    pub is_default: bool,
}
