/// D3D12 hardware rendering backend.
/// Offscreen RT → readback heap → CPU pixel buffer → RetroArch.
/// No swap chain — RetroArch owns the window.
/// Required for Xbox UWP (D3D11 not available in dev mode).

#[cfg(windows)]
use windows::{
    core::Interface, Win32::Foundation::*, Win32::Graphics::Direct3D::*,
    Win32::Graphics::Direct3D12::*, Win32::Graphics::Dxgi::Common::*, Win32::Graphics::Dxgi::*,
    Win32::System::Threading::*,
};

use super::shaders;
use super::{GpuBackend, NV2AVertex};

/// Wrapper for raw pointers (Send required by GpuBackend trait).
struct MappedPtr(*mut u8);
unsafe impl Send for MappedPtr {}

/// Wrapper for HANDLE (Send required by GpuBackend trait).
struct SendHandle(HANDLE);
unsafe impl Send for SendHandle {}

/// Map NV097 primitive type → D3D12 topology
fn map_prim_type(nv097: i32) -> D3D_PRIMITIVE_TOPOLOGY {
    match nv097 {
        1 => D3D_PRIMITIVE_TOPOLOGY_POINTLIST,
        2 => D3D_PRIMITIVE_TOPOLOGY_LINELIST,
        3 | 4 => D3D_PRIMITIVE_TOPOLOGY_LINESTRIP,
        5 => D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST,
        6 | 7 => D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP,
        _ => D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST,
    }
}

const MAX_VERTS: usize = 4096;
const VERTEX_SIZE: usize = std::mem::size_of::<NV2AVertex>(); // 28 bytes

pub struct D3D12Backend {
    width: i32,
    height: i32,
    aligned_row_pitch: u32,
    #[cfg(windows)]
    device: Option<ID3D12Device>,
    #[cfg(windows)]
    command_queue: Option<ID3D12CommandQueue>,
    #[cfg(windows)]
    command_allocator: Option<ID3D12CommandAllocator>,
    #[cfg(windows)]
    command_list: Option<ID3D12GraphicsCommandList>,
    #[cfg(windows)]
    fence: Option<ID3D12Fence>,
    fence_value: u64,
    fence_event: SendHandle,
    #[cfg(windows)]
    rt_resource: Option<ID3D12Resource>,
    #[cfg(windows)]
    ds_resource: Option<ID3D12Resource>,
    #[cfg(windows)]
    readback_resource: Option<ID3D12Resource>,
    #[cfg(windows)]
    upload_vb: Option<ID3D12Resource>,
    upload_vb_ptr: MappedPtr,
    #[cfg(windows)]
    rtv_heap: Option<ID3D12DescriptorHeap>,
    #[cfg(windows)]
    dsv_heap: Option<ID3D12DescriptorHeap>,
    #[cfg(windows)]
    root_signature: Option<ID3D12RootSignature>,
    #[cfg(windows)]
    pso: Option<ID3D12PipelineState>,
    readback_buf: Vec<u32>,
}

