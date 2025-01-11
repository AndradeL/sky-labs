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

use windows::{
    core::w,
    Win32::Graphics::{
        Direct2D::{
            Common::{D2D1_COLOR_F, D2D_RECT_F},
            D2D1_DRAW_TEXT_OPTIONS_NONE,
        },
        DirectWrite::{
            DWriteCreateFactory, IDWriteFactory, IDWriteTextFormat, DWRITE_FACTORY_TYPE_SHARED,
            DWRITE_FONT_STRETCH_NORMAL, DWRITE_FONT_STYLE_NORMAL, DWRITE_FONT_WEIGHT,
            DWRITE_MEASURING_MODE_NATURAL,
        },
    },
};

use super::performance_counter::PerformanceCounter;
use crate::events::Event;

#[derive(Default)]
pub struct FramerateCounter {
    frames_this_second: u32,
    time: PerformanceCounter,
    pub frames_per_second: u32,
    render_text_format: Option<IDWriteTextFormat>,
}

impl FramerateCounter {
    pub(super) fn new() -> Self {
        FramerateCounter {
            frames_this_second: 0,
            time: PerformanceCounter::default(),
            frames_per_second: 0,
            render_text_format: None,
        }
    }

    pub(super) fn tick(&self, delta: PerformanceCounter) -> Self {
        let now = self.time + delta;
        let (frames_this_second, frames_per_second) =
            if now.ticks >= PerformanceCounter::frequency() {
                (0, self.frames_this_second)
            } else {
                (self.frames_this_second + 1, self.frames_per_second)
            };
        FramerateCounter {
            frames_this_second,
            time: PerformanceCounter {
                ticks: now.ticks % PerformanceCounter::frequency(),
            },
            frames_per_second,
            render_text_format: self.render_text_format.clone(),
        }
    }
    
    fn render(&self, render_target: &windows::Win32::Graphics::Direct2D::ID2D1RenderTarget) {
        unsafe {
            let frames_per_second = format!("{}", self.frames_per_second);
            let mut u16_string: Vec<u16> = frames_per_second.encode_utf16().collect();
            u16_string.push(0);
            let layout_rect = D2D_RECT_F {
                top: 0.0,
                left: 0.0,
                right: 80.0,
                bottom: 40.0,
            };
            let green = D2D1_COLOR_F {
                r: 0.0,
                g: 1.0,
                b: 0.0,
                a: 1.0,
            };
            let brush = render_target.CreateSolidColorBrush(&green, None).unwrap();
            render_target.DrawText(
                &u16_string,
                self.render_text_format.as_ref().unwrap(),
                &layout_rect,
                &brush,
                D2D1_DRAW_TEXT_OPTIONS_NONE,
                DWRITE_MEASURING_MODE_NATURAL,
            )
        }
    }

    fn create_resources(
        &mut self,
        _render_target: &windows::Win32::Graphics::Direct2D::ID2D1RenderTarget,
    ) {
        unsafe {
            let dwrite_factory: IDWriteFactory =
                DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED).unwrap();
            let text_format = dwrite_factory
                .CreateTextFormat(
                    w!("Consolas"),
                    None,
                    DWRITE_FONT_WEIGHT(400),
                    DWRITE_FONT_STYLE_NORMAL,
                    DWRITE_FONT_STRETCH_NORMAL,
                    40.0,
                    w!("en-us"),
                )
                .unwrap();
            self.render_text_format = Some(text_format);
        }
    }
}
