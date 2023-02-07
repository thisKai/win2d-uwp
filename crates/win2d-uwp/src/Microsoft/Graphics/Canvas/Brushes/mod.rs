#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, clippy::all)]
///*Required features: `"Graphics_Canvas_Brushes"`*
#[repr(transparent)]
pub struct ICanvasBrush(::windows::core::IUnknown);
impl ICanvasBrush {
    pub fn Opacity(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Opacity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn SetOpacity(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetOpacity)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Transform(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Matrix3x2> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Transform)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetTransform(
        &self,
        value: ::windows::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetTransform)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn Device(&self) -> ::windows::core::Result<super::CanvasDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Device)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::IClosable,
        >(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .Close)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
}
::windows::core::interface_hierarchy!(
    ICanvasBrush, ::windows::core::IUnknown, ::windows::core::IInspectable
);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ICanvasBrush> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ICanvasBrush) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ICanvasBrush> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ICanvasBrush) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ICanvasBrush>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ICanvasBrush) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for ICanvasBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICanvasBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICanvasBrush {}
impl ::core::fmt::Debug for ICanvasBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICanvasBrush").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICanvasBrush {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"{f5d58591-c803-41b4-878e-79d92ab13295}",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ICanvasBrush {
    type Vtable = ICanvasBrush_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasBrush {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xf5d58591_c803_41b4_878e_79d92ab13295,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasBrush_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Opacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Transform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Transform: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransform: usize,
    pub Device: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasImageBrush(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasImageBrush {
    type Vtable = ICanvasImageBrush_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasImageBrush {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x5a21c4e7_d450_4942_8bbd_f017097ab763,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasImageBrush_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Image: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetImage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ExtendX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::CanvasEdgeBehavior,
    ) -> ::windows::core::HRESULT,
    pub SetExtendX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::CanvasEdgeBehavior,
    ) -> ::windows::core::HRESULT,
    pub ExtendY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::CanvasEdgeBehavior,
    ) -> ::windows::core::HRESULT,
    pub SetExtendY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::CanvasEdgeBehavior,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SourceRectangle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourceRectangle: usize,
    #[cfg(feature = "Foundation")]
    pub SetSourceRectangle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSourceRectangle: usize,
    pub Interpolation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::CanvasImageInterpolation,
    ) -> ::windows::core::HRESULT,
    pub SetInterpolation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: super::CanvasImageInterpolation,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasImageBrushFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasImageBrushFactory {
    type Vtable = ICanvasImageBrushFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasImageBrushFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xc40ccb59_1b87_4394_a6a6_ecd278e877d6,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasImageBrushFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateWithImage: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        image: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasLinearGradientBrush(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasLinearGradientBrush {
    type Vtable = ICanvasLinearGradientBrush_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasLinearGradientBrush {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xa4ffbcb1_ec22_48c8_b1af_09bcfd34eebd,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasLinearGradientBrush_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub StartPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    StartPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetStartPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetStartPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub EndPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    EndPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetEndPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetEndPoint: usize,
    #[cfg(feature = "UI")]
    pub Stops: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut CanvasGradientStop,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Stops: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub StopsHdr: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut CanvasGradientStopHdr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    StopsHdr: usize,
    pub EdgeBehavior: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::CanvasEdgeBehavior,
    ) -> ::windows::core::HRESULT,
    pub PreInterpolationSpace: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::CanvasColorSpace,
    ) -> ::windows::core::HRESULT,
    pub PostInterpolationSpace: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::CanvasColorSpace,
    ) -> ::windows::core::HRESULT,
    pub BufferPrecision: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::CanvasBufferPrecision,
    ) -> ::windows::core::HRESULT,
    pub AlphaMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::CanvasAlphaMode,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasLinearGradientBrushFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasLinearGradientBrushFactory {
    type Vtable = ICanvasLinearGradientBrushFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasLinearGradientBrushFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xa2e8e34a_8592_4b9d_ba1c_00d3f3c34f54,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasLinearGradientBrushFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "UI")]
    pub CreateSimple: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        startcolor: ::windows::UI::Color,
        endcolor: ::windows::UI::Color,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    CreateSimple: usize,
    #[cfg(feature = "UI")]
    pub CreateWithStops: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        gradientStops_array_size: u32,
        gradientstops: *const CanvasGradientStop,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    CreateWithStops: usize,
    #[cfg(feature = "UI")]
    pub CreateWithEdgeBehaviorAndAlphaMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        gradientStops_array_size: u32,
        gradientstops: *const CanvasGradientStop,
        edgebehavior: super::CanvasEdgeBehavior,
        alphamode: super::CanvasAlphaMode,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    CreateWithEdgeBehaviorAndAlphaMode: usize,
    #[cfg(feature = "UI")]
    pub CreateWithEdgeBehaviorAndInterpolationOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        gradientStops_array_size: u32,
        gradientstops: *const CanvasGradientStop,
        edgebehavior: super::CanvasEdgeBehavior,
        alphamode: super::CanvasAlphaMode,
        preinterpolationspace: super::CanvasColorSpace,
        postinterpolationspace: super::CanvasColorSpace,
        bufferprecision: super::CanvasBufferPrecision,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    CreateWithEdgeBehaviorAndInterpolationOptions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasLinearGradientBrushStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasLinearGradientBrushStatics {
    type Vtable = ICanvasLinearGradientBrushStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasLinearGradientBrushStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xe9de3392_8fbf_478c_8e6a_d0a0ea753b37,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasLinearGradientBrushStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateHdrSimple: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        startcolorhdr: ::windows::Foundation::Numerics::Vector4,
        endcolorhdr: ::windows::Foundation::Numerics::Vector4,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateHdrSimple: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateHdrWithStops: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        gradientStopsHdr_array_size: u32,
        gradientstopshdr: *const CanvasGradientStopHdr,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateHdrWithStops: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateHdrWithEdgeBehaviorAndAlphaMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        gradientStopsHdr_array_size: u32,
        gradientstopshdr: *const CanvasGradientStopHdr,
        edgebehavior: super::CanvasEdgeBehavior,
        alphamode: super::CanvasAlphaMode,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateHdrWithEdgeBehaviorAndAlphaMode: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateHdrWithEdgeBehaviorAndInterpolationOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        gradientStopsHdr_array_size: u32,
        gradientstopshdr: *const CanvasGradientStopHdr,
        edgebehavior: super::CanvasEdgeBehavior,
        alphamode: super::CanvasAlphaMode,
        preinterpolationspace: super::CanvasColorSpace,
        postinterpolationspace: super::CanvasColorSpace,
        bufferprecision: super::CanvasBufferPrecision,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateHdrWithEdgeBehaviorAndInterpolationOptions: usize,
    pub CreateRainbow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        eldritchness: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasRadialGradientBrush(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasRadialGradientBrush {
    type Vtable = ICanvasRadialGradientBrush_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasRadialGradientBrush {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x4d27d756_14a9_4eb7_973f_e6614d4f89e7,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasRadialGradientBrush_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Center: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Center: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetCenter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetCenter: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub OriginOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    OriginOffset: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetOriginOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetOriginOffset: usize,
    pub RadiusX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetRadiusX: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub RadiusY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetRadiusY: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")]
    pub Stops: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut CanvasGradientStop,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Stops: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub StopsHdr: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut CanvasGradientStopHdr,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    StopsHdr: usize,
    pub EdgeBehavior: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::CanvasEdgeBehavior,
    ) -> ::windows::core::HRESULT,
    pub PreInterpolationSpace: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::CanvasColorSpace,
    ) -> ::windows::core::HRESULT,
    pub PostInterpolationSpace: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::CanvasColorSpace,
    ) -> ::windows::core::HRESULT,
    pub BufferPrecision: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::CanvasBufferPrecision,
    ) -> ::windows::core::HRESULT,
    pub AlphaMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut super::CanvasAlphaMode,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasRadialGradientBrushFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasRadialGradientBrushFactory {
    type Vtable = ICanvasRadialGradientBrushFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasRadialGradientBrushFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x7933a51f_a910_4548_849f_42ddec466f41,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasRadialGradientBrushFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "UI")]
    pub CreateSimple: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        startcolor: ::windows::UI::Color,
        endcolor: ::windows::UI::Color,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    CreateSimple: usize,
    #[cfg(feature = "UI")]
    pub CreateWithStops: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        gradientStops_array_size: u32,
        gradientstops: *const CanvasGradientStop,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    CreateWithStops: usize,
    #[cfg(feature = "UI")]
    pub CreateWithEdgeBehaviorAndAlphaMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        gradientStops_array_size: u32,
        gradientstops: *const CanvasGradientStop,
        edgebehavior: super::CanvasEdgeBehavior,
        alphamode: super::CanvasAlphaMode,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    CreateWithEdgeBehaviorAndAlphaMode: usize,
    #[cfg(feature = "UI")]
    pub CreateWithEdgeBehaviorAndInterpolationOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        gradientStops_array_size: u32,
        gradientstops: *const CanvasGradientStop,
        edgebehavior: super::CanvasEdgeBehavior,
        alphamode: super::CanvasAlphaMode,
        preinterpolationspace: super::CanvasColorSpace,
        postinterpolationspace: super::CanvasColorSpace,
        bufferprecision: super::CanvasBufferPrecision,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    CreateWithEdgeBehaviorAndInterpolationOptions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasRadialGradientBrushStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasRadialGradientBrushStatics {
    type Vtable = ICanvasRadialGradientBrushStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasRadialGradientBrushStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x3b0d4dae_3e21_4818_99b4_779acaaf18be,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasRadialGradientBrushStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateHdrSimple: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        startcolorhdr: ::windows::Foundation::Numerics::Vector4,
        endcolorhdr: ::windows::Foundation::Numerics::Vector4,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateHdrSimple: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateHdrWithStops: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        gradientStops_array_size: u32,
        gradientstops: *const CanvasGradientStopHdr,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateHdrWithStops: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateHdrWithEdgeBehaviorAndAlphaMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        gradientStops_array_size: u32,
        gradientstops: *const CanvasGradientStopHdr,
        edgebehavior: super::CanvasEdgeBehavior,
        alphamode: super::CanvasAlphaMode,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateHdrWithEdgeBehaviorAndAlphaMode: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateHdrWithEdgeBehaviorAndInterpolationOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        gradientStops_array_size: u32,
        gradientstops: *const CanvasGradientStopHdr,
        edgebehavior: super::CanvasEdgeBehavior,
        alphamode: super::CanvasAlphaMode,
        preinterpolationspace: super::CanvasColorSpace,
        postinterpolationspace: super::CanvasColorSpace,
        bufferprecision: super::CanvasBufferPrecision,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateHdrWithEdgeBehaviorAndInterpolationOptions: usize,
    pub CreateRainbow: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        eldritchness: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasSolidColorBrush(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasSolidColorBrush {
    type Vtable = ICanvasSolidColorBrush_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasSolidColorBrush {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x8bc30f87_bad5_4871_88b8_9fe3c63d204a,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasSolidColorBrush_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "UI")]
    pub Color: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Color: usize,
    #[cfg(feature = "UI")]
    pub SetColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColor: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub ColorHdr: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Numerics::Vector4,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ColorHdr: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetColorHdr: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Numerics::Vector4,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetColorHdr: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasSolidColorBrushFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasSolidColorBrushFactory {
    type Vtable = ICanvasSolidColorBrushFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasSolidColorBrushFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x8abf4780_4edd_4f2b_bf63_2c385115b201,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasSolidColorBrushFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "UI")]
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        color: ::windows::UI::Color,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasSolidColorBrushStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasSolidColorBrushStatics {
    type Vtable = ICanvasSolidColorBrushStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasSolidColorBrushStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x629b7244_5b9e_4ef8_8e09_264714201ee8,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasSolidColorBrushStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateHdr: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        colorhdr: ::windows::Foundation::Numerics::Vector4,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateHdr: usize,
}
///*Required features: `"Graphics_Canvas_Brushes"`*
#[repr(transparent)]
pub struct CanvasImageBrush(::windows::core::IUnknown);
impl CanvasImageBrush {
    pub fn Opacity(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<ICanvasBrush>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Opacity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn SetOpacity(&self, value: f32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICanvasBrush>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetOpacity)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Transform(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Matrix3x2> {
        let this = &::windows::core::Interface::cast::<ICanvasBrush>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Transform)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetTransform(
        &self,
        value: ::windows::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICanvasBrush>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetTransform)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn Device(&self) -> ::windows::core::Result<super::CanvasDevice> {
        let this = &::windows::core::Interface::cast::<ICanvasBrush>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Device)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn Image(&self) -> ::windows::core::Result<super::ICanvasImage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Image)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn SetImage<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasImage>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetImage)(
                    ::windows::core::Vtable::as_raw(this),
                    value.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    pub fn ExtendX(&self) -> ::windows::core::Result<super::CanvasEdgeBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ExtendX)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn SetExtendX(
        &self,
        value: super::CanvasEdgeBehavior,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetExtendX)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn ExtendY(&self) -> ::windows::core::Result<super::CanvasEdgeBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ExtendY)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn SetExtendY(
        &self,
        value: super::CanvasEdgeBehavior,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetExtendY)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn SourceRectangle(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::IReference::<::windows::Foundation::Rect>,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .SourceRectangle)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn SetSourceRectangle<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<
                ::windows::Foundation::IReference::<::windows::Foundation::Rect>,
            >,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetSourceRectangle)(
                    ::windows::core::Vtable::as_raw(this),
                    value.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    pub fn Interpolation(
        &self,
    ) -> ::windows::core::Result<super::CanvasImageInterpolation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Interpolation)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetInterpolation(
        &self,
        value: super::CanvasImageInterpolation,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetInterpolation)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn Create<P0, E0>(
        resourcecreator: P0,
    ) -> ::windows::core::Result<CanvasImageBrush>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasImageBrushFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Create)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    pub fn CreateWithImage<P0, E0, P1, E1>(
        resourcecreator: P0,
        image: P1,
    ) -> ::windows::core::Result<CanvasImageBrush>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasImage>,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasImageBrushFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateWithImage)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    image.try_into().map_err(|e| e.into())?.abi(),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::IClosable,
        >(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .Close)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    #[doc(hidden)]
    pub fn ICanvasImageBrushFactory<
        R,
        F: FnOnce(&ICanvasImageBrushFactory) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasImageBrush,
            ICanvasImageBrushFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CanvasImageBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasImageBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasImageBrush {}
