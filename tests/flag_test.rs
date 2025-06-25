use ecp::builder::Flag;

#[test]
fn full_flag() {
    let release = Flag::new("release")
        .description("Build artifacts in release mode, with optimizations")
        .short('r');

    assert_matches!(release.name.as_str(), "release")
}
