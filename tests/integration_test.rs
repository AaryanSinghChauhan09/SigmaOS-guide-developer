// Integration test for SigmaOS-guide-developer
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_guide_developer as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
