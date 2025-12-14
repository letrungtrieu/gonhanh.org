//! IBus Engine D-Bus interface implementation
//!
//! Implements the org.freedesktop.IBus.Engine interface via zbus

use crate::{ffi, keycode};
use zbus::{interface, object_server::SignalEmitter};

/// Gõ Nhanh IBus Engine
///
/// Implements the IBus Engine D-Bus interface
pub struct GoNhanhEngine;

impl GoNhanhEngine {
    pub fn new() -> Self {
        log::info!("Creating new GoNhanhEngine instance");

        // Initialize with Telex method (default)
        ffi::set_method(0); // 0 = Telex
        ffi::set_enabled(true);

        Self
    }

    /// Send text to the application via D-Bus signals
    async fn send_text(
        &self,
        ctxt: &SignalEmitter<'_>,
        backspace: u8,
        text: &str,
    ) -> zbus::Result<()> {
        log::info!("Sending: backspace={}, text=\"{}\"", backspace, text);

        // Send forward key events for backspace
        for _ in 0..backspace {
            GoNhanhEngine::forward_key_event(ctxt, 0xFF08, 0, 0).await?;
        }

        // Commit the new text
        GoNhanhEngine::commit_text(ctxt, text).await?;

        Ok(())
    }
}

/// Gõ Nhanh IBus Engine
///
/// Implements the org.freedesktop.IBus.Engine D-Bus interface
#[interface(name = "org.freedesktop.IBus.GoNhanh", spawn = false)]
impl GoNhanhEngine {
    /// Process key event (main method)
    ///
    /// Returns true if the key was handled, false to pass through
    async fn process_key_event(
        &self,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
        keyval: u32,
        keycode: u32,
        state: u32,
    ) -> zbus::fdo::Result<bool> {
        log::debug!(
            "ProcessKeyEvent: keyval={:#x}, keycode={}, state={:#x}",
            keyval,
            keycode,
            state
        );

        // Skip modifier-only keys
        if keycode::is_modifier_key(keyval) {
            return Ok(false);
        }

        // Extract modifier states (IBus uses X11 modifier masks)
        const SHIFT_MASK: u32 = 1 << 0;
        const LOCK_MASK: u32 = 1 << 1;
        const CONTROL_MASK: u32 = 1 << 2;
        const MOD1_MASK: u32 = 1 << 3; // Alt

        let shift = (state & SHIFT_MASK) != 0;
        let ctrl = (state & CONTROL_MASK) != 0;
        let alt = (state & MOD1_MASK) != 0;
        let caps_lock = (state & LOCK_MASK) != 0;

        // Uppercase if shift XOR caps_lock
        let caps = shift ^ caps_lock;

        // Bypass IME if Ctrl or Alt is pressed
        let bypass_ime = ctrl || alt;

        // Clear buffer on word boundary keys
        if keycode::is_buffer_clear_key(keyval) {
            ffi::clear_buffer();
            return Ok(false);
        }

        // Translate X11 keysym to macOS keycode
        let Some(macos_keycode) = keycode::keysym_to_macos_keycode(keyval) else {
            return Ok(false) // Unknown key, pass through
        };

        log::debug!(
            "Translated keysym {:#x} to macOS keycode {} (caps={}, bypass={})",
            keyval,
            macos_keycode,
            caps,
            bypass_ime
        );

        // Process through gonhanh_core
        match ffi::process_key(macos_keycode, caps, bypass_ime, shift) {
            ffi::ImeAction::Send { backspace, text } => {
                // Log errors but continue - don't break input flow on D-Bus errors
                if let Err(e) = self.send_text(&emitter, backspace, &text).await {
                    log::error!("Failed to send text via D-Bus: {}", e);
                }
                Ok(true) // Consume the event
            }
            ffi::ImeAction::None => Ok(false), // Pass through
        }
    }

    /// Engine is enabled
    async fn enable(&self) {
        log::info!("Engine enabled");
        ffi::set_enabled(true);
    }

