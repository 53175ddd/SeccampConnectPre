use rmk::action::KeyAction;
use rmk::{k, layer, mo};
pub(crate) const COL: usize = 3;
pub(crate) const ROW: usize = 2;
pub(crate) const NUM_LAYER: usize = 2;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        layer!([
            [k!(A), k!(B), k!(Backspace)],
            [mo!(1), k!(D), k!(E)]
        ]),
        layer!([
            [k!(F), k!(G), k!(H)],
            [k!(I), k!(Kc5), k!(Kc6)]
        ]),
    ]
}
