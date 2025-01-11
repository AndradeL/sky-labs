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

mod text;

use crate::{
    math::{Size, Vector2},
    renderer::*,
    window::Window,
};

use windows::Win32::Graphics::{
    Direct3D::*,
    Direct3D12::*,
    Dxgi::{Common::*, *},
};

const FRAME_COUNT: u32 = 2;

pub struct Direct3D12Renderer {
    device: ID3D12Device,
    command_queue: ID3D12CommandQueue,
    swap_chain: IDXGISwapChain1,
    rtv_descriptor_heap: ID3D12DescriptorHeap,
    rtv_descriptor_size: u32,
    render_target_views: [ID3D12Resource; FRAME_COUNT as usize],
    command_allocator: ID3D12CommandAllocator,
}

impl Renderer for Direct3D12Renderer {
    fn create_for_window(window: &Window) -> Self {
        // #[cfg(debug_assertions)]
        enable_debug().unwrap();

        let device = create_d3d_device().unwrap();

        let command_queue = create_command_queue(&device).unwrap();

        let swap_chain = create_swap_chain(&window, &command_queue).unwrap();

        let rtv_descriptor_heap = create_rtv_descriptor_heap(&device).unwrap();
        let rtv_descriptor_size =
            unsafe { device.GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_RTV) };

        let render_target_views = create_render_target_views(
            &device,
            &rtv_descriptor_heap,
            rtv_descriptor_size,
            &swap_chain,
        );

        let command_allocator = create_command_allocator(&device).unwrap();

        // TODO: erase
        let  bg_color = DXGI_RGBA {
            r: 0.0,
            g: 1.0,
            b: 0.0,
            a: 1.0,
        };
        unsafe { swap_chain.SetBackgroundColor(&bg_color).unwrap() };
        let parameter = DXGI_PRESENT_PARAMETERS {
            ..Default::default()
        };
        unsafe { swap_chain.Present1(1, DXGI_PRESENT::default(), &parameter).ok().expect("unable to present swap chain"); };

        Self {
            device,
            command_queue,
            swap_chain,
            rtv_descriptor_heap,
            rtv_descriptor_size,
            render_target_views,
            command_allocator,
        }
    }

    fn size(&self) -> Size<f32> {
        let result = unsafe { self.swap_chain.GetDesc1() };
        match result {
            Ok(desc) => Size::<f32> {
                width: desc.Width as f32,
                height: desc.Height as f32,
            },
            Err(e) => {
                println!("RendererD3D12::size() error: {}", e);
                Size::<f32>::default()
            }
        }
    }

    fn draw_text(&self, text: &String, format: &TextFormat, rect: &Rect) {
        let text_renderer = text::Direct3D12TextRenderer::create_for_renderer(self);
        text_renderer.render_text(text, format, rect).unwrap();
    }

    fn draw_rectangle(&self, rect: &Rect, color: &Color) {
        todo!()
    }

    fn draw_circle(&self, bounds: &Rect, color: &Color) {
        todo!()
    }

    fn draw_circle_centered_at(&self, center: &Vector2<f32>, radius: f32, color: &Color) {
        todo!()
    }
}

/// Enables Direct3D12 Debug layer
fn enable_debug() -> Result<(), String> {
    let mut debug_layer: Option<ID3D12Debug> = None;
    unsafe {
        let result = D3D12GetDebugInterface(&mut debug_layer);
        match result {
            Ok(_) => {
                debug_layer.unwrap().EnableDebugLayer();
                Ok(())
            }
            Err(s) => Err(s.to_string()),
        }
    }
}

/// Creates the D3D device to be used throughout application for resource loading
/// panics if fail because the application can't run without it.
fn create_d3d_device() -> Result<ID3D12Device, String> {
    let mut device: Option<ID3D12Device> = None;

    let result = unsafe { D3D12CreateDevice(None, D3D_FEATURE_LEVEL_12_0, &mut device) };

    match result {
        Ok(_) => Ok(device.unwrap()),
        Err(e) => Err(e.to_string()),
    }
}

/// Creates the D3D12 Command Queue for the given device.
fn create_command_queue(device: &ID3D12Device) -> Result<ID3D12CommandQueue, String> {
    let desc = D3D12_COMMAND_QUEUE_DESC::default();

    let result = unsafe { device.CreateCommandQueue(&desc) };

    match result {
        Ok(queue) => Ok(queue),
        Err(e) => Err(e.to_string()),
    }
}

/// Calls DXGI to create a Swap Chain for the given Window.
/// note: using double-buffer, flip-discard.
fn create_swap_chain(
    window: &Window,
    command_queue: &ID3D12CommandQueue,
) -> Result<IDXGISwapChain1, String> {
    let desc = DXGI_SWAP_CHAIN_DESC1 {
        BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
        SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
        Format: DXGI_FORMAT_R8G8B8A8_UNORM,
        BufferCount: FRAME_COUNT,
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            Quality: 0,
        },
        Flags: DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING.0 as u32,
        ..Default::default()
    };

    let factory: IDXGIFactory2 = unsafe { CreateDXGIFactory2(DXGI_CREATE_FACTORY_DEBUG).expect("Unable to create DXGI Factory") };

    let result = unsafe {
        factory.CreateSwapChainForHwnd(
            command_queue,
            window.native_window_handle(),
            &desc,
            None,
            None,
        )
    };
    match result {
        Ok(r) => Ok(r),
        Err(e) => Err(e.to_string()),
    }
}

/// Creates a Render Target View (RTV) Descriptor Heap on a device
fn create_rtv_descriptor_heap(device: &ID3D12Device) -> Result<ID3D12DescriptorHeap, String> {
    let desc = D3D12_DESCRIPTOR_HEAP_DESC {
        Type: D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
        NumDescriptors: FRAME_COUNT,
        ..Default::default()
    };
    let result = unsafe { device.CreateDescriptorHeap(&desc) };
    match result {
        Ok(heap) => Ok(heap),
        Err(e) => Err(e.to_string()),
    }
}

/// Creates the Render Target View for each of the Swap Chain buffers (we're using `FRAME_COUNT = 2` buffers)
fn create_render_target_views(
    device: &ID3D12Device,
    descriptor_heap: &ID3D12DescriptorHeap,
    descriptor_size: u32,
    swap_chain: &IDXGISwapChain1,
) -> [ID3D12Resource; FRAME_COUNT as usize] {
    // Prefer static allocated array, to maintain memory locallity. Can't do it without using the uninitialized unsafe operation.
    let mut buffers_array =
        std::mem::MaybeUninit::<[ID3D12Resource; FRAME_COUNT as usize]>::uninit();
    let mut ptr = buffers_array.as_mut_ptr() as *mut ID3D12Resource;
    let mut handle = unsafe { descriptor_heap.GetCPUDescriptorHandleForHeapStart() };

    unsafe {
        for idx in 0..FRAME_COUNT {
            let buffer: ID3D12Resource = swap_chain.GetBuffer(idx).unwrap();
            device.CreateRenderTargetView(&buffer, None, handle);
            handle.ptr += descriptor_size as usize;
            ptr.write(buffer);
            ptr = ptr.add(1);
        }
        buffers_array.assume_init()
    }
}

fn create_command_allocator(device: &ID3D12Device) -> Result<ID3D12CommandAllocator, String> {
    let result = unsafe { device.CreateCommandAllocator(D3D12_COMMAND_LIST_TYPE_DIRECT) };
    match result {
        Ok(r) => Ok(r),
        Err(e) => Err(e.to_string()),
    }
}
