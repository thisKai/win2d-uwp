///*Required features: `"Graphics_Canvas_Text"`, `"implement"`*
pub trait ICanvasTextAnalyzerOptions_Impl: Sized {
    fn GetLocaleName(
        &self,
        characterindex: i32,
        charactercount: &mut i32,
    ) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetNumberSubstitution(
        &self,
        characterindex: i32,
        charactercount: &mut i32,
    ) -> ::windows::core::Result<CanvasNumberSubstitution>;
    fn GetVerticalGlyphOrientation(
        &self,
        characterindex: i32,
        charactercount: &mut i32,
    ) -> ::windows::core::Result<CanvasVerticalGlyphOrientation>;
    fn GetBidiLevel(
        &self,
        characterindex: i32,
        charactercount: &mut i32,
    ) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for ICanvasTextAnalyzerOptions {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Text.ICanvasTextAnalyzerOptions";
}
impl ICanvasTextAnalyzerOptions_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: ICanvasTextAnalyzerOptions_Impl,
        const OFFSET: isize,
    >() -> ICanvasTextAnalyzerOptions_Vtbl {
        unsafe extern "system" fn GetLocaleName<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasTextAnalyzerOptions_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            characterindex: i32,
            charactercount: *mut i32,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this
                .GetLocaleName(
                    characterindex,
                    ::core::mem::transmute_copy(&charactercount),
                )
            {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumberSubstitution<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasTextAnalyzerOptions_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            characterindex: i32,
            charactercount: *mut i32,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this
                .GetNumberSubstitution(
                    characterindex,
                    ::core::mem::transmute_copy(&charactercount),
                )
            {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVerticalGlyphOrientation<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasTextAnalyzerOptions_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            characterindex: i32,
            charactercount: *mut i32,
            result__: *mut CanvasVerticalGlyphOrientation,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this
                .GetVerticalGlyphOrientation(
                    characterindex,
                    ::core::mem::transmute_copy(&charactercount),
                )
            {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBidiLevel<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasTextAnalyzerOptions_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            characterindex: i32,
            charactercount: *mut i32,
            result__: *mut u32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this
                .GetBidiLevel(
                    characterindex,
                    ::core::mem::transmute_copy(&charactercount),
                )
            {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<
                Identity,
                ICanvasTextAnalyzerOptions,
                OFFSET,
            >(),
            GetLocaleName: GetLocaleName::<Identity, Impl, OFFSET>,
            GetNumberSubstitution: GetNumberSubstitution::<Identity, Impl, OFFSET>,
            GetVerticalGlyphOrientation: GetVerticalGlyphOrientation::<
                Identity,
                Impl,
                OFFSET,
            >,
            GetBidiLevel: GetBidiLevel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICanvasTextAnalyzerOptions as ::windows::core::Interface>::IID
    }
}
///*Required features: `"Graphics_Canvas_Text"`, `"Foundation_Numerics"`, `"implement"`*
#[cfg(feature = "Foundation_Numerics")]
pub trait ICanvasTextInlineObject_Impl: Sized {
    fn Draw(
        &self,
        textrenderer: &::core::option::Option<ICanvasTextRenderer>,
        point: &::windows::Foundation::Numerics::Vector2,
        issideways: bool,
        isrighttoleft: bool,
        brush: &::core::option::Option<::windows::core::IInspectable>,
    ) -> ::windows::core::Result<()>;
    fn Size(&self) -> ::windows::core::Result<::windows::Foundation::Size>;
    fn Baseline(&self) -> ::windows::core::Result<f32>;
    fn SupportsSideways(&self) -> ::windows::core::Result<bool>;
    fn DrawBounds(&self) -> ::windows::core::Result<::windows::Foundation::Rect>;
    fn BreakBefore(&self) -> ::windows::core::Result<CanvasLineBreakCondition>;
    fn BreakAfter(&self) -> ::windows::core::Result<CanvasLineBreakCondition>;
}
#[cfg(feature = "Foundation_Numerics")]
impl ::windows::core::RuntimeName for ICanvasTextInlineObject {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Text.ICanvasTextInlineObject";
}
#[cfg(feature = "Foundation_Numerics")]
impl ICanvasTextInlineObject_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: ICanvasTextInlineObject_Impl,
        const OFFSET: isize,
    >() -> ICanvasTextInlineObject_Vtbl {
        unsafe extern "system" fn Draw<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasTextInlineObject_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            textrenderer: *mut ::core::ffi::c_void,
            point: ::windows::Foundation::Numerics::Vector2,
            issideways: bool,
            isrighttoleft: bool,
            brush: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Draw(
                    ::core::mem::transmute(&textrenderer),
                    ::core::mem::transmute(&point),
                    issideways,
                    isrighttoleft,
                    ::core::mem::transmute(&brush),
                )
                .into()
        }
        unsafe extern "system" fn Size<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasTextInlineObject_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::Size,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Size() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Baseline<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasTextInlineObject_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut f32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Baseline() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportsSideways<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasTextInlineObject_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportsSideways() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DrawBounds<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasTextInlineObject_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::Rect,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DrawBounds() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BreakBefore<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasTextInlineObject_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut CanvasLineBreakCondition,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BreakBefore() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BreakAfter<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasTextInlineObject_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut CanvasLineBreakCondition,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BreakAfter() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<
                Identity,
                ICanvasTextInlineObject,
                OFFSET,
            >(),
            Draw: Draw::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            Baseline: Baseline::<Identity, Impl, OFFSET>,
            SupportsSideways: SupportsSideways::<Identity, Impl, OFFSET>,
            DrawBounds: DrawBounds::<Identity, Impl, OFFSET>,
            BreakBefore: BreakBefore::<Identity, Impl, OFFSET>,
            BreakAfter: BreakAfter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICanvasTextInlineObject as ::windows::core::Interface>::IID
    }
}
///*Required features: `"Graphics_Canvas_Text"`, `"Foundation_Numerics"`, `"implement"`*
#[cfg(feature = "Foundation_Numerics")]
pub trait ICanvasTextRenderer_Impl: Sized {
    fn DrawGlyphRun(
        &self,
        point: &::windows::Foundation::Numerics::Vector2,
        fontface: &::core::option::Option<CanvasFontFace>,
        fontsize: f32,
        glyphs: &[CanvasGlyph],
        issideways: bool,
        bidilevel: u32,
        brush: &::core::option::Option<::windows::core::IInspectable>,
        measuringmode: CanvasTextMeasuringMode,
        localename: &::windows::core::HSTRING,
        textstring: &::windows::core::HSTRING,
        clustermapindices: &[i32],
        characterindex: u32,
        glyphorientation: CanvasGlyphOrientation,
    ) -> ::windows::core::Result<()>;
    fn DrawStrikethrough(
        &self,
        point: &::windows::Foundation::Numerics::Vector2,
        strikethroughwidth: f32,
        strikethroughthickness: f32,
        strikethroughoffset: f32,
        textdirection: CanvasTextDirection,
        brush: &::core::option::Option<::windows::core::IInspectable>,
        textmeasuringmode: CanvasTextMeasuringMode,
        localename: &::windows::core::HSTRING,
        glyphorientation: CanvasGlyphOrientation,
    ) -> ::windows::core::Result<()>;
    fn DrawUnderline(
        &self,
        point: &::windows::Foundation::Numerics::Vector2,
        underlinewidth: f32,
        underlinethickness: f32,
        underlineoffset: f32,
        runheight: f32,
        textdirection: CanvasTextDirection,
        brush: &::core::option::Option<::windows::core::IInspectable>,
        textmeasuringmode: CanvasTextMeasuringMode,
        localename: &::windows::core::HSTRING,
        glyphorientation: CanvasGlyphOrientation,
    ) -> ::windows::core::Result<()>;
    fn DrawInlineObject(
        &self,
        point: &::windows::Foundation::Numerics::Vector2,
        inlineobject: &::core::option::Option<ICanvasTextInlineObject>,
        issideways: bool,
        isrighttoleft: bool,
        brush: &::core::option::Option<::windows::core::IInspectable>,
        glyphorientation: CanvasGlyphOrientation,
    ) -> ::windows::core::Result<()>;
    fn PixelSnappingDisabled(&self) -> ::windows::core::Result<bool>;
    fn Transform(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Matrix3x2>;
    fn Dpi(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "Foundation_Numerics")]
impl ::windows::core::RuntimeName for ICanvasTextRenderer {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Text.ICanvasTextRenderer";
}
#[cfg(feature = "Foundation_Numerics")]
impl ICanvasTextRenderer_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: ICanvasTextRenderer_Impl,
        const OFFSET: isize,
    >() -> ICanvasTextRenderer_Vtbl {
        unsafe extern "system" fn DrawGlyphRun<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasTextRenderer_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            point: ::windows::Foundation::Numerics::Vector2,
            fontface: *mut ::core::ffi::c_void,
            fontsize: f32,
            glyphs_array_size: u32,
            glyphs: *const CanvasGlyph,
            issideways: bool,
            bidilevel: u32,
            brush: *mut ::core::ffi::c_void,
            measuringmode: CanvasTextMeasuringMode,
            localename: *mut ::core::ffi::c_void,
            textstring: *mut ::core::ffi::c_void,
            clusterMapIndices_array_size: u32,
            clustermapindices: *const i32,
            characterindex: u32,
            glyphorientation: CanvasGlyphOrientation,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DrawGlyphRun(
                    ::core::mem::transmute(&point),
                    ::core::mem::transmute(&fontface),
                    fontsize,
                    ::core::slice::from_raw_parts(
                        ::core::mem::transmute_copy(&glyphs),
                        glyphs_array_size as _,
                    ),
                    issideways,
                    bidilevel,
                    ::core::mem::transmute(&brush),
                    measuringmode,
                    ::core::mem::transmute(&localename),
                    ::core::mem::transmute(&textstring),
                    ::core::slice::from_raw_parts(
                        ::core::mem::transmute_copy(&clustermapindices),
                        clusterMapIndices_array_size as _,
                    ),
                    characterindex,
                    glyphorientation,
                )
                .into()
        }
        unsafe extern "system" fn DrawStrikethrough<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasTextRenderer_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            point: ::windows::Foundation::Numerics::Vector2,
            strikethroughwidth: f32,
            strikethroughthickness: f32,
            strikethroughoffset: f32,
            textdirection: CanvasTextDirection,
            brush: *mut ::core::ffi::c_void,
            textmeasuringmode: CanvasTextMeasuringMode,
            localename: *mut ::core::ffi::c_void,
            glyphorientation: CanvasGlyphOrientation,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DrawStrikethrough(
                    ::core::mem::transmute(&point),
                    strikethroughwidth,
                    strikethroughthickness,
                    strikethroughoffset,
                    textdirection,
                    ::core::mem::transmute(&brush),
                    textmeasuringmode,
                    ::core::mem::transmute(&localename),
                    glyphorientation,
                )
                .into()
        }
        unsafe extern "system" fn DrawUnderline<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasTextRenderer_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            point: ::windows::Foundation::Numerics::Vector2,
            underlinewidth: f32,
            underlinethickness: f32,
            underlineoffset: f32,
            runheight: f32,
            textdirection: CanvasTextDirection,
            brush: *mut ::core::ffi::c_void,
            textmeasuringmode: CanvasTextMeasuringMode,
            localename: *mut ::core::ffi::c_void,
            glyphorientation: CanvasGlyphOrientation,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DrawUnderline(
                    ::core::mem::transmute(&point),
                    underlinewidth,
                    underlinethickness,
                    underlineoffset,
                    runheight,
                    textdirection,
                    ::core::mem::transmute(&brush),
                    textmeasuringmode,
                    ::core::mem::transmute(&localename),
                    glyphorientation,
                )
                .into()
        }
        unsafe extern "system" fn DrawInlineObject<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasTextRenderer_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            point: ::windows::Foundation::Numerics::Vector2,
            inlineobject: *mut ::core::ffi::c_void,
            issideways: bool,
            isrighttoleft: bool,
            brush: *mut ::core::ffi::c_void,
            glyphorientation: CanvasGlyphOrientation,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DrawInlineObject(
                    ::core::mem::transmute(&point),
                    ::core::mem::transmute(&inlineobject),
                    issideways,
                    isrighttoleft,
                    ::core::mem::transmute(&brush),
                    glyphorientation,
                )
                .into()
        }
        unsafe extern "system" fn PixelSnappingDisabled<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasTextRenderer_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PixelSnappingDisabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transform<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasTextRenderer_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::Numerics::Matrix3x2,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Transform() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dpi<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasTextRenderer_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut f32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Dpi() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<
                Identity,
                ICanvasTextRenderer,
                OFFSET,
            >(),
            DrawGlyphRun: DrawGlyphRun::<Identity, Impl, OFFSET>,
            DrawStrikethrough: DrawStrikethrough::<Identity, Impl, OFFSET>,
            DrawUnderline: DrawUnderline::<Identity, Impl, OFFSET>,
            DrawInlineObject: DrawInlineObject::<Identity, Impl, OFFSET>,
            PixelSnappingDisabled: PixelSnappingDisabled::<Identity, Impl, OFFSET>,
            Transform: Transform::<Identity, Impl, OFFSET>,
            Dpi: Dpi::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICanvasTextRenderer as ::windows::core::Interface>::IID
    }
}
