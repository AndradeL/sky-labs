// Copyright (c) 2025 Lucas B. Andrade
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use windows::Win32::UI::Input::KeyboardAndMouse::*;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyState {
    /// The key is currently pressed
    Pressed,
    /// The key is not currently pressed
    Released,
    /// The key was pressed since the last call to get_key_state
    WasPressed,
}

pub fn get_key_state(key: VIRTUAL_KEY) -> KeyState {
    unsafe {
        match GetAsyncKeyState(key.0 as i32) {
            i16::MIN..=-1 => KeyState::Pressed,
            0 => KeyState::Released,
            1.. => KeyState::WasPressed,
        }
    }
}