    /// Engine is disabled
    async fn disable(&self) {
        log::info!("Engine disabled");
        ffi::set_enabled(false);
    }

    /// Reset engine state
    async fn reset(&self) {
        log::debug!("Engine reset");
        ffi::clear_buffer();
    }

    /// Focus in
    async fn focus_in(&self) {
        log::debug!("Focus in");
        ffi::clear_buffer();
    }

    /// Focus out
    async fn focus_out(&self) {
        log::debug!("Focus out");
        ffi::clear_buffer();
    }

    /// Set cursor location (not used)
    async fn set_cursor_location(&self, x: i32, y: i32, w: i32, h: i32) {
        log::debug!("Set cursor location: x={}, y={}, w={}, h={}", x, y, w, h);
    }

    /// Set capabilities (not used)
    async fn set_capabilities(&self, caps: u32) {
        log::debug!("Set capabilities: {:#x}", caps);
    }

    /// Page up in candidate list (not used)
    async fn page_up(&self) {
        log::debug!("Page up");
    }

    /// Page down in candidate list (not used)
    async fn page_down(&self) {
        log::debug!("Page down");
    }

    /// Cursor up in candidate list (not used)
    async fn cursor_up(&self) {
        log::debug!("Cursor up");
    }

    /// Cursor down in candidate list (not used)
    async fn cursor_down(&self) {
        log::debug!("Cursor down");
    }

    /// Property activate (for toolbar buttons, not used)
    async fn property_activate(&self, prop_name: &str, state: u32) {
        log::debug!("Property activate: {} = {}", prop_name, state);
    }

    /// Property show (not used)
    async fn property_show(&self, prop_name: &str) {
        log::debug!("Property show: {}", prop_name);
    }

    /// Property hide (not used)
    async fn property_hide(&self, prop_name: &str) {
        log::debug!("Property hide: {}", prop_name);
    }

    // ========== Signals (methods to emit signals to IBus daemon) ==========

    /// CommitText signal - sends committed text to the application
    ///
    /// This signal is emitted when the engine has finalized text that should be
    /// inserted into the application (e.g., after completing Vietnamese composition).
    #[zbus(signal)]
    async fn commit_text(emitter: &SignalEmitter<'_>, text: &str) -> zbus::Result<()>;

    /// ForwardKeyEvent signal - forwards a key event to the application
    ///
    /// This signal is emitted when the engine wants to forward a key event
    /// (typically for backspace) to the application as if the user typed it.
    ///
    /// # Arguments
    /// * `keyval` - X11 keysym value
    /// * `keycode` - Hardware keycode
    /// * `state` - Modifier state mask
    #[zbus(signal)]
    async fn forward_key_event(
        emitter: &SignalEmitter<'_>,
        keyval: u32,
        keycode: u32,
        state: u32,
    ) -> zbus::Result<()>;

    /// UpdatePreeditText signal - updates pre-edit buffer display
    ///
    /// This signal is emitted to update the composition text being displayed
    /// before it is committed. Not currently used in this implementation.
    ///
    /// # Arguments
    /// * `text` - The preedit text to display
    /// * `cursor_pos` - Cursor position within the preedit text
    /// * `visible` - Whether the preedit text should be visible
    #[zbus(signal)]
    async fn update_preedit_text(
        emmiter: &SignalEmitter<'_>,
        text: &str,
        cursor_pos: u32,
        visible: bool,
    ) -> zbus::Result<()>;

    /// HidePreeditText signal - hides the pre-edit buffer display
    ///
    /// This signal is emitted to hide the composition text display.
    #[zbus(signal)]
    async fn hide_preedit_text(emitter: &SignalEmitter<'_>) -> zbus::Result<()>;

    /// ShowPreeditText signal - shows the pre-edit buffer display
    ///
    /// This signal is emitted to show the composition text display.
    #[zbus(signal)]
    async fn show_preedit_text(emitter: &SignalEmitter<'_>) -> zbus::Result<()>;
}