impl ::core::fmt::Debug for CanvasImageBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasImageBrush").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasImageBrush {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.Brushes.CanvasImageBrush;{5a21c4e7-d450-4942-8bbd-f017097ab763})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasImageBrush {
    type Vtable = ICanvasImageBrush_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasImageBrush {
    const IID: ::windows::core::GUID = <ICanvasImageBrush as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasImageBrush {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Brushes.CanvasImageBrush";
}
::windows::core::interface_hierarchy!(
    CanvasImageBrush,::windows::core::IUnknown,::windows::core::IInspectable
);
impl ::core::convert::TryFrom<CanvasImageBrush> for ICanvasBrush {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasImageBrush) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CanvasImageBrush> for ICanvasBrush {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasImageBrush) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&CanvasImageBrush>
for ::windows::core::InParam<ICanvasBrush> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasImageBrush) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasImageBrush> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasImageBrush) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasImageBrush> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasImageBrush) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasImageBrush>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasImageBrush) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CanvasImageBrush {}
unsafe impl ::core::marker::Sync for CanvasImageBrush {}
///*Required features: `"Graphics_Canvas_Brushes"`*
#[repr(transparent)]
pub struct CanvasLinearGradientBrush(::windows::core::IUnknown);
impl CanvasLinearGradientBrush {
    pub fn Opacity(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<ICanvasBrush>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Opacity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn SetOpacity(&self, value: f32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICanvasBrush>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetOpacity)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Transform(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Matrix3x2> {
        let this = &::windows::core::Interface::cast::<ICanvasBrush>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Transform)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetTransform(
        &self,
        value: ::windows::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICanvasBrush>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetTransform)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn Device(&self) -> ::windows::core::Result<super::CanvasDevice> {
        let this = &::windows::core::Interface::cast::<ICanvasBrush>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Device)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn StartPoint(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .StartPoint)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetStartPoint(
        &self,
        value: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetStartPoint)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn EndPoint(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .EndPoint)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetEndPoint(
        &self,
        value: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetEndPoint)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn Stops(
        &self,
    ) -> ::windows::core::Result<::windows::core::Array<CanvasGradientStop>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Stops)(
                    ::windows::core::Vtable::as_raw(this),
                    ::windows::core::Array::<
                        CanvasGradientStop,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn StopsHdr(
        &self,
    ) -> ::windows::core::Result<::windows::core::Array<CanvasGradientStopHdr>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .StopsHdr)(
                    ::windows::core::Vtable::as_raw(this),
                    ::windows::core::Array::<
                        CanvasGradientStopHdr,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn EdgeBehavior(&self) -> ::windows::core::Result<super::CanvasEdgeBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .EdgeBehavior)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn PreInterpolationSpace(
        &self,
    ) -> ::windows::core::Result<super::CanvasColorSpace> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .PreInterpolationSpace)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn PostInterpolationSpace(
        &self,
    ) -> ::windows::core::Result<super::CanvasColorSpace> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .PostInterpolationSpace)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn BufferPrecision(
        &self,
    ) -> ::windows::core::Result<super::CanvasBufferPrecision> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .BufferPrecision)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn AlphaMode(&self) -> ::windows::core::Result<super::CanvasAlphaMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .AlphaMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn CreateSimple<P0, E0>(
        resourcecreator: P0,
        startcolor: ::windows::UI::Color,
        endcolor: ::windows::UI::Color,
    ) -> ::windows::core::Result<CanvasLinearGradientBrush>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasLinearGradientBrushFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateSimple)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    startcolor,
                    endcolor,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn CreateWithStops<P0, E0>(
        resourcecreator: P0,
        gradientstops: &[CanvasGradientStop],
    ) -> ::windows::core::Result<CanvasLinearGradientBrush>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasLinearGradientBrushFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateWithStops)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    gradientstops.len() as u32,
                    gradientstops.as_ptr(),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn CreateWithEdgeBehaviorAndAlphaMode<P0, E0>(
        resourcecreator: P0,
        gradientstops: &[CanvasGradientStop],
        edgebehavior: super::CanvasEdgeBehavior,
        alphamode: super::CanvasAlphaMode,
    ) -> ::windows::core::Result<CanvasLinearGradientBrush>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasLinearGradientBrushFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateWithEdgeBehaviorAndAlphaMode)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    gradientstops.len() as u32,
                    gradientstops.as_ptr(),
                    edgebehavior,
                    alphamode,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn CreateWithEdgeBehaviorAndInterpolationOptions<P0, E0>(
        resourcecreator: P0,
        gradientstops: &[CanvasGradientStop],
        edgebehavior: super::CanvasEdgeBehavior,
        alphamode: super::CanvasAlphaMode,
        preinterpolationspace: super::CanvasColorSpace,
        postinterpolationspace: super::CanvasColorSpace,
        bufferprecision: super::CanvasBufferPrecision,
    ) -> ::windows::core::Result<CanvasLinearGradientBrush>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasLinearGradientBrushFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateWithEdgeBehaviorAndInterpolationOptions)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    gradientstops.len() as u32,
                    gradientstops.as_ptr(),
                    edgebehavior,
                    alphamode,
                    preinterpolationspace,
                    postinterpolationspace,
                    bufferprecision,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateHdrSimple<P0, E0>(
        resourcecreator: P0,
        startcolorhdr: ::windows::Foundation::Numerics::Vector4,
        endcolorhdr: ::windows::Foundation::Numerics::Vector4,
    ) -> ::windows::core::Result<CanvasLinearGradientBrush>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasLinearGradientBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateHdrSimple)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    startcolorhdr,
                    endcolorhdr,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateHdrWithStops<P0, E0>(
        resourcecreator: P0,
        gradientstopshdr: &[CanvasGradientStopHdr],
    ) -> ::windows::core::Result<CanvasLinearGradientBrush>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasLinearGradientBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateHdrWithStops)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    gradientstopshdr.len() as u32,
                    gradientstopshdr.as_ptr(),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateHdrWithEdgeBehaviorAndAlphaMode<P0, E0>(
        resourcecreator: P0,
        gradientstopshdr: &[CanvasGradientStopHdr],
        edgebehavior: super::CanvasEdgeBehavior,
        alphamode: super::CanvasAlphaMode,
    ) -> ::windows::core::Result<CanvasLinearGradientBrush>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasLinearGradientBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateHdrWithEdgeBehaviorAndAlphaMode)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    gradientstopshdr.len() as u32,
                    gradientstopshdr.as_ptr(),
                    edgebehavior,
                    alphamode,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateHdrWithEdgeBehaviorAndInterpolationOptions<P0, E0>(
        resourcecreator: P0,
        gradientstopshdr: &[CanvasGradientStopHdr],
        edgebehavior: super::CanvasEdgeBehavior,
        alphamode: super::CanvasAlphaMode,
        preinterpolationspace: super::CanvasColorSpace,
        postinterpolationspace: super::CanvasColorSpace,
        bufferprecision: super::CanvasBufferPrecision,
    ) -> ::windows::core::Result<CanvasLinearGradientBrush>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasLinearGradientBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateHdrWithEdgeBehaviorAndInterpolationOptions)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    gradientstopshdr.len() as u32,
                    gradientstopshdr.as_ptr(),
                    edgebehavior,
                    alphamode,
                    preinterpolationspace,
                    postinterpolationspace,
                    bufferprecision,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    pub fn CreateRainbow<P0, E0>(
        resourcecreator: P0,
        eldritchness: f32,
    ) -> ::windows::core::Result<CanvasLinearGradientBrush>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasLinearGradientBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateRainbow)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    eldritchness,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::IClosable,
        >(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .Close)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    #[doc(hidden)]
    pub fn ICanvasLinearGradientBrushFactory<
        R,
        F: FnOnce(&ICanvasLinearGradientBrushFactory) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasLinearGradientBrush,
            ICanvasLinearGradientBrushFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICanvasLinearGradientBrushStatics<
        R,
        F: FnOnce(&ICanvasLinearGradientBrushStatics) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasLinearGradientBrush,
            ICanvasLinearGradientBrushStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CanvasLinearGradientBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasLinearGradientBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasLinearGradientBrush {}
