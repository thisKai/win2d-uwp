#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, clippy::all)]
#[cfg(feature = "Graphics_Canvas_UI_Composition")]
pub mod Composition;
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasCreateResourcesEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasCreateResourcesEventArgs {
    type Vtable = ICanvasCreateResourcesEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasCreateResourcesEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xedc52108_f6ba_4a09_9fa3_10c97a24e49a,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasCreateResourcesEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasCreateResourcesReason,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TrackAsyncAction: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        action: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrackAsyncAction: usize,
    #[cfg(feature = "Foundation")]
    pub GetTrackedAction: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetTrackedAction: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasCreateResourcesEventArgsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasCreateResourcesEventArgsFactory {
    type Vtable = ICanvasCreateResourcesEventArgsFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasCreateResourcesEventArgsFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x3a21c766_0781_4389_bbc3_86b1f5022af1,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasCreateResourcesEventArgsFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        createresourcesreason: CanvasCreateResourcesReason,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
///*Required features: `"Graphics_Canvas_UI"`*
#[repr(transparent)]
pub struct CanvasCreateResourcesEventArgs(::windows::core::IUnknown);
impl CanvasCreateResourcesEventArgs {
    pub fn Reason(&self) -> ::windows::core::Result<CanvasCreateResourcesReason> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Reason)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn TrackAsyncAction<P0, E0>(&self, action: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<::windows::Foundation::IAsyncAction>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .TrackAsyncAction)(
                    ::windows::core::Vtable::as_raw(this),
                    action.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn GetTrackedAction(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetTrackedAction)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn Create(
        createresourcesreason: CanvasCreateResourcesReason,
    ) -> ::windows::core::Result<CanvasCreateResourcesEventArgs> {
        Self::ICanvasCreateResourcesEventArgsFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Create)(
                    ::windows::core::Vtable::as_raw(this),
                    createresourcesreason,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICanvasCreateResourcesEventArgsFactory<
        R,
        F: FnOnce(&ICanvasCreateResourcesEventArgsFactory) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasCreateResourcesEventArgs,
            ICanvasCreateResourcesEventArgsFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CanvasCreateResourcesEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasCreateResourcesEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasCreateResourcesEventArgs {}
impl ::core::fmt::Debug for CanvasCreateResourcesEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasCreateResourcesEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasCreateResourcesEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.UI.CanvasCreateResourcesEventArgs;{edc52108-f6ba-4a09-9fa3-10c97a24e49a})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasCreateResourcesEventArgs {
    type Vtable = ICanvasCreateResourcesEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasCreateResourcesEventArgs {
    const IID: ::windows::core::GUID = <ICanvasCreateResourcesEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasCreateResourcesEventArgs {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.UI.CanvasCreateResourcesEventArgs";
}
::windows::core::interface_hierarchy!(
    CanvasCreateResourcesEventArgs,::windows::core::IUnknown,::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for CanvasCreateResourcesEventArgs {}
unsafe impl ::core::marker::Sync for CanvasCreateResourcesEventArgs {}
///*Required features: `"Graphics_Canvas_UI"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasCreateResourcesReason(pub i32);
impl CanvasCreateResourcesReason {
    pub const FirstTime: Self = Self(0i32);
    pub const NewDevice: Self = Self(1i32);
    pub const DpiChanged: Self = Self(2i32);
}
impl ::core::marker::Copy for CanvasCreateResourcesReason {}
impl ::core::clone::Clone for CanvasCreateResourcesReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasCreateResourcesReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasCreateResourcesReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasCreateResourcesReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasCreateResourcesReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasCreateResourcesReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.UI.CanvasCreateResourcesReason;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
///*Required features: `"Graphics_Canvas_UI"`, `"Foundation"`*
#[cfg(feature = "Foundation")]
pub struct CanvasTimingInformation {
    pub UpdateCount: i64,
    pub TotalTime: ::windows::Foundation::TimeSpan,
    pub ElapsedTime: ::windows::Foundation::TimeSpan,
    pub IsRunningSlowly: bool,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for CanvasTimingInformation {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for CanvasTimingInformation {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for CanvasTimingInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CanvasTimingInformation")
            .field("UpdateCount", &self.UpdateCount)
            .field("TotalTime", &self.TotalTime)
            .field("ElapsedTime", &self.ElapsedTime)
            .field("IsRunningSlowly", &self.IsRunningSlowly)
            .finish()
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Abi for CanvasTimingInformation {
    type Abi = Self;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for CanvasTimingInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.Graphics.Canvas.UI.CanvasTimingInformation;i8;struct(Windows.Foundation.TimeSpan;i8);struct(Windows.Foundation.TimeSpan;i8);b1)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for CanvasTimingInformation {
    fn eq(&self, other: &Self) -> bool {
        self.UpdateCount == other.UpdateCount && self.TotalTime == other.TotalTime
            && self.ElapsedTime == other.ElapsedTime
            && self.IsRunningSlowly == other.IsRunningSlowly
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for CanvasTimingInformation {}
#[cfg(feature = "Foundation")]
impl ::core::default::Default for CanvasTimingInformation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