impl D3D12Backend {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            aligned_row_pitch: 0,
            #[cfg(windows)]
            device: None,
            #[cfg(windows)]
            command_queue: None,
            #[cfg(windows)]
            command_allocator: None,
            #[cfg(windows)]
            command_list: None,
            #[cfg(windows)]
            fence: None,
            fence_value: 1,
            #[cfg(windows)]
            fence_event: SendHandle(HANDLE(std::ptr::null_mut())),
            #[cfg(windows)]
            rt_resource: None,
            #[cfg(windows)]
            ds_resource: None,
            #[cfg(windows)]
            readback_resource: None,
            #[cfg(windows)]
            upload_vb: None,
            upload_vb_ptr: MappedPtr(std::ptr::null_mut()),
            #[cfg(windows)]
            rtv_heap: None,
            #[cfg(windows)]
            dsv_heap: None,
            #[cfg(windows)]
            root_signature: None,
            #[cfg(windows)]
            pso: None,
            readback_buf: Vec::new(),
        }
    }

    #[cfg(windows)]
    fn signal_and_wait(&mut self) {
        unsafe {
            let queue = match &self.command_queue {
                Some(q) => q,
                None => return,
            };
            let fence = match &self.fence {
                Some(f) => f,
                None => return,
            };
            let val = self.fence_value;
            let _ = queue.Signal(fence, val);
            if fence.GetCompletedValue() < val {
                let _ = fence.SetEventOnCompletion(val, self.fence_event.0);
                let _ = WaitForSingleObject(self.fence_event.0, 5000);
            }
            self.fence_value += 1;
        }
    }

    #[cfg(windows)]
    fn barrier(
        list: &ID3D12GraphicsCommandList,
        resource: &ID3D12Resource,
        before: D3D12_RESOURCE_STATES,
        after: D3D12_RESOURCE_STATES,
    ) {
        let barrier = D3D12_RESOURCE_BARRIER {
            Type: D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
            Flags: D3D12_RESOURCE_BARRIER_FLAG_NONE,
            Anonymous: D3D12_RESOURCE_BARRIER_0 {
                Transition: std::mem::ManuallyDrop::new(D3D12_RESOURCE_TRANSITION_BARRIER {
                    pResource: unsafe { std::mem::transmute_copy(resource) },
                    StateBefore: before,
                    StateAfter: after,
                    Subresource: D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES,
                }),
            },
        };
        unsafe {
            list.ResourceBarrier(&[barrier]);
        }
    }
}

