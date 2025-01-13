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

use crate::{math::Vector2, renderer::{Color, Rect, Renderer, RendererType, TextFormat}, window::Window};

use super::window::{NativeWindowHandle};

use windows::Win32::{Foundation::HMODULE, Graphics::{
        Direct2D::{Common::*, *},
        Direct3D::*,
        Direct3D11::*,
        Dxgi::{Common::*, *},
    }};
use windows_core::Interface;

pub struct Direct2DRenderer {}

impl Renderer for Direct2DRenderer {
    fn create_for_window(window: &Window) -> Self {
        Self{}
    }
    
    fn size(&self) -> crate::math::Size<f32> {
        todo!()
    }
}

/// Creates the D3D device to be used throughout application for resource loading
/// panics if fail because the application can't run without it.
fn create_d3d_device() -> (D3D_FEATURE_LEVEL, ID3D11Device, ID3D11DeviceContext) {
    let levels = [D3D_FEATURE_LEVEL_11_1, D3D_FEATURE_LEVEL_11_0];

    // This flag adds support for surfaces with a color-channel ordering different
    // from the API default. It is required for compatibility with Direct2D.
    let mut device: Option<ID3D11Device> = None;
    let mut immediate_ctx: Option<ID3D11DeviceContext> = None;
    let mut feature_level: D3D_FEATURE_LEVEL = D3D_FEATURE_LEVEL(0);
    let device_flags = D3D11_CREATE_DEVICE_BGRA_SUPPORT | D3D11_CREATE_DEVICE_DEBUG;
    unsafe {
        D3D11CreateDevice(
            None,
            D3D_DRIVER_TYPE_HARDWARE,
            HMODULE::default(),
            device_flags,
            Some(&levels),
            D3D11_SDK_VERSION,
            Some(&mut device),
            Some(&mut feature_level),
            Some(&mut immediate_ctx),
        ).expect("Could not create D3D11 device.");
    }

    (feature_level, device.unwrap(), immediate_ctx.unwrap())
}

fn create_swap_chain(window: &Window, d3d_device: &ID3D11Device) -> IDXGISwapChain1 {
    let desc = DXGI_SWAP_CHAIN_DESC1 {
        BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
        SwapEffect: DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
        Format: DXGI_FORMAT_R8G8B8A8_UNORM,
        BufferCount: 2,
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            Quality: 0,
        },
        Flags: DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING.0 as u32,
        ..Default::default()
    };

    let dxgi_device: IDXGIDevice = d3d_device.cast().unwrap();

    unsafe {
        // Create Swap Chain
        let adapter = dxgi_device.GetAdapter().unwrap();
        let factory: IDXGIFactory3 = adapter.GetParent().unwrap();
        factory
            .CreateSwapChainForHwnd(
                &dxgi_device,
                window.native_window_handle(),
                &desc,
                None,
                None,
            )
            .unwrap()
    }
}

fn create_render_target(swap_chain: &IDXGISwapChain1) -> ID2D1RenderTarget {
    unsafe {
        let surface: IDXGISurface = swap_chain.GetBuffer(0).unwrap();
        let d2d_factory: ID2D1Factory =
            D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None).unwrap();

        let properties = D2D1_RENDER_TARGET_PROPERTIES {
            r#type: D2D1_RENDER_TARGET_TYPE_DEFAULT,
            pixelFormat: D2D1_PIXEL_FORMAT {
                format: DXGI_FORMAT_R8G8B8A8_UNORM,
                alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
            },
            dpiX: 96.0,
            dpiY: 96.0,
            ..Default::default()
        };
        d2d_factory
            .CreateDxgiSurfaceRenderTarget(&surface, &properties)
            .unwrap()
    }
}
