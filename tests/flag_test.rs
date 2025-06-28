use ecp::builder::*;

#[test]
fn full_flag() {
    let release = Flag::new("release")
        .description("Build artifacts in release mode, with optimizations")
        .short('r');

    assert_eq!(release.get_long(), "release".to_string());
    assert_eq!(release.get_short(), Some('r'))
}