impl GpuBackend for D3D12Backend {
    #[cfg(windows)]
    fn init(&mut self, width: i32, height: i32) -> bool {
        self.width = width;
        self.height = height;
        self.aligned_row_pitch = ((width as u32 * 4) + 255) & !255;
        self.readback_buf = vec![0u32; (width * height) as usize];

        unsafe {
            // --- DXGI Factory + Adapter ---
            let factory: IDXGIFactory4 = match CreateDXGIFactory1() {
                Ok(f) => f,
                Err(e) => {
                    crate::xbox::emulator::debug_log(&format!(
                        "[D3D12] CreateDXGIFactory1 FAILED: {}",
                        e
                    ));
                    return false;
                }
            };

            let adapter: IDXGIAdapter1 = match factory.EnumAdapters1(0) {
                Ok(a) => a,
                Err(e) => {
                    crate::xbox::emulator::debug_log(&format!(
                        "[D3D12] EnumAdapters1 FAILED: {}",
                        e
                    ));
                    return false;
                }
            };

            // --- Device ---
            let mut device: Option<ID3D12Device> = None;
            if let Err(e) = D3D12CreateDevice(&adapter, D3D_FEATURE_LEVEL_11_0, &mut device) {
                crate::xbox::emulator::debug_log(&format!("[D3D12] CreateDevice FAILED: {}", e));
                return false;
            }
            let device = device.unwrap();

            crate::xbox::emulator::debug_log("[D3D12] Device created (feature level 11_0)");

            // --- Command Queue ---
            let queue_desc = D3D12_COMMAND_QUEUE_DESC {
                Type: D3D12_COMMAND_LIST_TYPE_DIRECT,
                ..Default::default()
            };
            let command_queue: ID3D12CommandQueue = match device.CreateCommandQueue(&queue_desc) {
                Ok(q) => q,
                Err(e) => {
                    crate::xbox::emulator::debug_log(&format!(
                        "[D3D12] CreateCommandQueue FAILED: {}",
                        e
                    ));
                    return false;
                }
            };

            // --- Command Allocator + List ---
            let allocator: ID3D12CommandAllocator =
                match device.CreateCommandAllocator(D3D12_COMMAND_LIST_TYPE_DIRECT) {
                    Ok(a) => a,
                    Err(e) => {
                        crate::xbox::emulator::debug_log(&format!(
                            "[D3D12] CreateCommandAllocator FAILED: {}",
                            e
                        ));
                        return false;
                    }
                };

            let command_list: ID3D12GraphicsCommandList =
                match device.CreateCommandList(0, D3D12_COMMAND_LIST_TYPE_DIRECT, &allocator, None)
                {
                    Ok(l) => l,
                    Err(e) => {
                        crate::xbox::emulator::debug_log(&format!(
                            "[D3D12] CreateCommandList FAILED: {}",
                            e
                        ));
                        return false;
                    }
                };
            let _ = command_list.Close(); // start closed

            // --- Fence + Event ---
            let fence: ID3D12Fence = match device.CreateFence(0, D3D12_FENCE_FLAG_NONE) {
                Ok(f) => f,
                Err(e) => {
                    crate::xbox::emulator::debug_log(&format!("[D3D12] CreateFence FAILED: {}", e));
                    return false;
                }
            };
            let fence_event = CreateEventW(None, false, false, None).unwrap_or_default();

            // --- RTV Descriptor Heap ---
            let rtv_heap: ID3D12DescriptorHeap =
                match device.CreateDescriptorHeap(&D3D12_DESCRIPTOR_HEAP_DESC {
                    Type: D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
                    NumDescriptors: 1,
                    ..Default::default()
                }) {
                    Ok(h) => h,
                    Err(e) => {
                        crate::xbox::emulator::debug_log(&format!(
                            "[D3D12] RTV heap FAILED: {}",
                            e
                        ));
                        return false;
                    }
                };

            // --- DSV Descriptor Heap ---
            let dsv_heap: ID3D12DescriptorHeap =
                match device.CreateDescriptorHeap(&D3D12_DESCRIPTOR_HEAP_DESC {
                    Type: D3D12_DESCRIPTOR_HEAP_TYPE_DSV,
                    NumDescriptors: 1,
                    ..Default::default()
                }) {
                    Ok(h) => h,
                    Err(e) => {
                        crate::xbox::emulator::debug_log(&format!(
                            "[D3D12] DSV heap FAILED: {}",
                            e
                        ));
                        return false;
                    }
                };

            // --- Render Target ---
            let rt_desc = D3D12_RESOURCE_DESC {
                Dimension: D3D12_RESOURCE_DIMENSION_TEXTURE2D,
                Width: width as u64,
                Height: height as u32,
                DepthOrArraySize: 1,
                MipLevels: 1,
                Format: DXGI_FORMAT_B8G8R8A8_UNORM,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                Layout: D3D12_TEXTURE_LAYOUT_UNKNOWN,
                Flags: D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET,
                ..Default::default()
            };
            let rt_clear = D3D12_CLEAR_VALUE {
                Format: DXGI_FORMAT_B8G8R8A8_UNORM,
                Anonymous: D3D12_CLEAR_VALUE_0 {
                    Color: [0.0, 0.0, 0.0, 1.0],
                },
            };
            let heap_props_default = D3D12_HEAP_PROPERTIES {
                Type: D3D12_HEAP_TYPE_DEFAULT,
                ..Default::default()
            };
            let mut rt_resource: Option<ID3D12Resource> = None;
            if let Err(e) = device.CreateCommittedResource(
                &heap_props_default,
                D3D12_HEAP_FLAG_NONE,
                &rt_desc,
                D3D12_RESOURCE_STATE_COMMON,
                Some(&rt_clear),
                &mut rt_resource,
            ) {
                crate::xbox::emulator::debug_log(&format!("[D3D12] RT resource FAILED: {}", e));
                return false;
            }
            let rt_resource = rt_resource.unwrap();

            // Create RTV
            let rtv_handle = rtv_heap.GetCPUDescriptorHandleForHeapStart();
            device.CreateRenderTargetView(&rt_resource, None, rtv_handle);

            // --- Depth/Stencil ---
            let ds_desc = D3D12_RESOURCE_DESC {
                Dimension: D3D12_RESOURCE_DIMENSION_TEXTURE2D,
                Width: width as u64,
                Height: height as u32,
                DepthOrArraySize: 1,
                MipLevels: 1,
                Format: DXGI_FORMAT_D24_UNORM_S8_UINT,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                Layout: D3D12_TEXTURE_LAYOUT_UNKNOWN,
                Flags: D3D12_RESOURCE_FLAG_ALLOW_DEPTH_STENCIL,
                ..Default::default()
            };
            let ds_clear = D3D12_CLEAR_VALUE {
                Format: DXGI_FORMAT_D24_UNORM_S8_UINT,
                Anonymous: D3D12_CLEAR_VALUE_0 {
                    DepthStencil: D3D12_DEPTH_STENCIL_VALUE {
                        Depth: 1.0,
                        Stencil: 0,
                    },
                },
            };
            let mut ds_resource: Option<ID3D12Resource> = None;
            if let Err(e) = device.CreateCommittedResource(
                &heap_props_default,
                D3D12_HEAP_FLAG_NONE,
                &ds_desc,
                D3D12_RESOURCE_STATE_DEPTH_WRITE,
                Some(&ds_clear),
                &mut ds_resource,
            ) {
                crate::xbox::emulator::debug_log(&format!("[D3D12] DS resource FAILED: {}", e));
                return false;
            }
            let ds_resource = ds_resource.unwrap();

            // Create DSV
            let dsv_handle = dsv_heap.GetCPUDescriptorHandleForHeapStart();
            device.CreateDepthStencilView(&ds_resource, None, dsv_handle);

            // --- Readback Buffer ---
            let readback_size = self.aligned_row_pitch as u64 * height as u64;
            let heap_props_readback = D3D12_HEAP_PROPERTIES {
                Type: D3D12_HEAP_TYPE_READBACK,
                ..Default::default()
            };
            let readback_desc = D3D12_RESOURCE_DESC {
                Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
                Width: readback_size,
                Height: 1,
                DepthOrArraySize: 1,
                MipLevels: 1,
                Format: DXGI_FORMAT_UNKNOWN,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
                ..Default::default()
            };
            let mut readback_resource: Option<ID3D12Resource> = None;
            if let Err(e) = device.CreateCommittedResource(
                &heap_props_readback,
                D3D12_HEAP_FLAG_NONE,
                &readback_desc,
                D3D12_RESOURCE_STATE_COPY_DEST,
                None,
                &mut readback_resource,
            ) {
                crate::xbox::emulator::debug_log(&format!("[D3D12] Readback buffer FAILED: {}", e));
                return false;
            }
            let readback_resource = readback_resource.unwrap();

            // --- Upload Vertex Buffer (persistently mapped) ---
            let vb_size = (MAX_VERTS * VERTEX_SIZE) as u64;
            let heap_props_upload = D3D12_HEAP_PROPERTIES {
                Type: D3D12_HEAP_TYPE_UPLOAD,
                ..Default::default()
            };
            let vb_desc = D3D12_RESOURCE_DESC {
                Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
                Width: vb_size,
                Height: 1,
                DepthOrArraySize: 1,
                MipLevels: 1,
                Format: DXGI_FORMAT_UNKNOWN,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
                ..Default::default()
            };
            let mut upload_vb: Option<ID3D12Resource> = None;
            if let Err(e) = device.CreateCommittedResource(
                &heap_props_upload,
                D3D12_HEAP_FLAG_NONE,
                &vb_desc,
                D3D12_RESOURCE_STATE_GENERIC_READ,
                None,
                &mut upload_vb,
            ) {
                crate::xbox::emulator::debug_log(&format!("[D3D12] Upload VB FAILED: {}", e));
                return false;
            }
            let upload_vb = upload_vb.unwrap();

            // Persistent map
            let mut vb_ptr: *mut std::ffi::c_void = std::ptr::null_mut();
            if let Err(e) = upload_vb.Map(0, None, Some(&mut vb_ptr)) {
                crate::xbox::emulator::debug_log(&format!("[D3D12] VB Map FAILED: {}", e));
                return false;
            }

            // --- Root Signature (empty — no CBVs/SRVs needed) ---
            let rs_desc = D3D12_ROOT_SIGNATURE_DESC {
                Flags: D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
                ..Default::default()
            };
            let mut rs_blob: Option<ID3DBlob> = None;
            let mut rs_error: Option<ID3DBlob> = None;
            if D3D12SerializeRootSignature(
                &rs_desc,
                D3D_ROOT_SIGNATURE_VERSION_1,
                &mut rs_blob,
                Some(&mut rs_error),
            )
            .is_err()
            {
                crate::xbox::emulator::debug_log("[D3D12] SerializeRootSignature FAILED");
                return false;
            }
            let rs_blob = rs_blob.unwrap();
            let root_signature: ID3D12RootSignature = match device.CreateRootSignature(
                0,
                std::slice::from_raw_parts(
                    rs_blob.GetBufferPointer() as *const u8,
                    rs_blob.GetBufferSize(),
                ),
            ) {
                Ok(rs) => rs,
                Err(e) => {
                    crate::xbox::emulator::debug_log(&format!(
                        "[D3D12] CreateRootSignature FAILED: {}",
                        e
                    ));
                    return false;
                }
            };

            // --- Pipeline State Object ---
            let input_elements = [
                D3D12_INPUT_ELEMENT_DESC {
                    SemanticName: windows::core::s!("POSITION"),
                    SemanticIndex: 0,
                    Format: DXGI_FORMAT_R32G32B32A32_FLOAT,
                    InputSlot: 0,
                    AlignedByteOffset: 0,
                    InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
                    InstanceDataStepRate: 0,
                },
                D3D12_INPUT_ELEMENT_DESC {
                    SemanticName: windows::core::s!("COLOR"),
                    SemanticIndex: 0,
                    Format: DXGI_FORMAT_R32_UINT,
                    InputSlot: 0,
                    AlignedByteOffset: 16,
                    InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
                    InstanceDataStepRate: 0,
                },
                D3D12_INPUT_ELEMENT_DESC {
                    SemanticName: windows::core::s!("TEXCOORD"),
                    SemanticIndex: 0,
                    Format: DXGI_FORMAT_R32G32_FLOAT,
                    InputSlot: 0,
                    AlignedByteOffset: 20,
                    InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
                    InstanceDataStepRate: 0,
                },
            ];

            let mut pso_desc = D3D12_GRAPHICS_PIPELINE_STATE_DESC {
                pRootSignature: std::mem::transmute_copy(&root_signature),
                VS: D3D12_SHADER_BYTECODE {
                    pShaderBytecode: shaders::VS_PASSTHROUGH.as_ptr() as *const _,
                    BytecodeLength: shaders::VS_PASSTHROUGH.len(),
                },
                PS: D3D12_SHADER_BYTECODE {
                    pShaderBytecode: shaders::PS_PASSTHROUGH.as_ptr() as *const _,
                    BytecodeLength: shaders::PS_PASSTHROUGH.len(),
                },
                InputLayout: D3D12_INPUT_LAYOUT_DESC {
                    pInputElementDescs: input_elements.as_ptr(),
                    NumElements: input_elements.len() as u32,
                },
                PrimitiveTopologyType: D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE,
                NumRenderTargets: 1,
                DSVFormat: DXGI_FORMAT_D24_UNORM_S8_UINT,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                SampleMask: 0xFFFF_FFFF,
                ..Default::default()
            };
            pso_desc.RTVFormats[0] = DXGI_FORMAT_B8G8R8A8_UNORM;

            // Rasterizer: solid, no culling
            pso_desc.RasterizerState = D3D12_RASTERIZER_DESC {
                FillMode: D3D12_FILL_MODE_SOLID,
                CullMode: D3D12_CULL_MODE_NONE,
                DepthClipEnable: true.into(),
                ..Default::default()
            };

            // Depth stencil: enabled, less-equal
            pso_desc.DepthStencilState = D3D12_DEPTH_STENCIL_DESC {
                DepthEnable: true.into(),
                DepthWriteMask: D3D12_DEPTH_WRITE_MASK_ALL,
                DepthFunc: D3D12_COMPARISON_FUNC_LESS_EQUAL,
                ..Default::default()
            };

            // Blend: alpha blending
            pso_desc.BlendState.RenderTarget[0] = D3D12_RENDER_TARGET_BLEND_DESC {
                BlendEnable: true.into(),
                SrcBlend: D3D12_BLEND_SRC_ALPHA,
                DestBlend: D3D12_BLEND_INV_SRC_ALPHA,
                BlendOp: D3D12_BLEND_OP_ADD,
                SrcBlendAlpha: D3D12_BLEND_ONE,
                DestBlendAlpha: D3D12_BLEND_ZERO,
                BlendOpAlpha: D3D12_BLEND_OP_ADD,
                LogicOp: D3D12_LOGIC_OP_NOOP,
                RenderTargetWriteMask: 0x0F, // all channels
                LogicOpEnable: false.into(),
            };

            let pso: ID3D12PipelineState = match device.CreateGraphicsPipelineState(&pso_desc) {
                Ok(p) => p,
                Err(e) => {
                    crate::xbox::emulator::debug_log(&format!(
                        "[D3D12] CreatePipelineState FAILED: {}",
                        e
                    ));
                    return false;
                }
            };

            // Store all state
            self.device = Some(device);
            self.command_queue = Some(command_queue);
            self.command_allocator = Some(allocator);
            self.command_list = Some(command_list);
            self.fence = Some(fence);
            self.fence_event = SendHandle(fence_event);
            self.rt_resource = Some(rt_resource);
            self.ds_resource = Some(ds_resource);
            self.readback_resource = Some(readback_resource);
            self.upload_vb = Some(upload_vb);
            self.upload_vb_ptr = MappedPtr(vb_ptr as *mut u8);
            self.rtv_heap = Some(rtv_heap);
            self.dsv_heap = Some(dsv_heap);
            self.root_signature = Some(root_signature);
            self.pso = Some(pso);
        }

        crate::xbox::emulator::debug_log(&format!(
            "[D3D12] Initialized ({}x{}), RT=B8G8R8A8, readback_pitch={}",
            width, height, self.aligned_row_pitch
        ));
        true
    }

