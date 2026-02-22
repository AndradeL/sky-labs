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

use std::mem::ManuallyDrop;

use windows::{
    core::Interface,
    Win32::{
        Foundation::RECT,
        Graphics::{
            Direct3D::*,
            Direct3D12::*,
            Dxgi::{Common::*, *},
        },
    },
};

use crate::{
    math::{Rect, Vector2},
    renderer::{Color, DrawingSession, Renderer, TextFormat},
};

use super::{text::Direct3D12TextRenderer, Direct3D12Renderer};

pub struct Direct3D12DrawingSession<'a> {
    renderer: &'a Direct3D12Renderer,
    pub(super) command_list: ID3D12GraphicsCommandList,
    resources: Vec<ID3D12Resource>,
}

impl<'a> DrawingSession for Direct3D12DrawingSession<'a> {
    /// Clear the game window with the given color
    fn clear(&mut self, color: &Color<f32>) {
        #[cfg(debug_assertions)]
        debug_assert!(super::debug::check_render_target_state(
            &self.command_list,
            &self.renderer.current_frame(),
        ));



        unsafe {
            let mut rtv_handle = self.renderer.rtv_descriptor_heap.GetCPUDescriptorHandleForHeapStart();
            rtv_handle.ptr 
                += self.renderer.rtv_descriptor_size as usize * self.renderer.current_frame_index();
            self.command_list.ClearRenderTargetView(
                rtv_handle,
                color.as_slice(),
                None,
            );
        }
    }

    /// Draw a text to the game window
    fn draw_text(&mut self, text: &String, format: &TextFormat, rect: &Rect<f32>) {
        let text_renderer = Direct3D12TextRenderer::create_for_renderer(&self.renderer);
        text_renderer.render_text(text, format, rect).unwrap();
    }

    fn draw_triangle(&mut self, points: &[Vector2<f32>; 3], color: &Color<f32>) {
        #[cfg(debug_assertions)]
        debug_assert!(super::debug::check_render_target_state(
            &self.command_list,
            &self.renderer.current_frame(),
        ));

        let rtv_handle = unsafe {
            self.renderer
                .rtv_descriptor_heap
                .GetCPUDescriptorHandleForHeapStart()
        };
        let vertex_buffer_heap = load_triangle_buffer(&self.renderer, points);

        #[cfg(debug_assertions)]
        debug_assert!(super::debug::check_vertex_buffer_state(
            &self.command_list,
            &vertex_buffer_heap
        ));

        let vertex_buffer_view = D3D12_VERTEX_BUFFER_VIEW {
            BufferLocation: unsafe { vertex_buffer_heap.GetGPUVirtualAddress() },
            SizeInBytes: 24u32, // TODO: Fix this
            StrideInBytes: 8u32, // TODO: Fix this
        };
        unsafe {
            self.command_list
                .IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);

            self.command_list
                .IASetVertexBuffers(0, Some(&[vertex_buffer_view]));

            self.command_list.DrawInstanced(3, 1, 0, 0);
        }

        // Add the vertex buffer to the list of resources to be released
        self.resources
            .push(vertex_buffer_heap);
    }

    /// Draw a rectangle to the game window
    fn draw_rectangle(&mut self, rect: &Rect<f32>, color: &Color<f32>) {
        todo!()
    }

    /// Draw a circle within bounds to the game window
    fn draw_circle(&mut self, bounds: &Rect<f32>, color: &Color<f32>) {
        todo!()
    }

    /// Draw a circle centered at 'center' with given 'radius'
    fn draw_circle_centered_at(&mut self, center: &Vector2<f32>, radius: f32, color: &Color<f32>) {
        todo!()
    }
}

