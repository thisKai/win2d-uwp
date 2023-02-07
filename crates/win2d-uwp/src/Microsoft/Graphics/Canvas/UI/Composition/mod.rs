#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasCompositionStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasCompositionStatics {
    type Vtable = ICanvasCompositionStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasCompositionStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x162deb43_1cf5_46f8_a0af_356b23158f92,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasCompositionStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Composition")]
    pub CreateCompositionGraphicsDevice: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        compositor: *mut ::core::ffi::c_void,
        canvasdevice: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateCompositionGraphicsDevice: usize,
    #[cfg(feature = "UI_Composition")]
    pub CreateCompositionSurfaceForSwapChain: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        compositor: *mut ::core::ffi::c_void,
        swapchain: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateCompositionSurfaceForSwapChain: usize,
    #[cfg(feature = "UI_Composition")]
    pub GetCanvasDevice: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        graphicsdevice: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    GetCanvasDevice: usize,
    #[cfg(feature = "UI_Composition")]
    pub SetCanvasDevice: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        graphicsdevice: *mut ::core::ffi::c_void,
        canvasdevice: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    SetCanvasDevice: usize,
    #[cfg(feature = "UI_Composition")]
    pub CreateDrawingSession: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        drawingsurface: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateDrawingSession: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Composition"))]
    pub CreateDrawingSessionWithUpdateRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        drawingsurface: *mut ::core::ffi::c_void,
        updaterect: ::windows::Foundation::Rect,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Composition")))]
    CreateDrawingSessionWithUpdateRect: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Composition"))]
    pub CreateDrawingSessionWithUpdateRectAndDpi: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        drawingsurface: *mut ::core::ffi::c_void,
        updaterectinpixels: ::windows::Foundation::Rect,
        dpi: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Composition")))]
    CreateDrawingSessionWithUpdateRectAndDpi: usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Composition"))]
    pub Resize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        drawingsurface: *mut ::core::ffi::c_void,
        sizeinpixels: ::windows::Foundation::Size,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Composition")))]
    Resize: usize,
}
///*Required features: `"Graphics_Canvas_UI_Composition"`*
pub struct CanvasComposition;
impl CanvasComposition {
    ///*Required features: `"UI_Composition"`*
    #[cfg(feature = "UI_Composition")]
    pub fn CreateCompositionGraphicsDevice(
        compositor: &::windows::UI::Composition::Compositor,
        canvasdevice: &super::super::CanvasDevice,
    ) -> ::windows::core::Result<::windows::UI::Composition::CompositionGraphicsDevice> {
        Self::ICanvasCompositionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateCompositionGraphicsDevice)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(compositor),
                    ::core::mem::transmute_copy(canvasdevice),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"UI_Composition"`*
    #[cfg(feature = "UI_Composition")]
    pub fn CreateCompositionSurfaceForSwapChain(
        compositor: &::windows::UI::Composition::Compositor,
        swapchain: &super::super::CanvasSwapChain,
    ) -> ::windows::core::Result<::windows::UI::Composition::ICompositionSurface> {
        Self::ICanvasCompositionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateCompositionSurfaceForSwapChain)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(compositor),
                    ::core::mem::transmute_copy(swapchain),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"UI_Composition"`*
    #[cfg(feature = "UI_Composition")]
    pub fn GetCanvasDevice(
        graphicsdevice: &::windows::UI::Composition::CompositionGraphicsDevice,
    ) -> ::windows::core::Result<super::super::CanvasDevice> {
        Self::ICanvasCompositionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetCanvasDevice)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(graphicsdevice),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"UI_Composition"`*
    #[cfg(feature = "UI_Composition")]
    pub fn SetCanvasDevice(
        graphicsdevice: &::windows::UI::Composition::CompositionGraphicsDevice,
        canvasdevice: &super::super::CanvasDevice,
    ) -> ::windows::core::Result<()> {
        Self::ICanvasCompositionStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetCanvasDevice)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(graphicsdevice),
                    ::core::mem::transmute_copy(canvasdevice),
                )
                .ok()
        })
    }
    ///*Required features: `"UI_Composition"`*
    #[cfg(feature = "UI_Composition")]
    pub fn CreateDrawingSession<P0>(
        drawingsurface: P0,
    ) -> ::windows::core::Result<super::super::CanvasDrawingSession>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<
                ::windows::UI::Composition::CompositionDrawingSurface,
            >,
        >,
    {
        Self::ICanvasCompositionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateDrawingSession)(
                    ::windows::core::Vtable::as_raw(this),
                    drawingsurface.into().abi(),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation"`, `"UI_Composition"`*
    #[cfg(all(feature = "Foundation", feature = "UI_Composition"))]
    pub fn CreateDrawingSessionWithUpdateRect<P0>(
        drawingsurface: P0,
        updaterect: ::windows::Foundation::Rect,
    ) -> ::windows::core::Result<super::super::CanvasDrawingSession>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<
                ::windows::UI::Composition::CompositionDrawingSurface,
            >,
        >,
    {
        Self::ICanvasCompositionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateDrawingSessionWithUpdateRect)(
                    ::windows::core::Vtable::as_raw(this),
                    drawingsurface.into().abi(),
                    updaterect,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation"`, `"UI_Composition"`*
    #[cfg(all(feature = "Foundation", feature = "UI_Composition"))]
    pub fn CreateDrawingSessionWithUpdateRectAndDpi<P0>(
        drawingsurface: P0,
        updaterectinpixels: ::windows::Foundation::Rect,
        dpi: f32,
    ) -> ::windows::core::Result<super::super::CanvasDrawingSession>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<
                ::windows::UI::Composition::CompositionDrawingSurface,
            >,
        >,
    {
        Self::ICanvasCompositionStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateDrawingSessionWithUpdateRectAndDpi)(
                    ::windows::core::Vtable::as_raw(this),
                    drawingsurface.into().abi(),
                    updaterectinpixels,
                    dpi,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation"`, `"UI_Composition"`*
    #[cfg(all(feature = "Foundation", feature = "UI_Composition"))]
    pub fn Resize<P0>(
        drawingsurface: P0,
        sizeinpixels: ::windows::Foundation::Size,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<
                ::windows::UI::Composition::CompositionDrawingSurface,
            >,
        >,
    {
        Self::ICanvasCompositionStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this)
                .Resize)(
                    ::windows::core::Vtable::as_raw(this),
                    drawingsurface.into().abi(),
                    sizeinpixels,
                )
                .ok()
        })
    }
    #[doc(hidden)]
    pub fn ICanvasCompositionStatics<
        R,
        F: FnOnce(&ICanvasCompositionStatics) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasComposition,
            ICanvasCompositionStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for CanvasComposition {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.UI.Composition.CanvasComposition";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
