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

use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::{Direct3D12::*, DirectWrite::*};
use windows::Win32::System::Com::*;
use windows::Win32::UI::HiDpi::GetDpiForWindow;
use windows_core::*;
use windows_implement::implement;

use crate::math::Vector2;
use crate::renderer::{Color, Rect, Renderer};

use super::TextFormat;

const GLYPH_METRIC_STEP_SIZE: usize = 128;
const USER_DEFAULT_SCREEN_DPI: u32 = 96;

// #[implement(IDWriteTextRenderer1)]
#[implement(IDWriteTextRenderer1)]
pub(super) struct Direct3D12TextRenderer<'a> {
    renderer: &'a super::Direct3D12Renderer,
    factory: IDWriteFactory,
    text_format: IDWriteTextFormat,
}

impl<'a> Direct3D12TextRenderer<'a> {
    pub fn create_for_renderer(renderer: &'a super::Direct3D12Renderer) -> Self {
        let factory: IDWriteFactory =
            unsafe { DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED).unwrap() };
        let text_format = unsafe {
            factory
                .CreateTextFormat(
                    w!("Segoe UI"),
                    None,
                    DWRITE_FONT_WEIGHT_REGULAR,
                    DWRITE_FONT_STYLE_NORMAL,
                    DWRITE_FONT_STRETCH_NORMAL,
                    14.0,
                    w!("en-us"),
                )
                .unwrap()
        };
        Self {
            renderer,
            factory,
            text_format,
        }
    }

    pub fn render_text(self, text: &String, format: &TextFormat, rect: &Rect<f32>) -> Result<()> {
        let windows_str = HSTRING::from(text);
        let text_layout = unsafe {
            self.factory
                .CreateTextLayout(&windows_str, &self.text_format, rect.width, rect.height)
                .unwrap()
        };
        let text_renderer: IDWriteTextRenderer1 = self.into();
        unsafe {
            text_layout
                .Draw(None, &text_renderer, rect.x, rect.y)
                .unwrap()
        };
        Ok(())
    }
}

impl<'a> IDWriteTextRenderer_Impl for Direct3D12TextRenderer_Impl<'a> {
    fn DrawGlyphRun(
        &self,
        clientdrawingcontext: *const core::ffi::c_void,
        baselineoriginx: f32,
        baselineoriginy: f32,
        measuringmode: DWRITE_MEASURING_MODE,
        glyphrun: *const DWRITE_GLYPH_RUN,
        glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION,
        clientdrawingeffect: Ref<IUnknown>,
    ) -> Result<()> {
        IDWriteTextRenderer1_Impl::DrawGlyphRun(
            self,
            clientdrawingcontext,
            baselineoriginx,
            baselineoriginy,
            DWRITE_GLYPH_ORIENTATION_ANGLE_0_DEGREES,
            measuringmode,
            glyphrun,
            glyphrundescription,
            clientdrawingeffect,
        )
    }

    fn DrawUnderline(
        &self,
        clientdrawingcontext: *const core::ffi::c_void,
        baselineoriginx: f32,
        baselineoriginy: f32,
        underline: *const DWRITE_UNDERLINE,
        clientdrawingeffect: Ref<IUnknown>,
    ) -> Result<()> {
        IDWriteTextRenderer1_Impl::DrawUnderline(
            self,
            clientdrawingcontext,
            baselineoriginx,
            baselineoriginy,
            DWRITE_GLYPH_ORIENTATION_ANGLE_0_DEGREES,
            underline,
            clientdrawingeffect,
        )
    }

    fn DrawStrikethrough(
        &self,
        clientdrawingcontext: *const core::ffi::c_void,
        baselineoriginx: f32,
        baselineoriginy: f32,
        strikethrough: *const DWRITE_STRIKETHROUGH,
        clientdrawingeffect: Ref<IUnknown>,
    ) -> Result<()> {
        IDWriteTextRenderer1_Impl::DrawStrikethrough(
            self,
            clientdrawingcontext,
            baselineoriginx,
            baselineoriginy,
            DWRITE_GLYPH_ORIENTATION_ANGLE_0_DEGREES,
            strikethrough,
            clientdrawingeffect,
        )
    }

    fn DrawInlineObject(
        &self,
        clientdrawingcontext: *const core::ffi::c_void,
        originx: f32,
        originy: f32,
        inlineobject: Ref<IDWriteInlineObject>,
        issideways: BOOL,
        isrighttoleft: BOOL,
        clientdrawingeffect: Ref<IUnknown>,
    ) -> Result<()> {
        IDWriteTextRenderer1_Impl::DrawInlineObject(
            self,
            clientdrawingcontext,
            originx,
            originy,
            DWRITE_GLYPH_ORIENTATION_ANGLE_0_DEGREES,
            inlineobject,
            issideways,
            isrighttoleft,
            clientdrawingeffect,
        )
    }
}

