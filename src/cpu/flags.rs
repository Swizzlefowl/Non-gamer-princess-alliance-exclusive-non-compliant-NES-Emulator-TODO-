//! https://www.nesdev.org/wiki/Status_flags

use tartan_bitfield::bitfield;

bitfield! {
    // 7  bit  0
    // ---- ----
    // NVss DIZC
    // |||| ||||
    // |||| |||+- Carry
    // |||| ||+-- Zero
    // |||| |+--- Interrupt Disable
    // |||| +---- Decimal
    // ||++------ No CPU effect, see: the B flag
    // |+-------- Overflow
    // +--------- Negative
    pub struct Status(u8) {
        [0] pub carry,
        [1] pub zero,
        [2] pub interrupt_disable,
        [3] pub decimal,
        [4] pub break_1,
        [5] pub break_2,
        [6] pub overflow,
        [7] pub negative
    }
}