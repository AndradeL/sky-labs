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

#[cfg(debug_assertions)]
mod debug;
mod drawing_session;
mod text;

use std::{mem::ManuallyDrop, sync::Mutex};

use crate::{math::Size, renderer::*, window::Window};

use drawing_session::Direct3D12DrawingSession;
use windows::{
    core::s,
    Win32::{
        Foundation::{BOOL, HANDLE, WAIT_OBJECT_0},
        Graphics::{
            Direct3D::{
                Fxc::{D3DCompile, D3DCOMPILE_DEBUG, D3DCOMPILE_SKIP_OPTIMIZATION},
                *,
            },
            Direct3D12::*,
            Dxgi::{Common::*, *},
        },
        System::Threading::{CreateEventW, WaitForSingleObject},
    },
};
use windows_core::Interface;

/// Number of frames in the swap chain
const FRAME_COUNT: u32 = 2;

/// Direct3D12 Renderer
pub struct Direct3D12Renderer {
    rtv_descriptor_size: u32,
    pipeline_state: ID3D12PipelineState, // TODO: move out of here
    command_allocator: ID3D12CommandAllocator,
    render_target_views: [ID3D12Resource; FRAME_COUNT as usize],
    rtv_descriptor_heap: ID3D12DescriptorHeap,
    swap_chain: IDXGISwapChain3,
    command_queue: ID3D12CommandQueue,
    frame_fence: ID3D12Fence,
    frame_event: HANDLE,
    fence_value: Mutex<u64>,
    device: ID3D12Device,
}

