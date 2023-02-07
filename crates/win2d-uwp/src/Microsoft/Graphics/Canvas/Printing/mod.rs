#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasPreviewEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasPreviewEventArgs {
    type Vtable = ICanvasPreviewEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasPreviewEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x0a6a80a0_b07d_4db2_bdeb_0368bb18c0f8,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasPreviewEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub PageNumber: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Printing")]
    pub PrintTaskOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))]
    PrintTaskOptions: usize,
    pub GetDeferral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub DrawingSession: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasPrintDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasPrintDeferral {
    type Vtable = ICanvasPrintDeferral_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasPrintDeferral {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x08ca99a2_5801_4ea4_8687_896cbda69a47,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasPrintDeferral_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasPrintDocument(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasPrintDocument {
    type Vtable = ICanvasPrintDocument_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasPrintDocument {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x0a99cdee_bf11_49d0_aa34_3ba5c32c51a5,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasPrintDocument_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub PrintTaskOptionsChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrintTaskOptionsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePrintTaskOptionsChanged: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePrintTaskOptionsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub Preview: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Preview: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePreview: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePreview: usize,
    #[cfg(feature = "Foundation")]
    pub Print: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Print: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePrint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePrint: usize,
    pub InvalidatePreview: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetPageCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        count: u32,
    ) -> ::windows::core::HRESULT,
    pub SetIntermediatePageCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        count: u32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasPrintDocumentFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasPrintDocumentFactory {
    type Vtable = ICanvasPrintDocumentFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasPrintDocumentFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xa201af1e_ce4a_401d_a719_2bf004d5c26a,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasPrintDocumentFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateWithDevice: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        device: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasPrintEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasPrintEventArgs {
    type Vtable = ICanvasPrintEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasPrintEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x0c6148c4_0216_4561_a817_34c8942aac8b,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasPrintEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Printing")]
    pub PrintTaskOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))]
    PrintTaskOptions: usize,
    pub Dpi: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetDpi: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateDrawingSession: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasPrintTaskOptionsChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasPrintTaskOptionsChangedEventArgs {
    type Vtable = ICanvasPrintTaskOptionsChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasPrintTaskOptionsChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xf92089ba_6c99_4cac_b28a_b5dcec7a310d,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasPrintTaskOptionsChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CurrentPreviewPageNumber: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub SetNewPreviewPageNumber: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u32,
    ) -> ::windows::core::HRESULT,
    pub NewPreviewPageNumber: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Printing")]
    pub PrintTaskOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))]
    PrintTaskOptions: usize,
}
///*Required features: `"Graphics_Canvas_Printing"`*
#[repr(transparent)]
pub struct CanvasPreviewEventArgs(::windows::core::IUnknown);
impl CanvasPreviewEventArgs {
    pub fn PageNumber(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .PageNumber)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Graphics_Printing"`*
    #[cfg(feature = "Graphics_Printing")]
    pub fn PrintTaskOptions(
        &self,
    ) -> ::windows::core::Result<::windows::Graphics::Printing::PrintTaskOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .PrintTaskOptions)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows::core::Result<CanvasPrintDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetDeferral)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn DrawingSession(
        &self,
    ) -> ::windows::core::Result<super::CanvasDrawingSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .DrawingSession)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for CanvasPreviewEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasPreviewEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasPreviewEventArgs {}
