//! Safe FFI wrapper around gonhanh_core
//!
//! Provides safe Rust interface to the gonhanh_core engine.

use std::sync::Once;

/// IME action result
#[derive(Debug)]
pub enum ImeAction {
    None,
    Send { backspace: u8, text: String },
}

/// Initialize gonhanh_core engine (call once at startup)
static INIT: Once = Once::new();

pub fn initialize() {
    INIT.call_once(|| {
        gonhanh_core::ime_init();
        log::debug!("gonhanh_core::ime_init() called");
    });
}

/// Process a key event through gonhanh_core
///
/// # Arguments
/// * `key` - macOS virtual keycode (from keycode.rs translation)
/// * `caps` - true if character should be uppercase
/// * `ctrl` - true if Ctrl/Alt/Cmd is pressed (bypasses IME)
/// * `shift` - true if Shift is pressed (for VNI symbols)
pub fn process_key(key: u16, caps: bool, ctrl: bool, shift: bool) -> ImeAction {
    let ptr = gonhanh_core::ime_key_ext(key, caps, ctrl, shift);
    if ptr.is_null() {
        log::warn!("ime_key_ext returned null pointer");
        return ImeAction::None;
    }

    unsafe {
        let result = &*ptr;
        let action = match result.action {
            1 => {
                // Action::Send - convert UTF-32 chars to String
                let chars: Vec<char> = (0..result.count as usize)
                    .filter_map(|i| char::from_u32(result.chars[i]))
                    .collect();

                let text: String = chars.into_iter().collect();
                log::debug!(
                    "IME action: Send (backspace={}, text=\"{}\")",
                    result.backspace,
                    text
                );

                ImeAction::Send {
                    backspace: result.backspace,
                    text,
                }
            }
            _ => {
                log::debug!("IME action: None (pass through)");
                ImeAction::None
            }
        };

        gonhanh_core::ime_free(ptr);
        action
    }
}

/// Set input method (0=Telex, 1=VNI)
pub fn set_method(method: u8) {
    gonhanh_core::ime_method(method);
    log::info!(
        "Input method set to: {}",
        if method == 0 { "Telex" } else { "VNI" }
    );
}

/// Enable or disable IME processing
pub fn set_enabled(enabled: bool) {
    gonhanh_core::ime_enabled(enabled);
    log::info!("IME enabled: {}", enabled);
}

/// Clear the typing buffer (call on word boundaries)
pub fn clear_buffer() {
    gonhanh_core::ime_clear();
    log::debug!("IME buffer cleared");
}
