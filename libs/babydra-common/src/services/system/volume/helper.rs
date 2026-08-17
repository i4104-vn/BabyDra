//! Internal sound routing and profile lookup helpers.

use crate::models::AudioDevice;

pub fn check_node_is_default(node_id: i64, is_source: bool) -> bool {
    let output = std::process::Command::new("wpctl").arg("status").output();
    if let Ok(out) = output {
        let stdout = String::from_utf8_lossy(&out.stdout);
        let target_section = if is_source { "Sources:" } else { "Sinks:" };
        let mut in_target_section = false;

        for line in stdout.lines() {
            let line_trimmed = line.trim();
            if line_trimmed.contains(target_section) {
                in_target_section = true;
                continue;
            } else if line_trimmed.contains("Devices:")
                || line_trimmed.contains("Filters:")
                || line_trimmed.contains("Streams:")
                || line_trimmed.contains("Settings:")
            {
                in_target_section = false;
                continue;
            }

            if in_target_section {
                let clean_line = line
                    .replace('│', "")
                    .replace('├', "")
                    .replace('└', "")
                    .replace('─', "");
                let clean_trimmed = clean_line.trim();
                if clean_trimmed.starts_with('*') {
                    let parts: Vec<&str> = clean_trimmed[1..].trim().split('.').collect();
                    if !parts.is_empty() {
                        if let Ok(id) = parts[0].trim().parse::<i64>() {
                            if id == node_id {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

pub fn get_audio_devices_wpctl_fallback(is_source: bool) -> Vec<AudioDevice> {
    let mut devices = Vec::new();
    let output = std::process::Command::new("wpctl").arg("status").output();

    if let Ok(out) = output {
        let stdout = String::from_utf8_lossy(&out.stdout);
        let mut in_sinks_section = false;
        let mut in_sources_section = false;

        for line in stdout.lines() {
            let line_trimmed = line.trim();

            if line_trimmed.contains("Sinks:") {
                in_sinks_section = true;
                in_sources_section = false;
                continue;
            } else if line_trimmed.contains("Sources:") {
                in_sinks_section = false;
                in_sources_section = true;
                continue;
            } else if line_trimmed.contains("Devices:")
                || line_trimmed.contains("Filters:")
                || line_trimmed.contains("Streams:")
                || line_trimmed.contains("Settings:")
            {
                in_sinks_section = false;
                in_sources_section = false;
                continue;
            }

            let active_section = if is_source {
                in_sources_section
            } else {
                in_sinks_section
            };
            if !active_section {
                continue;
            }

            let clean_line = line
                .replace('│', "")
                .replace('├', "")
                .replace('└', "")
                .replace('─', "");
            let mut clean_trimmed = clean_line.trim().to_string();
            if clean_trimmed.is_empty() {
                continue;
            }

            let mut is_default = false;
            if clean_trimmed.starts_with('*') {
                is_default = true;
                clean_trimmed = clean_trimmed[1..].trim().to_string();
            }

            if let Some(dot_pos) = clean_trimmed.find('.') {
                let id_str = &clean_trimmed[..dot_pos];
                if id_str.chars().all(|c| c.is_ascii_digit()) {
                    let id = id_str.to_string();
                    let mut desc = clean_trimmed[dot_pos + 1..].trim().to_string();

                    if let Some(bracket_pos) = desc.rfind('[') {
                        desc = desc[..bracket_pos].trim().to_string();
                    }

                    if !id.is_empty() && !desc.is_empty() {
                        devices.push(AudioDevice {
                            name: id,
                            description: desc,
                            is_default,
                        });
                    }
                }
            }
        }
    }
    devices
}

pub fn parse_profile_parts(name: &str) -> (Vec<String>, Vec<String>) {
    let mut outputs = Vec::new();
    let mut inputs = Vec::new();
    for part in name.split('+') {
        if let Some(stripped) = part.strip_prefix("output:") {
            outputs.push(stripped.to_string());
        } else if let Some(stripped) = part.strip_prefix("input:") {
            inputs.push(stripped.to_string());
        }
    }
    (outputs, inputs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_profile_parts_splits_outputs_and_inputs() {
        let (outputs, inputs) = parse_profile_parts("output:analog-stereo+input:analog-stereo");
        assert_eq!(outputs, vec!["analog-stereo"]);
        assert_eq!(inputs, vec!["analog-stereo"]);
    }

    #[test]
    fn parse_profile_parts_handles_output_only() {
        let (outputs, inputs) = parse_profile_parts("output:hdmi-stereo");
        assert_eq!(outputs, vec!["hdmi-stereo"]);
        assert!(inputs.is_empty());
    }

    #[test]
    fn parse_profile_parts_handles_multiple_outputs() {
        let (outputs, _) = parse_profile_parts("output:analog-stereo+output:hdmi-stereo");
        assert_eq!(outputs, vec!["analog-stereo", "hdmi-stereo"]);
    }

    #[test]
    fn parse_profile_parts_ignores_unprefixed_parts() {
        let (outputs, inputs) = parse_profile_parts("off+input:mic");
        assert!(outputs.is_empty());
        assert_eq!(inputs, vec!["mic"]);
    }

    #[test]
    fn parse_profile_parts_empty_input_yields_empty_vectors() {
        let (outputs, inputs) = parse_profile_parts("");
        assert!(outputs.is_empty());
        assert!(inputs.is_empty());
    }
}
