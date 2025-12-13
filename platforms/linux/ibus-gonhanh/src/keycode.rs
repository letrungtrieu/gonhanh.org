//! X11 keysym to macOS virtual keycode translation
//!
//! IBus provides X11 keysyms, but gonhanh_core expects macOS virtual keycodes.
//! This module translates between the two formats.
//!
//! References:
//! - X11 keysyms: /usr/include/X11/keysymdef.h
//! - macOS keycodes: core/src/data/keys.rs

/// Translate X11 keysym to macOS virtual keycode
///
/// # Arguments
/// * `keysym` - X11 keysym value (e.g., 0x061 for 'a')
///
/// # Returns
/// * `Some(keycode)` - macOS virtual keycode if recognized
/// * `None` - if keysym is not relevant for Vietnamese input
pub fn keysym_to_macos_keycode(keysym: u32) -> Option<u16> {
    match keysym {
        // Letters (case-insensitive, both lowercase and uppercase map to same keycode)
        // macOS keycode constants from core/src/data/keys.rs
        0x061 | 0x041 => Some(0),   // a/A → macOS::A
        0x073 | 0x053 => Some(1),   // s/S → macOS::S
        0x064 | 0x044 => Some(2),   // d/D → macOS::D
        0x066 | 0x046 => Some(3),   // f/F → macOS::F
        0x068 | 0x048 => Some(4),   // h/H → macOS::H
        0x067 | 0x047 => Some(5),   // g/G → macOS::G
        0x07a | 0x05a => Some(6),   // z/Z → macOS::Z
        0x078 | 0x058 => Some(7),   // x/X → macOS::X
        0x063 | 0x043 => Some(8),   // c/C → macOS::C
        0x076 | 0x056 => Some(9),   // v/V → macOS::V
        0x062 | 0x042 => Some(11),  // b/B → macOS::B
        0x071 | 0x051 => Some(12),  // q/Q → macOS::Q
        0x077 | 0x057 => Some(13),  // w/W → macOS::W
        0x065 | 0x045 => Some(14),  // e/E → macOS::E
        0x072 | 0x052 => Some(15),  // r/R → macOS::R
        0x079 | 0x059 => Some(16),  // y/Y → macOS::Y
        0x074 | 0x054 => Some(17),  // t/T → macOS::T
        0x06f | 0x04f => Some(31),  // o/O → macOS::O
        0x075 | 0x055 => Some(32),  // u/U → macOS::U
        0x069 | 0x049 => Some(34),  // i/I → macOS::I
        0x070 | 0x050 => Some(35),  // p/P → macOS::P
        0x06c | 0x04c => Some(37),  // l/L → macOS::L
        0x06a | 0x04a => Some(38),  // j/J → macOS::J
        0x06b | 0x04b => Some(40),  // k/K → macOS::K
        0x06e | 0x04e => Some(45),  // n/N → macOS::N
        0x06d | 0x04d => Some(46),  // m/M → macOS::M

        // Numbers (for VNI mode)
        0x031 => Some(18),  // 1 → macOS::N1
        0x032 => Some(19),  // 2 → macOS::N2
        0x033 => Some(20),  // 3 → macOS::N3
        0x034 => Some(21),  // 4 → macOS::N4
        0x035 => Some(23),  // 5 → macOS::N5
        0x036 => Some(22),  // 6 → macOS::N6
        0x037 => Some(26),  // 7 → macOS::N7
        0x038 => Some(28),  // 8 → macOS::N8
        0x039 => Some(25),  // 9 → macOS::N9
        0x030 => Some(29),  // 0 → macOS::N0

        // Special keys (X11 keysyms with 0xFF prefix are in /usr/include/X11/keysymdef.h)
        0x0020 => Some(49),     // Space
        0xFF08 => Some(51),     // BackSpace
        0xFF09 => Some(48),     // Tab
        0xFF0D => Some(36),     // Return
        0xFF8D => Some(76),     // KP_Enter (numpad enter)
        0xFF1B => Some(53),     // Escape
        0xFF51 => Some(123),    // Left arrow
        0xFF53 => Some(124),    // Right arrow
        0xFF54 => Some(125),    // Down arrow
        0xFF52 => Some(126),    // Up arrow

        // Punctuation (US keyboard layout)
        0x02e => Some(47),      // . (period)
        0x02c => Some(43),      // , (comma)
        0x02f => Some(44),      // / (slash)
        0x03b => Some(41),      // ; (semicolon)
        0x027 => Some(39),      // ' (quote)
        0x05b => Some(33),      // [ (left bracket)
        0x05d => Some(30),      // ] (right bracket)
        0x05c => Some(42),      // \ (backslash)
        0x02d => Some(27),      // - (minus)
        0x03d => Some(24),      // = (equal)
        0x060 => Some(50),      // ` (backquote/grave)

        // Unknown or irrelevant keysym
        _ => None,
    }
}

/// Check if a keysym is a modifier key (Shift, Control, Alt, etc.)
///
/// We want to skip these keys as they don't produce characters.
pub fn is_modifier_key(keysym: u32) -> bool {
    matches!(
        keysym,
        0xFFE1 | 0xFFE2 |  // Shift_L, Shift_R
        0xFFE3 | 0xFFE4 |  // Control_L, Control_R
        0xFFE9 | 0xFFEA |  // Alt_L, Alt_R
        0xFFEB | 0xFFEC |  // Super_L, Super_R (Windows/Command key)
        0xFFE5             // Caps_Lock
    )
}

/// Check if a keysym should trigger buffer clear (word boundaries)
pub fn is_buffer_clear_key(keysym: u32) -> bool {
    matches!(
        keysym,
        0x0020 |       // Space
        0xFF0D |       // Return
        0xFF8D |       // KP_Enter
        0xFF09 |       // Tab
        0xFF1B |       // Escape
        0xFF51..=0xFF54 // Arrow keys
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_mapping() {
        assert_eq!(keysym_to_macos_keycode(0x061), Some(0)); // a → 0
        assert_eq!(keysym_to_macos_keycode(0x041), Some(0)); // A → 0 (same)
        assert_eq!(keysym_to_macos_keycode(0x073), Some(1)); // s → 1
        assert_eq!(keysym_to_macos_keycode(0x064), Some(2)); // d → 2
    }

    #[test]
    fn test_number_mapping() {
        assert_eq!(keysym_to_macos_keycode(0x031), Some(18)); // 1 → 18
        assert_eq!(keysym_to_macos_keycode(0x039), Some(25)); // 9 → 25
    }

    #[test]
    fn test_special_keys() {
        assert_eq!(keysym_to_macos_keycode(0x0020), Some(49)); // Space
        assert_eq!(keysym_to_macos_keycode(0xFF08), Some(51)); // BackSpace
    }

    #[test]
    fn test_modifier_detection() {
        assert!(is_modifier_key(0xFFE1)); // Shift_L
        assert!(is_modifier_key(0xFFE3)); // Control_L
        assert!(!is_modifier_key(0x061)); // 'a' is not a modifier
    }

    #[test]
    fn test_buffer_clear_detection() {
        assert!(is_buffer_clear_key(0x0020)); // Space
        assert!(is_buffer_clear_key(0xFF0D)); // Return
        assert!(!is_buffer_clear_key(0x061)); // 'a' doesn't clear buffer
    }
}