    #[cfg(not(windows))]
    fn init(&mut self, _width: i32, _height: i32) -> bool {
        false
    }

    fn shutdown(&mut self) {
        crate::xbox::emulator::debug_log("[D3D12] Shutdown");
        #[cfg(windows)]
        {
            // Wait for GPU idle before releasing resources
            self.signal_and_wait();
            if !self.fence_event.0.is_invalid() {
                let _ = unsafe { CloseHandle(self.fence_event.0) };
                self.fence_event = SendHandle(HANDLE(std::ptr::null_mut()));
            }
            self.pso = None;
            self.root_signature = None;
            self.upload_vb = None;
            self.upload_vb_ptr = MappedPtr(std::ptr::null_mut());
            self.readback_resource = None;
            self.dsv_heap = None;
            self.rtv_heap = None;
            self.ds_resource = None;
            self.rt_resource = None;
            self.command_list = None;
            self.command_allocator = None;
            self.fence = None;
            self.command_queue = None;
            self.device = None;
        }
    }

    #[cfg(windows)]
    fn draw_primitive(&mut self, verts: &[NV2AVertex], prim_type: i32) {
        let count = verts.len().min(MAX_VERTS);
        if count == 0 {
            return;
        }

        let allocator = match &self.command_allocator {
            Some(a) => a,
            None => return,
        };
        let list = match &self.command_list {
            Some(l) => l,
            None => return,
        };
        let pso = match &self.pso {
            Some(p) => p,
            None => return,
        };
        let rt = match &self.rt_resource {
            Some(r) => r,
            None => return,
        };
        let upload_vb = match &self.upload_vb {
            Some(v) => v,
            None => return,
        };
        let root_sig = match &self.root_signature {
            Some(r) => r,
            None => return,
        };

        unsafe {
            // Copy vertices to upload heap
            std::ptr::copy_nonoverlapping(
                verts.as_ptr() as *const u8,
                self.upload_vb_ptr.0,
                count * VERTEX_SIZE,
            );

            // Reset + record commands
            let _ = allocator.Reset();
            let _ = list.Reset(allocator, pso);

            // Barrier: COMMON → RENDER_TARGET
            Self::barrier(
                list,
                rt,
                D3D12_RESOURCE_STATE_COMMON,
                D3D12_RESOURCE_STATE_RENDER_TARGET,
            );

            // Set pipeline state
            list.SetGraphicsRootSignature(root_sig);

            let viewport = D3D12_VIEWPORT {
                TopLeftX: 0.0,
                TopLeftY: 0.0,
                Width: self.width as f32,
                Height: self.height as f32,
                MinDepth: 0.0,
                MaxDepth: 1.0,
            };
            list.RSSetViewports(&[viewport]);

            let scissor = RECT {
                left: 0,
                top: 0,
                right: self.width,
                bottom: self.height,
            };
            list.RSSetScissorRects(&[scissor]);

            // Set render targets
            let rtv_handle = self
                .rtv_heap
                .as_ref()
                .unwrap()
                .GetCPUDescriptorHandleForHeapStart();
            let dsv_handle = self
                .dsv_heap
                .as_ref()
                .unwrap()
                .GetCPUDescriptorHandleForHeapStart();
            list.OMSetRenderTargets(1, Some(&rtv_handle), false, Some(&dsv_handle));

            // Set vertex buffer
            let vbv = D3D12_VERTEX_BUFFER_VIEW {
                BufferLocation: upload_vb.GetGPUVirtualAddress(),
                SizeInBytes: (count * VERTEX_SIZE) as u32,
                StrideInBytes: VERTEX_SIZE as u32,
            };
            list.IASetVertexBuffers(0, Some(&[vbv]));
            list.IASetPrimitiveTopology(map_prim_type(prim_type));

            // Draw
            list.DrawInstanced(count as u32, 1, 0, 0);

            // Barrier: RENDER_TARGET → COMMON
            Self::barrier(
                list,
                rt,
                D3D12_RESOURCE_STATE_RENDER_TARGET,
                D3D12_RESOURCE_STATE_COMMON,
            );

            // Execute
            let _ = list.Close();
            let lists: [Option<ID3D12CommandList>; 1] = [Some(list.cast().unwrap())];
            self.command_queue
                .as_ref()
                .unwrap()
                .ExecuteCommandLists(&lists);
            self.signal_and_wait();
        }
    }

