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

use super::math::size::Size;

#[cfg(target_os = "windows")]
use super::win::window::{NativeWindowHandle, Win32Window};

pub trait NativeWindow: Sized {
    fn create() -> Self;
    fn size(&self) -> Size<u32>;
    fn handle(&self) -> NativeWindowHandle;
    fn process_until_end(&mut self);
    fn process_message_if_available(&mut self) -> WindowProcessResult;
}

#[derive(PartialEq, Eq)]
pub enum WindowProcessResult {
    Ok,
    Skip,
    Exit,
    Error(String), // TODO Add error info
}

struct WindowGeneric<TNativeWindow: NativeWindow> {
    native_window: TNativeWindow,
}

impl<T> WindowGeneric<T>
where
    T: NativeWindow,
{
    pub fn create() -> Self {
        Self {
            native_window: T::create(),
        }
    }

    pub fn size(&self) -> Size<u32> {
        self.native_window.size()
    }

    pub fn process_until_end(&mut self) {
        self.native_window.process_until_end();
    }

    pub fn process_message_if_available(&mut self) -> WindowProcessResult {
        self.native_window.process_message_if_available()
    }

    pub fn native_window_handle(&self) -> NativeWindowHandle {
        self.native_window.handle()
    }
}

#[cfg(target_os = "windows")]
pub struct Window {
    window_generic: WindowGeneric<Win32Window>,
}

impl Window {
    pub fn create() -> Self {
        Self {
            #[cfg(target_os = "windows")]
            window_generic: WindowGeneric::<Win32Window>::create(),
        }
    }

    pub fn size(&self) -> Size<u32> {
        self.window_generic.size()
    }

    pub fn process_until_end(&mut self) {
        self.window_generic.process_until_end();
    }

    pub fn process_message_if_available(&mut self) -> WindowProcessResult {
        self.window_generic.process_message_if_available()
    }

    pub fn native_window_handle(&self) -> NativeWindowHandle {
        self.window_generic.native_window_handle()
    }
}
