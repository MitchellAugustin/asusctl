use crate::keys::Key;

impl From<Key> for &str {
    fn from(k: Key) -> Self {
        (&k).into()
    }
}

impl From<&Key> for &str {
    fn from(k: &Key) -> Self {
        match k {
            Key::VolUp => "Volume Up",
            Key::VolDown => "Volume Down",
            Key::MicMute => "Mute Mic",
            Key::Rog => "ROG",
            Key::Fan => "Fan Control",
            Key::Esc => "Escape",
            Key::F1 => "F1",
            Key::F2 => "F2",
            Key::F3 => "F3",
            Key::F4 => "F4",
            Key::F5 => "F5",
            Key::F6 => "F6",
            Key::F7 => "F7",
            Key::F8 => "F8",
            Key::F9 => "F9",
            Key::F10 => "F10",
            Key::F11 => "F11",
            Key::F12 => "F12",
            Key::Del => "Delete",
            Key::Tilde => "Tilde",
            Key::N1 => "1",
            Key::N2 => "2",
            Key::N3 => "3",
            Key::N4 => "4",
            Key::N5 => "5",
            Key::N6 => "6",
            Key::N7 => "7",
            Key::N8 => "8",
            Key::N9 => "9",
            Key::N0 => "0",
            Key::Hyphen => "-",
            Key::Equals => "=",
            Key::BkSpc => "Backspace",
            Key::BkSpc3_1 => "Backspace LED 1",
            Key::BkSpc3_2 => "Backspace LED 2",
            Key::BkSpc3_3 => "Backspace LED 3",
            Key::Home => "Home",
            Key::Tab => "Tab",
            Key::Q => "Q",
            Key::W => "W",
            Key::E => "E",
            Key::R => "R",
            Key::T => "T",
            Key::Y => "Y",
            Key::U => "U",
            Key::I => "I",
            Key::O => "O",
            Key::P => "P",
            Key::LBracket => "[",
            Key::RBracket => "]",
            Key::BackSlash => "\\",
            Key::PgUp => "Page Up",
            Key::Caps => "Caps Lock",
            Key::A => "A",
            Key::S => "S",
            Key::D => "D",
            Key::F => "F",
            Key::G => "G",
            Key::H => "H",
            Key::J => "J",
            Key::K => "K",
            Key::L => "L",
            Key::SemiColon => ";",
            Key::Quote => "'",
            Key::Return => "Return",
            Key::Return3_1 => "Return LED 1",
            Key::Return3_2 => "Return LED 2",
            Key::Return3_3 => "Return LED 3",
            Key::PgDn => "Page Down",
            Key::LShift => "Left Shift",
            Key::LShift3_1 => "Left Shift LED 1",
            Key::LShift3_2 => "Left Shift LED 2",
            Key::LShift3_3 => "Left Shift LED 3",
            Key::Z => "Z",
            Key::X => "X",
            Key::C => "C",
            Key::V => "V",
            Key::B => "B",
            Key::N => "N",
            Key::M => "M",
            Key::Comma => ",",
            Key::Period => ".",
            Key::Star => "*",
            Key::NumPadDel => "Delete",
            Key::NumPadPlus => "+",
            Key::NumPadEnter => "Enter",
            Key::NumPadPause => "Pause",
            Key::NumPadPrtSc => "Print Screen",
            Key::NumPadHome => "Home",
            Key::NumLock => "Num-Lock",
            Key::FwdSlash => "/",
            Key::Rshift => "Right Shift",
            Key::RshiftSmall => "Right Shift",
            Key::Rshift3_1 => "Right Shift LED 1",
            Key::Rshift3_2 => "Right Shift LED 2",
            Key::Rshift3_3 => "Right Shift LED 3",
            Key::End => "End",
            Key::LCtrl => "Left Control",
            Key::LCtrlMed => "Left Control",
            Key::LFn => "Left Fn",
            Key::Meta => "Meta",
            Key::LAlt => "Left Alt",
            Key::Space => "Space",
            Key::Space5_1 => "Space LED 1",
            Key::Space5_2 => "Space LED 2",
            Key::Space5_3 => "Space LED 3",
            Key::Space5_4 => "Space LED 4",
            Key::Space5_5 => "Space LED 5",
            Key::RAlt => "Right Alt",
            Key::PrtSc => "Print Screen",
            Key::RCtrl => "Right Control",
            Key::RCtrlLarge => "Right Control",
            Key::Pause => "Pause",
            Key::Up => "Up",
            Key::Down => "Down",
            Key::Left => "Left",
            Key::Right => "Right",
            Key::UpRegular => "Up",
            Key::DownRegular => "Down",
            Key::LeftRegular => "Left",
            Key::RightRegular => "Right",
            Key::UpSplit => "Up",
            Key::DownSplit => "Down",
            Key::LeftSplit => "Left",
            Key::RightSplit => "Right",
            Key::RFn => "Right Fn",
            Key::MediaPlay => "Media Play",
            Key::MediaStop => "Media Stop",
            Key::MediaNext => "Media Next",
            Key::MediaPrev => "Media Previous",
            Key::NormalBlank => "",
            Key::NormalSpacer => "",
            Key::FuncBlank => "",
            Key::FuncSpacer => "",
            Key::ArrowBlank => "",
            Key::ArrowSpacer => "",
            Key::ArrowRegularBlank => "",
            Key::ArrowRegularSpacer => "",
            Key::ArrowSplitBlank => "",
            Key::ArrowSplitSpacer => "",
            Key::RowEndSpacer => "",
        }
    }
}