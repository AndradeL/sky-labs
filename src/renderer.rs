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

use std::ops::Deref;

use crate::win::renderer_d3d12::Direct3D12Renderer;

use super::{
    math::{Size, Vector2, Rect},
    window::Window,
};

#[cfg(target_os = "windows")]
pub struct DefaultRenderer(Direct3D12Renderer);
impl DefaultRenderer {
    pub fn create_for_window(window: &Window) -> Self {
        DefaultRenderer(Direct3D12Renderer::create_for_window(window))
    }
}
impl Deref for DefaultRenderer {
    type Target = Direct3D12Renderer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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

/// Drawing session to draw on a surface.
/// Use Renderer::begin_draw to get a DrawingSession from the renderer in use.
/// Call Renderer::end_draw to submit the changes to the surface.
pub trait DrawingSession {
    /// Draw a text to the game window
    fn draw_text(&self, text: &String, format: &TextFormat, coord: &Rect<f32>);

    /// Draw a rectangle to the game window
    fn draw_rectangle(&self, rect: &Rect<f32>, color: &Color);

    /// Draw a circle within bounds to the game window
    fn draw_circle(&self, bounds: &Rect<f32>, color: &Color);

    /// Draw a circle centered at 'center' with given 'radius'
    fn draw_circle_centered_at(&self, center: &Vector2<f32>, radius: f32, color: &Color);
}

pub trait Renderer<'a, T: 'a + DrawingSession> {
    /// Creates renderer for specified window
    fn create_for_window(window: &Window) -> Self
    where
        Self: Sized;

    /// Returns the size of the render target
    fn size(&'a self) -> Size<f32>;

    /// Returns a drawing session to draw on the window
    fn begin_draw(&'a self) -> T;

    /// Ends the drawing session, submitting the changes to the window
    /// This method should be called after all drawing operations are done
    /// to display the changes on the window
    fn end_draw(&'a self, drawing_session: T);
}