impl ::core::fmt::Debug for CanvasPreviewEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasPreviewEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasPreviewEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.Printing.CanvasPreviewEventArgs;{0a6a80a0-b07d-4db2-bdeb-0368bb18c0f8})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasPreviewEventArgs {
    type Vtable = ICanvasPreviewEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasPreviewEventArgs {
    const IID: ::windows::core::GUID = <ICanvasPreviewEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasPreviewEventArgs {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Printing.CanvasPreviewEventArgs";
}
::windows::core::interface_hierarchy!(
    CanvasPreviewEventArgs,::windows::core::IUnknown,::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for CanvasPreviewEventArgs {}
unsafe impl ::core::marker::Sync for CanvasPreviewEventArgs {}
///*Required features: `"Graphics_Canvas_Printing"`*
#[repr(transparent)]
pub struct CanvasPrintDeferral(::windows::core::IUnknown);
impl CanvasPrintDeferral {
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .Complete)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
}
impl ::core::clone::Clone for CanvasPrintDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasPrintDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasPrintDeferral {}
impl ::core::fmt::Debug for CanvasPrintDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasPrintDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasPrintDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.Printing.CanvasPrintDeferral;{08ca99a2-5801-4ea4-8687-896cbda69a47})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasPrintDeferral {
    type Vtable = ICanvasPrintDeferral_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasPrintDeferral {
    const IID: ::windows::core::GUID = <ICanvasPrintDeferral as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasPrintDeferral {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Printing.CanvasPrintDeferral";
}
::windows::core::interface_hierarchy!(
    CanvasPrintDeferral,::windows::core::IUnknown,::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for CanvasPrintDeferral {}
unsafe impl ::core::marker::Sync for CanvasPrintDeferral {}
///*Required features: `"Graphics_Canvas_Printing"`*
#[repr(transparent)]
pub struct CanvasPrintDocument(::windows::core::IUnknown);
impl CanvasPrintDocument {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasPrintDocument,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn PrintTaskOptionsChanged(
        &self,
        value: &::windows::Foundation::TypedEventHandler::<
            CanvasPrintDocument,
            CanvasPrintTaskOptionsChangedEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .PrintTaskOptionsChanged)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(value),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn RemovePrintTaskOptionsChanged(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .RemovePrintTaskOptionsChanged)(
                    ::windows::core::Vtable::as_raw(this),
                    token,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn Preview(
        &self,
        value: &::windows::Foundation::TypedEventHandler::<
            CanvasPrintDocument,
            CanvasPreviewEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Preview)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(value),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn RemovePreview(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .RemovePreview)(::windows::core::Vtable::as_raw(this), token)
                .ok()
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn Print(
        &self,
        value: &::windows::Foundation::TypedEventHandler::<
            CanvasPrintDocument,
            CanvasPrintEventArgs,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Print)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(value),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn RemovePrint(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .RemovePrint)(::windows::core::Vtable::as_raw(this), token)
                .ok()
        }
    }
    pub fn InvalidatePreview(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .InvalidatePreview)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn SetPageCount(&self, count: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetPageCount)(::windows::core::Vtable::as_raw(this), count)
                .ok()
        }
    }
    pub fn SetIntermediatePageCount(&self, count: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetIntermediatePageCount)(::windows::core::Vtable::as_raw(this), count)
                .ok()
        }
    }
    pub fn CreateWithDevice(
        device: &super::CanvasDevice,
    ) -> ::windows::core::Result<CanvasPrintDocument> {
        Self::ICanvasPrintDocumentFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateWithDevice)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(device),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    pub fn Device(&self) -> ::windows::core::Result<super::CanvasDevice> {
        let this = &::windows::core::Interface::cast::<
            super::ICanvasResourceCreator,
        >(self)?;
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
    #[doc(hidden)]
    pub fn ICanvasPrintDocumentFactory<
        R,
        F: FnOnce(&ICanvasPrintDocumentFactory) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasPrintDocument,
            ICanvasPrintDocumentFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CanvasPrintDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasPrintDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasPrintDocument {}
impl ::core::fmt::Debug for CanvasPrintDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasPrintDocument").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasPrintDocument {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.Printing.CanvasPrintDocument;{0a99cdee-bf11-49d0-aa34-3ba5c32c51a5})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasPrintDocument {
    type Vtable = ICanvasPrintDocument_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasPrintDocument {
    const IID: ::windows::core::GUID = <ICanvasPrintDocument as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasPrintDocument {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Printing.CanvasPrintDocument";
}
::windows::core::interface_hierarchy!(
    CanvasPrintDocument,::windows::core::IUnknown,::windows::core::IInspectable
);
impl ::core::convert::TryFrom<CanvasPrintDocument> for super::ICanvasResourceCreator {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasPrintDocument) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CanvasPrintDocument> for super::ICanvasResourceCreator {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasPrintDocument) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&CanvasPrintDocument>
for ::windows::core::InParam<super::ICanvasResourceCreator> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasPrintDocument) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasPrintDocument> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasPrintDocument) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasPrintDocument>
for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasPrintDocument) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasPrintDocument>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasPrintDocument) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Graphics_Printing")]
impl ::core::convert::TryFrom<CanvasPrintDocument>
for ::windows::Graphics::Printing::IPrintDocumentSource {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasPrintDocument) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Graphics_Printing")]
impl ::core::convert::TryFrom<&CanvasPrintDocument>
for ::windows::Graphics::Printing::IPrintDocumentSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasPrintDocument) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Graphics_Printing")]
impl ::core::convert::TryFrom<&CanvasPrintDocument>
for ::windows::core::InParam<::windows::Graphics::Printing::IPrintDocumentSource> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasPrintDocument) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CanvasPrintDocument {}
unsafe impl ::core::marker::Sync for CanvasPrintDocument {}
///*Required features: `"Graphics_Canvas_Printing"`*
#[repr(transparent)]
pub struct CanvasPrintEventArgs(::windows::core::IUnknown);
impl CanvasPrintEventArgs {
    ///*Required features: `"Graphics_Printing"`*
    #[cfg(feature = "Graphics_Printing")]
    pub fn PrintTaskOptions(
        &self,
    ) -> ::windows::core::Result<::windows::Graphics::Printing::PrintTaskOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .PrintTaskOptions)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn Dpi(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Dpi)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn SetDpi(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetDpi)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn GetDeferral(&self) -> ::windows::core::Result<CanvasPrintDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetDeferral)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn CreateDrawingSession(
        &self,
    ) -> ::windows::core::Result<super::CanvasDrawingSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateDrawingSession)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for CanvasPrintEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasPrintEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasPrintEventArgs {}