impl<'a> Direct3D12DrawingSession<'a> {
    pub fn new(renderer: &'a Direct3D12Renderer) -> Self {
        let command_list = match renderer.create_command_list() {
            Ok(c) => c,
            Err(e) => {
                #[cfg(debug_assertions)]
                super::debug::dump_debug_messages(&renderer.device);

                panic!("Failed to create command list: {}", e);
            }
        };
        
        unsafe {
            let root_signature =
                get_root_signature(&renderer.device).expect("failed to create root signature");
            command_list.SetGraphicsRootSignature(&root_signature);

            command_list.RSSetViewports(&[D3D12_VIEWPORT {
                TopLeftX: 0.0,
                TopLeftY: 0.0,
                Width: renderer.size().width as f32,
                Height: renderer.size().height as f32,
                MinDepth: 0.0,
                MaxDepth: 1.0,
            }]);

            command_list.RSSetScissorRects(&[RECT {
                left: 0,
                top: 0,
                right: renderer.size().width as i32,
                bottom: renderer.size().height as i32,
            }]);

            let current_frame_back_buffer = renderer.current_frame().clone();
            let transition_barrier = D3D12_RESOURCE_TRANSITION_BARRIER {
                pResource: ManuallyDrop::new(Some(current_frame_back_buffer)),
                Subresource: D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES,
                StateBefore: D3D12_RESOURCE_STATE_PRESENT,
                StateAfter: D3D12_RESOURCE_STATE_RENDER_TARGET,
            };
            let barrier = D3D12_RESOURCE_BARRIER {
                Type: D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
                Flags: D3D12_RESOURCE_BARRIER_FLAG_NONE,
                Anonymous: D3D12_RESOURCE_BARRIER_0 {
                    Transition: ManuallyDrop::new(transition_barrier),
                },
            };

            command_list.ResourceBarrier(&[barrier]);

            let mut rtv_handle = renderer
                .rtv_descriptor_heap
                .GetCPUDescriptorHandleForHeapStart();
            rtv_handle.ptr +=
                renderer.rtv_descriptor_size as usize * renderer.current_frame_index();
            command_list.OMSetRenderTargets(1, Some([rtv_handle].as_ptr()), false, None);
        };
        Direct3D12DrawingSession {
            renderer,
            command_list,
            resources: Vec::new(),
        }
    }
}

fn load_triangle_buffer(
    renderer: &Direct3D12Renderer,
    triangle_vertices: &[Vector2<f32>; 3],
) -> ID3D12Resource {
    let device = &renderer.device;
    let heap_properties = D3D12_HEAP_PROPERTIES {
        Type: D3D12_HEAP_TYPE_UPLOAD,
        CPUPageProperty: D3D12_CPU_PAGE_PROPERTY_UNKNOWN,
        MemoryPoolPreference: D3D12_MEMORY_POOL_UNKNOWN,
        CreationNodeMask: 1,
        VisibleNodeMask: 1,
    };
    let resource_desc = D3D12_RESOURCE_DESC {
        Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
        Alignment: 0,
        Width: std::mem::size_of_val(triangle_vertices) as u64,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: DXGI_FORMAT_UNKNOWN,
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            Quality: 0,
        },
        Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
        Flags: D3D12_RESOURCE_FLAG_NONE,
    };

    let mut resource: Option<ID3D12Resource> = None;
    let result = unsafe {
        device.CreateCommittedResource(
            &heap_properties,
            D3D12_HEAP_FLAG_NONE,
            &resource_desc,
            D3D12_RESOURCE_STATE_GENERIC_READ,
            None,
            &mut resource,
        )
    };
    let resource = match result {
        Ok(_) => resource.unwrap(),
        Err(e) => panic!("Failed to create vertex buffer: {}", e.to_string()),
    };

    let no_read_range = D3D12_RANGE::default();
    let mut data: *mut std::ffi::c_void = std::ptr::null_mut();
    let result = unsafe { resource.Map(0, Some(&no_read_range), Some(&mut data)) };
    match result {
        Ok(_) => unsafe {
            std::ptr::copy(
                triangle_vertices
                    .as_ptr() as *const std::ffi::c_void,
                data,
                std::mem::size_of_val(triangle_vertices),
            );

            resource.Unmap(0, None);

            renderer.wait_for_frame();

            resource
        },
        Err(e) => panic!("Failed to map vertex buffer: {}", e.to_string()),
    }
}

fn get_root_signature(device: &ID3D12Device) -> Result<ID3D12RootSignature, String> {
    let root_signature_desc = D3D12_ROOT_SIGNATURE_DESC {
        NumParameters: 0,
        pParameters: std::ptr::null(),
        NumStaticSamplers: 0,
        pStaticSamplers: std::ptr::null(),
        Flags: D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
    };

    let mut root_signature_blob: Option<ID3DBlob> = None;
    let mut error_blob: Option<ID3DBlob> = None;

    let result = unsafe {
        D3D12SerializeRootSignature(
            &root_signature_desc,
            D3D_ROOT_SIGNATURE_VERSION_1,
            &mut root_signature_blob,
            Some(&mut error_blob),
        )
    };
    match result {
        Ok(_) => {}
        Err(e) => return Err(e.to_string()),
    }

    let root_signature_blob = root_signature_blob.unwrap();
    let result = unsafe {
        device.CreateRootSignature(
            0,
            std::slice::from_raw_parts(
                std::mem::transmute(root_signature_blob.GetBufferPointer()),
                root_signature_blob.GetBufferSize(),
            ),
        )
    };
    match result {
        Ok(rs) => Ok(rs),
        Err(e) => Err(e.to_string()),
    }
}
