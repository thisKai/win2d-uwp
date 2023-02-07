///*Required features: `"Graphics_Canvas_Brushes"`, `"Foundation_Numerics"`, `"implement"`*
#[cfg(feature = "Foundation_Numerics")]
pub trait ICanvasBrush_Impl: Sized + ::windows::Foundation::IClosable_Impl {
    fn Opacity(&self) -> ::windows::core::Result<f32>;
    fn SetOpacity(&self, value: f32) -> ::windows::core::Result<()>;
    fn Transform(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Matrix3x2>;
    fn SetTransform(
        &self,
        value: &::windows::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::core::Result<()>;
    fn Device(&self) -> ::windows::core::Result<super::CanvasDevice>;
}
#[cfg(feature = "Foundation_Numerics")]
impl ::windows::core::RuntimeName for ICanvasBrush {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Brushes.ICanvasBrush";
}
#[cfg(feature = "Foundation_Numerics")]
impl ICanvasBrush_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: ICanvasBrush_Impl,
        const OFFSET: isize,
    >() -> ICanvasBrush_Vtbl {
        unsafe extern "system" fn Opacity<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasBrush_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut f32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Opacity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacity<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasBrush_Impl,
            const OFFSET: isize,
        >(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOpacity(value).into()
        }
        unsafe extern "system" fn Transform<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasBrush_Impl,
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
        unsafe extern "system" fn SetTransform<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasBrush_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            value: ::windows::Foundation::Numerics::Matrix3x2,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTransform(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Device<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasBrush_Impl,
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
                ICanvasBrush,
                OFFSET,
            >(),
            Opacity: Opacity::<Identity, Impl, OFFSET>,
            SetOpacity: SetOpacity::<Identity, Impl, OFFSET>,
            Transform: Transform::<Identity, Impl, OFFSET>,
            SetTransform: SetTransform::<Identity, Impl, OFFSET>,
            Device: Device::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICanvasBrush as ::windows::core::Interface>::IID
    }
}
