//! Smoke tests for rusty_a11y.

use rusty_a11y::{label, live, status};

#[test]
fn label_img_and_escape() {
    let s = label::img("ok", "verified");
    assert!(s.contains("role=\"img\""));
    assert!(s.contains("aria-label=\"verified\""));
    let q = label::img("x", "say \"hi\"");
    assert!(q.contains("&quot;"));
}

#[test]
fn live_and_status() {
    assert!(live::polite("Syncing").contains("aria-live=\"polite\""));
    assert!(live::assertive("Err").contains("aria-live=\"assertive\""));
    assert!(status::announce(status::Kind::Saved).contains("Saved"));
    assert_eq!(status::Kind::Offline.as_str(), "Offline");
}