impl ::core::fmt::Debug for CanvasLinearGradientBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasLinearGradientBrush").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasLinearGradientBrush {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.Brushes.CanvasLinearGradientBrush;{a4ffbcb1-ec22-48c8-b1af-09bcfd34eebd})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasLinearGradientBrush {
    type Vtable = ICanvasLinearGradientBrush_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasLinearGradientBrush {
    const IID: ::windows::core::GUID = <ICanvasLinearGradientBrush as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasLinearGradientBrush {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Brushes.CanvasLinearGradientBrush";
}
::windows::core::interface_hierarchy!(
    CanvasLinearGradientBrush,::windows::core::IUnknown,::windows::core::IInspectable
);
impl ::core::convert::TryFrom<CanvasLinearGradientBrush> for ICanvasBrush {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasLinearGradientBrush) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CanvasLinearGradientBrush> for ICanvasBrush {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasLinearGradientBrush) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&CanvasLinearGradientBrush>
for ::windows::core::InParam<ICanvasBrush> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasLinearGradientBrush) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasLinearGradientBrush>
for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasLinearGradientBrush) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasLinearGradientBrush>
for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasLinearGradientBrush) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasLinearGradientBrush>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasLinearGradientBrush) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CanvasLinearGradientBrush {}
unsafe impl ::core::marker::Sync for CanvasLinearGradientBrush {}
///*Required features: `"Graphics_Canvas_Brushes"`*
#[repr(transparent)]
pub struct CanvasRadialGradientBrush(::windows::core::IUnknown);
impl CanvasRadialGradientBrush {
    pub fn Opacity(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<ICanvasBrush>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Opacity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn SetOpacity(&self, value: f32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICanvasBrush>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetOpacity)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Transform(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Matrix3x2> {
        let this = &::windows::core::Interface::cast::<ICanvasBrush>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Transform)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetTransform(
        &self,
        value: ::windows::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICanvasBrush>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetTransform)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn Device(&self) -> ::windows::core::Result<super::CanvasDevice> {
        let this = &::windows::core::Interface::cast::<ICanvasBrush>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Device)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Center(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Center)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetCenter(
        &self,
        value: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetCenter)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn OriginOffset(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .OriginOffset)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetOriginOffset(
        &self,
        value: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetOriginOffset)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn RadiusX(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .RadiusX)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn SetRadiusX(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetRadiusX)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn RadiusY(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .RadiusY)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn SetRadiusY(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetRadiusY)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn Stops(
        &self,
    ) -> ::windows::core::Result<::windows::core::Array<CanvasGradientStop>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Stops)(
                    ::windows::core::Vtable::as_raw(this),
                    ::windows::core::Array::<
                        CanvasGradientStop,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn StopsHdr(
        &self,
    ) -> ::windows::core::Result<::windows::core::Array<CanvasGradientStopHdr>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .StopsHdr)(
                    ::windows::core::Vtable::as_raw(this),
                    ::windows::core::Array::<
                        CanvasGradientStopHdr,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn EdgeBehavior(&self) -> ::windows::core::Result<super::CanvasEdgeBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .EdgeBehavior)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn PreInterpolationSpace(
        &self,
    ) -> ::windows::core::Result<super::CanvasColorSpace> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .PreInterpolationSpace)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn PostInterpolationSpace(
        &self,
    ) -> ::windows::core::Result<super::CanvasColorSpace> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .PostInterpolationSpace)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn BufferPrecision(
        &self,
    ) -> ::windows::core::Result<super::CanvasBufferPrecision> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .BufferPrecision)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn AlphaMode(&self) -> ::windows::core::Result<super::CanvasAlphaMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .AlphaMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn CreateSimple<P0, E0>(
        resourcecreator: P0,
        startcolor: ::windows::UI::Color,
        endcolor: ::windows::UI::Color,
    ) -> ::windows::core::Result<CanvasRadialGradientBrush>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasRadialGradientBrushFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateSimple)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    startcolor,
                    endcolor,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn CreateWithStops<P0, E0>(
        resourcecreator: P0,
        gradientstops: &[CanvasGradientStop],
    ) -> ::windows::core::Result<CanvasRadialGradientBrush>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasRadialGradientBrushFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateWithStops)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    gradientstops.len() as u32,
                    gradientstops.as_ptr(),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn CreateWithEdgeBehaviorAndAlphaMode<P0, E0>(
        resourcecreator: P0,
        gradientstops: &[CanvasGradientStop],
        edgebehavior: super::CanvasEdgeBehavior,
        alphamode: super::CanvasAlphaMode,
    ) -> ::windows::core::Result<CanvasRadialGradientBrush>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasRadialGradientBrushFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateWithEdgeBehaviorAndAlphaMode)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    gradientstops.len() as u32,
                    gradientstops.as_ptr(),
                    edgebehavior,
                    alphamode,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn CreateWithEdgeBehaviorAndInterpolationOptions<P0, E0>(
        resourcecreator: P0,
        gradientstops: &[CanvasGradientStop],
        edgebehavior: super::CanvasEdgeBehavior,
        alphamode: super::CanvasAlphaMode,
        preinterpolationspace: super::CanvasColorSpace,
        postinterpolationspace: super::CanvasColorSpace,
        bufferprecision: super::CanvasBufferPrecision,
    ) -> ::windows::core::Result<CanvasRadialGradientBrush>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasRadialGradientBrushFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateWithEdgeBehaviorAndInterpolationOptions)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    gradientstops.len() as u32,
                    gradientstops.as_ptr(),
                    edgebehavior,
                    alphamode,
                    preinterpolationspace,
                    postinterpolationspace,
                    bufferprecision,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateHdrSimple<P0, E0>(
        resourcecreator: P0,
        startcolorhdr: ::windows::Foundation::Numerics::Vector4,
        endcolorhdr: ::windows::Foundation::Numerics::Vector4,
    ) -> ::windows::core::Result<CanvasRadialGradientBrush>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasRadialGradientBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateHdrSimple)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    startcolorhdr,
                    endcolorhdr,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateHdrWithStops<P0, E0>(
        resourcecreator: P0,
        gradientstops: &[CanvasGradientStopHdr],
    ) -> ::windows::core::Result<CanvasRadialGradientBrush>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasRadialGradientBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateHdrWithStops)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    gradientstops.len() as u32,
                    gradientstops.as_ptr(),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateHdrWithEdgeBehaviorAndAlphaMode<P0, E0>(
        resourcecreator: P0,
        gradientstops: &[CanvasGradientStopHdr],
        edgebehavior: super::CanvasEdgeBehavior,
        alphamode: super::CanvasAlphaMode,
    ) -> ::windows::core::Result<CanvasRadialGradientBrush>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasRadialGradientBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateHdrWithEdgeBehaviorAndAlphaMode)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    gradientstops.len() as u32,
                    gradientstops.as_ptr(),
                    edgebehavior,
                    alphamode,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateHdrWithEdgeBehaviorAndInterpolationOptions<P0, E0>(
        resourcecreator: P0,
        gradientstops: &[CanvasGradientStopHdr],
        edgebehavior: super::CanvasEdgeBehavior,
        alphamode: super::CanvasAlphaMode,
        preinterpolationspace: super::CanvasColorSpace,
        postinterpolationspace: super::CanvasColorSpace,
        bufferprecision: super::CanvasBufferPrecision,
    ) -> ::windows::core::Result<CanvasRadialGradientBrush>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasRadialGradientBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateHdrWithEdgeBehaviorAndInterpolationOptions)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    gradientstops.len() as u32,
                    gradientstops.as_ptr(),
                    edgebehavior,
                    alphamode,
                    preinterpolationspace,
                    postinterpolationspace,
                    bufferprecision,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    pub fn CreateRainbow<P0, E0>(
        resourcecreator: P0,
        eldritchness: f32,
    ) -> ::windows::core::Result<CanvasRadialGradientBrush>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasRadialGradientBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateRainbow)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    eldritchness,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::IClosable,
        >(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .Close)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    #[doc(hidden)]
    pub fn ICanvasRadialGradientBrushFactory<
        R,
        F: FnOnce(&ICanvasRadialGradientBrushFactory) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasRadialGradientBrush,
            ICanvasRadialGradientBrushFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICanvasRadialGradientBrushStatics<
        R,
        F: FnOnce(&ICanvasRadialGradientBrushStatics) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasRadialGradientBrush,
            ICanvasRadialGradientBrushStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CanvasRadialGradientBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasRadialGradientBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasRadialGradientBrush {}
