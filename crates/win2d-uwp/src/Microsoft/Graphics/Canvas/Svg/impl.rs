///*Required features: `"Graphics_Canvas_Svg"`, `"Foundation"`, `"implement"`*
#[cfg(feature = "Foundation")]
pub trait ICanvasSvgAttribute_Impl: Sized + ::windows::Foundation::IClosable_Impl {
    fn Clone(&self) -> ::windows::core::Result<ICanvasSvgAttribute>;
    fn GetElement(&self) -> ::windows::core::Result<CanvasSvgNamedElement>;
    fn Device(&self) -> ::windows::core::Result<super::CanvasDevice>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ICanvasSvgAttribute {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Svg.ICanvasSvgAttribute";
}
#[cfg(feature = "Foundation")]
impl ICanvasSvgAttribute_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: ICanvasSvgAttribute_Impl,
        const OFFSET: isize,
    >() -> ICanvasSvgAttribute_Vtbl {
        unsafe extern "system" fn Clone<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasSvgAttribute_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElement<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasSvgAttribute_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetElement() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Device<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasSvgAttribute_Impl,
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
                ICanvasSvgAttribute,
                OFFSET,
            >(),
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetElement: GetElement::<Identity, Impl, OFFSET>,
            Device: Device::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICanvasSvgAttribute as ::windows::core::Interface>::IID
    }
}
///*Required features: `"Graphics_Canvas_Svg"`, `"Foundation"`, `"implement"`*
#[cfg(feature = "Foundation")]
pub trait ICanvasSvgElement_Impl: Sized + ::windows::Foundation::IClosable_Impl {
    fn ContainingDocument(&self) -> ::windows::core::Result<CanvasSvgDocument>;
    fn Parent(&self) -> ::windows::core::Result<CanvasSvgNamedElement>;
    fn Device(&self) -> ::windows::core::Result<super::CanvasDevice>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ICanvasSvgElement {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Svg.ICanvasSvgElement";
}
#[cfg(feature = "Foundation")]
impl ICanvasSvgElement_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: ICanvasSvgElement_Impl,
        const OFFSET: isize,
    >() -> ICanvasSvgElement_Vtbl {
        unsafe extern "system" fn ContainingDocument<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasSvgElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ContainingDocument() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Parent<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasSvgElement_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            result__: *mut *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Parent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Device<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasSvgElement_Impl,
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
                ICanvasSvgElement,
                OFFSET,
            >(),
            ContainingDocument: ContainingDocument::<Identity, Impl, OFFSET>,
            Parent: Parent::<Identity, Impl, OFFSET>,
            Device: Device::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICanvasSvgElement as ::windows::core::Interface>::IID
    }
}
