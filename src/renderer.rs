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

use super::{
    math::{Size, Vector2, Rect},
    window::Window,
};

#[cfg(target_os = "windows")]
pub use super::win::renderer_d3d12::Direct3D12Renderer as DefaultRenderer;

pub enum RendererType {
    Direct2D,
    Direct3D12,
}

pub struct TextFormat {}

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub trait Renderer {
    /// Creates renderer for specified window
    fn create_for_window(window: &Window) -> Self
    where
        Self: Sized;

    /// Returns the size of the render window
    fn size(&self) -> Size<f32>;

    /// Draw a text to the game window
    fn draw_text(&self, text: &String, format: &TextFormat, coord: &Rect<f32>);

    /// Draw a rectangle to the game window
    fn draw_rectangle(&self, rect: &Rect<f32>, color: &Color);

    /// Draw a circle within bounds to the game window
    fn draw_circle(&self, bounds: &Rect<f32>, color: &Color);

    /// Draw a circle centered at 'center' with given 'radius'
    fn draw_circle_centered_at(&self, center: &Vector2<f32>, radius: f32, color: &Color);
}