impl ::core::fmt::Debug for CanvasRadialGradientBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasRadialGradientBrush").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasRadialGradientBrush {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.Brushes.CanvasRadialGradientBrush;{4d27d756-14a9-4eb7-973f-e6614d4f89e7})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasRadialGradientBrush {
    type Vtable = ICanvasRadialGradientBrush_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasRadialGradientBrush {
    const IID: ::windows::core::GUID = <ICanvasRadialGradientBrush as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasRadialGradientBrush {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Brushes.CanvasRadialGradientBrush";
}
::windows::core::interface_hierarchy!(
    CanvasRadialGradientBrush,::windows::core::IUnknown,::windows::core::IInspectable
);
impl ::core::convert::TryFrom<CanvasRadialGradientBrush> for ICanvasBrush {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasRadialGradientBrush) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CanvasRadialGradientBrush> for ICanvasBrush {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasRadialGradientBrush) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&CanvasRadialGradientBrush>
for ::windows::core::InParam<ICanvasBrush> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasRadialGradientBrush) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasRadialGradientBrush>
for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasRadialGradientBrush) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasRadialGradientBrush>
for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasRadialGradientBrush) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasRadialGradientBrush>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasRadialGradientBrush) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CanvasRadialGradientBrush {}
unsafe impl ::core::marker::Sync for CanvasRadialGradientBrush {}
///*Required features: `"Graphics_Canvas_Brushes"`*
#[repr(transparent)]
pub struct CanvasSolidColorBrush(::windows::core::IUnknown);
impl CanvasSolidColorBrush {
    pub fn Opacity(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<ICanvasBrush>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Opacity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn SetOpacity(&self, value: f32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICanvasBrush>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetOpacity)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Transform(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Matrix3x2> {
        let this = &::windows::core::Interface::cast::<ICanvasBrush>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Transform)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetTransform(
        &self,
        value: ::windows::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICanvasBrush>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetTransform)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn Device(&self) -> ::windows::core::Result<super::CanvasDevice> {
        let this = &::windows::core::Interface::cast::<ICanvasBrush>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Device)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn Color(&self) -> ::windows::core::Result<::windows::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Color)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn SetColor(&self, value: ::windows::UI::Color) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetColor)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn ColorHdr(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector4> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ColorHdr)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetColorHdr(
        &self,
        value: ::windows::Foundation::Numerics::Vector4,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetColorHdr)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn Create<P0, E0>(
        resourcecreator: P0,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<CanvasSolidColorBrush>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasSolidColorBrushFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Create)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    color,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateHdr<P0, E0>(
        resourcecreator: P0,
        colorhdr: ::windows::Foundation::Numerics::Vector4,
    ) -> ::windows::core::Result<CanvasSolidColorBrush>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasSolidColorBrushStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateHdr)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    colorhdr,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Foundation::IClosable,
        >(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .Close)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    #[doc(hidden)]
    pub fn ICanvasSolidColorBrushFactory<
        R,
        F: FnOnce(&ICanvasSolidColorBrushFactory) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasSolidColorBrush,
            ICanvasSolidColorBrushFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICanvasSolidColorBrushStatics<
        R,
        F: FnOnce(&ICanvasSolidColorBrushStatics) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasSolidColorBrush,
            ICanvasSolidColorBrushStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CanvasSolidColorBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasSolidColorBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasSolidColorBrush {}