    #[cfg(not(windows))]
    fn draw_primitive(&mut self, _verts: &[NV2AVertex], _prim_type: i32) {}

    #[cfg(windows)]
    fn clear(&mut self, color: u32) {
        let allocator = match &self.command_allocator {
            Some(a) => a,
            None => return,
        };
        let list = match &self.command_list {
            Some(l) => l,
            None => return,
        };
        let pso = match &self.pso {
            Some(p) => p,
            None => return,
        };
        let rt = match &self.rt_resource {
            Some(r) => r,
            None => return,
        };

        let rgba: [f32; 4] = [
            ((color >> 16) & 0xFF) as f32 / 255.0,
            ((color >> 8) & 0xFF) as f32 / 255.0,
            (color & 0xFF) as f32 / 255.0,
            ((color >> 24) & 0xFF) as f32 / 255.0,
        ];

        unsafe {
            let _ = allocator.Reset();
            let _ = list.Reset(allocator, pso);

            Self::barrier(
                list,
                rt,
                D3D12_RESOURCE_STATE_COMMON,
                D3D12_RESOURCE_STATE_RENDER_TARGET,
            );

            let rtv_handle = self
                .rtv_heap
                .as_ref()
                .unwrap()
                .GetCPUDescriptorHandleForHeapStart();
            list.ClearRenderTargetView(rtv_handle, &rgba, None);

            let dsv_handle = self
                .dsv_heap
                .as_ref()
                .unwrap()
                .GetCPUDescriptorHandleForHeapStart();
            list.ClearDepthStencilView(
                dsv_handle,
                D3D12_CLEAR_FLAG_DEPTH | D3D12_CLEAR_FLAG_STENCIL,
                1.0,
                0,
                &[],
            );

            Self::barrier(
                list,
                rt,
                D3D12_RESOURCE_STATE_RENDER_TARGET,
                D3D12_RESOURCE_STATE_COMMON,
            );

            let _ = list.Close();
            let lists: [Option<ID3D12CommandList>; 1] = [Some(list.cast().unwrap())];
            self.command_queue
                .as_ref()
                .unwrap()
                .ExecuteCommandLists(&lists);
            self.signal_and_wait();
        }
    }

