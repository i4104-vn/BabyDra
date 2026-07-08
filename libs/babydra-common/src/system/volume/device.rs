//! Audio device source and sink enumerations.

pub use crate::models::AudioDevice;

fn check_node_is_default(node_id: i64, is_source: bool) -> bool {
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
            } else if line_trimmed.contains("Devices:") || line_trimmed.contains("Filters:") || line_trimmed.contains("Streams:") || line_trimmed.contains("Settings:") {
                in_target_section = false;
                continue;
            }
            
            if in_target_section {
                let clean_line = line.replace('│', "").replace('├', "").replace('└', "").replace('─', "");
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
    let output = std::process::Command::new("wpctl")
        .arg("status")
        .output();

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
            } else if line_trimmed.contains("Devices:") || line_trimmed.contains("Filters:") || line_trimmed.contains("Streams:") || line_trimmed.contains("Settings:") {
                in_sinks_section = false;
                in_sources_section = false;
                continue;
            }

            let active_section = if is_source { in_sources_section } else { in_sinks_section };
            if !active_section {
                continue;
            }

            let clean_line = line.replace('│', "")
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

fn parse_profile_parts(name: &str) -> (Vec<String>, Vec<String>) {
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

pub fn get_audio_devices(is_source: bool) -> Vec<AudioDevice> {
    let mut devices = Vec::new();

    let output = std::process::Command::new("pw-dump").output();
    if let Ok(out) = output {
        if let Ok(val) = serde_json::from_slice::<serde_json::Value>(&out.stdout) {
            if let Some(arr) = val.as_array() {
                let mut card_devices = Vec::new();
                let mut independent_nodes = Vec::new();

                for item in arr {
                    let obj_type = item.get("type").and_then(|t| t.as_str()).unwrap_or("");
                    if obj_type == "PipeWire:Interface:Device" {
                        card_devices.push(item);
                    } else if obj_type == "PipeWire:Interface:Node" {
                        let props = item.get("info").and_then(|i| i.get("props"));
                        let media_class = props.and_then(|p| p.get("media.class")).and_then(|m| m.as_str()).unwrap_or("");
                        let target_class = if is_source { "Audio/Source" } else { "Audio/Sink" };
                        if media_class == target_class {
                            let has_device_id = props.and_then(|p| p.get("device.id")).is_some();
                            if !has_device_id {
                                independent_nodes.push(item);
                            }
                        }
                    }
                }

                for card in card_devices {
                    let card_id = card.get("id").and_then(|id| id.as_i64()).unwrap_or(0);
                    let props = card.get("info").and_then(|i| i.get("props"));
                    let card_desc = props.and_then(|p| p.get("device.description").or(p.get("device.name"))).and_then(|d| d.as_str()).unwrap_or("Audio Card");

                    let params = card.get("info").and_then(|i| i.get("params"));
                    let mut active_profile_index = -1;
                    let mut active_profile_name = String::new();
                    if let Some(profile_arr) = params.and_then(|p| p.get("Profile")).and_then(|p| p.as_array()) {
                        if !profile_arr.is_empty() {
                            active_profile_index = profile_arr[0].get("index").and_then(|idx| idx.as_i64()).unwrap_or(-1);
                            active_profile_name = profile_arr[0].get("name").and_then(|n| n.as_str()).unwrap_or("").to_string();
                        }
                    }

                    let mut valid_profiles = Vec::new();
                    if let Some(enum_profiles) = params.and_then(|p| p.get("EnumProfile")).and_then(|ep| ep.as_array()) {
                        for prof in enum_profiles {
                            let prof_name = prof.get("name").and_then(|n| n.as_str()).unwrap_or("");
                            let prof_available = prof.get("available").and_then(|a| a.as_str()).unwrap_or("");
                            let prof_index = prof.get("index").and_then(|idx| idx.as_i64()).unwrap_or(-1);

                            if prof_name != "off" && prof_available != "no" && prof_index != -1 {
                                if !prof_name.contains("surround") && !prof_name.contains("extra") {
                                    valid_profiles.push(prof);
                                }
                            }
                        }
                    }

                    let valid_profile_indices: std::collections::HashSet<i64> = valid_profiles
                        .iter()
                        .filter_map(|p| p.get("index").and_then(|idx| idx.as_i64()))
                        .collect();

                    let mut active_routes = std::collections::HashSet::new();
                    if let Some(route_arr) = params.and_then(|p| p.get("Route")).and_then(|p| p.as_array()) {
                        for r in route_arr {
                            if let Some(name) = r.get("name").and_then(|n| n.as_str()) {
                                active_routes.insert(name.to_string());
                            }
                        }
                    }

                    let mut card_has_routes = false;
                    if let Some(enum_routes) = params.and_then(|p| p.get("EnumRoute")).and_then(|er| er.as_array()) {
                        for route in enum_routes {
                            let direction = route.get("direction").and_then(|d| d.as_str()).unwrap_or("");
                            let target_direction = if is_source { "Input" } else { "Output" };
                            if direction != target_direction {
                                continue;
                            }

                            let r_name = route.get("name").and_then(|n| n.as_str()).unwrap_or("");
                            let r_desc = route.get("description").and_then(|d| d.as_str()).unwrap_or(r_name);
                            let r_index = route.get("index").and_then(|idx| idx.as_i64()).unwrap_or(-1);
                            let r_available = route.get("available").and_then(|a| a.as_str()).unwrap_or("");

                            if r_index == -1 || r_available == "no" {
                                continue;
                            }

                            if let Some(prof_ids) = route.get("profiles").and_then(|p| p.as_array()) {
                                let mut best_pid = None;
                                for pid in prof_ids {
                                    if let Some(pid_val) = pid.as_i64() {
                                        if valid_profile_indices.contains(&pid_val) {
                                            if pid_val == active_profile_index {
                                                best_pid = Some(pid_val);
                                                break;
                                            } else if best_pid.is_none() {
                                                best_pid = Some(pid_val);
                                            }
                                        }
                                    }
                                }
                                if let Some(pid_val) = best_pid {
                                    let is_default = pid_val == active_profile_index && active_routes.contains(r_name);
                                    devices.push(AudioDevice {
                                        name: format!("route:{}:{}:{}", card_id, r_index, pid_val),
                                        description: format!("{} - {}", card_desc, r_desc),
                                        is_default,
                                    });
                                    card_has_routes = true;
                                }
                            }
                        }
                    }

                    if card_has_routes {
                        continue;
                    }

                    if !is_source {
                        let mut outputs_grouped: std::collections::HashMap<String, Vec<&serde_json::Value>> = std::collections::HashMap::new();
                        for p in &valid_profiles {
                            if let Some(name) = p.get("name").and_then(|n| n.as_str()) {
                                let (outputs, _) = parse_profile_parts(name);
                                for out in outputs {
                                    outputs_grouped.entry(out).or_default().push(p);
                                }
                            }
                        }

                        for (out, profs) in outputs_grouped {
                            let mut best_p = None;
                            for p in &profs {
                                if let Some(name) = p.get("name").and_then(|n| n.as_str()) {
                                    let (_, inputs) = parse_profile_parts(name);
                                    if !inputs.is_empty() {
                                        best_p = Some(*p);
                                        break;
                                    }
                                }
                            }
                            let best_p = best_p.unwrap_or(profs[0]);
                            let prof_index = best_p.get("index").and_then(|idx| idx.as_i64()).unwrap_or(-1);
                            
                            let is_default = prof_index == active_profile_index;
                            let desc = if out.contains("analog-stereo") {
                                "Speakers / Headphones"
                            } else if out.contains("hdmi-stereo") {
                                "HDMI Output"
                            } else {
                                best_p.get("description").and_then(|d| d.as_str()).unwrap_or(&out)
                            };

                            devices.push(AudioDevice {
                                name: format!("profile:{}:{}", card_id, prof_index),
                                description: format!("{} - {}", card_desc, desc),
                                is_default,
                            });
                        }
                    } else {
                        let mut inputs_grouped: std::collections::HashMap<String, Vec<&serde_json::Value>> = std::collections::HashMap::new();
                        for p in &valid_profiles {
                            if let Some(name) = p.get("name").and_then(|n| n.as_str()) {
                                let (_, inputs) = parse_profile_parts(name);
                                for inp in inputs {
                                    inputs_grouped.entry(inp).or_default().push(p);
                                }
                            }
                        }

                        for (inp, profs) in inputs_grouped {
                            let mut best_p = None;
                            for p in &profs {
                                let prof_index = p.get("index").and_then(|idx| idx.as_i64()).unwrap_or(-1);
                                if prof_index == active_profile_index {
                                    best_p = Some(*p);
                                    break;
                                }
                            }
                            if best_p.is_none() {
                                for p in &profs {
                                    if let Some(name) = p.get("name").and_then(|n| n.as_str()) {
                                        let (outputs, _) = parse_profile_parts(name);
                                        if outputs.iter().any(|o| o.contains("analog-stereo")) {
                                            best_p = Some(*p);
                                            break;
                                        }
                                    }
                                }
                            }
                            let best_p = best_p.unwrap_or(profs[0]);
                            let prof_index = best_p.get("index").and_then(|idx| idx.as_i64()).unwrap_or(-1);

                            let (_, active_inputs) = parse_profile_parts(&active_profile_name);
                            let is_default = active_inputs.iter().any(|ai| ai == &inp);
                            let desc = if inp.contains("analog-stereo") || inp.contains("analog-mono") {
                                "Microphone"
                            } else {
                                best_p.get("description").and_then(|d| d.as_str()).unwrap_or(&inp)
                            };

                            devices.push(AudioDevice {
                                name: format!("profile:{}:{}", card_id, prof_index),
                                description: format!("{} - {}", card_desc, desc),
                                is_default,
                            });
                        }
                    }
                }

                for node in independent_nodes {
                    let node_id = node.get("id").and_then(|id| id.as_i64()).unwrap_or(0);
                    let props = node.get("info").and_then(|i| i.get("props"));
                    let node_desc = props.and_then(|p| p.get("node.description").or(p.get("node.name"))).and_then(|d| d.as_str()).unwrap_or("Virtual Device");
                    
                    let is_default = check_node_is_default(node_id, is_source);

                    devices.push(AudioDevice {
                        name: node_id.to_string(),
                        description: node_desc.to_string(),
                        is_default,
                    });
                }
            }
        }
    }

    if devices.is_empty() {
        return get_audio_devices_wpctl_fallback(is_source);
    }

    devices.sort_by(|a, b| a.description.cmp(&b.description));
    devices
}

