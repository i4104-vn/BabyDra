//! Integration tests: volume subsystem helpers.
//!
//! Verifies WirePlumber profile name parsing (e.g. `output:analog-stereo`)
//! through the public helper API.

use babydra_common::services::system::volume::helper::parse_profile_parts;

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