    #[cfg(not(windows))]
    fn clear(&mut self, _color: u32) {}

    #[cfg(windows)]
    fn readback_framebuffer(&mut self) -> &[u32] {
        let allocator = match &self.command_allocator {
            Some(a) => a.clone(),
            None => return &self.readback_buf,
        };
        let list = match &self.command_list {
            Some(l) => l.clone(),
            None => return &self.readback_buf,
        };
        let rt = match &self.rt_resource {
            Some(r) => r.clone(),
            None => return &self.readback_buf,
        };
        let rb = match &self.readback_resource {
            Some(r) => r.clone(),
            None => return &self.readback_buf,
        };

        unsafe {
            let _ = allocator.Reset();
            let _ = list.Reset(&allocator, self.pso.as_ref().unwrap());

            // Barrier: COMMON → COPY_SOURCE
            Self::barrier(
                &list,
                &rt,
                D3D12_RESOURCE_STATE_COMMON,
                D3D12_RESOURCE_STATE_COPY_SOURCE,
            );

            // CopyTextureRegion: RT texture → readback buffer
            let dst = D3D12_TEXTURE_COPY_LOCATION {
                pResource: std::mem::transmute_copy(&rb),
                Type: D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT,
                Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 {
                    PlacedFootprint: D3D12_PLACED_SUBRESOURCE_FOOTPRINT {
                        Offset: 0,
                        Footprint: D3D12_SUBRESOURCE_FOOTPRINT {
                            Format: DXGI_FORMAT_B8G8R8A8_UNORM,
                            Width: self.width as u32,
                            Height: self.height as u32,
                            Depth: 1,
                            RowPitch: self.aligned_row_pitch,
                        },
                    },
                },
            };
            let src = D3D12_TEXTURE_COPY_LOCATION {
                pResource: std::mem::transmute_copy(&rt),
                Type: D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX,
                Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 {
                    SubresourceIndex: 0,
                },
            };
            list.CopyTextureRegion(&dst, 0, 0, 0, &src, None);

            // Barrier: COPY_SOURCE → COMMON
            Self::barrier(
                &list,
                &rt,
                D3D12_RESOURCE_STATE_COPY_SOURCE,
                D3D12_RESOURCE_STATE_COMMON,
            );

            let _ = list.Close();
            let lists: [Option<ID3D12CommandList>; 1] = [Some(list.cast().unwrap())];
            self.command_queue
                .as_ref()
                .unwrap()
                .ExecuteCommandLists(&lists);
            self.signal_and_wait();

            // Map readback buffer and copy row-by-row
            let mut data_ptr: *mut std::ffi::c_void = std::ptr::null_mut();
            if rb.Map(0, None, Some(&mut data_ptr)).is_ok() {
                let src_bytes = data_ptr as *const u8;
                let dst_bytes = self.readback_buf.as_mut_ptr() as *mut u8;
                let row_bytes = self.width as usize * 4;
                let pitch = self.aligned_row_pitch as usize;
                for y in 0..self.height as usize {
                    std::ptr::copy_nonoverlapping(
                        src_bytes.add(y * pitch),
                        dst_bytes.add(y * row_bytes),
                        row_bytes,
                    );
                }
                rb.Unmap(0, None);
            }
        }

        &self.readback_buf
    }

    #[cfg(not(windows))]
    fn readback_framebuffer(&mut self) -> &[u32] {
        &self.readback_buf
    }

    fn name(&self) -> &'static str {
        "D3D12"
    }
}
