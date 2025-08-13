use rmk::action::KeyAction;
use rmk::{k, layer, mo};
pub(crate) const COL: usize = 3;
pub(crate) const ROW: usize = 2;
pub(crate) const NUM_LAYER: usize = 2;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        layer!([
            [k!(Kp7), k!(Kp8), k!(Kp9)],
            [mo!(1), k!(Kp5), k!(Kp6)]
        ]),
        layer!([
            [k!(Kp7), k!(Kp8), k!(Kp9)],
            [k!(Kp4), k!(Kp5), k!(Kp6)]
        ]),
    ]
}