impl ::core::fmt::Debug for CanvasPrintEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasPrintEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasPrintEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.Printing.CanvasPrintEventArgs;{0c6148c4-0216-4561-a817-34c8942aac8b})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasPrintEventArgs {
    type Vtable = ICanvasPrintEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasPrintEventArgs {
    const IID: ::windows::core::GUID = <ICanvasPrintEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasPrintEventArgs {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Printing.CanvasPrintEventArgs";
}
::windows::core::interface_hierarchy!(
    CanvasPrintEventArgs,::windows::core::IUnknown,::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for CanvasPrintEventArgs {}
unsafe impl ::core::marker::Sync for CanvasPrintEventArgs {}
///*Required features: `"Graphics_Canvas_Printing"`*
#[repr(transparent)]
pub struct CanvasPrintTaskOptionsChangedEventArgs(::windows::core::IUnknown);
impl CanvasPrintTaskOptionsChangedEventArgs {
    pub fn CurrentPreviewPageNumber(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CurrentPreviewPageNumber)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetNewPreviewPageNumber(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetNewPreviewPageNumber)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn NewPreviewPageNumber(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .NewPreviewPageNumber)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn GetDeferral(&self) -> ::windows::core::Result<CanvasPrintDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetDeferral)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Graphics_Printing"`*
    #[cfg(feature = "Graphics_Printing")]
    pub fn PrintTaskOptions(
        &self,
    ) -> ::windows::core::Result<::windows::Graphics::Printing::PrintTaskOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .PrintTaskOptions)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for CanvasPrintTaskOptionsChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasPrintTaskOptionsChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasPrintTaskOptionsChangedEventArgs {}
impl ::core::fmt::Debug for CanvasPrintTaskOptionsChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasPrintTaskOptionsChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasPrintTaskOptionsChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.Printing.CanvasPrintTaskOptionsChangedEventArgs;{f92089ba-6c99-4cac-b28a-b5dcec7a310d})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasPrintTaskOptionsChangedEventArgs {
    type Vtable = ICanvasPrintTaskOptionsChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasPrintTaskOptionsChangedEventArgs {
    const IID: ::windows::core::GUID = <ICanvasPrintTaskOptionsChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasPrintTaskOptionsChangedEventArgs {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Printing.CanvasPrintTaskOptionsChangedEventArgs";
}
::windows::core::interface_hierarchy!(
    CanvasPrintTaskOptionsChangedEventArgs,::windows::core::IUnknown,::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for CanvasPrintTaskOptionsChangedEventArgs {}
unsafe impl ::core::marker::Sync for CanvasPrintTaskOptionsChangedEventArgs {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
