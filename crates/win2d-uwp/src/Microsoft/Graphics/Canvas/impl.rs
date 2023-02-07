///*Required features: `"Graphics_Canvas"`, `"Foundation_Numerics"`, `"Graphics_Effects"`, `"implement"`*
#[cfg(all(feature = "Foundation_Numerics", feature = "Graphics_Effects"))]
pub trait ICanvasImage_Impl: Sized + ::windows::Foundation::IClosable_Impl + ::windows::Graphics::Effects::IGraphicsEffectSource_Impl {
    fn GetBounds(
        &self,
        resourcecreator: &::core::option::Option<ICanvasResourceCreator>,
    ) -> ::windows::core::Result<::windows::Foundation::Rect>;
    fn GetBoundsWithTransform(
        &self,
        resourcecreator: &::core::option::Option<ICanvasResourceCreator>,
        transform: &::windows::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::core::Result<::windows::Foundation::Rect>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Graphics_Effects"))]
impl ::windows::core::RuntimeName for ICanvasImage {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.ICanvasImage";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "Graphics_Effects"))]
impl ICanvasImage_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: ICanvasImage_Impl,
        const OFFSET: isize,
    >() -> ICanvasImage_Vtbl {
        unsafe extern "system" fn GetBounds<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasImage_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            resourcecreator: *mut ::core::ffi::c_void,
            result__: *mut ::windows::Foundation::Rect,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBounds(::core::mem::transmute(&resourcecreator)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoundsWithTransform<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasImage_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            resourcecreator: *mut ::core::ffi::c_void,
            transform: ::windows::Foundation::Numerics::Matrix3x2,
            result__: *mut ::windows::Foundation::Rect,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this
                .GetBoundsWithTransform(
                    ::core::mem::transmute(&resourcecreator),
                    ::core::mem::transmute(&transform),
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
                ICanvasImage,
                OFFSET,
            >(),
            GetBounds: GetBounds::<Identity, Impl, OFFSET>,
            GetBoundsWithTransform: GetBoundsWithTransform::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICanvasImage as ::windows::core::Interface>::IID
    }
}
///*Required features: `"Graphics_Canvas"`, `"implement"`*
pub trait ICanvasResourceCreator_Impl: Sized {
    fn Device(&self) -> ::windows::core::Result<CanvasDevice>;
}
impl ::windows::core::RuntimeName for ICanvasResourceCreator {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.ICanvasResourceCreator";
}
impl ICanvasResourceCreator_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: ICanvasResourceCreator_Impl,
        const OFFSET: isize,
    >() -> ICanvasResourceCreator_Vtbl {
        unsafe extern "system" fn Device<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasResourceCreator_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Device() {
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
                ICanvasResourceCreator,
                OFFSET,
            >(),
            Device: Device::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICanvasResourceCreator as ::windows::core::Interface>::IID
    }
}
///*Required features: `"Graphics_Canvas"`, `"implement"`*
pub trait ICanvasResourceCreatorWithDpi_Impl: Sized + ICanvasResourceCreator_Impl {
    fn Dpi(&self) -> ::windows::core::Result<f32>;
    fn ConvertPixelsToDips(&self, pixels: i32) -> ::windows::core::Result<f32>;
    fn ConvertDipsToPixels(
        &self,
        dips: f32,
        dpirounding: CanvasDpiRounding,
    ) -> ::windows::core::Result<i32>;
}
impl ::windows::core::RuntimeName for ICanvasResourceCreatorWithDpi {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.ICanvasResourceCreatorWithDpi";
}
impl ICanvasResourceCreatorWithDpi_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: ICanvasResourceCreatorWithDpi_Impl,
        const OFFSET: isize,
    >() -> ICanvasResourceCreatorWithDpi_Vtbl {
        unsafe extern "system" fn Dpi<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasResourceCreatorWithDpi_Impl,
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
        unsafe extern "system" fn ConvertPixelsToDips<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasResourceCreatorWithDpi_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            pixels: i32,
            result__: *mut f32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ConvertPixelsToDips(pixels) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConvertDipsToPixels<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasResourceCreatorWithDpi_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            dips: f32,
            dpirounding: CanvasDpiRounding,
            result__: *mut i32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ConvertDipsToPixels(dips, dpirounding) {
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
                ICanvasResourceCreatorWithDpi,
                OFFSET,
            >(),
            Dpi: Dpi::<Identity, Impl, OFFSET>,
            ConvertPixelsToDips: ConvertPixelsToDips::<Identity, Impl, OFFSET>,
            ConvertDipsToPixels: ConvertDipsToPixels::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICanvasResourceCreatorWithDpi as ::windows::core::Interface>::IID
    }
}