impl ::core::fmt::Debug for CanvasSolidColorBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasSolidColorBrush").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasSolidColorBrush {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.Brushes.CanvasSolidColorBrush;{8bc30f87-bad5-4871-88b8-9fe3c63d204a})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasSolidColorBrush {
    type Vtable = ICanvasSolidColorBrush_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasSolidColorBrush {
    const IID: ::windows::core::GUID = <ICanvasSolidColorBrush as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasSolidColorBrush {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Brushes.CanvasSolidColorBrush";
}
::windows::core::interface_hierarchy!(
    CanvasSolidColorBrush,::windows::core::IUnknown,::windows::core::IInspectable
);
impl ::core::convert::TryFrom<CanvasSolidColorBrush> for ICanvasBrush {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasSolidColorBrush) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CanvasSolidColorBrush> for ICanvasBrush {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSolidColorBrush) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&CanvasSolidColorBrush>
for ::windows::core::InParam<ICanvasBrush> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSolidColorBrush) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasSolidColorBrush>
for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasSolidColorBrush) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasSolidColorBrush>
for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSolidColorBrush) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasSolidColorBrush>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSolidColorBrush) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CanvasSolidColorBrush {}
unsafe impl ::core::marker::Sync for CanvasSolidColorBrush {}
#[repr(C)]
///*Required features: `"Graphics_Canvas_Brushes"`, `"UI"`*
#[cfg(feature = "UI")]
pub struct CanvasGradientStop {
    pub Position: f32,
    pub Color: ::windows::UI::Color,
}
#[cfg(feature = "UI")]
impl ::core::marker::Copy for CanvasGradientStop {}
#[cfg(feature = "UI")]
impl ::core::clone::Clone for CanvasGradientStop {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "UI")]
impl ::core::fmt::Debug for CanvasGradientStop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CanvasGradientStop")
            .field("Position", &self.Position)
            .field("Color", &self.Color)
            .finish()
    }
}
#[cfg(feature = "UI")]
unsafe impl ::windows::core::Abi for CanvasGradientStop {
    type Abi = Self;
}
#[cfg(feature = "UI")]
unsafe impl ::windows::core::RuntimeType for CanvasGradientStop {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.Graphics.Canvas.Brushes.CanvasGradientStop;f4;struct(Windows.UI.Color;u1;u1;u1;u1))",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "UI")]
impl ::core::cmp::PartialEq for CanvasGradientStop {
    fn eq(&self, other: &Self) -> bool {
        self.Position == other.Position && self.Color == other.Color
    }
}
#[cfg(feature = "UI")]
impl ::core::cmp::Eq for CanvasGradientStop {}
#[cfg(feature = "UI")]
impl ::core::default::Default for CanvasGradientStop {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
///*Required features: `"Graphics_Canvas_Brushes"`, `"Foundation_Numerics"`*
#[cfg(feature = "Foundation_Numerics")]
pub struct CanvasGradientStopHdr {
    pub Position: f32,
    pub Color: ::windows::Foundation::Numerics::Vector4,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for CanvasGradientStopHdr {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for CanvasGradientStopHdr {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for CanvasGradientStopHdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CanvasGradientStopHdr")
            .field("Position", &self.Position)
            .field("Color", &self.Color)
            .finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::Abi for CanvasGradientStopHdr {
    type Abi = Self;
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::RuntimeType for CanvasGradientStopHdr {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.Graphics.Canvas.Brushes.CanvasGradientStopHdr;f4;struct(Windows.Foundation.Numerics.Vector4;f4;f4;f4;f4))",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for CanvasGradientStopHdr {
    fn eq(&self, other: &Self) -> bool {
        self.Position == other.Position && self.Color == other.Color
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for CanvasGradientStopHdr {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for CanvasGradientStopHdr {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
