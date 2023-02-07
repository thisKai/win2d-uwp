///*Required features: `"Graphics_Canvas_Effects"`, `"Foundation_Numerics"`, `"Graphics_Effects"`, `"implement"`*
#[cfg(all(feature = "Foundation_Numerics", feature = "Graphics_Effects"))]
pub trait ICanvasEffect_Impl: Sized + super::ICanvasImage_Impl + ::windows::Foundation::IClosable_Impl + ::windows::Graphics::Effects::IGraphicsEffect_Impl + ::windows::Graphics::Effects::IGraphicsEffectSource_Impl {
    fn CacheOutput(&self) -> ::windows::core::Result<bool>;
    fn SetCacheOutput(&self, value: bool) -> ::windows::core::Result<()>;
    fn BufferPrecision(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::IReference::<super::CanvasBufferPrecision>,
    >;
    fn SetBufferPrecision(
        &self,
        value: &::core::option::Option<
            ::windows::Foundation::IReference::<super::CanvasBufferPrecision>,
        >,
    ) -> ::windows::core::Result<()>;
    fn InvalidateSourceRectangle(
        &self,
        resourcecreator: &::core::option::Option<super::ICanvasResourceCreatorWithDpi>,
        sourceindex: u32,
        invalidrectangle: &::windows::Foundation::Rect,
    ) -> ::windows::core::Result<()>;
    fn GetInvalidRectangles(
        &self,
        resourcecreator: &::core::option::Option<super::ICanvasResourceCreatorWithDpi>,
    ) -> ::windows::core::Result<::windows::core::Array<::windows::Foundation::Rect>>;
    fn GetRequiredSourceRectangle(
        &self,
        resourcecreator: &::core::option::Option<super::ICanvasResourceCreatorWithDpi>,
        outputrectangle: &::windows::Foundation::Rect,
        sourceeffect: &::core::option::Option<ICanvasEffect>,
        sourceindex: u32,
        sourcebounds: &::windows::Foundation::Rect,
    ) -> ::windows::core::Result<::windows::Foundation::Rect>;
    fn GetRequiredSourceRectangles(
        &self,
        resourcecreator: &::core::option::Option<super::ICanvasResourceCreatorWithDpi>,
        outputrectangle: &::windows::Foundation::Rect,
        sourceeffects: &[::core::option::Option<ICanvasEffect>],
        sourceindices: &[u32],
        sourcebounds: &[::windows::Foundation::Rect],
    ) -> ::windows::core::Result<::windows::core::Array<::windows::Foundation::Rect>>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Graphics_Effects"))]
impl ::windows::core::RuntimeName for ICanvasEffect {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Effects.ICanvasEffect";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Graphics_Effects"))]
impl ICanvasEffect_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: ICanvasEffect_Impl,
        const OFFSET: isize,
    >() -> ICanvasEffect_Vtbl {
        unsafe extern "system" fn CacheOutput<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasEffect_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut bool,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CacheOutput() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCacheOutput<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasEffect_Impl,
            const OFFSET: isize,
        >(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCacheOutput(value).into()
        }
        unsafe extern "system" fn BufferPrecision<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasEffect_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BufferPrecision() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferPrecision<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasEffect_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBufferPrecision(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn InvalidateSourceRectangle<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasEffect_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            resourcecreator: *mut ::core::ffi::c_void,
            sourceindex: u32,
            invalidrectangle: ::windows::Foundation::Rect,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InvalidateSourceRectangle(
                    ::core::mem::transmute(&resourcecreator),
                    sourceindex,
                    ::core::mem::transmute(&invalidrectangle),
                )
                .into()
        }
        unsafe extern "system" fn GetInvalidRectangles<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasEffect_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            resourcecreator: *mut ::core::ffi::c_void,
            result_size__: *mut u32,
            result__: *mut *mut ::windows::Foundation::Rect,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInvalidRectangles(::core::mem::transmute(&resourcecreator)) {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    ::core::ptr::write(result__, ok_data__);
                    ::core::ptr::write(result_size__, ok_data_len__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRequiredSourceRectangle<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasEffect_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            resourcecreator: *mut ::core::ffi::c_void,
            outputrectangle: ::windows::Foundation::Rect,
            sourceeffect: *mut ::core::ffi::c_void,
            sourceindex: u32,
            sourcebounds: ::windows::Foundation::Rect,
            result__: *mut ::windows::Foundation::Rect,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this
                .GetRequiredSourceRectangle(
                    ::core::mem::transmute(&resourcecreator),
                    ::core::mem::transmute(&outputrectangle),
                    ::core::mem::transmute(&sourceeffect),
                    sourceindex,
                    ::core::mem::transmute(&sourcebounds),
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
        unsafe extern "system" fn GetRequiredSourceRectangles<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasEffect_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            resourcecreator: *mut ::core::ffi::c_void,
            outputrectangle: ::windows::Foundation::Rect,
            sourceEffects_array_size: u32,
            sourceeffects: *const *mut ::core::ffi::c_void,
            sourceIndices_array_size: u32,
            sourceindices: *const u32,
            sourceBounds_array_size: u32,
            sourcebounds: *const ::windows::Foundation::Rect,
            result_size__: *mut u32,
            result__: *mut *mut ::windows::Foundation::Rect,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this
                .GetRequiredSourceRectangles(
                    ::core::mem::transmute(&resourcecreator),
                    ::core::mem::transmute(&outputrectangle),
                    ::core::slice::from_raw_parts(
                        ::core::mem::transmute_copy(&sourceeffects),
                        sourceEffects_array_size as _,
                    ),
                    ::core::slice::from_raw_parts(
                        ::core::mem::transmute_copy(&sourceindices),
                        sourceIndices_array_size as _,
                    ),
                    ::core::slice::from_raw_parts(
                        ::core::mem::transmute_copy(&sourcebounds),
                        sourceBounds_array_size as _,
                    ),
                )
            {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    ::core::ptr::write(result__, ok_data__);
                    ::core::ptr::write(result_size__, ok_data_len__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<
                Identity,
                ICanvasEffect,
                OFFSET,
            >(),
            CacheOutput: CacheOutput::<Identity, Impl, OFFSET>,
            SetCacheOutput: SetCacheOutput::<Identity, Impl, OFFSET>,
            BufferPrecision: BufferPrecision::<Identity, Impl, OFFSET>,
            SetBufferPrecision: SetBufferPrecision::<Identity, Impl, OFFSET>,
            InvalidateSourceRectangle: InvalidateSourceRectangle::<
                Identity,
                Impl,
                OFFSET,
            >,
            GetInvalidRectangles: GetInvalidRectangles::<Identity, Impl, OFFSET>,
            GetRequiredSourceRectangle: GetRequiredSourceRectangle::<
                Identity,
                Impl,
                OFFSET,
            >,
            GetRequiredSourceRectangles: GetRequiredSourceRectangles::<
                Identity,
                Impl,
                OFFSET,
            >,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICanvasEffect as ::windows::core::Interface>::IID
    }
}
