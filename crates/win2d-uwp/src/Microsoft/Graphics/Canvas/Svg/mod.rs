#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, clippy::all)]
///*Required features: `"Graphics_Canvas_Svg"`*
#[repr(transparent)]
pub struct ICanvasSvgAttribute(::windows::core::IUnknown);
impl ICanvasSvgAttribute {
    pub fn Clone(&self) -> ::windows::core::Result<ICanvasSvgAttribute> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Clone)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn GetElement(&self) -> ::windows::core::Result<CanvasSvgNamedElement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetElement)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
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
    ICanvasSvgAttribute, ::windows::core::IUnknown, ::windows::core::IInspectable
);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ICanvasSvgAttribute> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ICanvasSvgAttribute) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ICanvasSvgAttribute>
for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ICanvasSvgAttribute) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ICanvasSvgAttribute>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ICanvasSvgAttribute) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for ICanvasSvgAttribute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICanvasSvgAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICanvasSvgAttribute {}
impl ::core::fmt::Debug for ICanvasSvgAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICanvasSvgAttribute").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICanvasSvgAttribute {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"{652786a8-f3ab-4083-991d-9748aa86bd6e}",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ICanvasSvgAttribute {
    type Vtable = ICanvasSvgAttribute_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasSvgAttribute {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x652786a8_f3ab_4083_991d_9748aa86bd6e,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasSvgAttribute_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Clone: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Device: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasSvgDocument(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasSvgDocument {
    type Vtable = ICanvasSvgDocument_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasSvgDocument {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xa0e34929_3551_44fe_a670_d9b3fd800516,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasSvgDocument_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Device: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetXml: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SaveAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        stream: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SaveAsync: usize,
    pub SetRoot: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Root: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub FindElementById: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        id: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreatePaintAttributeWithDefaults: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")]
    pub CreatePaintAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        painttype: CanvasSvgPaintType,
        color: ::windows::UI::Color,
        id: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    CreatePaintAttribute: usize,
    pub CreatePathAttributeWithDefaults: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreatePathAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        segmentData_array_size: u32,
        segmentdata: *const f32,
        commands_array_size: u32,
        commands: *const CanvasSvgPathCommand,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreatePointsAttributeWithDefaults: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreatePointsAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        points_array_size: u32,
        points: *const ::windows::Foundation::Numerics::Vector2,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreatePointsAttribute: usize,
    pub CreateStrokeDashArrayAttributeWithDefaults: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateStrokeDashArrayAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dashValues_array_size: u32,
        dashvalues: *const f32,
        unitValues_array_size: u32,
        unitvalues: *const CanvasSvgLengthUnits,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub LoadElementFromXml: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        xmlstring: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub LoadElementAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        stream: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    LoadElementAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasSvgDocumentFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasSvgDocumentFactory {
    type Vtable = ICanvasSvgDocumentFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasSvgDocumentFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xbab0f16d_4050_4ef6_8022_8a07e9e74a9d,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasSvgDocumentFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateEmpty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasSvgDocumentStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasSvgDocumentStatics {
    type Vtable = ICanvasSvgDocumentStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasSvgDocumentStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x7740e748_cb9a_453f_a678_8b3b3a7254d3,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasSvgDocumentStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub LoadFromXml: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        xmlstring: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub LoadAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        stream: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    LoadAsync: usize,
    pub IsSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        device: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
///*Required features: `"Graphics_Canvas_Svg"`*
#[repr(transparent)]
pub struct ICanvasSvgElement(::windows::core::IUnknown);
impl ICanvasSvgElement {
    pub fn ContainingDocument(&self) -> ::windows::core::Result<CanvasSvgDocument> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ContainingDocument)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::core::Result<CanvasSvgNamedElement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Parent)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
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
    ICanvasSvgElement, ::windows::core::IUnknown, ::windows::core::IInspectable
);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ICanvasSvgElement> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ICanvasSvgElement) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ICanvasSvgElement> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ICanvasSvgElement) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ICanvasSvgElement>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ICanvasSvgElement) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for ICanvasSvgElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICanvasSvgElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICanvasSvgElement {}
impl ::core::fmt::Debug for ICanvasSvgElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICanvasSvgElement").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICanvasSvgElement {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"{0775cb81-c555-45bf-9795-0ff59151c3be}",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ICanvasSvgElement {
    type Vtable = ICanvasSvgElement_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasSvgElement {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x0775cb81_c555_45bf_9795_0ff59151c3be,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasSvgElement_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ContainingDocument: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Parent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Device: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasSvgNamedElement(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasSvgNamedElement {
    type Vtable = ICanvasSvgNamedElement_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasSvgNamedElement {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xd8b7cb94_8167_495d_9c71_5e97e5d08d2b,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasSvgNamedElement_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AppendChild: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        child: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateAndAppendNamedChildElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        childname: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateAndAppendTextChildElement: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        textcontent: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub FirstChild: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub LastChild: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetPreviousSibling: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        child: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetNextSibling: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        child: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SpecifiedAttributes: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Tag: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub HasChildren: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub InsertChildBefore: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        child: *mut ::core::ffi::c_void,
        referencechild: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub IsAttributeSpecified: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub IsAttributeSpecifiedWithInherhited: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        inherited: bool,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub RemoveAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub RemoveChild: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        child: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ReplaceChild: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        newchild: *mut ::core::ffi::c_void,
        oldchild: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetStringAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        attributevalue: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetStringAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        attributevalue: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetIdAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        attributevalue: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetIdAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetFloatAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        attributevalue: f32,
    ) -> ::windows::core::HRESULT,
    pub GetFloatAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")]
    pub SetColorAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        attributevalue: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColorAttribute: usize,
    #[cfg(feature = "UI")]
    pub GetColorAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    GetColorAttribute: usize,
    #[cfg(feature = "Graphics_Canvas_Geometry")]
    pub SetFilledRegionDeterminationAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        attributevalue: super::Geometry::CanvasFilledRegionDetermination,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Geometry"))]
    SetFilledRegionDeterminationAttribute: usize,
    #[cfg(feature = "Graphics_Canvas_Geometry")]
    pub GetFilledRegionDeterminationAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        result__: *mut super::Geometry::CanvasFilledRegionDetermination,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Geometry"))]
    GetFilledRegionDeterminationAttribute: usize,
    pub SetDisplayAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        attributevalue: CanvasSvgDisplay,
    ) -> ::windows::core::HRESULT,
    pub GetDisplayAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        result__: *mut CanvasSvgDisplay,
    ) -> ::windows::core::HRESULT,
    pub SetOverflowAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        attributevalue: CanvasSvgOverflow,
    ) -> ::windows::core::HRESULT,
    pub GetOverflowAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        result__: *mut CanvasSvgOverflow,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Canvas_Geometry")]
    pub SetCapStyleAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        attributevalue: super::Geometry::CanvasCapStyle,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Geometry"))]
    SetCapStyleAttribute: usize,
    #[cfg(feature = "Graphics_Canvas_Geometry")]
    pub GetCapStyleAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        result__: *mut super::Geometry::CanvasCapStyle,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Geometry"))]
    GetCapStyleAttribute: usize,
    #[cfg(feature = "Graphics_Canvas_Geometry")]
    pub SetLineJoinAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        attributevalue: super::Geometry::CanvasLineJoin,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Geometry"))]
    SetLineJoinAttribute: usize,
    #[cfg(feature = "Graphics_Canvas_Geometry")]
    pub GetLineJoinAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        result__: *mut super::Geometry::CanvasLineJoin,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Geometry"))]
    GetLineJoinAttribute: usize,
    pub SetVisibilityAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        attributevalue: CanvasSvgVisibility,
    ) -> ::windows::core::HRESULT,
    pub GetVisibilityAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        result__: *mut CanvasSvgVisibility,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransformAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        attributevalue: ::windows::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransformAttribute: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub GetTransformAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetTransformAttribute: usize,
    pub SetUnitsAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        attributevalue: CanvasSvgUnits,
    ) -> ::windows::core::HRESULT,
    pub GetUnitsAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        result__: *mut CanvasSvgUnits,
    ) -> ::windows::core::HRESULT,
    pub SetEdgeBehaviorAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        attributevalue: super::CanvasEdgeBehavior,
    ) -> ::windows::core::HRESULT,
    pub GetEdgeBehaviorAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        result__: *mut super::CanvasEdgeBehavior,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetRectangleAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        attributevalue: ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRectangleAttribute: usize,
    #[cfg(feature = "Foundation")]
    pub GetRectangleAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetRectangleAttribute: usize,
    pub SetLengthAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        value: f32,
        units: CanvasSvgLengthUnits,
    ) -> ::windows::core::HRESULT,
    pub GetLengthAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        units: *mut CanvasSvgLengthUnits,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetAspectRatioAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        alignment: CanvasSvgAspectAlignment,
        meetorslice: CanvasSvgAspectScaling,
    ) -> ::windows::core::HRESULT,
    pub GetAspectRatioAttribute: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        attributename: *mut ::core::ffi::c_void,
        meetorslice: *mut CanvasSvgAspectScaling,
        result__: *mut CanvasSvgAspectAlignment,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasSvgPaintAttribute(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasSvgPaintAttribute {
    type Vtable = ICanvasSvgPaintAttribute_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasSvgPaintAttribute {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x653786a8_f3ab_4083_991d_9748aa86bd6e,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasSvgPaintAttribute_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetPaintType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CanvasSvgPaintType,
    ) -> ::windows::core::HRESULT,
    pub PaintType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasSvgPaintType,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")]
    pub SetColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColor: usize,
    #[cfg(feature = "UI")]
    pub Color: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Color: usize,
    pub SetId: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasSvgPathAttribute(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasSvgPathAttribute {
    type Vtable = ICanvasSvgPathAttribute_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasSvgPathAttribute {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x652786a8_f3ab_4083_991d_9748ab86bd6e,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasSvgPathAttribute_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Canvas_Geometry")]
    pub CreatePathGeometry: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Geometry"))]
    CreatePathGeometry: usize,
    #[cfg(feature = "Graphics_Canvas_Geometry")]
    pub CreatePathGeometryWithFill: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        fill: super::Geometry::CanvasFilledRegionDetermination,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Geometry"))]
    CreatePathGeometryWithFill: usize,
    pub Commands: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut CanvasSvgPathCommand,
    ) -> ::windows::core::HRESULT,
    pub GetCommands: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        startindex: i32,
        elementcount: i32,
        result_size__: *mut u32,
        result__: *mut *mut CanvasSvgPathCommand,
    ) -> ::windows::core::HRESULT,
    pub SegmentData: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut f32,
    ) -> ::windows::core::HRESULT,
    pub GetSegmentData: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        startindex: i32,
        elementcount: i32,
        result_size__: *mut u32,
        result__: *mut *mut f32,
    ) -> ::windows::core::HRESULT,
    pub RemoveCommandsAtEnd: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        commandscount: i32,
    ) -> ::windows::core::HRESULT,
    pub RemoveSegmentDataAtEnd: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        commandscount: i32,
    ) -> ::windows::core::HRESULT,
    pub SetCommands: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        startindex: i32,
        commands_array_size: u32,
        commands: *const CanvasSvgPathCommand,
    ) -> ::windows::core::HRESULT,
    pub SetSegmentData: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        startindex: i32,
        segmentData_array_size: u32,
        segmentdata: *const f32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasSvgPointsAttribute(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasSvgPointsAttribute {
    type Vtable = ICanvasSvgPointsAttribute_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasSvgPointsAttribute {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x652786a8_f3ab_4083_991d_9748aa86bd6f,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasSvgPointsAttribute_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Points: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Points: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub GetPoints: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        startindex: i32,
        elementcount: i32,
        result_size__: *mut u32,
        result__: *mut *mut ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetPoints: usize,
    pub RemovePointsAtEnd: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pointcount: i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetPoints: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        startindex: i32,
        points_array_size: u32,
        points: *const ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetPoints: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasSvgStrokeDashArrayAttribute(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasSvgStrokeDashArrayAttribute {
    type Vtable = ICanvasSvgStrokeDashArrayAttribute_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasSvgStrokeDashArrayAttribute {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x652786a8_f3ab_4083_991d_9748aa86bd70,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasSvgStrokeDashArrayAttribute_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetDashes: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut f32,
    ) -> ::windows::core::HRESULT,
    pub GetDashesWithUnits: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        startindex: i32,
        elementcount: i32,
        outputUnitsElements_array_size: *mut u32,
        outputunitselements: *mut *mut CanvasSvgLengthUnits,
        result_size__: *mut u32,
        result__: *mut *mut f32,
    ) -> ::windows::core::HRESULT,
    pub RemoveDashesAtEnd: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dashcount: i32,
    ) -> ::windows::core::HRESULT,
    pub SetDashes: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        startindex: i32,
        dashes_array_size: u32,
        dashes: *const f32,
    ) -> ::windows::core::HRESULT,
    pub SetDashesWithUnit: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        startindex: i32,
        dashes_array_size: u32,
        dashes: *const f32,
        units: CanvasSvgLengthUnits,
    ) -> ::windows::core::HRESULT,
    pub SetDashesWithUnits: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        startindex: i32,
        dashValues_array_size: u32,
        dashvalues: *const f32,
        unitValues_array_size: u32,
        unitvalues: *const CanvasSvgLengthUnits,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasSvgTextElement(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasSvgTextElement {
    type Vtable = ICanvasSvgTextElement_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasSvgTextElement {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x652786a8_f3ab_4083_991d_9748aa86bd6d,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasSvgTextElement_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetText: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Text: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
///*Required features: `"Graphics_Canvas_Svg"`*
#[repr(transparent)]
pub struct CanvasSvgDocument(::windows::core::IUnknown);
impl CanvasSvgDocument {
    pub fn Device(&self) -> ::windows::core::Result<super::CanvasDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Device)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn GetXml(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetXml)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`, `"Storage_Streams"`*
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SaveAsync<P0, E0>(
        &self,
        stream: P0,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<::windows::Storage::Streams::IRandomAccessStream>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .SaveAsync)(
                    ::windows::core::Vtable::as_raw(this),
                    stream.try_into().map_err(|e| e.into())?.abi(),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetRoot(&self, value: &CanvasSvgNamedElement) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetRoot)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(value),
                )
                .ok()
        }
    }
    pub fn Root(&self) -> ::windows::core::Result<CanvasSvgNamedElement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Root)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn FindElementById(
        &self,
        id: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<CanvasSvgNamedElement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .FindElementById)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(id),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn CreatePaintAttributeWithDefaults(
        &self,
    ) -> ::windows::core::Result<CanvasSvgPaintAttribute> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreatePaintAttributeWithDefaults)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn CreatePaintAttribute(
        &self,
        painttype: CanvasSvgPaintType,
        color: ::windows::UI::Color,
        id: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<CanvasSvgPaintAttribute> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreatePaintAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    painttype,
                    color,
                    ::core::mem::transmute_copy(id),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn CreatePathAttributeWithDefaults(
        &self,
    ) -> ::windows::core::Result<CanvasSvgPathAttribute> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreatePathAttributeWithDefaults)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn CreatePathAttribute(
        &self,
        segmentdata: &[f32],
        commands: &[CanvasSvgPathCommand],
    ) -> ::windows::core::Result<CanvasSvgPathAttribute> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreatePathAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    segmentdata.len() as u32,
                    segmentdata.as_ptr(),
                    commands.len() as u32,
                    commands.as_ptr(),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn CreatePointsAttributeWithDefaults(
        &self,
    ) -> ::windows::core::Result<CanvasSvgPointsAttribute> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreatePointsAttributeWithDefaults)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreatePointsAttribute(
        &self,
        points: &[::windows::Foundation::Numerics::Vector2],
    ) -> ::windows::core::Result<CanvasSvgPointsAttribute> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreatePointsAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    points.len() as u32,
                    points.as_ptr(),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn CreateStrokeDashArrayAttributeWithDefaults(
        &self,
    ) -> ::windows::core::Result<CanvasSvgStrokeDashArrayAttribute> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateStrokeDashArrayAttributeWithDefaults)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn CreateStrokeDashArrayAttribute(
        &self,
        dashvalues: &[f32],
        unitvalues: &[CanvasSvgLengthUnits],
    ) -> ::windows::core::Result<CanvasSvgStrokeDashArrayAttribute> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateStrokeDashArrayAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    dashvalues.len() as u32,
                    dashvalues.as_ptr(),
                    unitvalues.len() as u32,
                    unitvalues.as_ptr(),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn LoadElementFromXml(
        &self,
        xmlstring: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<CanvasSvgNamedElement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LoadElementFromXml)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(xmlstring),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`, `"Storage_Streams"`*
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn LoadElementAsync<P0, E0>(
        &self,
        stream: P0,
    ) -> ::windows::core::Result<
        ::windows::Foundation::IAsyncOperation::<CanvasSvgNamedElement>,
    >
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<::windows::Storage::Streams::IRandomAccessStream>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LoadElementAsync)(
                    ::windows::core::Vtable::as_raw(this),
                    stream.try_into().map_err(|e| e.into())?.abi(),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn CreateEmpty<P0, E0>(
        resourcecreator: P0,
    ) -> ::windows::core::Result<CanvasSvgDocument>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasSvgDocumentFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateEmpty)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    pub fn LoadFromXml<P0, E0>(
        resourcecreator: P0,
        xmlstring: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<CanvasSvgDocument>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasSvgDocumentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LoadFromXml)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    ::core::mem::transmute_copy(xmlstring),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation"`, `"Storage_Streams"`*
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn LoadAsync<P0, E0, P1, E1>(
        resourcecreator: P0,
        stream: P1,
    ) -> ::windows::core::Result<
        ::windows::Foundation::IAsyncOperation::<CanvasSvgDocument>,
    >
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<::windows::Storage::Streams::IRandomAccessStream>,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasSvgDocumentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LoadAsync)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    stream.try_into().map_err(|e| e.into())?.abi(),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    pub fn IsSupported(device: &super::CanvasDevice) -> ::windows::core::Result<bool> {
        Self::ICanvasSvgDocumentStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .IsSupported)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(device),
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
    pub fn ICanvasSvgDocumentFactory<
        R,
        F: FnOnce(&ICanvasSvgDocumentFactory) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasSvgDocument,
            ICanvasSvgDocumentFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICanvasSvgDocumentStatics<
        R,
        F: FnOnce(&ICanvasSvgDocumentStatics) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasSvgDocument,
            ICanvasSvgDocumentStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CanvasSvgDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasSvgDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasSvgDocument {}
impl ::core::fmt::Debug for CanvasSvgDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasSvgDocument").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasSvgDocument {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.Svg.CanvasSvgDocument;{a0e34929-3551-44fe-a670-d9b3fd800516})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasSvgDocument {
    type Vtable = ICanvasSvgDocument_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasSvgDocument {
    const IID: ::windows::core::GUID = <ICanvasSvgDocument as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasSvgDocument {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Svg.CanvasSvgDocument";
}
::windows::core::interface_hierarchy!(
    CanvasSvgDocument,::windows::core::IUnknown,::windows::core::IInspectable
);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasSvgDocument> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasSvgDocument) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasSvgDocument> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSvgDocument) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasSvgDocument>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSvgDocument) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CanvasSvgDocument {}
unsafe impl ::core::marker::Sync for CanvasSvgDocument {}
///*Required features: `"Graphics_Canvas_Svg"`*
#[repr(transparent)]
pub struct CanvasSvgNamedElement(::windows::core::IUnknown);
impl CanvasSvgNamedElement {
    pub fn ContainingDocument(&self) -> ::windows::core::Result<CanvasSvgDocument> {
        let this = &::windows::core::Interface::cast::<ICanvasSvgElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ContainingDocument)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::core::Result<CanvasSvgNamedElement> {
        let this = &::windows::core::Interface::cast::<ICanvasSvgElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Parent)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn Device(&self) -> ::windows::core::Result<super::CanvasDevice> {
        let this = &::windows::core::Interface::cast::<ICanvasSvgElement>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Device)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn AppendChild<P0, E0>(&self, child: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasSvgElement>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .AppendChild)(
                    ::windows::core::Vtable::as_raw(this),
                    child.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    pub fn CreateAndAppendNamedChildElement(
        &self,
        childname: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<CanvasSvgNamedElement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateAndAppendNamedChildElement)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(childname),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn CreateAndAppendTextChildElement(
        &self,
        textcontent: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<CanvasSvgTextElement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateAndAppendTextChildElement)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(textcontent),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn FirstChild(&self) -> ::windows::core::Result<ICanvasSvgElement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .FirstChild)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn LastChild(&self) -> ::windows::core::Result<ICanvasSvgElement> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LastChild)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn GetPreviousSibling<P0, E0>(
        &self,
        child: P0,
    ) -> ::windows::core::Result<ICanvasSvgElement>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasSvgElement>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetPreviousSibling)(
                    ::windows::core::Vtable::as_raw(this),
                    child.try_into().map_err(|e| e.into())?.abi(),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn GetNextSibling<P0, E0>(
        &self,
        child: P0,
    ) -> ::windows::core::Result<ICanvasSvgElement>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasSvgElement>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetNextSibling)(
                    ::windows::core::Vtable::as_raw(this),
                    child.try_into().map_err(|e| e.into())?.abi(),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SpecifiedAttributes(
        &self,
    ) -> ::windows::core::Result<::windows::core::Array<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .SpecifiedAttributes)(
                    ::windows::core::Vtable::as_raw(this),
                    ::windows::core::Array::<
                        ::windows::core::HSTRING,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn Tag(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Tag)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn HasChildren(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .HasChildren)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn InsertChildBefore<P0, E0, P1, E1>(
        &self,
        child: P0,
        referencechild: P1,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasSvgElement>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasSvgElement>,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .InsertChildBefore)(
                    ::windows::core::Vtable::as_raw(this),
                    child.try_into().map_err(|e| e.into())?.abi(),
                    referencechild.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    pub fn IsAttributeSpecified(
        &self,
        attributename: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .IsAttributeSpecified)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn IsAttributeSpecifiedWithInherhited(
        &self,
        attributename: &::windows::core::HSTRING,
        inherited: bool,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .IsAttributeSpecifiedWithInherhited)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    inherited,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn RemoveAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .RemoveAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                )
                .ok()
        }
    }
    pub fn RemoveChild<P0, E0>(&self, child: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasSvgElement>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .RemoveChild)(
                    ::windows::core::Vtable::as_raw(this),
                    child.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    pub fn ReplaceChild<P0, E0, P1, E1>(
        &self,
        newchild: P0,
        oldchild: P1,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasSvgElement>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasSvgElement>,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .ReplaceChild)(
                    ::windows::core::Vtable::as_raw(this),
                    newchild.try_into().map_err(|e| e.into())?.abi(),
                    oldchild.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    pub fn SetStringAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
        attributevalue: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetStringAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    ::core::mem::transmute_copy(attributevalue),
                )
                .ok()
        }
    }
    pub fn GetStringAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetStringAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetAttribute<P0, E0>(
        &self,
        attributename: &::windows::core::HSTRING,
        attributevalue: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasSvgAttribute>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    attributevalue.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    pub fn GetAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<ICanvasSvgAttribute> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetIdAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
        attributevalue: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetIdAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    ::core::mem::transmute_copy(attributevalue),
                )
                .ok()
        }
    }
    pub fn GetIdAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetIdAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetFloatAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
        attributevalue: f32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetFloatAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    attributevalue,
                )
                .ok()
        }
    }
    pub fn GetFloatAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetFloatAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn SetColorAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
        attributevalue: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetColorAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    attributevalue,
                )
                .ok()
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn GetColorAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetColorAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`*
    #[cfg(feature = "Graphics_Canvas_Geometry")]
    pub fn SetFilledRegionDeterminationAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
        attributevalue: super::Geometry::CanvasFilledRegionDetermination,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetFilledRegionDeterminationAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    attributevalue,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`*
    #[cfg(feature = "Graphics_Canvas_Geometry")]
    pub fn GetFilledRegionDeterminationAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<super::Geometry::CanvasFilledRegionDetermination> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetFilledRegionDeterminationAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetDisplayAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
        attributevalue: CanvasSvgDisplay,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetDisplayAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    attributevalue,
                )
                .ok()
        }
    }
    pub fn GetDisplayAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<CanvasSvgDisplay> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetDisplayAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetOverflowAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
        attributevalue: CanvasSvgOverflow,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetOverflowAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    attributevalue,
                )
                .ok()
        }
    }
    pub fn GetOverflowAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<CanvasSvgOverflow> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetOverflowAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`*
    #[cfg(feature = "Graphics_Canvas_Geometry")]
    pub fn SetCapStyleAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
        attributevalue: super::Geometry::CanvasCapStyle,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetCapStyleAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    attributevalue,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`*
    #[cfg(feature = "Graphics_Canvas_Geometry")]
    pub fn GetCapStyleAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<super::Geometry::CanvasCapStyle> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetCapStyleAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`*
    #[cfg(feature = "Graphics_Canvas_Geometry")]
    pub fn SetLineJoinAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
        attributevalue: super::Geometry::CanvasLineJoin,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetLineJoinAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    attributevalue,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`*
    #[cfg(feature = "Graphics_Canvas_Geometry")]
    pub fn GetLineJoinAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<super::Geometry::CanvasLineJoin> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetLineJoinAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetVisibilityAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
        attributevalue: CanvasSvgVisibility,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetVisibilityAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    attributevalue,
                )
                .ok()
        }
    }
    pub fn GetVisibilityAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<CanvasSvgVisibility> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetVisibilityAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetTransformAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
        attributevalue: ::windows::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetTransformAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    attributevalue,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn GetTransformAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Matrix3x2> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetTransformAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetUnitsAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
        attributevalue: CanvasSvgUnits,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetUnitsAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    attributevalue,
                )
                .ok()
        }
    }
    pub fn GetUnitsAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<CanvasSvgUnits> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetUnitsAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetEdgeBehaviorAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
        attributevalue: super::CanvasEdgeBehavior,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetEdgeBehaviorAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    attributevalue,
                )
                .ok()
        }
    }
    pub fn GetEdgeBehaviorAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<super::CanvasEdgeBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetEdgeBehaviorAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn SetRectangleAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
        attributevalue: ::windows::Foundation::Rect,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetRectangleAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    attributevalue,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn GetRectangleAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetRectangleAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetLengthAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
        value: f32,
        units: CanvasSvgLengthUnits,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetLengthAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    value,
                    units,
                )
                .ok()
        }
    }
    pub fn GetLengthAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
        units: &mut CanvasSvgLengthUnits,
    ) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetLengthAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    units,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetAspectRatioAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
        alignment: CanvasSvgAspectAlignment,
        meetorslice: CanvasSvgAspectScaling,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetAspectRatioAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    alignment,
                    meetorslice,
                )
                .ok()
        }
    }
    pub fn GetAspectRatioAttribute(
        &self,
        attributename: &::windows::core::HSTRING,
        meetorslice: &mut CanvasSvgAspectScaling,
    ) -> ::windows::core::Result<CanvasSvgAspectAlignment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetAspectRatioAttribute)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(attributename),
                    meetorslice,
                    result__.as_mut_ptr(),
                )
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
impl ::core::clone::Clone for CanvasSvgNamedElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasSvgNamedElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasSvgNamedElement {}
impl ::core::fmt::Debug for CanvasSvgNamedElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasSvgNamedElement").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasSvgNamedElement {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.Svg.CanvasSvgNamedElement;{d8b7cb94-8167-495d-9c71-5e97e5d08d2b})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasSvgNamedElement {
    type Vtable = ICanvasSvgNamedElement_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasSvgNamedElement {
    const IID: ::windows::core::GUID = <ICanvasSvgNamedElement as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasSvgNamedElement {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Svg.CanvasSvgNamedElement";
}
::windows::core::interface_hierarchy!(
    CanvasSvgNamedElement,::windows::core::IUnknown,::windows::core::IInspectable
);
impl ::core::convert::TryFrom<CanvasSvgNamedElement> for ICanvasSvgElement {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasSvgNamedElement) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CanvasSvgNamedElement> for ICanvasSvgElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSvgNamedElement) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&CanvasSvgNamedElement>
for ::windows::core::InParam<ICanvasSvgElement> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSvgNamedElement) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasSvgNamedElement>
for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasSvgNamedElement) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasSvgNamedElement>
for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSvgNamedElement) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasSvgNamedElement>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSvgNamedElement) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CanvasSvgNamedElement {}
unsafe impl ::core::marker::Sync for CanvasSvgNamedElement {}
///*Required features: `"Graphics_Canvas_Svg"`*
#[repr(transparent)]
pub struct CanvasSvgPaintAttribute(::windows::core::IUnknown);
impl CanvasSvgPaintAttribute {
    pub fn Clone(&self) -> ::windows::core::Result<ICanvasSvgAttribute> {
        let this = &::windows::core::Interface::cast::<ICanvasSvgAttribute>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Clone)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn GetElement(&self) -> ::windows::core::Result<CanvasSvgNamedElement> {
        let this = &::windows::core::Interface::cast::<ICanvasSvgAttribute>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetElement)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn Device(&self) -> ::windows::core::Result<super::CanvasDevice> {
        let this = &::windows::core::Interface::cast::<ICanvasSvgAttribute>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Device)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn SetPaintType(
        &self,
        value: CanvasSvgPaintType,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetPaintType)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn PaintType(&self) -> ::windows::core::Result<CanvasSvgPaintType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .PaintType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
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
    pub fn SetId(
        &self,
        value: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetId)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(value),
                )
                .ok()
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
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
impl ::core::clone::Clone for CanvasSvgPaintAttribute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasSvgPaintAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasSvgPaintAttribute {}
impl ::core::fmt::Debug for CanvasSvgPaintAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasSvgPaintAttribute").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasSvgPaintAttribute {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.Svg.CanvasSvgPaintAttribute;{653786a8-f3ab-4083-991d-9748aa86bd6e})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasSvgPaintAttribute {
    type Vtable = ICanvasSvgPaintAttribute_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasSvgPaintAttribute {
    const IID: ::windows::core::GUID = <ICanvasSvgPaintAttribute as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasSvgPaintAttribute {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Svg.CanvasSvgPaintAttribute";
}
::windows::core::interface_hierarchy!(
    CanvasSvgPaintAttribute,::windows::core::IUnknown,::windows::core::IInspectable
);
impl ::core::convert::TryFrom<CanvasSvgPaintAttribute> for ICanvasSvgAttribute {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasSvgPaintAttribute) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CanvasSvgPaintAttribute> for ICanvasSvgAttribute {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSvgPaintAttribute) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&CanvasSvgPaintAttribute>
for ::windows::core::InParam<ICanvasSvgAttribute> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSvgPaintAttribute) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasSvgPaintAttribute>
for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasSvgPaintAttribute) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasSvgPaintAttribute>
for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSvgPaintAttribute) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasSvgPaintAttribute>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSvgPaintAttribute) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CanvasSvgPaintAttribute {}
unsafe impl ::core::marker::Sync for CanvasSvgPaintAttribute {}
///*Required features: `"Graphics_Canvas_Svg"`*
#[repr(transparent)]
pub struct CanvasSvgPathAttribute(::windows::core::IUnknown);
impl CanvasSvgPathAttribute {
    pub fn Clone(&self) -> ::windows::core::Result<ICanvasSvgAttribute> {
        let this = &::windows::core::Interface::cast::<ICanvasSvgAttribute>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Clone)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn GetElement(&self) -> ::windows::core::Result<CanvasSvgNamedElement> {
        let this = &::windows::core::Interface::cast::<ICanvasSvgAttribute>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetElement)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn Device(&self) -> ::windows::core::Result<super::CanvasDevice> {
        let this = &::windows::core::Interface::cast::<ICanvasSvgAttribute>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Device)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`*
    #[cfg(feature = "Graphics_Canvas_Geometry")]
    pub fn CreatePathGeometry(
        &self,
    ) -> ::windows::core::Result<super::Geometry::CanvasGeometry> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreatePathGeometry)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`*
    #[cfg(feature = "Graphics_Canvas_Geometry")]
    pub fn CreatePathGeometryWithFill(
        &self,
        fill: super::Geometry::CanvasFilledRegionDetermination,
    ) -> ::windows::core::Result<super::Geometry::CanvasGeometry> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreatePathGeometryWithFill)(
                    ::windows::core::Vtable::as_raw(this),
                    fill,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn Commands(
        &self,
    ) -> ::windows::core::Result<::windows::core::Array<CanvasSvgPathCommand>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Commands)(
                    ::windows::core::Vtable::as_raw(this),
                    ::windows::core::Array::<
                        CanvasSvgPathCommand,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn GetCommands(
        &self,
        startindex: i32,
        elementcount: i32,
    ) -> ::windows::core::Result<::windows::core::Array<CanvasSvgPathCommand>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetCommands)(
                    ::windows::core::Vtable::as_raw(this),
                    startindex,
                    elementcount,
                    ::windows::core::Array::<
                        CanvasSvgPathCommand,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn SegmentData(&self) -> ::windows::core::Result<::windows::core::Array<f32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .SegmentData)(
                    ::windows::core::Vtable::as_raw(this),
                    ::windows::core::Array::<
                        f32,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn GetSegmentData(
        &self,
        startindex: i32,
        elementcount: i32,
    ) -> ::windows::core::Result<::windows::core::Array<f32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetSegmentData)(
                    ::windows::core::Vtable::as_raw(this),
                    startindex,
                    elementcount,
                    ::windows::core::Array::<
                        f32,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn RemoveCommandsAtEnd(
        &self,
        commandscount: i32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .RemoveCommandsAtEnd)(
                    ::windows::core::Vtable::as_raw(this),
                    commandscount,
                )
                .ok()
        }
    }
    pub fn RemoveSegmentDataAtEnd(
        &self,
        commandscount: i32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .RemoveSegmentDataAtEnd)(
                    ::windows::core::Vtable::as_raw(this),
                    commandscount,
                )
                .ok()
        }
    }
    pub fn SetCommands(
        &self,
        startindex: i32,
        commands: &[CanvasSvgPathCommand],
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetCommands)(
                    ::windows::core::Vtable::as_raw(this),
                    startindex,
                    commands.len() as u32,
                    commands.as_ptr(),
                )
                .ok()
        }
    }
    pub fn SetSegmentData(
        &self,
        startindex: i32,
        segmentdata: &[f32],
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetSegmentData)(
                    ::windows::core::Vtable::as_raw(this),
                    startindex,
                    segmentdata.len() as u32,
                    segmentdata.as_ptr(),
                )
                .ok()
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
impl ::core::clone::Clone for CanvasSvgPathAttribute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasSvgPathAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasSvgPathAttribute {}
impl ::core::fmt::Debug for CanvasSvgPathAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasSvgPathAttribute").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasSvgPathAttribute {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.Svg.CanvasSvgPathAttribute;{652786a8-f3ab-4083-991d-9748ab86bd6e})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasSvgPathAttribute {
    type Vtable = ICanvasSvgPathAttribute_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasSvgPathAttribute {
    const IID: ::windows::core::GUID = <ICanvasSvgPathAttribute as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasSvgPathAttribute {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Svg.CanvasSvgPathAttribute";
}
::windows::core::interface_hierarchy!(
    CanvasSvgPathAttribute,::windows::core::IUnknown,::windows::core::IInspectable
);
impl ::core::convert::TryFrom<CanvasSvgPathAttribute> for ICanvasSvgAttribute {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasSvgPathAttribute) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CanvasSvgPathAttribute> for ICanvasSvgAttribute {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSvgPathAttribute) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&CanvasSvgPathAttribute>
for ::windows::core::InParam<ICanvasSvgAttribute> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSvgPathAttribute) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasSvgPathAttribute>
for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasSvgPathAttribute) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasSvgPathAttribute>
for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSvgPathAttribute) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasSvgPathAttribute>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSvgPathAttribute) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CanvasSvgPathAttribute {}
unsafe impl ::core::marker::Sync for CanvasSvgPathAttribute {}
///*Required features: `"Graphics_Canvas_Svg"`*
#[repr(transparent)]
pub struct CanvasSvgPointsAttribute(::windows::core::IUnknown);
impl CanvasSvgPointsAttribute {
    pub fn Clone(&self) -> ::windows::core::Result<ICanvasSvgAttribute> {
        let this = &::windows::core::Interface::cast::<ICanvasSvgAttribute>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Clone)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn GetElement(&self) -> ::windows::core::Result<CanvasSvgNamedElement> {
        let this = &::windows::core::Interface::cast::<ICanvasSvgAttribute>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetElement)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn Device(&self) -> ::windows::core::Result<super::CanvasDevice> {
        let this = &::windows::core::Interface::cast::<ICanvasSvgAttribute>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Device)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Points(
        &self,
    ) -> ::windows::core::Result<
        ::windows::core::Array<::windows::Foundation::Numerics::Vector2>,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Points)(
                    ::windows::core::Vtable::as_raw(this),
                    ::windows::core::Array::<
                        ::windows::Foundation::Numerics::Vector2,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn GetPoints(
        &self,
        startindex: i32,
        elementcount: i32,
    ) -> ::windows::core::Result<
        ::windows::core::Array<::windows::Foundation::Numerics::Vector2>,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetPoints)(
                    ::windows::core::Vtable::as_raw(this),
                    startindex,
                    elementcount,
                    ::windows::core::Array::<
                        ::windows::Foundation::Numerics::Vector2,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn RemovePointsAtEnd(&self, pointcount: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .RemovePointsAtEnd)(::windows::core::Vtable::as_raw(this), pointcount)
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetPoints(
        &self,
        startindex: i32,
        points: &[::windows::Foundation::Numerics::Vector2],
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetPoints)(
                    ::windows::core::Vtable::as_raw(this),
                    startindex,
                    points.len() as u32,
                    points.as_ptr(),
                )
                .ok()
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
impl ::core::clone::Clone for CanvasSvgPointsAttribute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasSvgPointsAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasSvgPointsAttribute {}
impl ::core::fmt::Debug for CanvasSvgPointsAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasSvgPointsAttribute").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasSvgPointsAttribute {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.Svg.CanvasSvgPointsAttribute;{652786a8-f3ab-4083-991d-9748aa86bd6f})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasSvgPointsAttribute {
    type Vtable = ICanvasSvgPointsAttribute_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasSvgPointsAttribute {
    const IID: ::windows::core::GUID = <ICanvasSvgPointsAttribute as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasSvgPointsAttribute {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Svg.CanvasSvgPointsAttribute";
}
::windows::core::interface_hierarchy!(
    CanvasSvgPointsAttribute,::windows::core::IUnknown,::windows::core::IInspectable
);
impl ::core::convert::TryFrom<CanvasSvgPointsAttribute> for ICanvasSvgAttribute {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasSvgPointsAttribute) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CanvasSvgPointsAttribute> for ICanvasSvgAttribute {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSvgPointsAttribute) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&CanvasSvgPointsAttribute>
for ::windows::core::InParam<ICanvasSvgAttribute> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSvgPointsAttribute) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasSvgPointsAttribute>
for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasSvgPointsAttribute) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasSvgPointsAttribute>
for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSvgPointsAttribute) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasSvgPointsAttribute>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSvgPointsAttribute) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CanvasSvgPointsAttribute {}
unsafe impl ::core::marker::Sync for CanvasSvgPointsAttribute {}
///*Required features: `"Graphics_Canvas_Svg"`*
#[repr(transparent)]
pub struct CanvasSvgStrokeDashArrayAttribute(::windows::core::IUnknown);
impl CanvasSvgStrokeDashArrayAttribute {
    pub fn Clone(&self) -> ::windows::core::Result<ICanvasSvgAttribute> {
        let this = &::windows::core::Interface::cast::<ICanvasSvgAttribute>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Clone)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn GetElement(&self) -> ::windows::core::Result<CanvasSvgNamedElement> {
        let this = &::windows::core::Interface::cast::<ICanvasSvgAttribute>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetElement)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn Device(&self) -> ::windows::core::Result<super::CanvasDevice> {
        let this = &::windows::core::Interface::cast::<ICanvasSvgAttribute>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Device)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn GetDashes(&self) -> ::windows::core::Result<::windows::core::Array<f32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetDashes)(
                    ::windows::core::Vtable::as_raw(this),
                    ::windows::core::Array::<
                        f32,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn GetDashesWithUnits(
        &self,
        startindex: i32,
        elementcount: i32,
        outputunitselements: &mut ::windows::core::Array<CanvasSvgLengthUnits>,
    ) -> ::windows::core::Result<::windows::core::Array<f32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetDashesWithUnits)(
                    ::windows::core::Vtable::as_raw(this),
                    startindex,
                    elementcount,
                    outputunitselements.set_abi_len(),
                    outputunitselements as *mut _ as _,
                    ::windows::core::Array::<
                        f32,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn RemoveDashesAtEnd(&self, dashcount: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .RemoveDashesAtEnd)(::windows::core::Vtable::as_raw(this), dashcount)
                .ok()
        }
    }
    pub fn SetDashes(
        &self,
        startindex: i32,
        dashes: &[f32],
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetDashes)(
                    ::windows::core::Vtable::as_raw(this),
                    startindex,
                    dashes.len() as u32,
                    dashes.as_ptr(),
                )
                .ok()
        }
    }
    pub fn SetDashesWithUnit(
        &self,
        startindex: i32,
        dashes: &[f32],
        units: CanvasSvgLengthUnits,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetDashesWithUnit)(
                    ::windows::core::Vtable::as_raw(this),
                    startindex,
                    dashes.len() as u32,
                    dashes.as_ptr(),
                    units,
                )
                .ok()
        }
    }
    pub fn SetDashesWithUnits(
        &self,
        startindex: i32,
        dashvalues: &[f32],
        unitvalues: &[CanvasSvgLengthUnits],
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetDashesWithUnits)(
                    ::windows::core::Vtable::as_raw(this),
                    startindex,
                    dashvalues.len() as u32,
                    dashvalues.as_ptr(),
                    unitvalues.len() as u32,
                    unitvalues.as_ptr(),
                )
                .ok()
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
impl ::core::clone::Clone for CanvasSvgStrokeDashArrayAttribute {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasSvgStrokeDashArrayAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasSvgStrokeDashArrayAttribute {}
impl ::core::fmt::Debug for CanvasSvgStrokeDashArrayAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasSvgStrokeDashArrayAttribute").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasSvgStrokeDashArrayAttribute {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.Svg.CanvasSvgStrokeDashArrayAttribute;{652786a8-f3ab-4083-991d-9748aa86bd70})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasSvgStrokeDashArrayAttribute {
    type Vtable = ICanvasSvgStrokeDashArrayAttribute_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasSvgStrokeDashArrayAttribute {
    const IID: ::windows::core::GUID = <ICanvasSvgStrokeDashArrayAttribute as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasSvgStrokeDashArrayAttribute {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Svg.CanvasSvgStrokeDashArrayAttribute";
}
::windows::core::interface_hierarchy!(
    CanvasSvgStrokeDashArrayAttribute,::windows::core::IUnknown,::windows::core::IInspectable
);
impl ::core::convert::TryFrom<CanvasSvgStrokeDashArrayAttribute>
for ICanvasSvgAttribute {
    type Error = ::windows::core::Error;
    fn try_from(
        value: CanvasSvgStrokeDashArrayAttribute,
    ) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CanvasSvgStrokeDashArrayAttribute>
for ICanvasSvgAttribute {
    type Error = ::windows::core::Error;
    fn try_from(
        value: &CanvasSvgStrokeDashArrayAttribute,
    ) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&CanvasSvgStrokeDashArrayAttribute>
for ::windows::core::InParam<ICanvasSvgAttribute> {
    type Error = ::windows::core::Error;
    fn try_from(
        value: &CanvasSvgStrokeDashArrayAttribute,
    ) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasSvgStrokeDashArrayAttribute>
for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(
        value: CanvasSvgStrokeDashArrayAttribute,
    ) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasSvgStrokeDashArrayAttribute>
for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(
        value: &CanvasSvgStrokeDashArrayAttribute,
    ) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasSvgStrokeDashArrayAttribute>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(
        value: &CanvasSvgStrokeDashArrayAttribute,
    ) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CanvasSvgStrokeDashArrayAttribute {}
unsafe impl ::core::marker::Sync for CanvasSvgStrokeDashArrayAttribute {}
///*Required features: `"Graphics_Canvas_Svg"`*
#[repr(transparent)]
pub struct CanvasSvgTextElement(::windows::core::IUnknown);
impl CanvasSvgTextElement {
    pub fn SetText(
        &self,
        value: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetText)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(value),
                )
                .ok()
        }
    }
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Text)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
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
impl ::core::clone::Clone for CanvasSvgTextElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasSvgTextElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasSvgTextElement {}
impl ::core::fmt::Debug for CanvasSvgTextElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasSvgTextElement").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasSvgTextElement {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.Svg.CanvasSvgTextElement;{652786a8-f3ab-4083-991d-9748aa86bd6d})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasSvgTextElement {
    type Vtable = ICanvasSvgTextElement_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasSvgTextElement {
    const IID: ::windows::core::GUID = <ICanvasSvgTextElement as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasSvgTextElement {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Svg.CanvasSvgTextElement";
}
::windows::core::interface_hierarchy!(
    CanvasSvgTextElement,::windows::core::IUnknown,::windows::core::IInspectable
);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasSvgTextElement>
for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasSvgTextElement) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasSvgTextElement>
for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSvgTextElement) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasSvgTextElement>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSvgTextElement) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CanvasSvgTextElement {}
unsafe impl ::core::marker::Sync for CanvasSvgTextElement {}
///*Required features: `"Graphics_Canvas_Svg"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasSvgAspectAlignment(pub i32);
impl CanvasSvgAspectAlignment {
    pub const None: Self = Self(0i32);
    pub const XMinYMin: Self = Self(1i32);
    pub const XMidYMin: Self = Self(2i32);
    pub const XMaxYMin: Self = Self(3i32);
    pub const XMinYMid: Self = Self(4i32);
    pub const XMidYMid: Self = Self(5i32);
    pub const XMaxYMid: Self = Self(6i32);
    pub const XMinYMax: Self = Self(7i32);
    pub const XMidYMax: Self = Self(8i32);
    pub const XMaxYMax: Self = Self(9i32);
}
impl ::core::marker::Copy for CanvasSvgAspectAlignment {}
impl ::core::clone::Clone for CanvasSvgAspectAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasSvgAspectAlignment {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasSvgAspectAlignment {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasSvgAspectAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasSvgAspectAlignment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasSvgAspectAlignment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Svg.CanvasSvgAspectAlignment;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Svg"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasSvgAspectScaling(pub i32);
impl CanvasSvgAspectScaling {
    pub const Meet: Self = Self(0i32);
    pub const Slice: Self = Self(1i32);
}
impl ::core::marker::Copy for CanvasSvgAspectScaling {}
impl ::core::clone::Clone for CanvasSvgAspectScaling {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasSvgAspectScaling {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasSvgAspectScaling {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasSvgAspectScaling {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasSvgAspectScaling").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasSvgAspectScaling {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Svg.CanvasSvgAspectScaling;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Svg"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasSvgDisplay(pub i32);
impl CanvasSvgDisplay {
    pub const Inline: Self = Self(0i32);
    pub const None: Self = Self(1i32);
}
impl ::core::marker::Copy for CanvasSvgDisplay {}
impl ::core::clone::Clone for CanvasSvgDisplay {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasSvgDisplay {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasSvgDisplay {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasSvgDisplay {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasSvgDisplay").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasSvgDisplay {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Svg.CanvasSvgDisplay;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Svg"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasSvgLengthUnits(pub i32);
impl CanvasSvgLengthUnits {
    pub const Number: Self = Self(0i32);
    pub const Percentage: Self = Self(1i32);
}
impl ::core::marker::Copy for CanvasSvgLengthUnits {}
impl ::core::clone::Clone for CanvasSvgLengthUnits {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasSvgLengthUnits {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasSvgLengthUnits {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasSvgLengthUnits {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasSvgLengthUnits").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasSvgLengthUnits {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Svg.CanvasSvgLengthUnits;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Svg"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasSvgOverflow(pub i32);
impl CanvasSvgOverflow {
    pub const DoNotClipToViewport: Self = Self(0i32);
    pub const ClipToViewport: Self = Self(1i32);
}
impl ::core::marker::Copy for CanvasSvgOverflow {}
impl ::core::clone::Clone for CanvasSvgOverflow {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasSvgOverflow {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasSvgOverflow {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasSvgOverflow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasSvgOverflow").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasSvgOverflow {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Svg.CanvasSvgOverflow;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Svg"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasSvgPaintType(pub i32);
impl CanvasSvgPaintType {
    pub const None: Self = Self(0i32);
    pub const Color: Self = Self(1i32);
    pub const CurrentColor: Self = Self(2i32);
    pub const Uri: Self = Self(3i32);
    pub const UriThenNone: Self = Self(4i32);
    pub const UriThenColor: Self = Self(5i32);
    pub const UriThenCurrentColor: Self = Self(6i32);
}
impl ::core::marker::Copy for CanvasSvgPaintType {}
impl ::core::clone::Clone for CanvasSvgPaintType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasSvgPaintType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasSvgPaintType {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasSvgPaintType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasSvgPaintType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasSvgPaintType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Svg.CanvasSvgPaintType;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Svg"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasSvgPathCommand(pub i32);
impl CanvasSvgPathCommand {
    pub const ClosePath: Self = Self(0i32);
    pub const MoveAbsolute: Self = Self(1i32);
    pub const MoveRelative: Self = Self(2i32);
    pub const LineAbsolute: Self = Self(3i32);
    pub const LineRelative: Self = Self(4i32);
    pub const CubicAbsolute: Self = Self(5i32);
    pub const CubicRelative: Self = Self(6i32);
    pub const QuadraticAbsolute: Self = Self(7i32);
    pub const QuadraticRelative: Self = Self(8i32);
    pub const ArcAbsolute: Self = Self(9i32);
    pub const ArcRelative: Self = Self(10i32);
    pub const HorizontalAbsolute: Self = Self(11i32);
    pub const HorizontalRelative: Self = Self(12i32);
    pub const VerticalAbsolute: Self = Self(13i32);
    pub const VerticalRelative: Self = Self(14i32);
    pub const CubicSmoothAbsolute: Self = Self(15i32);
    pub const CubicSmoothRelative: Self = Self(16i32);
    pub const QuadraticSmoothAbsolute: Self = Self(17i32);
    pub const QuadraticSmoothRelative: Self = Self(18i32);
}
impl ::core::marker::Copy for CanvasSvgPathCommand {}
impl ::core::clone::Clone for CanvasSvgPathCommand {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasSvgPathCommand {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasSvgPathCommand {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasSvgPathCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasSvgPathCommand").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasSvgPathCommand {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Svg.CanvasSvgPathCommand;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Svg"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasSvgUnits(pub i32);
impl CanvasSvgUnits {
    pub const UserSpaceOnUse: Self = Self(0i32);
    pub const ObjectBoundingBox: Self = Self(1i32);
}
impl ::core::marker::Copy for CanvasSvgUnits {}
impl ::core::clone::Clone for CanvasSvgUnits {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasSvgUnits {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasSvgUnits {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasSvgUnits {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasSvgUnits").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasSvgUnits {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Svg.CanvasSvgUnits;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Svg"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasSvgVisibility(pub i32);
impl CanvasSvgVisibility {
    pub const Visible: Self = Self(0i32);
    pub const Hidden: Self = Self(1i32);
}
impl ::core::marker::Copy for CanvasSvgVisibility {}
impl ::core::clone::Clone for CanvasSvgVisibility {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasSvgVisibility {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasSvgVisibility {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasSvgVisibility {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasSvgVisibility").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasSvgVisibility {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Svg.CanvasSvgVisibility;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
