//! IBus Engine D-Bus interface implementation
//!
//! Implements the org.freedesktop.IBus.Engine interface via zbus

use crate::{ffi, keycode};
use std::sync::Mutex;
use zbus::{interface, Connection, object_server::SignalEmitter};

/// Gõ Nhanh IBus Engine
///
/// Implements the IBus Engine D-Bus interface
pub struct GoNhanhEngine {
    connection: Mutex<Option<Connection>>,
}

impl GoNhanhEngine {
    pub fn new() -> Self {
        log::info!("Creating new GoNhanhEngine instance");

        // Initialize with Telex method (default)
        ffi::set_method(0); // 0 = Telex
        ffi::set_enabled(true);

        Self {
            connection: Mutex::new(None),
        }
    }

    /// Set the D-Bus connection (called after interface is registered)
    pub fn set_connection(&self, conn: Connection) {
        *self.connection.lock().unwrap() = Some(conn);
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

#[interface(name = "org.freedesktop.IBus.Engine")]
impl GoNhanhEngine {
    /// Process key event (main method)
    ///
    /// Returns true if the key was handled, false to pass through
    async fn process_key_event(
        &self,
        #[zbus(signal_context)] ctxt: SignalEmitter<'_>,
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
                self.send_text(&ctxt, backspace, &text).await.ok();
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

    /// Commit text signal
    #[zbus(signal)]
    async fn commit_text(ctxt: &SignalEmitter<'_>, text: &str) -> zbus::Result<()>;

    /// Forward key event signal
    #[zbus(signal)]
    async fn forward_key_event(
        ctxt: &SignalEmitter<'_>,
        keyval: u32,
        keycode: u32,
        state: u32,
    ) -> zbus::Result<()>;

    /// Update preedit text signal (not used for now)
    #[zbus(signal)]
    async fn update_preedit_text(
        ctxt: &SignalEmitter<'_>,
        text: &str,
        cursor_pos: u32,
        visible: bool,
    ) -> zbus::Result<()>;

    /// Hide preedit text signal
    #[zbus(signal)]
    async fn hide_preedit_text(ctxt: &SignalEmitter<'_>) -> zbus::Result<()>;

    /// Show preedit text signal
    #[zbus(signal)]
    async fn show_preedit_text(ctxt: &SignalEmitter<'_>) -> zbus::Result<()>;
}
