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

pub use windows::Win32::Foundation::HWND as NativeWindowHandle;

use windows::{
    core::w,
    Win32::{
        Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
        System::{
            Com::{CoInitializeEx, COINIT_MULTITHREADED},
            LibraryLoader::GetModuleHandleW,
        },
        UI::WindowsAndMessaging::*,
    },
};
use windows_core::PCWSTR;

use crate::{
    math::Size,
    window::{NativeWindow, WindowProcessResult},
};

const WINDOW_CLASS_NAME: PCWSTR = w!("snake_main_wnd");

pub struct Win32Window {
    window_handle: HWND,
    size: Size<u32>,
}

impl NativeWindow for Win32Window {
    fn create() -> Self {
        ensure_single_instance();
        unsafe {
            CoInitializeEx(None, COINIT_MULTITHREADED).unwrap();
            let hinstance = GetModuleHandleW(None).unwrap();
            debug_assert!(!hinstance.is_invalid());

            let wndclass = WNDCLASSW {
                style: CS_DBLCLKS,
                hInstance: HINSTANCE::from(hinstance),
                hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
                lpszClassName: WINDOW_CLASS_NAME,
                lpfnWndProc: Some(Self::static_window_procedure),
                ..Default::default()
            };

            let atom = RegisterClassW(&wndclass);
            debug_assert!(atom != 0);

            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                WINDOW_CLASS_NAME,
                w!("snake-rs"),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                None,
                None,
                Some(hinstance.into()),
                None,
            )
            .expect("Could not create Window for game.");

            Self {
                window_handle: hwnd,
                size: Size::default(),
            }
        }
    }

    fn size(&self) -> crate::math::Size<u32> {
        self.size
    }

    fn handle(&self) -> NativeWindowHandle {
        self.window_handle
    }

    fn process_until_end(&mut self) {
        let mut message = MSG::default();
        unsafe {
            while GetMessageW(&mut message, None, 0, 0).as_bool() {
                let _ = TranslateMessage(&message);
                DispatchMessageW(&message);
            }
        }
    }

    fn process_message_if_available(&mut self) -> WindowProcessResult {
        let mut message = MSG::default();
        unsafe {
            if PeekMessageW(&mut message, None, 0, 0, PM_REMOVE).as_bool() {
                if message.message == WM_QUIT {
                    WindowProcessResult::Exit
                } else {
                    let _ = TranslateMessage(&message);
                    DispatchMessageW(&message);
                    WindowProcessResult::Ok
                }
            } else {
                WindowProcessResult::Ok
            }
        }
    }
}

impl Drop for Win32Window {
    fn drop(&mut self) {
        // Destroys the window and wait for it to end itself.
        unsafe {
            let _ = DestroyWindow(self.window_handle);
            self.process_until_end();
        }
    }
}

impl Win32Window {
    extern "system" fn static_window_procedure(
        window: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        unsafe {
            match message {
                WM_DESTROY => {
                    PostQuitMessage(0);
                    LRESULT(0)
                }
                _ => DefWindowProcW(window, message, wparam, lparam),
            }
        }
    }
}

fn ensure_single_instance() {
    unsafe {
        // panic if fail
        windows::Win32::System::Threading::CreateMutexW(None, true, w!("snake-rs-single-instance"))
            .unwrap();
    }
}
