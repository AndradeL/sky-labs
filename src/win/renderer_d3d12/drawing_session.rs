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

use crate::{
    math::{Rect, Vector2},
    renderer::{Color, DrawingSession, TextFormat},
};

use super::{text::Direct3D12TextRenderer, Direct3D12Renderer};

pub(super) struct Direct3D12DrawingSession<'a>(pub &'a Direct3D12Renderer);

impl<'a> DrawingSession for Direct3D12DrawingSession<'a> {
    /// Draw a text to the game window
    fn draw_text(&self, text: &String, format: &TextFormat, rect: &Rect<f32>) {
        let text_renderer = Direct3D12TextRenderer::create_for_renderer(&self.0);
        text_renderer.render_text(text, format, rect).unwrap();
    }

    /// Draw a rectangle to the game window
    fn draw_rectangle(&self, rect: &Rect<f32>, color: &Color) {
        todo!()
    }

    /// Draw a circle within bounds to the game window
    fn draw_circle(&self, bounds: &Rect<f32>, color: &Color) {
        todo!()
    }

    /// Draw a circle centered at 'center' with given 'radius'
    fn draw_circle_centered_at(&self, center: &Vector2<f32>, radius: f32, color: &Color) {
        todo!()
    }
}
