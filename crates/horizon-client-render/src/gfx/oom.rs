//! Tests for OOM-driven render-distance reduction and hysteresis.

use super::Renderer;

/// Each OOM reduction level halves the detail distance, floored at 4 chunks.
#[test]
fn effective_detail_distance_oom_halves_each_level() {
    let full = 128.0 * 16.0; // 128 chunks in blocks
    assert_eq!(Renderer::effective_detail_distance_oom(0, full), full);
    assert_eq!(Renderer::effective_detail_distance_oom(1, full), full / 2.0);
    assert_eq!(Renderer::effective_detail_distance_oom(2, full), full / 4.0);
    assert_eq!(Renderer::effective_detail_distance_oom(3, full), full / 8.0);
}

/// The floor is 4 chunks (64 blocks), never below.
#[test]
fn effective_detail_distance_oom_floors_at_four_chunks() {
    let tiny = 2.0 * 16.0; // 2 chunks in blocks
    assert_eq!(Renderer::effective_detail_distance_oom(1, tiny), 4.0 * 16.0);
    assert_eq!(Renderer::effective_detail_distance_oom(3, tiny), 4.0 * 16.0);
}

/// Hysteresis: cooldown ticks down, reduction only drops when cooldown reaches zero.
#[test]
fn oom_hysteresis_step_cooldown_ticks_down() {
    assert_eq!(Renderer::oom_hysteresis_step(2, 100), (2, 99));
    assert_eq!(Renderer::oom_hysteresis_step(2, 1), (2, 0));
}

/// When cooldown reaches zero and reduction > 0, one level is removed and cooldown restarts.
#[test]
fn oom_hysteresis_step_reduces_at_zero_cooldown() {
    assert_eq!(Renderer::oom_hysteresis_step(2, 0), (1, 600));
    assert_eq!(Renderer::oom_hysteresis_step(1, 0), (0, 600));
}

/// At reduction 0, cooldown stays 0 (no further reduction possible).
#[test]
fn oom_hysteresis_step_no_reduction_left() {
    assert_eq!(Renderer::oom_hysteresis_step(0, 0), (0, 0));
    assert_eq!(Renderer::oom_hysteresis_step(0, 500), (0, 499));
}