impl<'a> IDWriteTextRenderer1_Impl for Direct3D12TextRenderer_Impl<'a> {
    fn DrawGlyphRun(
        &self,
        _clientdrawingcontext: *const core::ffi::c_void,
        baselineoriginx: f32,
        baselineoriginy: f32,
        orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE,
        measuringmode: DWRITE_MEASURING_MODE,
        glyphrun: *const DWRITE_GLYPH_RUN,
        glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION,
        clientdrawingeffect: Ref<IUnknown>,
    ) -> Result<()> {
        if orientationangle != DWRITE_GLYPH_ORIENTATION_ANGLE_0_DEGREES {
            return Err(Error::from_hresult(E_NOTIMPL));
        }
        let glyphrun = unsafe { glyphrun.read() };
        let fontface = match glyphrun.fontFace.as_ref() {
            Some(f) => f,
            None => return Err(Error::from_hresult(E_POINTER)),
        };

        let mut glyphmetrics: [DWRITE_GLYPH_METRICS; GLYPH_METRIC_STEP_SIZE] =
            [Default::default(); GLYPH_METRIC_STEP_SIZE];
        let glyph_count = glyphrun.glyphCount;
        let mut offset_x = baselineoriginx;
        let offset_y = baselineoriginy;
        while glyph_count > 0 {
            let step_glyph_count = GLYPH_METRIC_STEP_SIZE.min(glyph_count as usize);
            unsafe {
                fontface.GetDesignGlyphMetrics(
                    glyphrun.glyphIndices,
                    step_glyph_count as u32,
                    glyphmetrics.as_mut_ptr(),
                    glyphrun.isSideways.as_bool(),
                )?;
            }

            for metric in glyphmetrics[0..step_glyph_count].iter() {
                let rect = Rect::<f32> {
                    x: offset_x + metric.leftSideBearing as f32,
                    y: offset_y + metric.verticalOriginY as f32 + metric.topSideBearing as f32,
                    width: (metric.advanceWidth as i32
                        - metric.leftSideBearing
                        - metric.rightSideBearing) as f32,
                    height: (metric.advanceHeight as i32
                        - metric.topSideBearing
                        - metric.bottomSideBearing) as f32,
                };
                let color = Color {
                    r: 127.0,
                    g: 127.0,
                    b: 127.0,
                };
                self.renderer.draw_rectangle(&rect, &color);
                offset_x += metric.advanceWidth as f32;
            }
        }

        Ok(())
    }

    fn DrawUnderline(
        &self,
        _clientdrawingcontext: *const core::ffi::c_void,
        baselineoriginx: f32,
        baselineoriginy: f32,
        orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE,
        underline: *const DWRITE_UNDERLINE,
        clientdrawingeffect: Ref<IUnknown>,
    ) -> Result<()> {
        todo!()
    }

    fn DrawStrikethrough(
        &self,
        _clientdrawingcontext: *const core::ffi::c_void,
        baselineoriginx: f32,
        baselineoriginy: f32,
        orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE,
        strikethrough: *const DWRITE_STRIKETHROUGH,
        clientdrawingeffect: Ref<IUnknown>,
    ) -> Result<()> {
        todo!()
    }

    fn DrawInlineObject(
        &self,
        _clientdrawingcontext: *const core::ffi::c_void,
        originx: f32,
        originy: f32,
        orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE,
        inlineobject: Ref<IDWriteInlineObject>,
        issideways: BOOL,
        isrighttoleft: BOOL,
        clientdrawingeffect: Ref<IUnknown>,
    ) -> Result<()> {
        todo!()
    }
}

impl<'a> IDWritePixelSnapping_Impl for Direct3D12TextRenderer_Impl<'a> {
    fn IsPixelSnappingDisabled(
        &self,
        _clientdrawingcontext: *const core::ffi::c_void,
    ) -> Result<BOOL> {
        Ok(TRUE)
    }

    fn GetCurrentTransform(
        &self,
        _clientdrawingcontext: *const core::ffi::c_void,
        transform: *mut DWRITE_MATRIX,
    ) -> Result<()> {
        let mut transform = unsafe { transform.as_mut() };
        let transform = match transform {
            Some(t) => t,
            None => return Err(Error::from_hresult(E_POINTER)),
        };
        transform.dx = 0.0;
        transform.dy = 0.0;
        transform.m11 = 1.0;
        transform.m12 = 0.0;
        transform.m21 = 0.0;
        transform.m22 = 1.0;
        Ok(())
    }

    fn GetPixelsPerDip(&self, _clientdrawingcontext: *const core::ffi::c_void) -> Result<f32> {
        // ref: https://learn.microsoft.com/en-us/windows/win32/learnwin32/dpi-and-device-independent-pixels
        let hwnd = unsafe { self.renderer.swap_chain.GetHwnd()? };
        let dpi = unsafe { GetDpiForWindow(hwnd) };
        Ok(dpi as f32 / USER_DEFAULT_SCREEN_DPI as f32)
    }
}

// impl<'a> IUnknownImpl for Direct3D12TextRenderer_Impl<'a> {
//     type Impl = Direct3D12TextRenderer_Impl<'a>;

//     fn get_impl(&self) -> &Self::Impl {
//         todo!()
//     }

//     fn get_impl_mut(&mut self) -> &mut Self::Impl {
//         todo!()
//     }

//     fn into_inner(self) -> Self::Impl {
//         todo!()
//     }

//     unsafe fn QueryInterface(&self, iid: *const GUID, interface: *mut *mut std::ffi::c_void) -> HRESULT {
//         todo!()
//     }

//     fn AddRef(&self) -> u32 {
//         todo!()
//     }

//     unsafe fn Release(self_: *mut Self) -> u32 {
//         todo!()
//     }

//     fn is_reference_count_one(&self) -> bool {
//         todo!()
//     }

//     unsafe fn GetTrustLevel(&self, value: *mut i32) -> HRESULT {
//         todo!()
//     }

//     unsafe fn from_inner_ref(inner: &Self::Impl) -> &Self {
//         todo!()
//     }

//     fn to_object(&self) -> ComObject<Self::Impl>
//     where
//         Self::Impl: ComObjectInner<Outer = Self> {
//         todo!()
//     }

//     const INNER_OFFSET_IN_POINTERS: usize = 0usize;
// }
