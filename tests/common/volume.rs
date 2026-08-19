//! Integration tests: volume subsystem helpers and backend detection.
//!
//! Verifies WirePlumber profile name parsing (e.g. `output:analog-stereo`)
//! and audio backend detection model.

use babydra_core::models::shell::volume::AudioBackendType;
use babydra_core::services::system::volume::control::get_audio_backend;
use babydra_core::services::system::volume::helper::parse_profile_parts;

#[test]
fn parses_output_and_input_parts() {
    let (outputs, inputs) = parse_profile_parts("output:analog-stereo+input:analog-stereo");
    assert_eq!(outputs, vec!["analog-stereo"]);
    assert_eq!(inputs, vec!["analog-stereo"]);
}

#[test]
fn parses_only_output() {
    let (outputs, inputs) = parse_profile_parts("output:hdmi-stereo");
    assert_eq!(outputs, vec!["hdmi-stereo"]);
    assert!(inputs.is_empty());
}

#[test]
fn parses_multiple_outputs() {
    let (outputs, _) = parse_profile_parts("output:analog-stereo+output:hdmi-stereo");
    assert_eq!(outputs, vec!["analog-stereo", "hdmi-stereo"]);
}

#[test]
fn ignores_unprefixed_segments() {
    let (outputs, inputs) = parse_profile_parts("off+input:mic");
    assert!(outputs.is_empty());
    assert_eq!(inputs, vec!["mic"]);
}

#[test]
fn empty_input_yields_empty_vectors() {
    let (outputs, inputs) = parse_profile_parts("");
    assert!(outputs.is_empty());
    assert!(inputs.is_empty());
}

#[test]
fn backend_detection_resolves_and_caches_consistently() {
    let backend1 = get_audio_backend();
    let backend2 = get_audio_backend();
    assert_eq!(
        backend1, backend2,
        "Cached audio backend must be stable across repeated queries"
    );
}

#[test]
fn backend_type_properties() {
    let wpctl = AudioBackendType::Wpctl;
    assert_eq!(wpctl.command_name(), "wpctl");
    assert!(wpctl.name().contains("PipeWire"));

    let pactl = AudioBackendType::Pactl;
    assert_eq!(pactl.command_name(), "pactl");
    assert!(pactl.name().contains("PulseAudio"));

    let amixer = AudioBackendType::Amixer;
    assert_eq!(amixer.command_name(), "amixer");
    assert!(amixer.name().contains("ALSA"));
}