impl<'a> Renderer<'a, Direct3D12DrawingSession<'a>> for Direct3D12Renderer {
    /// Creates renderer that draws directly into the specified window
    /// Since the renderer is a essential part of the application, it will panic if it fails to create.
    fn create_for_window(window: &Window) -> Self {
        #[cfg(debug_assertions)]
        debug::init();

        let device = create_d3d_device().unwrap();

        let frame_fence = unsafe { device.CreateFence(0, D3D12_FENCE_FLAG_NONE) }.unwrap();

        let frame_event = unsafe { CreateEventW(None, false, false, None) }.unwrap();

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

        let pipeline_state = compile_shaders(&device).unwrap();

        Self {
            device,
            command_queue,
            swap_chain,
            rtv_descriptor_heap,
            rtv_descriptor_size,
            render_target_views,
            command_allocator,
            pipeline_state,
            frame_fence,
            frame_event,
            fence_value: Mutex::new(0),
        }
    }

    /// Returns the size of the final draw size
    fn size(&'a self) -> Size<f32> {
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

    fn begin_draw(&'a self) -> Direct3D12DrawingSession<'a> {
        unsafe {
            self.command_allocator
                .Reset()
                .expect("Failed to reset Command Allocator.")
        };
        Direct3D12DrawingSession::new(&self)
    }

    fn end_draw(&'a self, drawing_session: Direct3D12DrawingSession<'a>) {
        let current_frame_back_buffer = self.current_frame().clone();
        let transition_barrier_desc = D3D12_RESOURCE_TRANSITION_BARRIER {
            pResource: ManuallyDrop::new(Some(current_frame_back_buffer)),
            Subresource: D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES,
            StateBefore: D3D12_RESOURCE_STATE_RENDER_TARGET,
            StateAfter: D3D12_RESOURCE_STATE_PRESENT,
        };
        let barrier = D3D12_RESOURCE_BARRIER {
            Type: D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
            Flags: D3D12_RESOURCE_BARRIER_FLAG_NONE,
            Anonymous: D3D12_RESOURCE_BARRIER_0 {
                Transition: ManuallyDrop::new(transition_barrier_desc),
            },
        };

        unsafe {
            drawing_session.command_list.ResourceBarrier(&[barrier]);
            drawing_session
                .command_list
                .Close()
                .expect("Failed to close Command List.");
        }

        unsafe {
            self.command_queue
                .ExecuteCommandLists(&[Some(drawing_session.command_list.cast().unwrap())])
        };

        // core::mem::forget(drawing_session);

        self.wait_for_frame();
    }
}

impl Drop for Direct3D12Renderer {
    fn drop(&mut self) {
        // Wait for the GPU to finish executing the command list before releasing resources.
        self.wait_for_frame();
    }
}

impl Direct3D12Renderer {
    pub(self) fn create_command_list(&self) -> Result<ID3D12GraphicsCommandList, String> {
        match unsafe {
            self.device.CreateCommandList(
                0,
                D3D12_COMMAND_LIST_TYPE_DIRECT,
                &self.command_allocator,
                &self.pipeline_state,
            )
        } {
            Ok(list) => Ok(list),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn present(&self) {
        #[cfg(debug_assertions)]
        debug_assert!(debug::check_present_state(
            &self.command_queue,
            self.current_frame()
        ));

        match unsafe {
            self.swap_chain
                .Present1(
                    1,
                    DXGI_PRESENT::default(),
                    &DXGI_PRESENT_PARAMETERS::default(),
                )
                .ok()
        } {
            Ok(_) => {}
            Err(e) => match e.code() {
                DXGI_ERROR_DEVICE_REMOVED => {
                    let reason = unsafe { self.device.GetDeviceRemovedReason() }.unwrap_err();

                    #[cfg(debug_assertions)]
                    debug::dump_debug_messages(&self.device);

                    panic!("Device removed: {}", reason.to_string())
                }
                _ => {
                    panic!("Unable to present swap chain: {}", e.to_string())
                }
            },
        };
    }

    pub fn wait_for_frame(&self) {
        unsafe {
            let mut lock = self.fence_value.lock().unwrap();
            let fence_value = *lock;
            match self
                .command_queue
                .Signal(&self.frame_fence, fence_value)
                .err()
            {
                Some(e) => panic!("Unable to signal fence for wait: {}", e.to_string()),
                None => {}
            };
            match self
                .frame_fence
                .SetEventOnCompletion(fence_value, self.frame_event)
                .err()
            {
                Some(e) => panic!("Unable to set event on completion: {}", e.to_string()),
                None => {}
            };
            if WaitForSingleObject(self.frame_event, 1000) != WAIT_OBJECT_0 {
                panic!("Timeout waiting for fence to signal");
            }
            *lock += 1;
        }
    }

    pub(crate) fn current_frame(&self) -> &ID3D12Resource {
        let index = self.current_frame_index();
        &self.render_target_views[index]
    }

    pub(crate) fn current_frame_index(&self) -> usize {
        unsafe { self.swap_chain.GetCurrentBackBufferIndex() as usize }
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
) -> Result<IDXGISwapChain3, String> {
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

    let factory: IDXGIFactory2 = unsafe {
        CreateDXGIFactory2(DXGI_CREATE_FACTORY_DEBUG).expect("Unable to create DXGI Factory")
    };

    let result = unsafe {
        factory.CreateSwapChainForHwnd(
            command_queue,
            window.native_window_handle(),
            &desc,
            None,
            None,
        )
    };
    let swap_chain = match result {
        Ok(r) => r,
        Err(e) => return Err(e.to_string()),
    };

    match swap_chain.cast::<IDXGISwapChain3>() {
        Ok(swap_chain) => Ok(swap_chain),
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

fn compile_shaders(device: &ID3D12Device) -> Result<ID3D12PipelineState, String> {
    // TODO: refactor this to reduce function size and complexity and actually return a Result object, instead of relying on the expect() method.
    let root_signature = get_root_signature(device)?;

    // let vertex_shader_bytecode = include_bytes!("renderer_d3d12/shaders/simple2d/vs.fxc");
    // let pixel_shader_bytecode = include_bytes!("renderer_d3d12/shaders/simple2d/ps.fxc");

    let vertex_shader_bytecode = compile_from_content(
        include_bytes!("renderer_d3d12/shaders/simple2d/vs_2d_screen_position.hlsl"),
        "VSMain\0",
        "vs_5_1\0",
    )?;
    let pixel_shader_bytecode = compile_from_content(
        include_bytes!("renderer_d3d12/shaders/simple2d/ps_2d_simple_color.hlsl"),
        "PSMain\0",
        "ps_5_1\0",
    )?;

    let pipeline_state_description = D3D12_GRAPHICS_PIPELINE_STATE_DESC {
        pRootSignature: ManuallyDrop::new(Some(root_signature)), // Root Signature
        VS: D3D12_SHADER_BYTECODE {
            pShaderBytecode: unsafe { vertex_shader_bytecode.GetBufferPointer() },
            BytecodeLength: unsafe { vertex_shader_bytecode.GetBufferSize() },
        }, // Vertex Shader
        PS: D3D12_SHADER_BYTECODE {
            pShaderBytecode: unsafe { pixel_shader_bytecode.GetBufferPointer() },
            BytecodeLength: unsafe { pixel_shader_bytecode.GetBufferSize() },
        }, // Pixel Shader
        DS: D3D12_SHADER_BYTECODE::default(),                    // Domain Shader
        HS: D3D12_SHADER_BYTECODE::default(),                    // Hull Shader
        GS: D3D12_SHADER_BYTECODE::default(),                    // Geometry Shader
        StreamOutput: D3D12_STREAM_OUTPUT_DESC::default(),
        BlendState: get_default_blend_state(),
        RasterizerState: D3D12_RASTERIZER_DESC {
            FillMode: D3D12_FILL_MODE_SOLID,
            CullMode: D3D12_CULL_MODE_BACK,
            FrontCounterClockwise: false.into(),
            DepthBias: D3D12_DEFAULT_DEPTH_BIAS,
            DepthBiasClamp: D3D12_DEFAULT_DEPTH_BIAS_CLAMP,
            SlopeScaledDepthBias: D3D12_DEFAULT_SLOPE_SCALED_DEPTH_BIAS,
            DepthClipEnable: true.into(),
            MultisampleEnable: false.into(),
            AntialiasedLineEnable: false.into(),
            ForcedSampleCount: 0,
            ConservativeRaster: D3D12_CONSERVATIVE_RASTERIZATION_MODE_OFF,
        },
        InputLayout: D3D12_INPUT_LAYOUT_DESC {
            pInputElementDescs: [D3D12_INPUT_ELEMENT_DESC {
                SemanticName: s!("POSITION"),
                SemanticIndex: 0,
                Format: DXGI_FORMAT_R32G32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: 0,
                InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            }]
            .as_ptr(),
            NumElements: 1,
        },
        SampleMask: u32::MAX,
        NumRenderTargets: 1,
        RTVFormats: [
            DXGI_FORMAT_R8G8B8A8_UNORM,
            DXGI_FORMAT_UNKNOWN,
            DXGI_FORMAT_UNKNOWN,
            DXGI_FORMAT_UNKNOWN,
            DXGI_FORMAT_UNKNOWN,
            DXGI_FORMAT_UNKNOWN,
            DXGI_FORMAT_UNKNOWN,
            DXGI_FORMAT_UNKNOWN,
        ],
        Flags: D3D12_PIPELINE_STATE_FLAG_NONE,
        PrimitiveTopologyType: D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE,
        DepthStencilState: D3D12_DEPTH_STENCIL_DESC::default(),
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            Quality: 0,
        },
        ..Default::default()
    };
    match unsafe { device.CreateGraphicsPipelineState(&pipeline_state_description) } {
        Ok(pso) => Ok(pso),
        Err(e) => return Err(e.to_string()),
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

fn compile_from_content(
    include_bytes: &[u8],
    entry_point: &'static str,
    profile: &'static str,
) -> Result<ID3DBlob, String> {
    let mut ppcode: Option<ID3DBlob> = None;
    let mut pperrormsgs: Option<ID3DBlob> = None;
    let entry_point = windows_core::PCSTR::from_raw(entry_point.as_ptr());
    let profile = windows_core::PCSTR::from_raw(profile.as_ptr());
    let result = unsafe {
        D3DCompile(
            include_bytes.as_ptr() as *const std::ffi::c_void,
            include_bytes.len(),
            None,
            None,
            None,
            entry_point,
            profile,
            0,
            0,
            &mut ppcode,
            Some(&mut pperrormsgs),
        )
    };
    match result {
        Ok(_) => Ok(ppcode.unwrap()),
        Err(e) => Err(e.to_string()),
    }
}

const fn get_default_blend_state() -> D3D12_BLEND_DESC {
    D3D12_BLEND_DESC {
        AlphaToCoverageEnable: BOOL(0),
        IndependentBlendEnable: BOOL(0),
        RenderTarget: [get_default_render_target_blend(); 8],
    }
}

const fn get_default_render_target_blend() -> D3D12_RENDER_TARGET_BLEND_DESC {
    D3D12_RENDER_TARGET_BLEND_DESC {
        BlendEnable: BOOL(0),
        LogicOpEnable: BOOL(0),
        SrcBlend: D3D12_BLEND_ONE,
        DestBlend: D3D12_BLEND_ZERO,
        BlendOp: D3D12_BLEND_OP_ADD,
        SrcBlendAlpha: D3D12_BLEND_ONE,
        DestBlendAlpha: D3D12_BLEND_ZERO,
        BlendOpAlpha: D3D12_BLEND_OP_ADD,
        LogicOp: D3D12_LOGIC_OP_NOOP,
        RenderTargetWriteMask: D3D12_COLOR_WRITE_ENABLE_ALL.0 as u8,
    }
}
