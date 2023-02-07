#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, clippy::all)]
#[cfg(feature = "Graphics_Canvas_Brushes")]
pub mod Brushes;
#[cfg(feature = "Graphics_Canvas_Effects")]
pub mod Effects;
#[cfg(feature = "Graphics_Canvas_Geometry")]
pub mod Geometry;
#[cfg(feature = "Graphics_Canvas_Printing")]
pub mod Printing;
#[cfg(feature = "Graphics_Canvas_Svg")]
pub mod Svg;
#[cfg(feature = "Graphics_Canvas_Text")]
pub mod Text;
#[cfg(feature = "Graphics_Canvas_UI")]
pub mod UI;
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasActiveLayer(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasActiveLayer {
    type Vtable = ICanvasActiveLayer_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasActiveLayer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x49ecfc58_5e1c_4ee3_8088_542f94e93c60,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasActiveLayer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasBitmap(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasBitmap {
    type Vtable = ICanvasBitmap_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasBitmap {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xc57532ed_709e_4ac2_86be_a1ec3a7fa8fe,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasBitmap_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub SizeInPixels: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Graphics::Imaging::BitmapSize,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SizeInPixels: usize,
    #[cfg(feature = "Foundation")]
    pub Size: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Size,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Size: usize,
    #[cfg(feature = "Foundation")]
    pub Bounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Bounds: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub Format: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Graphics::DirectX::DirectXPixelFormat,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    Format: usize,
    pub AlphaMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasAlphaMode,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SaveToFileAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        filename: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveToFileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SaveToFileWithBitmapFileFormatAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        filename: *mut ::core::ffi::c_void,
        fileformat: CanvasBitmapFileFormat,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveToFileWithBitmapFileFormatAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SaveToFileWithBitmapFileFormatAndQualityAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        filename: *mut ::core::ffi::c_void,
        fileformat: CanvasBitmapFileFormat,
        quality: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveToFileWithBitmapFileFormatAndQualityAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SaveToStreamAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        stream: *mut ::core::ffi::c_void,
        fileformat: CanvasBitmapFileFormat,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SaveToStreamAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SaveToStreamWithQualityAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        stream: *mut ::core::ffi::c_void,
        fileformat: CanvasBitmapFileFormat,
        quality: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SaveToStreamWithQualityAsync: usize,
    pub GetPixelBytes: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut u8,
    ) -> ::windows::core::HRESULT,
    pub GetPixelBytesWithSubrectangle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        left: i32,
        top: i32,
        width: i32,
        height: i32,
        result_size__: *mut u32,
        result__: *mut *mut u8,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetPixelBytesWithBuffer: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        buffer: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetPixelBytesWithBuffer: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GetPixelBytesWithBufferAndSubrectangle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        buffer: *mut ::core::ffi::c_void,
        left: i32,
        top: i32,
        width: i32,
        height: i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetPixelBytesWithBufferAndSubrectangle: usize,
    #[cfg(feature = "UI")]
    pub GetPixelColors: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    GetPixelColors: usize,
    #[cfg(feature = "UI")]
    pub GetPixelColorsWithSubrectangle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        left: i32,
        top: i32,
        width: i32,
        height: i32,
        result_size__: *mut u32,
        result__: *mut *mut ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    GetPixelColorsWithSubrectangle: usize,
    pub SetPixelBytes: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        valueElements_array_size: u32,
        valueelements: *const u8,
    ) -> ::windows::core::HRESULT,
    pub SetPixelBytesWithSubrectangle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        valueElements_array_size: u32,
        valueelements: *const u8,
        left: i32,
        top: i32,
        width: i32,
        height: i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub SetPixelBytesWithBuffer: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        buffer: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetPixelBytesWithBuffer: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetPixelBytesWithBufferAndSubrectangle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        buffer: *mut ::core::ffi::c_void,
        left: i32,
        top: i32,
        width: i32,
        height: i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetPixelBytesWithBufferAndSubrectangle: usize,
    #[cfg(feature = "UI")]
    pub SetPixelColors: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        valueElements_array_size: u32,
        valueelements: *const ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetPixelColors: usize,
    #[cfg(feature = "UI")]
    pub SetPixelColorsWithSubrectangle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        valueElements_array_size: u32,
        valueelements: *const ::windows::UI::Color,
        left: i32,
        top: i32,
        width: i32,
        height: i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetPixelColorsWithSubrectangle: usize,
    pub CopyPixelsFromBitmap: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        otherbitmap: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CopyPixelsFromBitmapWithDestPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        otherbitmap: *mut ::core::ffi::c_void,
        destx: i32,
        desty: i32,
    ) -> ::windows::core::HRESULT,
    pub CopyPixelsFromBitmapWithDestPointAndSourceRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        otherbitmap: *mut ::core::ffi::c_void,
        destx: i32,
        desty: i32,
        sourcerectleft: i32,
        sourcerecttop: i32,
        sourcerectwidth: i32,
        sourcerectheight: i32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasBitmapFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasBitmapFactory {
    type Vtable = ICanvasBitmapFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasBitmapFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xf2d0eb0e_16f3_4bcf_b1d1_04834ab97de4,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasBitmapFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasBitmapStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasBitmapStatics {
    type Vtable = ICanvasBitmapStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasBitmapStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xc8948dea_a41d_4cc2_af9a_fdde01b606dc,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasBitmapStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateFromDirect3D11Surface: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        surface: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateFromDirect3D11Surface: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateFromDirect3D11SurfaceWithDpi: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        surface: *mut ::core::ffi::c_void,
        dpi: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateFromDirect3D11SurfaceWithDpi: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateFromDirect3D11SurfaceWithDpiAndAlpha: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        surface: *mut ::core::ffi::c_void,
        dpi: f32,
        alpha: CanvasAlphaMode,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateFromDirect3D11SurfaceWithDpiAndAlpha: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub CreateFromBytes: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        bytes_array_size: u32,
        bytes: *const u8,
        widthinpixels: i32,
        heightinpixels: i32,
        format: ::windows::Graphics::DirectX::DirectXPixelFormat,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    CreateFromBytes: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub CreateFromBytesWithDpi: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        bytes_array_size: u32,
        bytes: *const u8,
        widthinpixels: i32,
        heightinpixels: i32,
        format: ::windows::Graphics::DirectX::DirectXPixelFormat,
        dpi: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    CreateFromBytesWithDpi: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub CreateFromBytesWithDpiAndAlpha: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        bytes_array_size: u32,
        bytes: *const u8,
        widthinpixels: i32,
        heightinpixels: i32,
        format: ::windows::Graphics::DirectX::DirectXPixelFormat,
        dpi: f32,
        alpha: CanvasAlphaMode,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    CreateFromBytesWithDpiAndAlpha: usize,
    #[cfg(all(feature = "Graphics_DirectX", feature = "Storage_Streams"))]
    pub CreateFromBytesWithBuffer: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        buffer: *mut ::core::ffi::c_void,
        widthinpixels: i32,
        heightinpixels: i32,
        format: ::windows::Graphics::DirectX::DirectXPixelFormat,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_DirectX", feature = "Storage_Streams")))]
    CreateFromBytesWithBuffer: usize,
    #[cfg(all(feature = "Graphics_DirectX", feature = "Storage_Streams"))]
    pub CreateFromBytesWithBufferAndDpi: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        buffer: *mut ::core::ffi::c_void,
        widthinpixels: i32,
        heightinpixels: i32,
        format: ::windows::Graphics::DirectX::DirectXPixelFormat,
        dpi: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_DirectX", feature = "Storage_Streams")))]
    CreateFromBytesWithBufferAndDpi: usize,
    #[cfg(all(feature = "Graphics_DirectX", feature = "Storage_Streams"))]
    pub CreateFromBytesWithBufferAndDpiAndAlpha: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        buffer: *mut ::core::ffi::c_void,
        widthinpixels: i32,
        heightinpixels: i32,
        format: ::windows::Graphics::DirectX::DirectXPixelFormat,
        dpi: f32,
        alpha: CanvasAlphaMode,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_DirectX", feature = "Storage_Streams")))]
    CreateFromBytesWithBufferAndDpiAndAlpha: usize,
    #[cfg(feature = "UI")]
    pub CreateFromColors: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        colors_array_size: u32,
        colors: *const ::windows::UI::Color,
        widthinpixels: i32,
        heightinpixels: i32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    CreateFromColors: usize,
    #[cfg(feature = "UI")]
    pub CreateFromColorsWithDpi: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        colors_array_size: u32,
        colors: *const ::windows::UI::Color,
        widthinpixels: i32,
        heightinpixels: i32,
        dpi: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    CreateFromColorsWithDpi: usize,
    #[cfg(feature = "UI")]
    pub CreateFromColorsWithDpiAndAlpha: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        colors_array_size: u32,
        colors: *const ::windows::UI::Color,
        widthinpixels: i32,
        heightinpixels: i32,
        dpi: f32,
        alpha: CanvasAlphaMode,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    CreateFromColorsWithDpiAndAlpha: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub CreateFromSoftwareBitmap: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        sourcebitmap: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    CreateFromSoftwareBitmap: usize,
    #[cfg(feature = "Foundation")]
    pub LoadAsyncFromHstring: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        filename: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadAsyncFromHstring: usize,
    #[cfg(feature = "Foundation")]
    pub LoadAsyncFromHstringWithDpi: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        filename: *mut ::core::ffi::c_void,
        dpi: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadAsyncFromHstringWithDpi: usize,
    #[cfg(feature = "Foundation")]
    pub LoadAsyncFromHstringWithDpiAndAlpha: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        filename: *mut ::core::ffi::c_void,
        dpi: f32,
        alpha: CanvasAlphaMode,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadAsyncFromHstringWithDpiAndAlpha: usize,
    #[cfg(feature = "Foundation")]
    pub LoadAsyncFromUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        uri: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadAsyncFromUri: usize,
    #[cfg(feature = "Foundation")]
    pub LoadAsyncFromUriWithDpi: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        uri: *mut ::core::ffi::c_void,
        dpi: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadAsyncFromUriWithDpi: usize,
    #[cfg(feature = "Foundation")]
    pub LoadAsyncFromUriWithDpiAndAlpha: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        uri: *mut ::core::ffi::c_void,
        dpi: f32,
        alpha: CanvasAlphaMode,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadAsyncFromUriWithDpiAndAlpha: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub LoadAsyncFromStream: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        stream: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    LoadAsyncFromStream: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub LoadAsyncFromStreamWithDpi: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        stream: *mut ::core::ffi::c_void,
        dpi: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    LoadAsyncFromStreamWithDpi: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub LoadAsyncFromStreamWithDpiAndAlpha: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        stream: *mut ::core::ffi::c_void,
        dpi: f32,
        alpha: CanvasAlphaMode,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    LoadAsyncFromStreamWithDpiAndAlpha: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasCommandList(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasCommandList {
    type Vtable = ICanvasCommandList_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasCommandList {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xb71e73cf_2fe7_4d3a_bbb8_19f016f5be1b,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasCommandList_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateDrawingSession: unsafe extern "system" fn(
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
pub struct ICanvasCommandListFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasCommandListFactory {
    type Vtable = ICanvasCommandListFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasCommandListFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xb3d44e68_d931_4b5b_b957_0888980a7d50,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasCommandListFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasDevice(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasDevice {
    type Vtable = ICanvasDevice_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasDevice {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xa27f0b5d_ec2c_4d4f_948f_0aa1e95e33e6,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasDevice_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ForceSoftwareRenderer: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub MaximumBitmapSizeInPixels: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_DirectX")]
    pub IsPixelFormatSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pixelformat: ::windows::Graphics::DirectX::DirectXPixelFormat,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    IsPixelFormatSupported: usize,
    pub IsBufferPrecisionSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bufferprecision: CanvasBufferPrecision,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub MaximumCacheSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u64,
    ) -> ::windows::core::HRESULT,
    pub SetMaximumCacheSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: u64,
    ) -> ::windows::core::HRESULT,
    pub LowPriority: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetLowPriority: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeviceLost: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeviceLost: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDeviceLost: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDeviceLost: usize,
    pub IsDeviceLost: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        hresult: i32,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub RaiseDeviceLost: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Lock: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasDeviceFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasDeviceFactory {
    type Vtable = ICanvasDeviceFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasDeviceFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xe2c2bf21_5418_43b9_a2da_539e287c790f,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasDeviceFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateWithForceSoftwareRendererOption: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        forcesoftwarerenderer: bool,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasDeviceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasDeviceStatics {
    type Vtable = ICanvasDeviceStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasDeviceStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x9b6e2b27_cd07_421a_8f69_0ae8a787fe8c,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasDeviceStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateFromDirect3D11Device: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        direct3ddevice: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateFromDirect3D11Device: usize,
    pub GetSharedDevice: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetSharedDeviceWithForceSoftwareRenderer: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        forcesoftwarerenderer: bool,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetDebugLevel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CanvasDebugLevel,
    ) -> ::windows::core::HRESULT,
    pub DebugLevel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasDebugLevel,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasDrawingSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasDrawingSession {
    type Vtable = ICanvasDrawingSession_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasDrawingSession {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xf60afd09_e623_4be0_b750_578aa920b1db,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasDrawingSession_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "UI")]
    pub Clear: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Clear: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub ClearHdr: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        color: ::windows::Foundation::Numerics::Vector4,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ClearHdr: usize,
    pub Flush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub DrawImageAtOrigin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        image: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub DrawImageAtOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        image: *mut ::core::ffi::c_void,
        offset: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DrawImageAtOffset: usize,
    pub DrawImageAtCoords: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        image: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DrawImageToRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bitmap: *mut ::core::ffi::c_void,
        destinationrectangle: ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DrawImageToRect: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub DrawImageAtOffsetWithSourceRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        image: *mut ::core::ffi::c_void,
        offset: ::windows::Foundation::Numerics::Vector2,
        sourcerectangle: ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DrawImageAtOffsetWithSourceRect: usize,
    #[cfg(feature = "Foundation")]
    pub DrawImageAtCoordsWithSourceRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        image: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        sourcerectangle: ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DrawImageAtCoordsWithSourceRect: usize,
    #[cfg(feature = "Foundation")]
    pub DrawImageToRectWithSourceRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        image: *mut ::core::ffi::c_void,
        destinationrectangle: ::windows::Foundation::Rect,
        sourcerectangle: ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DrawImageToRectWithSourceRect: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub DrawImageAtOffsetWithSourceRectAndOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        image: *mut ::core::ffi::c_void,
        offset: ::windows::Foundation::Numerics::Vector2,
        sourcerectangle: ::windows::Foundation::Rect,
        opacity: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DrawImageAtOffsetWithSourceRectAndOpacity: usize,
    #[cfg(feature = "Foundation")]
    pub DrawImageAtCoordsWithSourceRectAndOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        image: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        sourcerectangle: ::windows::Foundation::Rect,
        opacity: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DrawImageAtCoordsWithSourceRectAndOpacity: usize,
    #[cfg(feature = "Foundation")]
    pub DrawImageToRectWithSourceRectAndOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        image: *mut ::core::ffi::c_void,
        destinationrectangle: ::windows::Foundation::Rect,
        sourcerectangle: ::windows::Foundation::Rect,
        opacity: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DrawImageToRectWithSourceRectAndOpacity: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub DrawImageAtOffsetWithSourceRectAndOpacityAndInterpolation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        image: *mut ::core::ffi::c_void,
        offset: ::windows::Foundation::Numerics::Vector2,
        sourcerectangle: ::windows::Foundation::Rect,
        opacity: f32,
        interpolation: CanvasImageInterpolation,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DrawImageAtOffsetWithSourceRectAndOpacityAndInterpolation: usize,
    #[cfg(feature = "Foundation")]
    pub DrawImageAtCoordsWithSourceRectAndOpacityAndInterpolation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        image: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        sourcerectangle: ::windows::Foundation::Rect,
        opacity: f32,
        interpolation: CanvasImageInterpolation,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DrawImageAtCoordsWithSourceRectAndOpacityAndInterpolation: usize,
    #[cfg(feature = "Foundation")]
    pub DrawImageToRectWithSourceRectAndOpacityAndInterpolation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        image: *mut ::core::ffi::c_void,
        destinationrectangle: ::windows::Foundation::Rect,
        sourcerectangle: ::windows::Foundation::Rect,
        opacity: f32,
        interpolation: CanvasImageInterpolation,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DrawImageToRectWithSourceRectAndOpacityAndInterpolation: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub DrawImageAtOffsetWithSourceRectAndOpacityAndInterpolationAndComposite: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        image: *mut ::core::ffi::c_void,
        offset: ::windows::Foundation::Numerics::Vector2,
        sourcerectangle: ::windows::Foundation::Rect,
        opacity: f32,
        interpolation: CanvasImageInterpolation,
        composite: CanvasComposite,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DrawImageAtOffsetWithSourceRectAndOpacityAndInterpolationAndComposite: usize,
    #[cfg(feature = "Foundation")]
    pub DrawImageAtCoordsWithSourceRectAndOpacityAndInterpolationAndComposite: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        image: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        sourcerectangle: ::windows::Foundation::Rect,
        opacity: f32,
        interpolation: CanvasImageInterpolation,
        composite: CanvasComposite,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DrawImageAtCoordsWithSourceRectAndOpacityAndInterpolationAndComposite: usize,
    #[cfg(feature = "Foundation")]
    pub DrawImageToRectWithSourceRectAndOpacityAndInterpolationAndComposite: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        image: *mut ::core::ffi::c_void,
        destinationrectangle: ::windows::Foundation::Rect,
        sourcerectangle: ::windows::Foundation::Rect,
        opacity: f32,
        interpolation: CanvasImageInterpolation,
        composite: CanvasComposite,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DrawImageToRectWithSourceRectAndOpacityAndInterpolationAndComposite: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub DrawImageAtOffsetWithSourceRectAndOpacityAndInterpolationAndPerspective: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bitmap: *mut ::core::ffi::c_void,
        offset: ::windows::Foundation::Numerics::Vector2,
        sourcerectangle: ::windows::Foundation::Rect,
        opacity: f32,
        interpolation: CanvasImageInterpolation,
        perspective: ::windows::Foundation::Numerics::Matrix4x4,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DrawImageAtOffsetWithSourceRectAndOpacityAndInterpolationAndPerspective: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub DrawImageAtCoordsWithSourceRectAndOpacityAndInterpolationAndPerspective: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bitmap: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        sourcerectangle: ::windows::Foundation::Rect,
        opacity: f32,
        interpolation: CanvasImageInterpolation,
        perspective: ::windows::Foundation::Numerics::Matrix4x4,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DrawImageAtCoordsWithSourceRectAndOpacityAndInterpolationAndPerspective: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub DrawImageToRectWithSourceRectAndOpacityAndInterpolationAndPerspective: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bitmap: *mut ::core::ffi::c_void,
        destinationrectangle: ::windows::Foundation::Rect,
        sourcerectangle: ::windows::Foundation::Rect,
        opacity: f32,
        interpolation: CanvasImageInterpolation,
        perspective: ::windows::Foundation::Numerics::Matrix4x4,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DrawImageToRectWithSourceRectAndOpacityAndInterpolationAndPerspective: usize,
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation_Numerics"))]
    pub DrawLineWithBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        point0: ::windows::Foundation::Numerics::Vector2,
        point1: ::windows::Foundation::Numerics::Vector2,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation_Numerics"))
    )]
    DrawLineWithBrush: usize,
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub DrawLineAtCoordsWithBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Brushes"))]
    DrawLineAtCoordsWithBrush: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "UI"))]
    pub DrawLineWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        point0: ::windows::Foundation::Numerics::Vector2,
        point1: ::windows::Foundation::Numerics::Vector2,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "UI")))]
    DrawLineWithColor: usize,
    #[cfg(feature = "UI")]
    pub DrawLineAtCoordsWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    DrawLineAtCoordsWithColor: usize,
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation_Numerics"))]
    pub DrawLineWithBrushAndStrokeWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        point0: ::windows::Foundation::Numerics::Vector2,
        point1: ::windows::Foundation::Numerics::Vector2,
        brush: *mut ::core::ffi::c_void,
        strokewidth: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation_Numerics"))
    )]
    DrawLineWithBrushAndStrokeWidth: usize,
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub DrawLineAtCoordsWithBrushAndStrokeWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        brush: *mut ::core::ffi::c_void,
        strokewidth: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Brushes"))]
    DrawLineAtCoordsWithBrushAndStrokeWidth: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "UI"))]
    pub DrawLineWithColorAndStrokeWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        point0: ::windows::Foundation::Numerics::Vector2,
        point1: ::windows::Foundation::Numerics::Vector2,
        color: ::windows::UI::Color,
        strokewidth: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "UI")))]
    DrawLineWithColorAndStrokeWidth: usize,
    #[cfg(feature = "UI")]
    pub DrawLineAtCoordsWithColorAndStrokeWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    DrawLineAtCoordsWithColorAndStrokeWidth: usize,
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics"
        )
    )]
    pub DrawLineWithBrushAndStrokeWidthAndStrokeStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        point0: ::windows::Foundation::Numerics::Vector2,
        point1: ::windows::Foundation::Numerics::Vector2,
        brush: *mut ::core::ffi::c_void,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Geometry",
                feature = "Foundation_Numerics"
            )
        )
    )]
    DrawLineWithBrushAndStrokeWidthAndStrokeStyle: usize,
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub DrawLineAtCoordsWithBrushAndStrokeWidthAndStrokeStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        brush: *mut ::core::ffi::c_void,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Geometry"
            )
        )
    )]
    DrawLineAtCoordsWithBrushAndStrokeWidthAndStrokeStyle: usize,
    #[cfg(
        all(
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics",
            feature = "UI"
        )
    )]
    pub DrawLineWithColorAndStrokeWidthAndStrokeStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        point0: ::windows::Foundation::Numerics::Vector2,
        point1: ::windows::Foundation::Numerics::Vector2,
        color: ::windows::UI::Color,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Geometry",
                feature = "Foundation_Numerics",
                feature = "UI"
            )
        )
    )]
    DrawLineWithColorAndStrokeWidthAndStrokeStyle: usize,
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "UI"))]
    pub DrawLineAtCoordsWithColorAndStrokeWidthAndStrokeStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Canvas_Geometry", feature = "UI")))]
    DrawLineAtCoordsWithColorAndStrokeWidthAndStrokeStyle: usize,
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation"))]
    pub DrawRectangleWithBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rect: ::windows::Foundation::Rect,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation")))]
    DrawRectangleWithBrush: usize,
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub DrawRectangleAtCoordsWithBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Brushes"))]
    DrawRectangleAtCoordsWithBrush: usize,
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub DrawRectangleWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rect: ::windows::Foundation::Rect,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI")))]
    DrawRectangleWithColor: usize,
    #[cfg(feature = "UI")]
    pub DrawRectangleAtCoordsWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    DrawRectangleAtCoordsWithColor: usize,
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation"))]
    pub DrawRectangleWithBrushAndStrokeWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rect: ::windows::Foundation::Rect,
        brush: *mut ::core::ffi::c_void,
        strokewidth: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation")))]
    DrawRectangleWithBrushAndStrokeWidth: usize,
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub DrawRectangleAtCoordsWithBrushAndStrokeWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        brush: *mut ::core::ffi::c_void,
        strokewidth: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Brushes"))]
    DrawRectangleAtCoordsWithBrushAndStrokeWidth: usize,
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub DrawRectangleWithColorAndStrokeWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rect: ::windows::Foundation::Rect,
        color: ::windows::UI::Color,
        strokewidth: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI")))]
    DrawRectangleWithColorAndStrokeWidth: usize,
    #[cfg(feature = "UI")]
    pub DrawRectangleAtCoordsWithColorAndStrokeWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    DrawRectangleAtCoordsWithColorAndStrokeWidth: usize,
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation"
        )
    )]
    pub DrawRectangleWithBrushAndStrokeWidthAndStrokeStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rect: ::windows::Foundation::Rect,
        brush: *mut ::core::ffi::c_void,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Geometry",
                feature = "Foundation"
            )
        )
    )]
    DrawRectangleWithBrushAndStrokeWidthAndStrokeStyle: usize,
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub DrawRectangleAtCoordsWithBrushAndStrokeWidthAndStrokeStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        brush: *mut ::core::ffi::c_void,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Geometry"
            )
        )
    )]
    DrawRectangleAtCoordsWithBrushAndStrokeWidthAndStrokeStyle: usize,
    #[cfg(
        all(feature = "Graphics_Canvas_Geometry", feature = "Foundation", feature = "UI")
    )]
    pub DrawRectangleWithColorAndStrokeWidthAndStrokeStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rect: ::windows::Foundation::Rect,
        color: ::windows::UI::Color,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Geometry",
                feature = "Foundation",
                feature = "UI"
            )
        )
    )]
    DrawRectangleWithColorAndStrokeWidthAndStrokeStyle: usize,
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "UI"))]
    pub DrawRectangleAtCoordsWithColorAndStrokeWidthAndStrokeStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Canvas_Geometry", feature = "UI")))]
    DrawRectangleAtCoordsWithColorAndStrokeWidthAndStrokeStyle: usize,
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation"))]
    pub FillRectangleWithBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rect: ::windows::Foundation::Rect,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation")))]
    FillRectangleWithBrush: usize,
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub FillRectangleAtCoordsWithBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Brushes"))]
    FillRectangleAtCoordsWithBrush: usize,
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub FillRectangleWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rect: ::windows::Foundation::Rect,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI")))]
    FillRectangleWithColor: usize,
    #[cfg(feature = "UI")]
    pub FillRectangleAtCoordsWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    FillRectangleAtCoordsWithColor: usize,
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation"))]
    pub FillRectangleWithBrushAndOpacityBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rect: ::windows::Foundation::Rect,
        brush: *mut ::core::ffi::c_void,
        opacitybrush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation")))]
    FillRectangleWithBrushAndOpacityBrush: usize,
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub FillRectangleAtCoordsWithBrushAndOpacityBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        brush: *mut ::core::ffi::c_void,
        opacitybrush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Brushes"))]
    FillRectangleAtCoordsWithBrushAndOpacityBrush: usize,
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation"))]
    pub DrawRoundedRectangleWithBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rect: ::windows::Foundation::Rect,
        radiusx: f32,
        radiusy: f32,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation")))]
    DrawRoundedRectangleWithBrush: usize,
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub DrawRoundedRectangleAtCoordsWithBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radiusx: f32,
        radiusy: f32,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Brushes"))]
    DrawRoundedRectangleAtCoordsWithBrush: usize,
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub DrawRoundedRectangleWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rect: ::windows::Foundation::Rect,
        radiusx: f32,
        radiusy: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI")))]
    DrawRoundedRectangleWithColor: usize,
    #[cfg(feature = "UI")]
    pub DrawRoundedRectangleAtCoordsWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radiusx: f32,
        radiusy: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    DrawRoundedRectangleAtCoordsWithColor: usize,
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation"))]
    pub DrawRoundedRectangleWithBrushAndStrokeWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rect: ::windows::Foundation::Rect,
        radiusx: f32,
        radiusy: f32,
        brush: *mut ::core::ffi::c_void,
        strokewidth: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation")))]
    DrawRoundedRectangleWithBrushAndStrokeWidth: usize,
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub DrawRoundedRectangleAtCoordsWithBrushAndStrokeWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radiusx: f32,
        radiusy: f32,
        brush: *mut ::core::ffi::c_void,
        strokewidth: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Brushes"))]
    DrawRoundedRectangleAtCoordsWithBrushAndStrokeWidth: usize,
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub DrawRoundedRectangleWithColorAndStrokeWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rect: ::windows::Foundation::Rect,
        radiusx: f32,
        radiusy: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI")))]
    DrawRoundedRectangleWithColorAndStrokeWidth: usize,
    #[cfg(feature = "UI")]
    pub DrawRoundedRectangleAtCoordsWithColorAndStrokeWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radiusx: f32,
        radiusy: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    DrawRoundedRectangleAtCoordsWithColorAndStrokeWidth: usize,
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation"
        )
    )]
    pub DrawRoundedRectangleWithBrushAndStrokeWidthAndStrokeStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rect: ::windows::Foundation::Rect,
        radiusx: f32,
        radiusy: f32,
        brush: *mut ::core::ffi::c_void,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Geometry",
                feature = "Foundation"
            )
        )
    )]
    DrawRoundedRectangleWithBrushAndStrokeWidthAndStrokeStyle: usize,
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub DrawRoundedRectangleAtCoordsWithBrushAndStrokeWidthAndStrokeStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radiusx: f32,
        radiusy: f32,
        brush: *mut ::core::ffi::c_void,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Geometry"
            )
        )
    )]
    DrawRoundedRectangleAtCoordsWithBrushAndStrokeWidthAndStrokeStyle: usize,
    #[cfg(
        all(feature = "Graphics_Canvas_Geometry", feature = "Foundation", feature = "UI")
    )]
    pub DrawRoundedRectangleWithColorAndStrokeWidthAndStrokeStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rect: ::windows::Foundation::Rect,
        radiusx: f32,
        radiusy: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Geometry",
                feature = "Foundation",
                feature = "UI"
            )
        )
    )]
    DrawRoundedRectangleWithColorAndStrokeWidthAndStrokeStyle: usize,
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "UI"))]
    pub DrawRoundedRectangleAtCoordsWithColorAndStrokeWidthAndStrokeStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radiusx: f32,
        radiusy: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Canvas_Geometry", feature = "UI")))]
    DrawRoundedRectangleAtCoordsWithColorAndStrokeWidthAndStrokeStyle: usize,
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation"))]
    pub FillRoundedRectangleWithBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rect: ::windows::Foundation::Rect,
        radiusx: f32,
        radiusy: f32,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation")))]
    FillRoundedRectangleWithBrush: usize,
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub FillRoundedRectangleAtCoordsWithBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radiusx: f32,
        radiusy: f32,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Brushes"))]
    FillRoundedRectangleAtCoordsWithBrush: usize,
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub FillRoundedRectangleWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        rect: ::windows::Foundation::Rect,
        radiusx: f32,
        radiusy: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI")))]
    FillRoundedRectangleWithColor: usize,
    #[cfg(feature = "UI")]
    pub FillRoundedRectangleAtCoordsWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radiusx: f32,
        radiusy: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    FillRoundedRectangleAtCoordsWithColor: usize,
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation_Numerics"))]
    pub DrawEllipseWithBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radiusx: f32,
        radiusy: f32,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation_Numerics"))
    )]
    DrawEllipseWithBrush: usize,
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub DrawEllipseAtCoordsWithBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        radiusx: f32,
        radiusy: f32,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Brushes"))]
    DrawEllipseAtCoordsWithBrush: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "UI"))]
    pub DrawEllipseWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radiusx: f32,
        radiusy: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "UI")))]
    DrawEllipseWithColor: usize,
    #[cfg(feature = "UI")]
    pub DrawEllipseAtCoordsWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        radiusx: f32,
        radiusy: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    DrawEllipseAtCoordsWithColor: usize,
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation_Numerics"))]
    pub DrawEllipseWithBrushAndStrokeWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radiusx: f32,
        radiusy: f32,
        brush: *mut ::core::ffi::c_void,
        strokewidth: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation_Numerics"))
    )]
    DrawEllipseWithBrushAndStrokeWidth: usize,
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub DrawEllipseAtCoordsWithBrushAndStrokeWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        radiusx: f32,
        radiusy: f32,
        brush: *mut ::core::ffi::c_void,
        strokewidth: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Brushes"))]
    DrawEllipseAtCoordsWithBrushAndStrokeWidth: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "UI"))]
    pub DrawEllipseWithColorAndStrokeWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radiusx: f32,
        radiusy: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "UI")))]
    DrawEllipseWithColorAndStrokeWidth: usize,
    #[cfg(feature = "UI")]
    pub DrawEllipseAtCoordsWithColorAndStrokeWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        radiusx: f32,
        radiusy: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    DrawEllipseAtCoordsWithColorAndStrokeWidth: usize,
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics"
        )
    )]
    pub DrawEllipseWithBrushAndStrokeWidthAndStrokeStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radiusx: f32,
        radiusy: f32,
        brush: *mut ::core::ffi::c_void,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Geometry",
                feature = "Foundation_Numerics"
            )
        )
    )]
    DrawEllipseWithBrushAndStrokeWidthAndStrokeStyle: usize,
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub DrawEllipseAtCoordsWithBrushAndStrokeWidthAndStrokeStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        radiusx: f32,
        radiusy: f32,
        brush: *mut ::core::ffi::c_void,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Geometry"
            )
        )
    )]
    DrawEllipseAtCoordsWithBrushAndStrokeWidthAndStrokeStyle: usize,
    #[cfg(
        all(
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics",
            feature = "UI"
        )
    )]
    pub DrawEllipseWithColorAndStrokeWidthAndStrokeStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radiusx: f32,
        radiusy: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Geometry",
                feature = "Foundation_Numerics",
                feature = "UI"
            )
        )
    )]
    DrawEllipseWithColorAndStrokeWidthAndStrokeStyle: usize,
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "UI"))]
    pub DrawEllipseAtCoordsWithColorAndStrokeWidthAndStrokeStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        radiusx: f32,
        radiusy: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Canvas_Geometry", feature = "UI")))]
    DrawEllipseAtCoordsWithColorAndStrokeWidthAndStrokeStyle: usize,
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation_Numerics"))]
    pub FillEllipseWithBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radiusx: f32,
        radiusy: f32,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation_Numerics"))
    )]
    FillEllipseWithBrush: usize,
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub FillEllipseAtCoordsWithBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        radiusx: f32,
        radiusy: f32,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Brushes"))]
    FillEllipseAtCoordsWithBrush: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "UI"))]
    pub FillEllipseWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radiusx: f32,
        radiusy: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "UI")))]
    FillEllipseWithColor: usize,
    #[cfg(feature = "UI")]
    pub FillEllipseAtCoordsWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        radiusx: f32,
        radiusy: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    FillEllipseAtCoordsWithColor: usize,
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation_Numerics"))]
    pub DrawCircleWithBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radius: f32,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation_Numerics"))
    )]
    DrawCircleWithBrush: usize,
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub DrawCircleAtCoordsWithBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        radius: f32,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Brushes"))]
    DrawCircleAtCoordsWithBrush: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "UI"))]
    pub DrawCircleWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radius: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "UI")))]
    DrawCircleWithColor: usize,
    #[cfg(feature = "UI")]
    pub DrawCircleAtCoordsWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        radius: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    DrawCircleAtCoordsWithColor: usize,
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation_Numerics"))]
    pub DrawCircleWithBrushAndStrokeWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radius: f32,
        brush: *mut ::core::ffi::c_void,
        strokewidth: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation_Numerics"))
    )]
    DrawCircleWithBrushAndStrokeWidth: usize,
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub DrawCircleAtCoordsWithBrushAndStrokeWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        radius: f32,
        brush: *mut ::core::ffi::c_void,
        strokewidth: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Brushes"))]
    DrawCircleAtCoordsWithBrushAndStrokeWidth: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "UI"))]
    pub DrawCircleWithColorAndStrokeWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radius: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "UI")))]
    DrawCircleWithColorAndStrokeWidth: usize,
    #[cfg(feature = "UI")]
    pub DrawCircleAtCoordsWithColorAndStrokeWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        radius: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    DrawCircleAtCoordsWithColorAndStrokeWidth: usize,
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics"
        )
    )]
    pub DrawCircleWithBrushAndStrokeWidthAndStrokeStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radius: f32,
        brush: *mut ::core::ffi::c_void,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Geometry",
                feature = "Foundation_Numerics"
            )
        )
    )]
    DrawCircleWithBrushAndStrokeWidthAndStrokeStyle: usize,
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub DrawCircleAtCoordsWithBrushAndStrokeWidthAndStrokeStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        radius: f32,
        brush: *mut ::core::ffi::c_void,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Geometry"
            )
        )
    )]
    DrawCircleAtCoordsWithBrushAndStrokeWidthAndStrokeStyle: usize,
    #[cfg(
        all(
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics",
            feature = "UI"
        )
    )]
    pub DrawCircleWithColorAndStrokeWidthAndStrokeStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radius: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Geometry",
                feature = "Foundation_Numerics",
                feature = "UI"
            )
        )
    )]
    DrawCircleWithColorAndStrokeWidthAndStrokeStyle: usize,
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "UI"))]
    pub DrawCircleAtCoordsWithColorAndStrokeWidthAndStrokeStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        radius: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Canvas_Geometry", feature = "UI")))]
    DrawCircleAtCoordsWithColorAndStrokeWidthAndStrokeStyle: usize,
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation_Numerics"))]
    pub FillCircleWithBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radius: f32,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation_Numerics"))
    )]
    FillCircleWithBrush: usize,
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub FillCircleAtCoordsWithBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        radius: f32,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Brushes"))]
    FillCircleAtCoordsWithBrush: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "UI"))]
    pub FillCircleWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radius: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "UI")))]
    FillCircleWithColor: usize,
    #[cfg(feature = "UI")]
    pub FillCircleAtCoordsWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        radius: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    FillCircleAtCoordsWithColor: usize,
    #[cfg(all(feature = "Foundation_Numerics", feature = "UI"))]
    pub DrawTextAtPointWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        text: *mut ::core::ffi::c_void,
        point: ::windows::Foundation::Numerics::Vector2,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Numerics", feature = "UI")))]
    DrawTextAtPointWithColor: usize,
    #[cfg(feature = "UI")]
    pub DrawTextAtPointCoordsWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        text: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    DrawTextAtPointCoordsWithColor: usize,
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Text",
            feature = "Foundation_Numerics"
        )
    )]
    pub DrawTextAtPointWithBrushAndFormat: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        text: *mut ::core::ffi::c_void,
        point: ::windows::Foundation::Numerics::Vector2,
        brush: *mut ::core::ffi::c_void,
        format: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Text",
                feature = "Foundation_Numerics"
            )
        )
    )]
    DrawTextAtPointWithBrushAndFormat: usize,
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Text",
            feature = "Foundation"
        )
    )]
    pub DrawTextAtRectWithBrushAndFormat: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        text: *mut ::core::ffi::c_void,
        rectangle: ::windows::Foundation::Rect,
        brush: *mut ::core::ffi::c_void,
        format: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Text",
                feature = "Foundation"
            )
        )
    )]
    DrawTextAtRectWithBrushAndFormat: usize,
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Text"))]
    pub DrawTextAtPointCoordsWithBrushAndFormat: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        text: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        brush: *mut ::core::ffi::c_void,
        format: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Text"))
    )]
    DrawTextAtPointCoordsWithBrushAndFormat: usize,
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Text"))]
    pub DrawTextAtRectCoordsWithBrushAndFormat: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        text: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        brush: *mut ::core::ffi::c_void,
        format: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Text"))
    )]
    DrawTextAtRectCoordsWithBrushAndFormat: usize,
    #[cfg(
        all(
            feature = "Graphics_Canvas_Text",
            feature = "Foundation_Numerics",
            feature = "UI"
        )
    )]
    pub DrawTextAtPointWithColorAndFormat: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        text: *mut ::core::ffi::c_void,
        point: ::windows::Foundation::Numerics::Vector2,
        color: ::windows::UI::Color,
        format: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Text",
                feature = "Foundation_Numerics",
                feature = "UI"
            )
        )
    )]
    DrawTextAtPointWithColorAndFormat: usize,
    #[cfg(all(feature = "Graphics_Canvas_Text", feature = "Foundation", feature = "UI"))]
    pub DrawTextAtRectWithColorAndFormat: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        text: *mut ::core::ffi::c_void,
        rectangle: ::windows::Foundation::Rect,
        color: ::windows::UI::Color,
        format: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(feature = "Graphics_Canvas_Text", feature = "Foundation", feature = "UI")
        )
    )]
    DrawTextAtRectWithColorAndFormat: usize,
    #[cfg(all(feature = "Graphics_Canvas_Text", feature = "UI"))]
    pub DrawTextAtPointCoordsWithColorAndFormat: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        text: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        color: ::windows::UI::Color,
        format: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Canvas_Text", feature = "UI")))]
    DrawTextAtPointCoordsWithColorAndFormat: usize,
    #[cfg(all(feature = "Graphics_Canvas_Text", feature = "UI"))]
    pub DrawTextAtRectCoordsWithColorAndFormat: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        text: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: ::windows::UI::Color,
        format: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Canvas_Text", feature = "UI")))]
    DrawTextAtRectCoordsWithColorAndFormat: usize,
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics"
        )
    )]
    pub DrawGeometryWithBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        offset: ::windows::Foundation::Numerics::Vector2,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Geometry",
                feature = "Foundation_Numerics"
            )
        )
    )]
    DrawGeometryWithBrush: usize,
    #[cfg(
        all(
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics",
            feature = "UI"
        )
    )]
    pub DrawGeometryWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        offset: ::windows::Foundation::Numerics::Vector2,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Geometry",
                feature = "Foundation_Numerics",
                feature = "UI"
            )
        )
    )]
    DrawGeometryWithColor: usize,
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub DrawGeometryAtCoordsWithBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Geometry"
            )
        )
    )]
    DrawGeometryAtCoordsWithBrush: usize,
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "UI"))]
    pub DrawGeometryAtCoordsWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Canvas_Geometry", feature = "UI")))]
    DrawGeometryAtCoordsWithColor: usize,
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub DrawGeometryAtOriginWithBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Geometry"
            )
        )
    )]
    DrawGeometryAtOriginWithBrush: usize,
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "UI"))]
    pub DrawGeometryAtOriginWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Canvas_Geometry", feature = "UI")))]
    DrawGeometryAtOriginWithColor: usize,
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics"
        )
    )]
    pub DrawGeometryWithBrushAndStrokeWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        offset: ::windows::Foundation::Numerics::Vector2,
        brush: *mut ::core::ffi::c_void,
        strokewidth: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Geometry",
                feature = "Foundation_Numerics"
            )
        )
    )]
    DrawGeometryWithBrushAndStrokeWidth: usize,
    #[cfg(
        all(
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics",
            feature = "UI"
        )
    )]
    pub DrawGeometryWithColorAndStrokeWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        offset: ::windows::Foundation::Numerics::Vector2,
        color: ::windows::UI::Color,
        strokewidth: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Geometry",
                feature = "Foundation_Numerics",
                feature = "UI"
            )
        )
    )]
    DrawGeometryWithColorAndStrokeWidth: usize,
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub DrawGeometryAtCoordsWithBrushAndStrokeWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        brush: *mut ::core::ffi::c_void,
        strokewidth: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Geometry"
            )
        )
    )]
    DrawGeometryAtCoordsWithBrushAndStrokeWidth: usize,
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "UI"))]
    pub DrawGeometryAtCoordsWithColorAndStrokeWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Canvas_Geometry", feature = "UI")))]
    DrawGeometryAtCoordsWithColorAndStrokeWidth: usize,
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub DrawGeometryAtOriginWithBrushAndStrokeWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        brush: *mut ::core::ffi::c_void,
        strokewidth: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Geometry"
            )
        )
    )]
    DrawGeometryAtOriginWithBrushAndStrokeWidth: usize,
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "UI"))]
    pub DrawGeometryAtOriginWithColorAndStrokeWidth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        color: ::windows::UI::Color,
        strokewidth: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Canvas_Geometry", feature = "UI")))]
    DrawGeometryAtOriginWithColorAndStrokeWidth: usize,
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics"
        )
    )]
    pub DrawGeometryWithBrushAndStrokeWidthAndStrokeStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        offset: ::windows::Foundation::Numerics::Vector2,
        brush: *mut ::core::ffi::c_void,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Geometry",
                feature = "Foundation_Numerics"
            )
        )
    )]
    DrawGeometryWithBrushAndStrokeWidthAndStrokeStyle: usize,
    #[cfg(
        all(
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics",
            feature = "UI"
        )
    )]
    pub DrawGeometryWithColorAndStrokeWidthAndStrokeStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        offset: ::windows::Foundation::Numerics::Vector2,
        color: ::windows::UI::Color,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Geometry",
                feature = "Foundation_Numerics",
                feature = "UI"
            )
        )
    )]
    DrawGeometryWithColorAndStrokeWidthAndStrokeStyle: usize,
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub DrawGeometryAtCoordsWithBrushAndStrokeWidthAndStrokeStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        brush: *mut ::core::ffi::c_void,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Geometry"
            )
        )
    )]
    DrawGeometryAtCoordsWithBrushAndStrokeWidthAndStrokeStyle: usize,
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "UI"))]
    pub DrawGeometryAtCoordsWithColorAndStrokeWidthAndStrokeStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Canvas_Geometry", feature = "UI")))]
    DrawGeometryAtCoordsWithColorAndStrokeWidthAndStrokeStyle: usize,
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub DrawGeometryAtOriginWithBrushAndStrokeWidthAndStrokeStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        brush: *mut ::core::ffi::c_void,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Geometry"
            )
        )
    )]
    DrawGeometryAtOriginWithBrushAndStrokeWidthAndStrokeStyle: usize,
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "UI"))]
    pub DrawGeometryAtOriginWithColorAndStrokeWidthAndStrokeStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        color: ::windows::UI::Color,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Canvas_Geometry", feature = "UI")))]
    DrawGeometryAtOriginWithColorAndStrokeWidthAndStrokeStyle: usize,
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics"
        )
    )]
    pub FillGeometryWithBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        offset: ::windows::Foundation::Numerics::Vector2,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Geometry",
                feature = "Foundation_Numerics"
            )
        )
    )]
    FillGeometryWithBrush: usize,
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics"
        )
    )]
    pub FillGeometryWithBrushAndOpacityBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        offset: ::windows::Foundation::Numerics::Vector2,
        brush: *mut ::core::ffi::c_void,
        opacitybrush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Geometry",
                feature = "Foundation_Numerics"
            )
        )
    )]
    FillGeometryWithBrushAndOpacityBrush: usize,
    #[cfg(
        all(
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics",
            feature = "UI"
        )
    )]
    pub FillGeometryWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        offset: ::windows::Foundation::Numerics::Vector2,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Geometry",
                feature = "Foundation_Numerics",
                feature = "UI"
            )
        )
    )]
    FillGeometryWithColor: usize,
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub FillGeometryAtCoordsWithBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Geometry"
            )
        )
    )]
    FillGeometryAtCoordsWithBrush: usize,
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub FillGeometryAtCoordsWithBrushAndOpacityBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        brush: *mut ::core::ffi::c_void,
        opacitybrush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Geometry"
            )
        )
    )]
    FillGeometryAtCoordsWithBrushAndOpacityBrush: usize,
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "UI"))]
    pub FillGeometryAtCoordsWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Canvas_Geometry", feature = "UI")))]
    FillGeometryAtCoordsWithColor: usize,
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub FillGeometryAtOriginWithBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Geometry"
            )
        )
    )]
    FillGeometryAtOriginWithBrush: usize,
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub FillGeometryAtOriginWithBrushAndOpacityBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        brush: *mut ::core::ffi::c_void,
        opacitybrush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Geometry"
            )
        )
    )]
    FillGeometryAtOriginWithBrushAndOpacityBrush: usize,
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "UI"))]
    pub FillGeometryAtOriginWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Canvas_Geometry", feature = "UI")))]
    FillGeometryAtOriginWithColor: usize,
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics"
        )
    )]
    pub DrawCachedGeometryWithBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        offset: ::windows::Foundation::Numerics::Vector2,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Geometry",
                feature = "Foundation_Numerics"
            )
        )
    )]
    DrawCachedGeometryWithBrush: usize,
    #[cfg(
        all(
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics",
            feature = "UI"
        )
    )]
    pub DrawCachedGeometryWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        offset: ::windows::Foundation::Numerics::Vector2,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Geometry",
                feature = "Foundation_Numerics",
                feature = "UI"
            )
        )
    )]
    DrawCachedGeometryWithColor: usize,
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub DrawCachedGeometryAtCoordsWithBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Geometry"
            )
        )
    )]
    DrawCachedGeometryAtCoordsWithBrush: usize,
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "UI"))]
    pub DrawCachedGeometryAtCoordsWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Canvas_Geometry", feature = "UI")))]
    DrawCachedGeometryAtCoordsWithColor: usize,
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub DrawCachedGeometryAtOriginWithBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Geometry"
            )
        )
    )]
    DrawCachedGeometryAtOriginWithBrush: usize,
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "UI"))]
    pub DrawCachedGeometryAtOriginWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Canvas_Geometry", feature = "UI")))]
    DrawCachedGeometryAtOriginWithColor: usize,
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Text",
            feature = "Foundation_Numerics"
        )
    )]
    pub DrawTextLayoutWithBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        textlayout: *mut ::core::ffi::c_void,
        point: ::windows::Foundation::Numerics::Vector2,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Text",
                feature = "Foundation_Numerics"
            )
        )
    )]
    DrawTextLayoutWithBrush: usize,
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Text"))]
    pub DrawTextLayoutAtCoordsWithBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        textlayout: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Text"))
    )]
    DrawTextLayoutAtCoordsWithBrush: usize,
    #[cfg(
        all(
            feature = "Graphics_Canvas_Text",
            feature = "Foundation_Numerics",
            feature = "UI"
        )
    )]
    pub DrawTextLayoutWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        textlayout: *mut ::core::ffi::c_void,
        point: ::windows::Foundation::Numerics::Vector2,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Text",
                feature = "Foundation_Numerics",
                feature = "UI"
            )
        )
    )]
    DrawTextLayoutWithColor: usize,
    #[cfg(all(feature = "Graphics_Canvas_Text", feature = "UI"))]
    pub DrawTextLayoutAtCoordsWithColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        textlayout: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Canvas_Text", feature = "UI")))]
    DrawTextLayoutAtCoordsWithColor: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Input_Inking"))]
    pub DrawInk: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        inkstrokes: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Input_Inking")))]
    DrawInk: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Input_Inking"))]
    pub DrawInkWithHighContrast: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        inkstrokes: *mut ::core::ffi::c_void,
        highcontrast: bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Input_Inking")))]
    DrawInkWithHighContrast: usize,
    #[cfg(feature = "Graphics_Canvas_Geometry")]
    pub DrawGradientMeshAtOrigin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        gradientmesh: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Geometry"))]
    DrawGradientMeshAtOrigin: usize,
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "Foundation_Numerics"))]
    pub DrawGradientMesh: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        gradientmesh: *mut ::core::ffi::c_void,
        point: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(all(feature = "Graphics_Canvas_Geometry", feature = "Foundation_Numerics"))
    )]
    DrawGradientMesh: usize,
    #[cfg(feature = "Graphics_Canvas_Geometry")]
    pub DrawGradientMeshAtCoords: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        gradientmesh: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Geometry"))]
    DrawGradientMeshAtCoords: usize,
    #[cfg(all(feature = "Graphics_Canvas_Svg", feature = "Foundation"))]
    pub DrawSvgAtOrigin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        svgdocument: *mut ::core::ffi::c_void,
        viewportsize: ::windows::Foundation::Size,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Canvas_Svg", feature = "Foundation")))]
    DrawSvgAtOrigin: usize,
    #[cfg(all(feature = "Graphics_Canvas_Svg", feature = "Foundation_Numerics"))]
    pub DrawSvgAtPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        svgdocument: *mut ::core::ffi::c_void,
        viewportsize: ::windows::Foundation::Size,
        point: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Canvas_Svg", feature = "Foundation_Numerics")))]
    DrawSvgAtPoint: usize,
    #[cfg(all(feature = "Graphics_Canvas_Svg", feature = "Foundation"))]
    pub DrawSvgAtCoords: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        svgdocument: *mut ::core::ffi::c_void,
        viewportsize: ::windows::Foundation::Size,
        x: f32,
        y: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Canvas_Svg", feature = "Foundation")))]
    DrawSvgAtCoords: usize,
    pub Antialiasing: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasAntialiasing,
    ) -> ::windows::core::HRESULT,
    pub SetAntialiasing: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CanvasAntialiasing,
    ) -> ::windows::core::HRESULT,
    pub Blend: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasBlend,
    ) -> ::windows::core::HRESULT,
    pub SetBlend: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CanvasBlend,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Canvas_Text")]
    pub TextAntialiasing: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut Text::CanvasTextAntialiasing,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Text"))]
    TextAntialiasing: usize,
    #[cfg(feature = "Graphics_Canvas_Text")]
    pub SetTextAntialiasing: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: Text::CanvasTextAntialiasing,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Text"))]
    SetTextAntialiasing: usize,
    #[cfg(feature = "Graphics_Canvas_Text")]
    pub TextRenderingParameters: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Text"))]
    TextRenderingParameters: usize,
    #[cfg(feature = "Graphics_Canvas_Text")]
    pub SetTextRenderingParameters: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Text"))]
    SetTextRenderingParameters: usize,
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
    pub Units: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasUnits,
    ) -> ::windows::core::HRESULT,
    pub SetUnits: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CanvasUnits,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EffectBufferPrecision: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EffectBufferPrecision: usize,
    #[cfg(feature = "Foundation")]
    pub SetEffectBufferPrecision: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetEffectBufferPrecision: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub EffectTileSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Graphics::Imaging::BitmapSize,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    EffectTileSize: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetEffectTileSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Graphics::Imaging::BitmapSize,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetEffectTileSize: usize,
    pub CreateLayerWithOpacity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        opacity: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub CreateLayerWithOpacityBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        opacitybrush: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Brushes"))]
    CreateLayerWithOpacityBrush: usize,
    #[cfg(feature = "Foundation")]
    pub CreateLayerWithOpacityAndClipRectangle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        opacity: f32,
        cliprectangle: ::windows::Foundation::Rect,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateLayerWithOpacityAndClipRectangle: usize,
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation"))]
    pub CreateLayerWithOpacityBrushAndClipRectangle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        opacitybrush: *mut ::core::ffi::c_void,
        cliprectangle: ::windows::Foundation::Rect,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation")))]
    CreateLayerWithOpacityBrushAndClipRectangle: usize,
    #[cfg(feature = "Graphics_Canvas_Geometry")]
    pub CreateLayerWithOpacityAndClipGeometry: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        opacity: f32,
        clipgeometry: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Geometry"))]
    CreateLayerWithOpacityAndClipGeometry: usize,
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub CreateLayerWithOpacityBrushAndClipGeometry: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        opacitybrush: *mut ::core::ffi::c_void,
        clipgeometry: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Geometry"
            )
        )
    )]
    CreateLayerWithOpacityBrushAndClipGeometry: usize,
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "Foundation_Numerics"))]
    pub CreateLayerWithOpacityAndClipGeometryAndTransform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        opacity: f32,
        clipgeometry: *mut ::core::ffi::c_void,
        geometrytransform: ::windows::Foundation::Numerics::Matrix3x2,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(all(feature = "Graphics_Canvas_Geometry", feature = "Foundation_Numerics"))
    )]
    CreateLayerWithOpacityAndClipGeometryAndTransform: usize,
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics"
        )
    )]
    pub CreateLayerWithOpacityBrushAndClipGeometryAndTransform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        opacitybrush: *mut ::core::ffi::c_void,
        clipgeometry: *mut ::core::ffi::c_void,
        geometrytransform: ::windows::Foundation::Numerics::Matrix3x2,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Geometry",
                feature = "Foundation_Numerics"
            )
        )
    )]
    CreateLayerWithOpacityBrushAndClipGeometryAndTransform: usize,
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics"
        )
    )]
    pub CreateLayerWithAllOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        opacity: f32,
        opacitybrush: *mut ::core::ffi::c_void,
        cliprectangle: ::windows::Foundation::Rect,
        clipgeometry: *mut ::core::ffi::c_void,
        geometrytransform: ::windows::Foundation::Numerics::Matrix3x2,
        options: CanvasLayerOptions,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Geometry",
                feature = "Foundation_Numerics"
            )
        )
    )]
    CreateLayerWithAllOptions: usize,
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Text",
            feature = "Foundation_Numerics"
        )
    )]
    pub DrawGlyphRun: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        point: ::windows::Foundation::Numerics::Vector2,
        fontface: *mut ::core::ffi::c_void,
        fontsize: f32,
        glyphs_array_size: u32,
        glyphs: *const Text::CanvasGlyph,
        issideways: bool,
        bidilevel: u32,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Text",
                feature = "Foundation_Numerics"
            )
        )
    )]
    DrawGlyphRun: usize,
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Text",
            feature = "Foundation_Numerics"
        )
    )]
    pub DrawGlyphRunWithMeasuringMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        point: ::windows::Foundation::Numerics::Vector2,
        fontface: *mut ::core::ffi::c_void,
        fontsize: f32,
        glyphs_array_size: u32,
        glyphs: *const Text::CanvasGlyph,
        issideways: bool,
        bidilevel: u32,
        brush: *mut ::core::ffi::c_void,
        measuringmode: Text::CanvasTextMeasuringMode,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Text",
                feature = "Foundation_Numerics"
            )
        )
    )]
    DrawGlyphRunWithMeasuringMode: usize,
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Text",
            feature = "Foundation_Numerics"
        )
    )]
    pub DrawGlyphRunWithMeasuringModeAndDescription: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        point: ::windows::Foundation::Numerics::Vector2,
        fontface: *mut ::core::ffi::c_void,
        fontsize: f32,
        glyphs_array_size: u32,
        glyphs: *const Text::CanvasGlyph,
        issideways: bool,
        bidilevel: u32,
        brush: *mut ::core::ffi::c_void,
        measuringmode: Text::CanvasTextMeasuringMode,
        localename: *mut ::core::ffi::c_void,
        textstring: *mut ::core::ffi::c_void,
        clusterMapIndices_array_size: u32,
        clustermapindices: *const i32,
        textposition: u32,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Graphics_Canvas_Brushes",
                feature = "Graphics_Canvas_Text",
                feature = "Foundation_Numerics"
            )
        )
    )]
    DrawGlyphRunWithMeasuringModeAndDescription: usize,
    pub CreateSpriteBatch: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateSpriteBatchWithSortMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sortmode: CanvasSpriteSortMode,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateSpriteBatchWithSortModeAndInterpolation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sortmode: CanvasSpriteSortMode,
        interpolation: CanvasImageInterpolation,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateSpriteBatchWithSortModeAndInterpolationAndOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        sortmode: CanvasSpriteSortMode,
        interpolation: CanvasImageInterpolation,
        options: CanvasSpriteOptions,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
///*Required features: `"Graphics_Canvas"`*
#[repr(transparent)]
pub struct ICanvasImage(::windows::core::IUnknown);
impl ICanvasImage {
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn GetBounds<P0, E0>(
        &self,
        resourcecreator: P0,
    ) -> ::windows::core::Result<::windows::Foundation::Rect>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetBounds)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn GetBoundsWithTransform<P0, E0>(
        &self,
        resourcecreator: P0,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::core::Result<::windows::Foundation::Rect>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetBoundsWithTransform)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    transform,
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
::windows::core::interface_hierarchy!(
    ICanvasImage, ::windows::core::IUnknown, ::windows::core::IInspectable
);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ICanvasImage> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ICanvasImage) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ICanvasImage> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ICanvasImage) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ICanvasImage>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ICanvasImage) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Graphics_Effects")]
impl ::core::convert::TryFrom<ICanvasImage>
for ::windows::Graphics::Effects::IGraphicsEffectSource {
    type Error = ::windows::core::Error;
    fn try_from(value: ICanvasImage) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Graphics_Effects")]
impl ::core::convert::TryFrom<&ICanvasImage>
for ::windows::Graphics::Effects::IGraphicsEffectSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &ICanvasImage) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Graphics_Effects")]
impl ::core::convert::TryFrom<&ICanvasImage>
for ::windows::core::InParam<::windows::Graphics::Effects::IGraphicsEffectSource> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ICanvasImage) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for ICanvasImage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICanvasImage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICanvasImage {}
impl ::core::fmt::Debug for ICanvasImage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICanvasImage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICanvasImage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"{794966d3-6a64-47e9-8da8-b46aaa24d53b}",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ICanvasImage {
    type Vtable = ICanvasImage_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasImage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x794966d3_6a64_47e9_8da8_b46aaa24d53b,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasImage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetBounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetBounds: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub GetBoundsWithTransform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetBoundsWithTransform: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasImageStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasImageStatics {
    type Vtable = ICanvasImageStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasImageStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xc54eea15_5a14_489a_8fa0_6e84541f922d,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasImageStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SaveAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        image: *mut ::core::ffi::c_void,
        sourcerectangle: ::windows::Foundation::Rect,
        dpi: f32,
        resourcecreator: *mut ::core::ffi::c_void,
        stream: *mut ::core::ffi::c_void,
        fileformat: CanvasBitmapFileFormat,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SaveAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SaveWithQualityAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        image: *mut ::core::ffi::c_void,
        sourcerectangle: ::windows::Foundation::Rect,
        dpi: f32,
        resourcecreator: *mut ::core::ffi::c_void,
        stream: *mut ::core::ffi::c_void,
        fileformat: CanvasBitmapFileFormat,
        quality: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SaveWithQualityAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SaveWithQualityAndBufferPrecisionAsync: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        image: *mut ::core::ffi::c_void,
        sourcerectangle: ::windows::Foundation::Rect,
        dpi: f32,
        resourcecreator: *mut ::core::ffi::c_void,
        stream: *mut ::core::ffi::c_void,
        fileformat: CanvasBitmapFileFormat,
        quality: f32,
        bufferprecision: CanvasBufferPrecision,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SaveWithQualityAndBufferPrecisionAsync: usize,
    #[cfg(all(feature = "Graphics_Canvas_Effects", feature = "Foundation"))]
    pub ComputeHistogram: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        image: *mut ::core::ffi::c_void,
        sourcerectangle: ::windows::Foundation::Rect,
        resourcecreator: *mut ::core::ffi::c_void,
        channelselect: Effects::EffectChannelSelect,
        numberofbins: i32,
        result_size__: *mut u32,
        result__: *mut *mut f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Canvas_Effects", feature = "Foundation")))]
    ComputeHistogram: usize,
    pub IsHistogramSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        device: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasLock(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasLock {
    type Vtable = ICanvasLock_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasLock {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x7a0e8498_fba9_4fb0_aa8c_6a48b5ee3e4f,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasLock_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasRenderTarget(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasRenderTarget {
    type Vtable = ICanvasRenderTarget_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasRenderTarget {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x2d4c7349_9a32_41b9_b3cc_caf1b7e1099b,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasRenderTarget_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateDrawingSession: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasRenderTargetFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasRenderTargetFactory {
    type Vtable = ICanvasRenderTargetFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasRenderTargetFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x620dfdbb_9d08_406c_bfe6_d9b81e6df8e7,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasRenderTargetFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateWithSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        size: ::windows::Foundation::Size,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWithSize: usize,
    pub CreateWithWidthAndHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        width: f32,
        height: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateWithWidthAndHeightAndDpi: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        width: f32,
        height: f32,
        dpi: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_DirectX")]
    pub CreateWithWidthAndHeightAndDpiAndFormatAndAlpha: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        width: f32,
        height: f32,
        dpi: f32,
        format: ::windows::Graphics::DirectX::DirectXPixelFormat,
        alpha: CanvasAlphaMode,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    CreateWithWidthAndHeightAndDpiAndFormatAndAlpha: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasRenderTargetStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasRenderTargetStatics {
    type Vtable = ICanvasRenderTargetStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasRenderTargetStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xc7d1fe37_dd57_45d7_bcc1_15625a21e8d5,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasRenderTargetStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateFromDirect3D11Surface: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        surface: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateFromDirect3D11Surface: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateFromDirect3D11SurfaceWithDpi: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        surface: *mut ::core::ffi::c_void,
        dpi: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateFromDirect3D11SurfaceWithDpi: usize,
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub CreateFromDirect3D11SurfaceWithDpiAndAlpha: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        surface: *mut ::core::ffi::c_void,
        dpi: f32,
        alpha: CanvasAlphaMode,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX_Direct3D11"))]
    CreateFromDirect3D11SurfaceWithDpiAndAlpha: usize,
}
///*Required features: `"Graphics_Canvas"`*
#[repr(transparent)]
pub struct ICanvasResourceCreator(::windows::core::IUnknown);
impl ICanvasResourceCreator {
    pub fn Device(&self) -> ::windows::core::Result<CanvasDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Device)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(
    ICanvasResourceCreator, ::windows::core::IUnknown, ::windows::core::IInspectable
);
impl ::core::clone::Clone for ICanvasResourceCreator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICanvasResourceCreator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICanvasResourceCreator {}
impl ::core::fmt::Debug for ICanvasResourceCreator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICanvasResourceCreator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICanvasResourceCreator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"{8f6d8aa8-492f-4bc6-b3d0-e7f5eae84b11}",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ICanvasResourceCreator {
    type Vtable = ICanvasResourceCreator_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasResourceCreator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x8f6d8aa8_492f_4bc6_b3d0_e7f5eae84b11,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasResourceCreator_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Device: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
///*Required features: `"Graphics_Canvas"`*
#[repr(transparent)]
pub struct ICanvasResourceCreatorWithDpi(::windows::core::IUnknown);
impl ICanvasResourceCreatorWithDpi {
    pub fn Dpi(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Dpi)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn ConvertPixelsToDips(&self, pixels: i32) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ConvertPixelsToDips)(
                    ::windows::core::Vtable::as_raw(this),
                    pixels,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn ConvertDipsToPixels(
        &self,
        dips: f32,
        dpirounding: CanvasDpiRounding,
    ) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ConvertDipsToPixels)(
                    ::windows::core::Vtable::as_raw(this),
                    dips,
                    dpirounding,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn Device(&self) -> ::windows::core::Result<CanvasDevice> {
        let this = &::windows::core::Interface::cast::<ICanvasResourceCreator>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Device)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(
    ICanvasResourceCreatorWithDpi, ::windows::core::IUnknown,
    ::windows::core::IInspectable
);
impl ::core::convert::TryFrom<ICanvasResourceCreatorWithDpi> for ICanvasResourceCreator {
    type Error = ::windows::core::Error;
    fn try_from(value: ICanvasResourceCreatorWithDpi) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ICanvasResourceCreatorWithDpi>
for ICanvasResourceCreator {
    type Error = ::windows::core::Error;
    fn try_from(value: &ICanvasResourceCreatorWithDpi) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&ICanvasResourceCreatorWithDpi>
for ::windows::core::InParam<ICanvasResourceCreator> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ICanvasResourceCreatorWithDpi) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for ICanvasResourceCreatorWithDpi {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICanvasResourceCreatorWithDpi {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICanvasResourceCreatorWithDpi {}
impl ::core::fmt::Debug for ICanvasResourceCreatorWithDpi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICanvasResourceCreatorWithDpi").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICanvasResourceCreatorWithDpi {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"{1a75b512-e9fa-49e6-a876-38cae194013e}",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ICanvasResourceCreatorWithDpi {
    type Vtable = ICanvasResourceCreatorWithDpi_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasResourceCreatorWithDpi {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x1a75b512_e9fa_49e6_a876_38cae194013e,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasResourceCreatorWithDpi_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Dpi: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub ConvertPixelsToDips: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pixels: i32,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub ConvertDipsToPixels: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dips: f32,
        dpirounding: CanvasDpiRounding,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasSpriteBatch(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasSpriteBatch {
    type Vtable = ICanvasSpriteBatch_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasSpriteBatch {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xa065dce4_a7f2_4df4_8405_ea9e3a215bf8,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasSpriteBatch_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub DrawToRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bitmap: *mut ::core::ffi::c_void,
        destrect: ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DrawToRect: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub DrawAtOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bitmap: *mut ::core::ffi::c_void,
        offset: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DrawAtOffset: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub DrawWithTransform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bitmap: *mut ::core::ffi::c_void,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DrawWithTransform: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub DrawToRectWithTint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bitmap: *mut ::core::ffi::c_void,
        destrect: ::windows::Foundation::Rect,
        tint: ::windows::Foundation::Numerics::Vector4,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DrawToRectWithTint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub DrawAtOffsetWithTint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bitmap: *mut ::core::ffi::c_void,
        offset: ::windows::Foundation::Numerics::Vector2,
        tint: ::windows::Foundation::Numerics::Vector4,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DrawAtOffsetWithTint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub DrawWithTransformAndTint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bitmap: *mut ::core::ffi::c_void,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        tint: ::windows::Foundation::Numerics::Vector4,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DrawWithTransformAndTint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub DrawToRectWithTintAndFlip: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bitmap: *mut ::core::ffi::c_void,
        destrect: ::windows::Foundation::Rect,
        tint: ::windows::Foundation::Numerics::Vector4,
        flip: CanvasSpriteFlip,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DrawToRectWithTintAndFlip: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub DrawWithTransformAndTintAndFlip: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bitmap: *mut ::core::ffi::c_void,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        tint: ::windows::Foundation::Numerics::Vector4,
        flip: CanvasSpriteFlip,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DrawWithTransformAndTintAndFlip: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub DrawAtOffsetWithTintAndTransform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bitmap: *mut ::core::ffi::c_void,
        offset: ::windows::Foundation::Numerics::Vector2,
        tint: ::windows::Foundation::Numerics::Vector4,
        origin: ::windows::Foundation::Numerics::Vector2,
        rotation: f32,
        scale: ::windows::Foundation::Numerics::Vector2,
        flip: CanvasSpriteFlip,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DrawAtOffsetWithTintAndTransform: usize,
    #[cfg(feature = "Foundation")]
    pub DrawFromSpriteSheetToRect: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bitmap: *mut ::core::ffi::c_void,
        destrect: ::windows::Foundation::Rect,
        sourcerect: ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DrawFromSpriteSheetToRect: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub DrawFromSpriteSheetAtOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bitmap: *mut ::core::ffi::c_void,
        offset: ::windows::Foundation::Numerics::Vector2,
        sourcerect: ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DrawFromSpriteSheetAtOffset: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub DrawFromSpriteSheetWithTransform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bitmap: *mut ::core::ffi::c_void,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        sourcerect: ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DrawFromSpriteSheetWithTransform: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub DrawFromSpriteSheetToRectWithTint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bitmap: *mut ::core::ffi::c_void,
        destrect: ::windows::Foundation::Rect,
        sourcerect: ::windows::Foundation::Rect,
        tint: ::windows::Foundation::Numerics::Vector4,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DrawFromSpriteSheetToRectWithTint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub DrawFromSpriteSheetAtOffsetWithTint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bitmap: *mut ::core::ffi::c_void,
        offset: ::windows::Foundation::Numerics::Vector2,
        sourcerect: ::windows::Foundation::Rect,
        tint: ::windows::Foundation::Numerics::Vector4,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DrawFromSpriteSheetAtOffsetWithTint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub DrawFromSpriteSheetWithTransformAndTint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bitmap: *mut ::core::ffi::c_void,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        sourcerect: ::windows::Foundation::Rect,
        tint: ::windows::Foundation::Numerics::Vector4,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DrawFromSpriteSheetWithTransformAndTint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub DrawFromSpriteSheetToRectWithTintAndFlip: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bitmap: *mut ::core::ffi::c_void,
        destrect: ::windows::Foundation::Rect,
        sourcerect: ::windows::Foundation::Rect,
        tint: ::windows::Foundation::Numerics::Vector4,
        flip: CanvasSpriteFlip,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DrawFromSpriteSheetToRectWithTintAndFlip: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub DrawFromSpriteSheetWithTransformAndTintAndFlip: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bitmap: *mut ::core::ffi::c_void,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        sourcerect: ::windows::Foundation::Rect,
        tint: ::windows::Foundation::Numerics::Vector4,
        flip: CanvasSpriteFlip,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DrawFromSpriteSheetWithTransformAndTintAndFlip: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub DrawFromSpriteSheetAtOffsetWithTintAndTransform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        bitmap: *mut ::core::ffi::c_void,
        offset: ::windows::Foundation::Numerics::Vector2,
        sourcerect: ::windows::Foundation::Rect,
        tint: ::windows::Foundation::Numerics::Vector4,
        origin: ::windows::Foundation::Numerics::Vector2,
        rotation: f32,
        scale: ::windows::Foundation::Numerics::Vector2,
        flip: CanvasSpriteFlip,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DrawFromSpriteSheetAtOffsetWithTintAndTransform: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasSpriteBatchStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasSpriteBatchStatics {
    type Vtable = ICanvasSpriteBatchStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasSpriteBatchStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x851eb08d_9d01_4b57_9e94_24113151b74b,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasSpriteBatchStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        device: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasSwapChain(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasSwapChain {
    type Vtable = ICanvasSwapChain_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasSwapChain {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x882e3c3a_5725_409c_9e76_f80b3bacf1b4,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasSwapChain_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Present: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub PresentWithSyncInterval: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        syncinterval: i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ResizeBuffersWithSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        newsize: ::windows::Foundation::Size,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ResizeBuffersWithSize: usize,
    pub ResizeBuffersWithWidthAndHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        newwidth: f32,
        newheight: f32,
    ) -> ::windows::core::HRESULT,
    pub ResizeBuffersWithWidthAndHeightAndDpi: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        newwidth: f32,
        newheight: f32,
        newdpi: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_DirectX")]
    pub ResizeBuffersWithAllOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        newwidth: f32,
        newheight: f32,
        newdpi: f32,
        newformat: ::windows::Graphics::DirectX::DirectXPixelFormat,
        buffercount: i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    ResizeBuffersWithAllOptions: usize,
    #[cfg(feature = "Foundation")]
    pub Size: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Size,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Size: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SizeInPixels: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Graphics::Imaging::BitmapSize,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SizeInPixels: usize,
    #[cfg(feature = "Graphics_DirectX")]
    pub Format: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Graphics::DirectX::DirectXPixelFormat,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    Format: usize,
    pub BufferCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub AlphaMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasAlphaMode,
    ) -> ::windows::core::HRESULT,
    pub Rotation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasSwapChainRotation,
    ) -> ::windows::core::HRESULT,
    pub SetRotation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CanvasSwapChainRotation,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SourceSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Size,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourceSize: usize,
    #[cfg(feature = "Foundation")]
    pub SetSourceSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Size,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSourceSize: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TransformMatrix: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TransformMatrix: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransformMatrix: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransformMatrix: usize,
    #[cfg(feature = "UI")]
    pub CreateDrawingSession: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        clearcolor: ::windows::UI::Color,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    CreateDrawingSession: usize,
    pub WaitForVerticalBlank: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasSwapChainFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasSwapChainFactory {
    type Vtable = ICanvasSwapChainFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasSwapChainFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x133c25cb_ed3c_492b_bffe_7509b521842b,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasSwapChainFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateWithSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        size: ::windows::Foundation::Size,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWithSize: usize,
    pub CreateWithWidthAndHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        width: f32,
        height: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateWithWidthAndHeightAndDpi: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        width: f32,
        height: f32,
        dpi: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_DirectX")]
    pub CreateWithAllOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        width: f32,
        height: f32,
        dpi: f32,
        format: ::windows::Graphics::DirectX::DirectXPixelFormat,
        buffercount: i32,
        alphamode: CanvasAlphaMode,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_DirectX"))]
    CreateWithAllOptions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasSwapChainStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasSwapChainStatics {
    type Vtable = ICanvasSwapChainStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasSwapChainStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x05376d8f_3e8d_4a82_9838_691680d32a52,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasSwapChainStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Core")]
    pub CreateForCoreWindowWithDpi: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        corewindow: *mut ::core::ffi::c_void,
        dpi: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    CreateForCoreWindowWithDpi: usize,
    #[cfg(all(feature = "Graphics_DirectX", feature = "UI_Core"))]
    pub CreateForCoreWindowWithAllOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        corewindow: *mut ::core::ffi::c_void,
        width: f32,
        height: f32,
        dpi: f32,
        format: ::windows::Graphics::DirectX::DirectXPixelFormat,
        buffercount: i32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_DirectX", feature = "UI_Core")))]
    CreateForCoreWindowWithAllOptions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasVirtualBitmap(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasVirtualBitmap {
    type Vtable = ICanvasVirtualBitmap_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasVirtualBitmap {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x707d8bb0_05f9_484c_9ee2_179e0681c8a7,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasVirtualBitmap_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Device: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub IsCachedOnDemand: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")]
    pub SizeInPixels: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Graphics::Imaging::BitmapSize,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SizeInPixels: usize,
    #[cfg(feature = "Foundation")]
    pub Size: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Size,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Size: usize,
    #[cfg(feature = "Foundation")]
    pub Bounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Bounds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasVirtualBitmapStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasVirtualBitmapStatics {
    type Vtable = ICanvasVirtualBitmapStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasVirtualBitmapStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xb2f1f8e9_0770_4dd4_956d_78d911390957,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasVirtualBitmapStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub LoadAsyncFromFileName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        filename: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadAsyncFromFileName: usize,
    #[cfg(feature = "Foundation")]
    pub LoadAsyncFromFileNameWithOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        filename: *mut ::core::ffi::c_void,
        options: CanvasVirtualBitmapOptions,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadAsyncFromFileNameWithOptions: usize,
    #[cfg(feature = "Foundation")]
    pub LoadAsyncFromFileNameWithOptionsAndAlpha: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        filename: *mut ::core::ffi::c_void,
        options: CanvasVirtualBitmapOptions,
        alpha: CanvasAlphaMode,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadAsyncFromFileNameWithOptionsAndAlpha: usize,
    #[cfg(feature = "Foundation")]
    pub LoadAsyncFromUri: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        uri: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadAsyncFromUri: usize,
    #[cfg(feature = "Foundation")]
    pub LoadAsyncFromUriWithOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        uri: *mut ::core::ffi::c_void,
        options: CanvasVirtualBitmapOptions,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadAsyncFromUriWithOptions: usize,
    #[cfg(feature = "Foundation")]
    pub LoadAsyncFromUriWithOptionsAndAlpha: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        uri: *mut ::core::ffi::c_void,
        options: CanvasVirtualBitmapOptions,
        alpha: CanvasAlphaMode,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadAsyncFromUriWithOptionsAndAlpha: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub LoadAsyncFromStream: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        stream: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    LoadAsyncFromStream: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub LoadAsyncFromStreamWithOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        stream: *mut ::core::ffi::c_void,
        options: CanvasVirtualBitmapOptions,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    LoadAsyncFromStreamWithOptions: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub LoadAsyncFromStreamWithOptionsAndAlpha: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        stream: *mut ::core::ffi::c_void,
        options: CanvasVirtualBitmapOptions,
        alpha: CanvasAlphaMode,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    LoadAsyncFromStreamWithOptionsAndAlpha: usize,
}
///*Required features: `"Graphics_Canvas"`*
#[repr(transparent)]
pub struct CanvasActiveLayer(::windows::core::IUnknown);
impl CanvasActiveLayer {
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
impl ::core::clone::Clone for CanvasActiveLayer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasActiveLayer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasActiveLayer {}
impl ::core::fmt::Debug for CanvasActiveLayer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasActiveLayer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasActiveLayer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.CanvasActiveLayer;{49ecfc58-5e1c-4ee3-8088-542f94e93c60})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasActiveLayer {
    type Vtable = ICanvasActiveLayer_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasActiveLayer {
    const IID: ::windows::core::GUID = <ICanvasActiveLayer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasActiveLayer {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.CanvasActiveLayer";
}
::windows::core::interface_hierarchy!(
    CanvasActiveLayer,::windows::core::IUnknown,::windows::core::IInspectable
);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasActiveLayer> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasActiveLayer) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasActiveLayer> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasActiveLayer) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasActiveLayer>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasActiveLayer) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CanvasActiveLayer {}
unsafe impl ::core::marker::Sync for CanvasActiveLayer {}
///*Required features: `"Graphics_Canvas"`*
#[repr(transparent)]
pub struct CanvasBitmap(::windows::core::IUnknown);
impl CanvasBitmap {
    ///*Required features: `"Graphics_Imaging"`*
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SizeInPixels(
        &self,
    ) -> ::windows::core::Result<::windows::Graphics::Imaging::BitmapSize> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .SizeInPixels)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn Size(&self) -> ::windows::core::Result<::windows::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn Bounds(&self) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Bounds)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"Graphics_DirectX"`*
    #[cfg(feature = "Graphics_DirectX")]
    pub fn Format(
        &self,
    ) -> ::windows::core::Result<::windows::Graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Format)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn AlphaMode(&self) -> ::windows::core::Result<CanvasAlphaMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .AlphaMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn SaveToFileAsync(
        &self,
        filename: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .SaveToFileAsync)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(filename),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn SaveToFileWithBitmapFileFormatAsync(
        &self,
        filename: &::windows::core::HSTRING,
        fileformat: CanvasBitmapFileFormat,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .SaveToFileWithBitmapFileFormatAsync)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(filename),
                    fileformat,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn SaveToFileWithBitmapFileFormatAndQualityAsync(
        &self,
        filename: &::windows::core::HSTRING,
        fileformat: CanvasBitmapFileFormat,
        quality: f32,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .SaveToFileWithBitmapFileFormatAndQualityAsync)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(filename),
                    fileformat,
                    quality,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`, `"Storage_Streams"`*
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SaveToStreamAsync<P0, E0>(
        &self,
        stream: P0,
        fileformat: CanvasBitmapFileFormat,
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
                .SaveToStreamAsync)(
                    ::windows::core::Vtable::as_raw(this),
                    stream.try_into().map_err(|e| e.into())?.abi(),
                    fileformat,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`, `"Storage_Streams"`*
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SaveToStreamWithQualityAsync<P0, E0>(
        &self,
        stream: P0,
        fileformat: CanvasBitmapFileFormat,
        quality: f32,
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
                .SaveToStreamWithQualityAsync)(
                    ::windows::core::Vtable::as_raw(this),
                    stream.try_into().map_err(|e| e.into())?.abi(),
                    fileformat,
                    quality,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn GetPixelBytes(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetPixelBytes)(
                    ::windows::core::Vtable::as_raw(this),
                    ::windows::core::Array::<
                        u8,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn GetPixelBytesWithSubrectangle(
        &self,
        left: i32,
        top: i32,
        width: i32,
        height: i32,
    ) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetPixelBytesWithSubrectangle)(
                    ::windows::core::Vtable::as_raw(this),
                    left,
                    top,
                    width,
                    height,
                    ::windows::core::Array::<
                        u8,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    ///*Required features: `"Storage_Streams"`*
    #[cfg(feature = "Storage_Streams")]
    pub fn GetPixelBytesWithBuffer<P0, E0>(
        &self,
        buffer: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<::windows::Storage::Streams::IBuffer>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .GetPixelBytesWithBuffer)(
                    ::windows::core::Vtable::as_raw(this),
                    buffer.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Storage_Streams"`*
    #[cfg(feature = "Storage_Streams")]
    pub fn GetPixelBytesWithBufferAndSubrectangle<P0, E0>(
        &self,
        buffer: P0,
        left: i32,
        top: i32,
        width: i32,
        height: i32,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<::windows::Storage::Streams::IBuffer>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .GetPixelBytesWithBufferAndSubrectangle)(
                    ::windows::core::Vtable::as_raw(this),
                    buffer.try_into().map_err(|e| e.into())?.abi(),
                    left,
                    top,
                    width,
                    height,
                )
                .ok()
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn GetPixelColors(
        &self,
    ) -> ::windows::core::Result<::windows::core::Array<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetPixelColors)(
                    ::windows::core::Vtable::as_raw(this),
                    ::windows::core::Array::<
                        ::windows::UI::Color,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn GetPixelColorsWithSubrectangle(
        &self,
        left: i32,
        top: i32,
        width: i32,
        height: i32,
    ) -> ::windows::core::Result<::windows::core::Array<::windows::UI::Color>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetPixelColorsWithSubrectangle)(
                    ::windows::core::Vtable::as_raw(this),
                    left,
                    top,
                    width,
                    height,
                    ::windows::core::Array::<
                        ::windows::UI::Color,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn SetPixelBytes(&self, valueelements: &[u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetPixelBytes)(
                    ::windows::core::Vtable::as_raw(this),
                    valueelements.len() as u32,
                    valueelements.as_ptr(),
                )
                .ok()
        }
    }
    pub fn SetPixelBytesWithSubrectangle(
        &self,
        valueelements: &[u8],
        left: i32,
        top: i32,
        width: i32,
        height: i32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetPixelBytesWithSubrectangle)(
                    ::windows::core::Vtable::as_raw(this),
                    valueelements.len() as u32,
                    valueelements.as_ptr(),
                    left,
                    top,
                    width,
                    height,
                )
                .ok()
        }
    }
    ///*Required features: `"Storage_Streams"`*
    #[cfg(feature = "Storage_Streams")]
    pub fn SetPixelBytesWithBuffer<P0, E0>(
        &self,
        buffer: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<::windows::Storage::Streams::IBuffer>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetPixelBytesWithBuffer)(
                    ::windows::core::Vtable::as_raw(this),
                    buffer.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Storage_Streams"`*
    #[cfg(feature = "Storage_Streams")]
    pub fn SetPixelBytesWithBufferAndSubrectangle<P0, E0>(
        &self,
        buffer: P0,
        left: i32,
        top: i32,
        width: i32,
        height: i32,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<::windows::Storage::Streams::IBuffer>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetPixelBytesWithBufferAndSubrectangle)(
                    ::windows::core::Vtable::as_raw(this),
                    buffer.try_into().map_err(|e| e.into())?.abi(),
                    left,
                    top,
                    width,
                    height,
                )
                .ok()
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn SetPixelColors(
        &self,
        valueelements: &[::windows::UI::Color],
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetPixelColors)(
                    ::windows::core::Vtable::as_raw(this),
                    valueelements.len() as u32,
                    valueelements.as_ptr(),
                )
                .ok()
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn SetPixelColorsWithSubrectangle(
        &self,
        valueelements: &[::windows::UI::Color],
        left: i32,
        top: i32,
        width: i32,
        height: i32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetPixelColorsWithSubrectangle)(
                    ::windows::core::Vtable::as_raw(this),
                    valueelements.len() as u32,
                    valueelements.as_ptr(),
                    left,
                    top,
                    width,
                    height,
                )
                .ok()
        }
    }
    pub fn CopyPixelsFromBitmap<P0>(
        &self,
        otherbitmap: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<CanvasBitmap>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .CopyPixelsFromBitmap)(
                    ::windows::core::Vtable::as_raw(this),
                    otherbitmap.into().abi(),
                )
                .ok()
        }
    }
    pub fn CopyPixelsFromBitmapWithDestPoint<P0>(
        &self,
        otherbitmap: P0,
        destx: i32,
        desty: i32,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<CanvasBitmap>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .CopyPixelsFromBitmapWithDestPoint)(
                    ::windows::core::Vtable::as_raw(this),
                    otherbitmap.into().abi(),
                    destx,
                    desty,
                )
                .ok()
        }
    }
    pub fn CopyPixelsFromBitmapWithDestPointAndSourceRect<P0>(
        &self,
        otherbitmap: P0,
        destx: i32,
        desty: i32,
        sourcerectleft: i32,
        sourcerecttop: i32,
        sourcerectwidth: i32,
        sourcerectheight: i32,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<CanvasBitmap>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .CopyPixelsFromBitmapWithDestPointAndSourceRect)(
                    ::windows::core::Vtable::as_raw(this),
                    otherbitmap.into().abi(),
                    destx,
                    desty,
                    sourcerectleft,
                    sourcerecttop,
                    sourcerectwidth,
                    sourcerectheight,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_DirectX_Direct3D11"`*
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CreateFromDirect3D11Surface<P0, E0, P1, E1>(
        resourcecreator: P0,
        surface: P1,
    ) -> ::windows::core::Result<CanvasBitmap>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<
                ::windows::Graphics::DirectX::Direct3D11::IDirect3DSurface,
            >,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateFromDirect3D11Surface)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    surface.try_into().map_err(|e| e.into())?.abi(),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Graphics_DirectX_Direct3D11"`*
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CreateFromDirect3D11SurfaceWithDpi<P0, E0, P1, E1>(
        resourcecreator: P0,
        surface: P1,
        dpi: f32,
    ) -> ::windows::core::Result<CanvasBitmap>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<
                ::windows::Graphics::DirectX::Direct3D11::IDirect3DSurface,
            >,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateFromDirect3D11SurfaceWithDpi)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    surface.try_into().map_err(|e| e.into())?.abi(),
                    dpi,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Graphics_DirectX_Direct3D11"`*
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CreateFromDirect3D11SurfaceWithDpiAndAlpha<P0, E0, P1, E1>(
        resourcecreator: P0,
        surface: P1,
        dpi: f32,
        alpha: CanvasAlphaMode,
    ) -> ::windows::core::Result<CanvasBitmap>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<
                ::windows::Graphics::DirectX::Direct3D11::IDirect3DSurface,
            >,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateFromDirect3D11SurfaceWithDpiAndAlpha)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    surface.try_into().map_err(|e| e.into())?.abi(),
                    dpi,
                    alpha,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Graphics_DirectX"`*
    #[cfg(feature = "Graphics_DirectX")]
    pub fn CreateFromBytes<P0, E0>(
        resourcecreator: P0,
        bytes: &[u8],
        widthinpixels: i32,
        heightinpixels: i32,
        format: ::windows::Graphics::DirectX::DirectXPixelFormat,
    ) -> ::windows::core::Result<CanvasBitmap>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateFromBytes)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    bytes.len() as u32,
                    bytes.as_ptr(),
                    widthinpixels,
                    heightinpixels,
                    format,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Graphics_DirectX"`*
    #[cfg(feature = "Graphics_DirectX")]
    pub fn CreateFromBytesWithDpi<P0, E0>(
        resourcecreator: P0,
        bytes: &[u8],
        widthinpixels: i32,
        heightinpixels: i32,
        format: ::windows::Graphics::DirectX::DirectXPixelFormat,
        dpi: f32,
    ) -> ::windows::core::Result<CanvasBitmap>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateFromBytesWithDpi)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    bytes.len() as u32,
                    bytes.as_ptr(),
                    widthinpixels,
                    heightinpixels,
                    format,
                    dpi,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Graphics_DirectX"`*
    #[cfg(feature = "Graphics_DirectX")]
    pub fn CreateFromBytesWithDpiAndAlpha<P0, E0>(
        resourcecreator: P0,
        bytes: &[u8],
        widthinpixels: i32,
        heightinpixels: i32,
        format: ::windows::Graphics::DirectX::DirectXPixelFormat,
        dpi: f32,
        alpha: CanvasAlphaMode,
    ) -> ::windows::core::Result<CanvasBitmap>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateFromBytesWithDpiAndAlpha)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    bytes.len() as u32,
                    bytes.as_ptr(),
                    widthinpixels,
                    heightinpixels,
                    format,
                    dpi,
                    alpha,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Graphics_DirectX"`, `"Storage_Streams"`*
    #[cfg(all(feature = "Graphics_DirectX", feature = "Storage_Streams"))]
    pub fn CreateFromBytesWithBuffer<P0, E0, P1, E1>(
        resourcecreator: P0,
        buffer: P1,
        widthinpixels: i32,
        heightinpixels: i32,
        format: ::windows::Graphics::DirectX::DirectXPixelFormat,
    ) -> ::windows::core::Result<CanvasBitmap>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<::windows::Storage::Streams::IBuffer>,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateFromBytesWithBuffer)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    buffer.try_into().map_err(|e| e.into())?.abi(),
                    widthinpixels,
                    heightinpixels,
                    format,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Graphics_DirectX"`, `"Storage_Streams"`*
    #[cfg(all(feature = "Graphics_DirectX", feature = "Storage_Streams"))]
    pub fn CreateFromBytesWithBufferAndDpi<P0, E0, P1, E1>(
        resourcecreator: P0,
        buffer: P1,
        widthinpixels: i32,
        heightinpixels: i32,
        format: ::windows::Graphics::DirectX::DirectXPixelFormat,
        dpi: f32,
    ) -> ::windows::core::Result<CanvasBitmap>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<::windows::Storage::Streams::IBuffer>,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateFromBytesWithBufferAndDpi)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    buffer.try_into().map_err(|e| e.into())?.abi(),
                    widthinpixels,
                    heightinpixels,
                    format,
                    dpi,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Graphics_DirectX"`, `"Storage_Streams"`*
    #[cfg(all(feature = "Graphics_DirectX", feature = "Storage_Streams"))]
    pub fn CreateFromBytesWithBufferAndDpiAndAlpha<P0, E0, P1, E1>(
        resourcecreator: P0,
        buffer: P1,
        widthinpixels: i32,
        heightinpixels: i32,
        format: ::windows::Graphics::DirectX::DirectXPixelFormat,
        dpi: f32,
        alpha: CanvasAlphaMode,
    ) -> ::windows::core::Result<CanvasBitmap>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<::windows::Storage::Streams::IBuffer>,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateFromBytesWithBufferAndDpiAndAlpha)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    buffer.try_into().map_err(|e| e.into())?.abi(),
                    widthinpixels,
                    heightinpixels,
                    format,
                    dpi,
                    alpha,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn CreateFromColors<P0, E0>(
        resourcecreator: P0,
        colors: &[::windows::UI::Color],
        widthinpixels: i32,
        heightinpixels: i32,
    ) -> ::windows::core::Result<CanvasBitmap>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateFromColors)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    colors.len() as u32,
                    colors.as_ptr(),
                    widthinpixels,
                    heightinpixels,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn CreateFromColorsWithDpi<P0, E0>(
        resourcecreator: P0,
        colors: &[::windows::UI::Color],
        widthinpixels: i32,
        heightinpixels: i32,
        dpi: f32,
    ) -> ::windows::core::Result<CanvasBitmap>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateFromColorsWithDpi)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    colors.len() as u32,
                    colors.as_ptr(),
                    widthinpixels,
                    heightinpixels,
                    dpi,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn CreateFromColorsWithDpiAndAlpha<P0, E0>(
        resourcecreator: P0,
        colors: &[::windows::UI::Color],
        widthinpixels: i32,
        heightinpixels: i32,
        dpi: f32,
        alpha: CanvasAlphaMode,
    ) -> ::windows::core::Result<CanvasBitmap>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateFromColorsWithDpiAndAlpha)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    colors.len() as u32,
                    colors.as_ptr(),
                    widthinpixels,
                    heightinpixels,
                    dpi,
                    alpha,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Graphics_Imaging"`*
    #[cfg(feature = "Graphics_Imaging")]
    pub fn CreateFromSoftwareBitmap<P0, E0>(
        resourcecreator: P0,
        sourcebitmap: &::windows::Graphics::Imaging::SoftwareBitmap,
    ) -> ::windows::core::Result<CanvasBitmap>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateFromSoftwareBitmap)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    ::core::mem::transmute_copy(sourcebitmap),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn LoadAsyncFromHstring<P0, E0>(
        resourcecreator: P0,
        filename: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncOperation::<CanvasBitmap>>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LoadAsyncFromHstring)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    ::core::mem::transmute_copy(filename),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn LoadAsyncFromHstringWithDpi<P0, E0>(
        resourcecreator: P0,
        filename: &::windows::core::HSTRING,
        dpi: f32,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncOperation::<CanvasBitmap>>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LoadAsyncFromHstringWithDpi)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    ::core::mem::transmute_copy(filename),
                    dpi,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn LoadAsyncFromHstringWithDpiAndAlpha<P0, E0>(
        resourcecreator: P0,
        filename: &::windows::core::HSTRING,
        dpi: f32,
        alpha: CanvasAlphaMode,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncOperation::<CanvasBitmap>>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LoadAsyncFromHstringWithDpiAndAlpha)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    ::core::mem::transmute_copy(filename),
                    dpi,
                    alpha,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn LoadAsyncFromUri<P0, E0>(
        resourcecreator: P0,
        uri: &::windows::Foundation::Uri,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncOperation::<CanvasBitmap>>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LoadAsyncFromUri)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    ::core::mem::transmute_copy(uri),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn LoadAsyncFromUriWithDpi<P0, E0>(
        resourcecreator: P0,
        uri: &::windows::Foundation::Uri,
        dpi: f32,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncOperation::<CanvasBitmap>>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LoadAsyncFromUriWithDpi)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    ::core::mem::transmute_copy(uri),
                    dpi,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn LoadAsyncFromUriWithDpiAndAlpha<P0, E0>(
        resourcecreator: P0,
        uri: &::windows::Foundation::Uri,
        dpi: f32,
        alpha: CanvasAlphaMode,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncOperation::<CanvasBitmap>>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LoadAsyncFromUriWithDpiAndAlpha)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    ::core::mem::transmute_copy(uri),
                    dpi,
                    alpha,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation"`, `"Storage_Streams"`*
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn LoadAsyncFromStream<P0, E0, P1, E1>(
        resourcecreator: P0,
        stream: P1,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncOperation::<CanvasBitmap>>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<::windows::Storage::Streams::IRandomAccessStream>,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LoadAsyncFromStream)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    stream.try_into().map_err(|e| e.into())?.abi(),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation"`, `"Storage_Streams"`*
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn LoadAsyncFromStreamWithDpi<P0, E0, P1, E1>(
        resourcecreator: P0,
        stream: P1,
        dpi: f32,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncOperation::<CanvasBitmap>>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<::windows::Storage::Streams::IRandomAccessStream>,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LoadAsyncFromStreamWithDpi)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    stream.try_into().map_err(|e| e.into())?.abi(),
                    dpi,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation"`, `"Storage_Streams"`*
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn LoadAsyncFromStreamWithDpiAndAlpha<P0, E0, P1, E1>(
        resourcecreator: P0,
        stream: P1,
        dpi: f32,
        alpha: CanvasAlphaMode,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncOperation::<CanvasBitmap>>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<::windows::Storage::Streams::IRandomAccessStream>,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LoadAsyncFromStreamWithDpiAndAlpha)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    stream.try_into().map_err(|e| e.into())?.abi(),
                    dpi,
                    alpha,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn GetBounds<P0, E0>(
        &self,
        resourcecreator: P0,
    ) -> ::windows::core::Result<::windows::Foundation::Rect>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<ICanvasImage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetBounds)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn GetBoundsWithTransform<P0, E0>(
        &self,
        resourcecreator: P0,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::core::Result<::windows::Foundation::Rect>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<ICanvasImage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetBoundsWithTransform)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    transform,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn Device(&self) -> ::windows::core::Result<CanvasDevice> {
        let this = &::windows::core::Interface::cast::<ICanvasResourceCreator>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Device)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn Dpi(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<
            ICanvasResourceCreatorWithDpi,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Dpi)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn ConvertPixelsToDips(&self, pixels: i32) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<
            ICanvasResourceCreatorWithDpi,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ConvertPixelsToDips)(
                    ::windows::core::Vtable::as_raw(this),
                    pixels,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn ConvertDipsToPixels(
        &self,
        dips: f32,
        dpirounding: CanvasDpiRounding,
    ) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<
            ICanvasResourceCreatorWithDpi,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ConvertDipsToPixels)(
                    ::windows::core::Vtable::as_raw(this),
                    dips,
                    dpirounding,
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
    ///*Required features: `"UI_Xaml"`*
    #[cfg(feature = "UI_Xaml")]
    pub fn GetValue(
        &self,
        dp: &::windows::UI::Xaml::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<
            ::windows::UI::Xaml::IDependencyObject,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetValue)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(dp),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"UI_Xaml"`*
    #[cfg(feature = "UI_Xaml")]
    pub fn SetValue<P0>(
        &self,
        dp: &::windows::UI::Xaml::DependencyProperty,
        value: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<::windows::core::IInspectable>,
        >,
    {
        let this = &::windows::core::Interface::cast::<
            ::windows::UI::Xaml::IDependencyObject,
        >(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetValue)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(dp),
                    value.into().abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"UI_Xaml"`*
    #[cfg(feature = "UI_Xaml")]
    pub fn ClearValue(
        &self,
        dp: &::windows::UI::Xaml::DependencyProperty,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<
            ::windows::UI::Xaml::IDependencyObject,
        >(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .ClearValue)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(dp),
                )
                .ok()
        }
    }
    ///*Required features: `"UI_Xaml"`*
    #[cfg(feature = "UI_Xaml")]
    pub fn ReadLocalValue(
        &self,
        dp: &::windows::UI::Xaml::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<
            ::windows::UI::Xaml::IDependencyObject,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ReadLocalValue)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(dp),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"UI_Xaml"`*
    #[cfg(feature = "UI_Xaml")]
    pub fn GetAnimationBaseValue(
        &self,
        dp: &::windows::UI::Xaml::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<
            ::windows::UI::Xaml::IDependencyObject,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetAnimationBaseValue)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(dp),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"UI_Core"`, `"UI_Xaml"`*
    #[cfg(all(feature = "UI_Core", feature = "UI_Xaml"))]
    pub fn Dispatcher(
        &self,
    ) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<
            ::windows::UI::Xaml::IDependencyObject,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Dispatcher)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"UI_Xaml"`*
    #[cfg(feature = "UI_Xaml")]
    pub fn RegisterPropertyChangedCallback(
        &self,
        dp: &::windows::UI::Xaml::DependencyProperty,
        callback: &::windows::UI::Xaml::DependencyPropertyChangedCallback,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<
            ::windows::UI::Xaml::IDependencyObject2,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .RegisterPropertyChangedCallback)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(dp),
                    ::core::mem::transmute_copy(callback),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"UI_Xaml"`*
    #[cfg(feature = "UI_Xaml")]
    pub fn UnregisterPropertyChangedCallback(
        &self,
        dp: &::windows::UI::Xaml::DependencyProperty,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<
            ::windows::UI::Xaml::IDependencyObject2,
        >(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .UnregisterPropertyChangedCallback)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(dp),
                    token,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_DirectX_Direct3D11"`*
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Description(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Graphics::DirectX::Direct3D11::Direct3DSurfaceDescription,
    > {
        let this = &::windows::core::Interface::cast::<
            ::windows::Graphics::DirectX::Direct3D11::IDirect3DSurface,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Description)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn ICanvasBitmapStatics<
        R,
        F: FnOnce(&ICanvasBitmapStatics) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasBitmap,
            ICanvasBitmapStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CanvasBitmap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasBitmap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasBitmap {}
impl ::core::fmt::Debug for CanvasBitmap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasBitmap").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasBitmap {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.CanvasBitmap;{c57532ed-709e-4ac2-86be-a1ec3a7fa8fe})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasBitmap {
    type Vtable = ICanvasBitmap_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasBitmap {
    const IID: ::windows::core::GUID = <ICanvasBitmap as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasBitmap {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.CanvasBitmap";
}
::windows::core::interface_hierarchy!(
    CanvasBitmap,::windows::core::IUnknown,::windows::core::IInspectable
);
impl ::core::convert::TryFrom<CanvasBitmap> for ICanvasImage {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasBitmap) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CanvasBitmap> for ICanvasImage {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasBitmap) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&CanvasBitmap> for ::windows::core::InParam<ICanvasImage> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasBitmap) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<CanvasBitmap> for ICanvasResourceCreator {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasBitmap) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CanvasBitmap> for ICanvasResourceCreator {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasBitmap) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&CanvasBitmap>
for ::windows::core::InParam<ICanvasResourceCreator> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasBitmap) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<CanvasBitmap> for ICanvasResourceCreatorWithDpi {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasBitmap) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CanvasBitmap> for ICanvasResourceCreatorWithDpi {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasBitmap) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&CanvasBitmap>
for ::windows::core::InParam<ICanvasResourceCreatorWithDpi> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasBitmap) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasBitmap> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasBitmap) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasBitmap> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasBitmap) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasBitmap>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasBitmap) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Graphics_DirectX_Direct3D11")]
impl ::core::convert::TryFrom<CanvasBitmap>
for ::windows::Graphics::DirectX::Direct3D11::IDirect3DSurface {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasBitmap) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Graphics_DirectX_Direct3D11")]
impl ::core::convert::TryFrom<&CanvasBitmap>
for ::windows::Graphics::DirectX::Direct3D11::IDirect3DSurface {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasBitmap) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Graphics_DirectX_Direct3D11")]
impl ::core::convert::TryFrom<&CanvasBitmap>
for ::windows::core::InParam<
    ::windows::Graphics::DirectX::Direct3D11::IDirect3DSurface,
> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasBitmap) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Graphics_Effects")]
impl ::core::convert::TryFrom<CanvasBitmap>
for ::windows::Graphics::Effects::IGraphicsEffectSource {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasBitmap) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Graphics_Effects")]
impl ::core::convert::TryFrom<&CanvasBitmap>
for ::windows::Graphics::Effects::IGraphicsEffectSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasBitmap) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Graphics_Effects")]
impl ::core::convert::TryFrom<&CanvasBitmap>
for ::windows::core::InParam<::windows::Graphics::Effects::IGraphicsEffectSource> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasBitmap) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "UI_Xaml")]
impl ::core::convert::From<CanvasBitmap> for ::windows::UI::Xaml::DependencyObject {
    fn from(value: CanvasBitmap) -> Self {
        ::core::convert::From::from(&value)
    }
}
#[cfg(feature = "UI_Xaml")]
impl ::core::convert::From<&CanvasBitmap> for ::windows::UI::Xaml::DependencyObject {
    fn from(value: &CanvasBitmap) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
#[cfg(feature = "UI_Xaml")]
impl ::core::convert::From<&CanvasBitmap>
for ::windows::core::InParam<::windows::UI::Xaml::DependencyObject> {
    fn from(value: &CanvasBitmap) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for CanvasBitmap {}
unsafe impl ::core::marker::Sync for CanvasBitmap {}
///*Required features: `"Graphics_Canvas"`*
#[repr(transparent)]
pub struct CanvasCommandList(::windows::core::IUnknown);
impl CanvasCommandList {
    pub fn CreateDrawingSession(&self) -> ::windows::core::Result<CanvasDrawingSession> {
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
    pub fn Device(&self) -> ::windows::core::Result<CanvasDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Device)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn Create<P0, E0>(
        resourcecreator: P0,
    ) -> ::windows::core::Result<CanvasCommandList>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasCommandListFactory(|this| unsafe {
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
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn GetBounds<P0, E0>(
        &self,
        resourcecreator: P0,
    ) -> ::windows::core::Result<::windows::Foundation::Rect>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<ICanvasImage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetBounds)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn GetBoundsWithTransform<P0, E0>(
        &self,
        resourcecreator: P0,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::core::Result<::windows::Foundation::Rect>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<ICanvasImage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetBoundsWithTransform)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    transform,
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
    #[doc(hidden)]
    pub fn ICanvasCommandListFactory<
        R,
        F: FnOnce(&ICanvasCommandListFactory) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasCommandList,
            ICanvasCommandListFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CanvasCommandList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasCommandList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasCommandList {}
impl ::core::fmt::Debug for CanvasCommandList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasCommandList").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasCommandList {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.CanvasCommandList;{b71e73cf-2fe7-4d3a-bbb8-19f016f5be1b})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasCommandList {
    type Vtable = ICanvasCommandList_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasCommandList {
    const IID: ::windows::core::GUID = <ICanvasCommandList as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasCommandList {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.CanvasCommandList";
}
::windows::core::interface_hierarchy!(
    CanvasCommandList,::windows::core::IUnknown,::windows::core::IInspectable
);
impl ::core::convert::TryFrom<CanvasCommandList> for ICanvasImage {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasCommandList) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CanvasCommandList> for ICanvasImage {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasCommandList) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&CanvasCommandList>
for ::windows::core::InParam<ICanvasImage> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasCommandList) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasCommandList> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasCommandList) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasCommandList> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasCommandList) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasCommandList>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasCommandList) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Graphics_Effects")]
impl ::core::convert::TryFrom<CanvasCommandList>
for ::windows::Graphics::Effects::IGraphicsEffectSource {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasCommandList) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Graphics_Effects")]
impl ::core::convert::TryFrom<&CanvasCommandList>
for ::windows::Graphics::Effects::IGraphicsEffectSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasCommandList) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Graphics_Effects")]
impl ::core::convert::TryFrom<&CanvasCommandList>
for ::windows::core::InParam<::windows::Graphics::Effects::IGraphicsEffectSource> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasCommandList) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CanvasCommandList {}
unsafe impl ::core::marker::Sync for CanvasCommandList {}
///*Required features: `"Graphics_Canvas"`*
#[repr(transparent)]
pub struct CanvasDevice(::windows::core::IUnknown);
impl CanvasDevice {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasDevice,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ForceSoftwareRenderer(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ForceSoftwareRenderer)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn MaximumBitmapSizeInPixels(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .MaximumBitmapSizeInPixels)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Graphics_DirectX"`*
    #[cfg(feature = "Graphics_DirectX")]
    pub fn IsPixelFormatSupported(
        &self,
        pixelformat: ::windows::Graphics::DirectX::DirectXPixelFormat,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .IsPixelFormatSupported)(
                    ::windows::core::Vtable::as_raw(this),
                    pixelformat,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn IsBufferPrecisionSupported(
        &self,
        bufferprecision: CanvasBufferPrecision,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .IsBufferPrecisionSupported)(
                    ::windows::core::Vtable::as_raw(this),
                    bufferprecision,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn MaximumCacheSize(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .MaximumCacheSize)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetMaximumCacheSize(&self, value: u64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetMaximumCacheSize)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn LowPriority(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LowPriority)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetLowPriority(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetLowPriority)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn DeviceLost(
        &self,
        value: &::windows::Foundation::TypedEventHandler::<
            CanvasDevice,
            ::windows::core::IInspectable,
        >,
    ) -> ::windows::core::Result<::windows::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .DeviceLost)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(value),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn RemoveDeviceLost(
        &self,
        token: ::windows::Foundation::EventRegistrationToken,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .RemoveDeviceLost)(::windows::core::Vtable::as_raw(this), token)
                .ok()
        }
    }
    pub fn IsDeviceLost(&self, hresult: i32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .IsDeviceLost)(
                    ::windows::core::Vtable::as_raw(this),
                    hresult,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn RaiseDeviceLost(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .RaiseDeviceLost)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn Lock(&self) -> ::windows::core::Result<CanvasLock> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Lock)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn CreateWithForceSoftwareRendererOption(
        forcesoftwarerenderer: bool,
    ) -> ::windows::core::Result<CanvasDevice> {
        Self::ICanvasDeviceFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateWithForceSoftwareRendererOption)(
                    ::windows::core::Vtable::as_raw(this),
                    forcesoftwarerenderer,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Graphics_DirectX_Direct3D11"`*
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CreateFromDirect3D11Device<P0, E0>(
        direct3ddevice: P0,
    ) -> ::windows::core::Result<CanvasDevice>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<
                ::windows::Graphics::DirectX::Direct3D11::IDirect3DDevice,
            >,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateFromDirect3D11Device)(
                    ::windows::core::Vtable::as_raw(this),
                    direct3ddevice.try_into().map_err(|e| e.into())?.abi(),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    pub fn GetSharedDevice() -> ::windows::core::Result<CanvasDevice> {
        Self::ICanvasDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetSharedDevice)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    pub fn GetSharedDeviceWithForceSoftwareRenderer(
        forcesoftwarerenderer: bool,
    ) -> ::windows::core::Result<CanvasDevice> {
        Self::ICanvasDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetSharedDeviceWithForceSoftwareRenderer)(
                    ::windows::core::Vtable::as_raw(this),
                    forcesoftwarerenderer,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    pub fn SetDebugLevel(value: CanvasDebugLevel) -> ::windows::core::Result<()> {
        Self::ICanvasDeviceStatics(|this| unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetDebugLevel)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        })
    }
    pub fn DebugLevel() -> ::windows::core::Result<CanvasDebugLevel> {
        Self::ICanvasDeviceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .DebugLevel)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    pub fn Device(&self) -> ::windows::core::Result<CanvasDevice> {
        let this = &::windows::core::Interface::cast::<ICanvasResourceCreator>(self)?;
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
    ///*Required features: `"Graphics_DirectX_Direct3D11"`*
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Trim(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<
            ::windows::Graphics::DirectX::Direct3D11::IDirect3DDevice,
        >(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .Trim)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    #[doc(hidden)]
    pub fn ICanvasDeviceFactory<
        R,
        F: FnOnce(&ICanvasDeviceFactory) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasDevice,
            ICanvasDeviceFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICanvasDeviceStatics<
        R,
        F: FnOnce(&ICanvasDeviceStatics) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasDevice,
            ICanvasDeviceStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CanvasDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasDevice {}
impl ::core::fmt::Debug for CanvasDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.CanvasDevice;{a27f0b5d-ec2c-4d4f-948f-0aa1e95e33e6})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasDevice {
    type Vtable = ICanvasDevice_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasDevice {
    const IID: ::windows::core::GUID = <ICanvasDevice as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasDevice {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.CanvasDevice";
}
::windows::core::interface_hierarchy!(
    CanvasDevice,::windows::core::IUnknown,::windows::core::IInspectable
);
impl ::core::convert::TryFrom<CanvasDevice> for ICanvasResourceCreator {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasDevice) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CanvasDevice> for ICanvasResourceCreator {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasDevice) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&CanvasDevice>
for ::windows::core::InParam<ICanvasResourceCreator> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasDevice) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasDevice> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasDevice) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasDevice> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasDevice) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasDevice>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasDevice) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Graphics_DirectX_Direct3D11")]
impl ::core::convert::TryFrom<CanvasDevice>
for ::windows::Graphics::DirectX::Direct3D11::IDirect3DDevice {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasDevice) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Graphics_DirectX_Direct3D11")]
impl ::core::convert::TryFrom<&CanvasDevice>
for ::windows::Graphics::DirectX::Direct3D11::IDirect3DDevice {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasDevice) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Graphics_DirectX_Direct3D11")]
impl ::core::convert::TryFrom<&CanvasDevice>
for ::windows::core::InParam<::windows::Graphics::DirectX::Direct3D11::IDirect3DDevice> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasDevice) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CanvasDevice {}
unsafe impl ::core::marker::Sync for CanvasDevice {}
///*Required features: `"Graphics_Canvas"`*
#[repr(transparent)]
pub struct CanvasDrawingSession(::windows::core::IUnknown);
impl CanvasDrawingSession {
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn Clear(&self, color: ::windows::UI::Color) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .Clear)(::windows::core::Vtable::as_raw(this), color)
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn ClearHdr(
        &self,
        color: ::windows::Foundation::Numerics::Vector4,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .ClearHdr)(::windows::core::Vtable::as_raw(this), color)
                .ok()
        }
    }
    pub fn Flush(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .Flush)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn DrawImageAtOrigin<P0, E0>(&self, image: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<ICanvasImage>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawImageAtOrigin)(
                    ::windows::core::Vtable::as_raw(this),
                    image.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DrawImageAtOffset<P0, E0>(
        &self,
        image: P0,
        offset: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<ICanvasImage>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawImageAtOffset)(
                    ::windows::core::Vtable::as_raw(this),
                    image.try_into().map_err(|e| e.into())?.abi(),
                    offset,
                )
                .ok()
        }
    }
    pub fn DrawImageAtCoords<P0, E0>(
        &self,
        image: P0,
        x: f32,
        y: f32,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<ICanvasImage>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawImageAtCoords)(
                    ::windows::core::Vtable::as_raw(this),
                    image.try_into().map_err(|e| e.into())?.abi(),
                    x,
                    y,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn DrawImageToRect<P0>(
        &self,
        bitmap: P0,
        destinationrectangle: ::windows::Foundation::Rect,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<CanvasBitmap>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawImageToRect)(
                    ::windows::core::Vtable::as_raw(this),
                    bitmap.into().abi(),
                    destinationrectangle,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DrawImageAtOffsetWithSourceRect<P0, E0>(
        &self,
        image: P0,
        offset: ::windows::Foundation::Numerics::Vector2,
        sourcerectangle: ::windows::Foundation::Rect,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<ICanvasImage>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawImageAtOffsetWithSourceRect)(
                    ::windows::core::Vtable::as_raw(this),
                    image.try_into().map_err(|e| e.into())?.abi(),
                    offset,
                    sourcerectangle,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn DrawImageAtCoordsWithSourceRect<P0, E0>(
        &self,
        image: P0,
        x: f32,
        y: f32,
        sourcerectangle: ::windows::Foundation::Rect,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<ICanvasImage>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawImageAtCoordsWithSourceRect)(
                    ::windows::core::Vtable::as_raw(this),
                    image.try_into().map_err(|e| e.into())?.abi(),
                    x,
                    y,
                    sourcerectangle,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn DrawImageToRectWithSourceRect<P0, E0>(
        &self,
        image: P0,
        destinationrectangle: ::windows::Foundation::Rect,
        sourcerectangle: ::windows::Foundation::Rect,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<ICanvasImage>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawImageToRectWithSourceRect)(
                    ::windows::core::Vtable::as_raw(this),
                    image.try_into().map_err(|e| e.into())?.abi(),
                    destinationrectangle,
                    sourcerectangle,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DrawImageAtOffsetWithSourceRectAndOpacity<P0, E0>(
        &self,
        image: P0,
        offset: ::windows::Foundation::Numerics::Vector2,
        sourcerectangle: ::windows::Foundation::Rect,
        opacity: f32,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<ICanvasImage>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawImageAtOffsetWithSourceRectAndOpacity)(
                    ::windows::core::Vtable::as_raw(this),
                    image.try_into().map_err(|e| e.into())?.abi(),
                    offset,
                    sourcerectangle,
                    opacity,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn DrawImageAtCoordsWithSourceRectAndOpacity<P0, E0>(
        &self,
        image: P0,
        x: f32,
        y: f32,
        sourcerectangle: ::windows::Foundation::Rect,
        opacity: f32,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<ICanvasImage>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawImageAtCoordsWithSourceRectAndOpacity)(
                    ::windows::core::Vtable::as_raw(this),
                    image.try_into().map_err(|e| e.into())?.abi(),
                    x,
                    y,
                    sourcerectangle,
                    opacity,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn DrawImageToRectWithSourceRectAndOpacity<P0, E0>(
        &self,
        image: P0,
        destinationrectangle: ::windows::Foundation::Rect,
        sourcerectangle: ::windows::Foundation::Rect,
        opacity: f32,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<ICanvasImage>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawImageToRectWithSourceRectAndOpacity)(
                    ::windows::core::Vtable::as_raw(this),
                    image.try_into().map_err(|e| e.into())?.abi(),
                    destinationrectangle,
                    sourcerectangle,
                    opacity,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DrawImageAtOffsetWithSourceRectAndOpacityAndInterpolation<P0, E0>(
        &self,
        image: P0,
        offset: ::windows::Foundation::Numerics::Vector2,
        sourcerectangle: ::windows::Foundation::Rect,
        opacity: f32,
        interpolation: CanvasImageInterpolation,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<ICanvasImage>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawImageAtOffsetWithSourceRectAndOpacityAndInterpolation)(
                    ::windows::core::Vtable::as_raw(this),
                    image.try_into().map_err(|e| e.into())?.abi(),
                    offset,
                    sourcerectangle,
                    opacity,
                    interpolation,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn DrawImageAtCoordsWithSourceRectAndOpacityAndInterpolation<P0, E0>(
        &self,
        image: P0,
        x: f32,
        y: f32,
        sourcerectangle: ::windows::Foundation::Rect,
        opacity: f32,
        interpolation: CanvasImageInterpolation,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<ICanvasImage>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawImageAtCoordsWithSourceRectAndOpacityAndInterpolation)(
                    ::windows::core::Vtable::as_raw(this),
                    image.try_into().map_err(|e| e.into())?.abi(),
                    x,
                    y,
                    sourcerectangle,
                    opacity,
                    interpolation,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn DrawImageToRectWithSourceRectAndOpacityAndInterpolation<P0, E0>(
        &self,
        image: P0,
        destinationrectangle: ::windows::Foundation::Rect,
        sourcerectangle: ::windows::Foundation::Rect,
        opacity: f32,
        interpolation: CanvasImageInterpolation,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<ICanvasImage>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawImageToRectWithSourceRectAndOpacityAndInterpolation)(
                    ::windows::core::Vtable::as_raw(this),
                    image.try_into().map_err(|e| e.into())?.abi(),
                    destinationrectangle,
                    sourcerectangle,
                    opacity,
                    interpolation,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DrawImageAtOffsetWithSourceRectAndOpacityAndInterpolationAndComposite<P0, E0>(
        &self,
        image: P0,
        offset: ::windows::Foundation::Numerics::Vector2,
        sourcerectangle: ::windows::Foundation::Rect,
        opacity: f32,
        interpolation: CanvasImageInterpolation,
        composite: CanvasComposite,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<ICanvasImage>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawImageAtOffsetWithSourceRectAndOpacityAndInterpolationAndComposite)(
                    ::windows::core::Vtable::as_raw(this),
                    image.try_into().map_err(|e| e.into())?.abi(),
                    offset,
                    sourcerectangle,
                    opacity,
                    interpolation,
                    composite,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn DrawImageAtCoordsWithSourceRectAndOpacityAndInterpolationAndComposite<P0, E0>(
        &self,
        image: P0,
        x: f32,
        y: f32,
        sourcerectangle: ::windows::Foundation::Rect,
        opacity: f32,
        interpolation: CanvasImageInterpolation,
        composite: CanvasComposite,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<ICanvasImage>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawImageAtCoordsWithSourceRectAndOpacityAndInterpolationAndComposite)(
                    ::windows::core::Vtable::as_raw(this),
                    image.try_into().map_err(|e| e.into())?.abi(),
                    x,
                    y,
                    sourcerectangle,
                    opacity,
                    interpolation,
                    composite,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn DrawImageToRectWithSourceRectAndOpacityAndInterpolationAndComposite<P0, E0>(
        &self,
        image: P0,
        destinationrectangle: ::windows::Foundation::Rect,
        sourcerectangle: ::windows::Foundation::Rect,
        opacity: f32,
        interpolation: CanvasImageInterpolation,
        composite: CanvasComposite,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<ICanvasImage>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawImageToRectWithSourceRectAndOpacityAndInterpolationAndComposite)(
                    ::windows::core::Vtable::as_raw(this),
                    image.try_into().map_err(|e| e.into())?.abi(),
                    destinationrectangle,
                    sourcerectangle,
                    opacity,
                    interpolation,
                    composite,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DrawImageAtOffsetWithSourceRectAndOpacityAndInterpolationAndPerspective<P0>(
        &self,
        bitmap: P0,
        offset: ::windows::Foundation::Numerics::Vector2,
        sourcerectangle: ::windows::Foundation::Rect,
        opacity: f32,
        interpolation: CanvasImageInterpolation,
        perspective: ::windows::Foundation::Numerics::Matrix4x4,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<CanvasBitmap>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawImageAtOffsetWithSourceRectAndOpacityAndInterpolationAndPerspective)(
                    ::windows::core::Vtable::as_raw(this),
                    bitmap.into().abi(),
                    offset,
                    sourcerectangle,
                    opacity,
                    interpolation,
                    perspective,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DrawImageAtCoordsWithSourceRectAndOpacityAndInterpolationAndPerspective<P0>(
        &self,
        bitmap: P0,
        x: f32,
        y: f32,
        sourcerectangle: ::windows::Foundation::Rect,
        opacity: f32,
        interpolation: CanvasImageInterpolation,
        perspective: ::windows::Foundation::Numerics::Matrix4x4,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<CanvasBitmap>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawImageAtCoordsWithSourceRectAndOpacityAndInterpolationAndPerspective)(
                    ::windows::core::Vtable::as_raw(this),
                    bitmap.into().abi(),
                    x,
                    y,
                    sourcerectangle,
                    opacity,
                    interpolation,
                    perspective,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DrawImageToRectWithSourceRectAndOpacityAndInterpolationAndPerspective<P0>(
        &self,
        bitmap: P0,
        destinationrectangle: ::windows::Foundation::Rect,
        sourcerectangle: ::windows::Foundation::Rect,
        opacity: f32,
        interpolation: CanvasImageInterpolation,
        perspective: ::windows::Foundation::Numerics::Matrix4x4,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<CanvasBitmap>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawImageToRectWithSourceRectAndOpacityAndInterpolationAndPerspective)(
                    ::windows::core::Vtable::as_raw(this),
                    bitmap.into().abi(),
                    destinationrectangle,
                    sourcerectangle,
                    opacity,
                    interpolation,
                    perspective,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Foundation_Numerics"`*
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation_Numerics"))]
    pub fn DrawLineWithBrush<P0, E0>(
        &self,
        point0: ::windows::Foundation::Numerics::Vector2,
        point1: ::windows::Foundation::Numerics::Vector2,
        brush: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawLineWithBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    point0,
                    point1,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`*
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub fn DrawLineAtCoordsWithBrush<P0, E0>(
        &self,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        brush: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawLineAtCoordsWithBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    x0,
                    y0,
                    x1,
                    y1,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`, `"UI"`*
    #[cfg(all(feature = "Foundation_Numerics", feature = "UI"))]
    pub fn DrawLineWithColor(
        &self,
        point0: ::windows::Foundation::Numerics::Vector2,
        point1: ::windows::Foundation::Numerics::Vector2,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawLineWithColor)(
                    ::windows::core::Vtable::as_raw(this),
                    point0,
                    point1,
                    color,
                )
                .ok()
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn DrawLineAtCoordsWithColor(
        &self,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawLineAtCoordsWithColor)(
                    ::windows::core::Vtable::as_raw(this),
                    x0,
                    y0,
                    x1,
                    y1,
                    color,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Foundation_Numerics"`*
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation_Numerics"))]
    pub fn DrawLineWithBrushAndStrokeWidth<P0, E0>(
        &self,
        point0: ::windows::Foundation::Numerics::Vector2,
        point1: ::windows::Foundation::Numerics::Vector2,
        brush: P0,
        strokewidth: f32,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawLineWithBrushAndStrokeWidth)(
                    ::windows::core::Vtable::as_raw(this),
                    point0,
                    point1,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    strokewidth,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`*
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub fn DrawLineAtCoordsWithBrushAndStrokeWidth<P0, E0>(
        &self,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        brush: P0,
        strokewidth: f32,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawLineAtCoordsWithBrushAndStrokeWidth)(
                    ::windows::core::Vtable::as_raw(this),
                    x0,
                    y0,
                    x1,
                    y1,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    strokewidth,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`, `"UI"`*
    #[cfg(all(feature = "Foundation_Numerics", feature = "UI"))]
    pub fn DrawLineWithColorAndStrokeWidth(
        &self,
        point0: ::windows::Foundation::Numerics::Vector2,
        point1: ::windows::Foundation::Numerics::Vector2,
        color: ::windows::UI::Color,
        strokewidth: f32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawLineWithColorAndStrokeWidth)(
                    ::windows::core::Vtable::as_raw(this),
                    point0,
                    point1,
                    color,
                    strokewidth,
                )
                .ok()
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn DrawLineAtCoordsWithColorAndStrokeWidth(
        &self,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawLineAtCoordsWithColorAndStrokeWidth)(
                    ::windows::core::Vtable::as_raw(this),
                    x0,
                    y0,
                    x1,
                    y1,
                    color,
                    strokewidth,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Geometry"`, `"Foundation_Numerics"`*
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics"
        )
    )]
    pub fn DrawLineWithBrushAndStrokeWidthAndStrokeStyle<P0, E0>(
        &self,
        point0: ::windows::Foundation::Numerics::Vector2,
        point1: ::windows::Foundation::Numerics::Vector2,
        brush: P0,
        strokewidth: f32,
        strokestyle: &Geometry::CanvasStrokeStyle,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawLineWithBrushAndStrokeWidthAndStrokeStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    point0,
                    point1,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Geometry"`*
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub fn DrawLineAtCoordsWithBrushAndStrokeWidthAndStrokeStyle<P0, E0>(
        &self,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        brush: P0,
        strokewidth: f32,
        strokestyle: &Geometry::CanvasStrokeStyle,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawLineAtCoordsWithBrushAndStrokeWidthAndStrokeStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    x0,
                    y0,
                    x1,
                    y1,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`, `"Foundation_Numerics"`, `"UI"`*
    #[cfg(
        all(
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics",
            feature = "UI"
        )
    )]
    pub fn DrawLineWithColorAndStrokeWidthAndStrokeStyle(
        &self,
        point0: ::windows::Foundation::Numerics::Vector2,
        point1: ::windows::Foundation::Numerics::Vector2,
        color: ::windows::UI::Color,
        strokewidth: f32,
        strokestyle: &Geometry::CanvasStrokeStyle,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawLineWithColorAndStrokeWidthAndStrokeStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    point0,
                    point1,
                    color,
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`, `"UI"`*
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "UI"))]
    pub fn DrawLineAtCoordsWithColorAndStrokeWidthAndStrokeStyle(
        &self,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
        strokestyle: &Geometry::CanvasStrokeStyle,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawLineAtCoordsWithColorAndStrokeWidthAndStrokeStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    x0,
                    y0,
                    x1,
                    y1,
                    color,
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Foundation"`*
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation"))]
    pub fn DrawRectangleWithBrush<P0, E0>(
        &self,
        rect: ::windows::Foundation::Rect,
        brush: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawRectangleWithBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    rect,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`*
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub fn DrawRectangleAtCoordsWithBrush<P0, E0>(
        &self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        brush: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawRectangleAtCoordsWithBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    w,
                    h,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation"`, `"UI"`*
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub fn DrawRectangleWithColor(
        &self,
        rect: ::windows::Foundation::Rect,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawRectangleWithColor)(
                    ::windows::core::Vtable::as_raw(this),
                    rect,
                    color,
                )
                .ok()
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn DrawRectangleAtCoordsWithColor(
        &self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawRectangleAtCoordsWithColor)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    w,
                    h,
                    color,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Foundation"`*
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation"))]
    pub fn DrawRectangleWithBrushAndStrokeWidth<P0, E0>(
        &self,
        rect: ::windows::Foundation::Rect,
        brush: P0,
        strokewidth: f32,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawRectangleWithBrushAndStrokeWidth)(
                    ::windows::core::Vtable::as_raw(this),
                    rect,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    strokewidth,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`*
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub fn DrawRectangleAtCoordsWithBrushAndStrokeWidth<P0, E0>(
        &self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        brush: P0,
        strokewidth: f32,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawRectangleAtCoordsWithBrushAndStrokeWidth)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    w,
                    h,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    strokewidth,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation"`, `"UI"`*
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub fn DrawRectangleWithColorAndStrokeWidth(
        &self,
        rect: ::windows::Foundation::Rect,
        color: ::windows::UI::Color,
        strokewidth: f32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawRectangleWithColorAndStrokeWidth)(
                    ::windows::core::Vtable::as_raw(this),
                    rect,
                    color,
                    strokewidth,
                )
                .ok()
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn DrawRectangleAtCoordsWithColorAndStrokeWidth(
        &self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawRectangleAtCoordsWithColorAndStrokeWidth)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    w,
                    h,
                    color,
                    strokewidth,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Geometry"`, `"Foundation"`*
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation"
        )
    )]
    pub fn DrawRectangleWithBrushAndStrokeWidthAndStrokeStyle<P0, E0>(
        &self,
        rect: ::windows::Foundation::Rect,
        brush: P0,
        strokewidth: f32,
        strokestyle: &Geometry::CanvasStrokeStyle,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawRectangleWithBrushAndStrokeWidthAndStrokeStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    rect,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Geometry"`*
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub fn DrawRectangleAtCoordsWithBrushAndStrokeWidthAndStrokeStyle<P0, E0>(
        &self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        brush: P0,
        strokewidth: f32,
        strokestyle: &Geometry::CanvasStrokeStyle,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawRectangleAtCoordsWithBrushAndStrokeWidthAndStrokeStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    w,
                    h,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`, `"Foundation"`, `"UI"`*
    #[cfg(
        all(feature = "Graphics_Canvas_Geometry", feature = "Foundation", feature = "UI")
    )]
    pub fn DrawRectangleWithColorAndStrokeWidthAndStrokeStyle(
        &self,
        rect: ::windows::Foundation::Rect,
        color: ::windows::UI::Color,
        strokewidth: f32,
        strokestyle: &Geometry::CanvasStrokeStyle,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawRectangleWithColorAndStrokeWidthAndStrokeStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    rect,
                    color,
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`, `"UI"`*
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "UI"))]
    pub fn DrawRectangleAtCoordsWithColorAndStrokeWidthAndStrokeStyle(
        &self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
        strokestyle: &Geometry::CanvasStrokeStyle,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawRectangleAtCoordsWithColorAndStrokeWidthAndStrokeStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    w,
                    h,
                    color,
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Foundation"`*
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation"))]
    pub fn FillRectangleWithBrush<P0, E0>(
        &self,
        rect: ::windows::Foundation::Rect,
        brush: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .FillRectangleWithBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    rect,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`*
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub fn FillRectangleAtCoordsWithBrush<P0, E0>(
        &self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        brush: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .FillRectangleAtCoordsWithBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    w,
                    h,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation"`, `"UI"`*
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub fn FillRectangleWithColor(
        &self,
        rect: ::windows::Foundation::Rect,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .FillRectangleWithColor)(
                    ::windows::core::Vtable::as_raw(this),
                    rect,
                    color,
                )
                .ok()
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn FillRectangleAtCoordsWithColor(
        &self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .FillRectangleAtCoordsWithColor)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    w,
                    h,
                    color,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Foundation"`*
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation"))]
    pub fn FillRectangleWithBrushAndOpacityBrush<P0, E0, P1, E1>(
        &self,
        rect: ::windows::Foundation::Rect,
        brush: P0,
        opacitybrush: P1,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .FillRectangleWithBrushAndOpacityBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    rect,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    opacitybrush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`*
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub fn FillRectangleAtCoordsWithBrushAndOpacityBrush<P0, E0, P1, E1>(
        &self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        brush: P0,
        opacitybrush: P1,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .FillRectangleAtCoordsWithBrushAndOpacityBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    w,
                    h,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    opacitybrush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Foundation"`*
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation"))]
    pub fn DrawRoundedRectangleWithBrush<P0, E0>(
        &self,
        rect: ::windows::Foundation::Rect,
        radiusx: f32,
        radiusy: f32,
        brush: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawRoundedRectangleWithBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    rect,
                    radiusx,
                    radiusy,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`*
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub fn DrawRoundedRectangleAtCoordsWithBrush<P0, E0>(
        &self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radiusx: f32,
        radiusy: f32,
        brush: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawRoundedRectangleAtCoordsWithBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    w,
                    h,
                    radiusx,
                    radiusy,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation"`, `"UI"`*
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub fn DrawRoundedRectangleWithColor(
        &self,
        rect: ::windows::Foundation::Rect,
        radiusx: f32,
        radiusy: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawRoundedRectangleWithColor)(
                    ::windows::core::Vtable::as_raw(this),
                    rect,
                    radiusx,
                    radiusy,
                    color,
                )
                .ok()
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn DrawRoundedRectangleAtCoordsWithColor(
        &self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radiusx: f32,
        radiusy: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawRoundedRectangleAtCoordsWithColor)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    w,
                    h,
                    radiusx,
                    radiusy,
                    color,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Foundation"`*
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation"))]
    pub fn DrawRoundedRectangleWithBrushAndStrokeWidth<P0, E0>(
        &self,
        rect: ::windows::Foundation::Rect,
        radiusx: f32,
        radiusy: f32,
        brush: P0,
        strokewidth: f32,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawRoundedRectangleWithBrushAndStrokeWidth)(
                    ::windows::core::Vtable::as_raw(this),
                    rect,
                    radiusx,
                    radiusy,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    strokewidth,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`*
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub fn DrawRoundedRectangleAtCoordsWithBrushAndStrokeWidth<P0, E0>(
        &self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radiusx: f32,
        radiusy: f32,
        brush: P0,
        strokewidth: f32,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawRoundedRectangleAtCoordsWithBrushAndStrokeWidth)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    w,
                    h,
                    radiusx,
                    radiusy,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    strokewidth,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation"`, `"UI"`*
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub fn DrawRoundedRectangleWithColorAndStrokeWidth(
        &self,
        rect: ::windows::Foundation::Rect,
        radiusx: f32,
        radiusy: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawRoundedRectangleWithColorAndStrokeWidth)(
                    ::windows::core::Vtable::as_raw(this),
                    rect,
                    radiusx,
                    radiusy,
                    color,
                    strokewidth,
                )
                .ok()
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn DrawRoundedRectangleAtCoordsWithColorAndStrokeWidth(
        &self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radiusx: f32,
        radiusy: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawRoundedRectangleAtCoordsWithColorAndStrokeWidth)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    w,
                    h,
                    radiusx,
                    radiusy,
                    color,
                    strokewidth,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Geometry"`, `"Foundation"`*
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation"
        )
    )]
    pub fn DrawRoundedRectangleWithBrushAndStrokeWidthAndStrokeStyle<P0, E0>(
        &self,
        rect: ::windows::Foundation::Rect,
        radiusx: f32,
        radiusy: f32,
        brush: P0,
        strokewidth: f32,
        strokestyle: &Geometry::CanvasStrokeStyle,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawRoundedRectangleWithBrushAndStrokeWidthAndStrokeStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    rect,
                    radiusx,
                    radiusy,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Geometry"`*
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub fn DrawRoundedRectangleAtCoordsWithBrushAndStrokeWidthAndStrokeStyle<P0, E0>(
        &self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radiusx: f32,
        radiusy: f32,
        brush: P0,
        strokewidth: f32,
        strokestyle: &Geometry::CanvasStrokeStyle,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawRoundedRectangleAtCoordsWithBrushAndStrokeWidthAndStrokeStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    w,
                    h,
                    radiusx,
                    radiusy,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`, `"Foundation"`, `"UI"`*
    #[cfg(
        all(feature = "Graphics_Canvas_Geometry", feature = "Foundation", feature = "UI")
    )]
    pub fn DrawRoundedRectangleWithColorAndStrokeWidthAndStrokeStyle(
        &self,
        rect: ::windows::Foundation::Rect,
        radiusx: f32,
        radiusy: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
        strokestyle: &Geometry::CanvasStrokeStyle,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawRoundedRectangleWithColorAndStrokeWidthAndStrokeStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    rect,
                    radiusx,
                    radiusy,
                    color,
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`, `"UI"`*
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "UI"))]
    pub fn DrawRoundedRectangleAtCoordsWithColorAndStrokeWidthAndStrokeStyle(
        &self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radiusx: f32,
        radiusy: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
        strokestyle: &Geometry::CanvasStrokeStyle,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawRoundedRectangleAtCoordsWithColorAndStrokeWidthAndStrokeStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    w,
                    h,
                    radiusx,
                    radiusy,
                    color,
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Foundation"`*
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation"))]
    pub fn FillRoundedRectangleWithBrush<P0, E0>(
        &self,
        rect: ::windows::Foundation::Rect,
        radiusx: f32,
        radiusy: f32,
        brush: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .FillRoundedRectangleWithBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    rect,
                    radiusx,
                    radiusy,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`*
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub fn FillRoundedRectangleAtCoordsWithBrush<P0, E0>(
        &self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radiusx: f32,
        radiusy: f32,
        brush: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .FillRoundedRectangleAtCoordsWithBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    w,
                    h,
                    radiusx,
                    radiusy,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation"`, `"UI"`*
    #[cfg(all(feature = "Foundation", feature = "UI"))]
    pub fn FillRoundedRectangleWithColor(
        &self,
        rect: ::windows::Foundation::Rect,
        radiusx: f32,
        radiusy: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .FillRoundedRectangleWithColor)(
                    ::windows::core::Vtable::as_raw(this),
                    rect,
                    radiusx,
                    radiusy,
                    color,
                )
                .ok()
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn FillRoundedRectangleAtCoordsWithColor(
        &self,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radiusx: f32,
        radiusy: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .FillRoundedRectangleAtCoordsWithColor)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    w,
                    h,
                    radiusx,
                    radiusy,
                    color,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Foundation_Numerics"`*
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation_Numerics"))]
    pub fn DrawEllipseWithBrush<P0, E0>(
        &self,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radiusx: f32,
        radiusy: f32,
        brush: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawEllipseWithBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    centerpoint,
                    radiusx,
                    radiusy,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`*
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub fn DrawEllipseAtCoordsWithBrush<P0, E0>(
        &self,
        x: f32,
        y: f32,
        radiusx: f32,
        radiusy: f32,
        brush: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawEllipseAtCoordsWithBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    radiusx,
                    radiusy,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`, `"UI"`*
    #[cfg(all(feature = "Foundation_Numerics", feature = "UI"))]
    pub fn DrawEllipseWithColor(
        &self,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radiusx: f32,
        radiusy: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawEllipseWithColor)(
                    ::windows::core::Vtable::as_raw(this),
                    centerpoint,
                    radiusx,
                    radiusy,
                    color,
                )
                .ok()
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn DrawEllipseAtCoordsWithColor(
        &self,
        x: f32,
        y: f32,
        radiusx: f32,
        radiusy: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawEllipseAtCoordsWithColor)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    radiusx,
                    radiusy,
                    color,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Foundation_Numerics"`*
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation_Numerics"))]
    pub fn DrawEllipseWithBrushAndStrokeWidth<P0, E0>(
        &self,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radiusx: f32,
        radiusy: f32,
        brush: P0,
        strokewidth: f32,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawEllipseWithBrushAndStrokeWidth)(
                    ::windows::core::Vtable::as_raw(this),
                    centerpoint,
                    radiusx,
                    radiusy,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    strokewidth,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`*
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub fn DrawEllipseAtCoordsWithBrushAndStrokeWidth<P0, E0>(
        &self,
        x: f32,
        y: f32,
        radiusx: f32,
        radiusy: f32,
        brush: P0,
        strokewidth: f32,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawEllipseAtCoordsWithBrushAndStrokeWidth)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    radiusx,
                    radiusy,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    strokewidth,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`, `"UI"`*
    #[cfg(all(feature = "Foundation_Numerics", feature = "UI"))]
    pub fn DrawEllipseWithColorAndStrokeWidth(
        &self,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radiusx: f32,
        radiusy: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawEllipseWithColorAndStrokeWidth)(
                    ::windows::core::Vtable::as_raw(this),
                    centerpoint,
                    radiusx,
                    radiusy,
                    color,
                    strokewidth,
                )
                .ok()
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn DrawEllipseAtCoordsWithColorAndStrokeWidth(
        &self,
        x: f32,
        y: f32,
        radiusx: f32,
        radiusy: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawEllipseAtCoordsWithColorAndStrokeWidth)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    radiusx,
                    radiusy,
                    color,
                    strokewidth,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Geometry"`, `"Foundation_Numerics"`*
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics"
        )
    )]
    pub fn DrawEllipseWithBrushAndStrokeWidthAndStrokeStyle<P0, E0>(
        &self,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radiusx: f32,
        radiusy: f32,
        brush: P0,
        strokewidth: f32,
        strokestyle: &Geometry::CanvasStrokeStyle,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawEllipseWithBrushAndStrokeWidthAndStrokeStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    centerpoint,
                    radiusx,
                    radiusy,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Geometry"`*
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub fn DrawEllipseAtCoordsWithBrushAndStrokeWidthAndStrokeStyle<P0, E0>(
        &self,
        x: f32,
        y: f32,
        radiusx: f32,
        radiusy: f32,
        brush: P0,
        strokewidth: f32,
        strokestyle: &Geometry::CanvasStrokeStyle,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawEllipseAtCoordsWithBrushAndStrokeWidthAndStrokeStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    radiusx,
                    radiusy,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`, `"Foundation_Numerics"`, `"UI"`*
    #[cfg(
        all(
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics",
            feature = "UI"
        )
    )]
    pub fn DrawEllipseWithColorAndStrokeWidthAndStrokeStyle(
        &self,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radiusx: f32,
        radiusy: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
        strokestyle: &Geometry::CanvasStrokeStyle,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawEllipseWithColorAndStrokeWidthAndStrokeStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    centerpoint,
                    radiusx,
                    radiusy,
                    color,
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`, `"UI"`*
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "UI"))]
    pub fn DrawEllipseAtCoordsWithColorAndStrokeWidthAndStrokeStyle(
        &self,
        x: f32,
        y: f32,
        radiusx: f32,
        radiusy: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
        strokestyle: &Geometry::CanvasStrokeStyle,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawEllipseAtCoordsWithColorAndStrokeWidthAndStrokeStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    radiusx,
                    radiusy,
                    color,
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Foundation_Numerics"`*
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation_Numerics"))]
    pub fn FillEllipseWithBrush<P0, E0>(
        &self,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radiusx: f32,
        radiusy: f32,
        brush: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .FillEllipseWithBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    centerpoint,
                    radiusx,
                    radiusy,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`*
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub fn FillEllipseAtCoordsWithBrush<P0, E0>(
        &self,
        x: f32,
        y: f32,
        radiusx: f32,
        radiusy: f32,
        brush: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .FillEllipseAtCoordsWithBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    radiusx,
                    radiusy,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`, `"UI"`*
    #[cfg(all(feature = "Foundation_Numerics", feature = "UI"))]
    pub fn FillEllipseWithColor(
        &self,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radiusx: f32,
        radiusy: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .FillEllipseWithColor)(
                    ::windows::core::Vtable::as_raw(this),
                    centerpoint,
                    radiusx,
                    radiusy,
                    color,
                )
                .ok()
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn FillEllipseAtCoordsWithColor(
        &self,
        x: f32,
        y: f32,
        radiusx: f32,
        radiusy: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .FillEllipseAtCoordsWithColor)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    radiusx,
                    radiusy,
                    color,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Foundation_Numerics"`*
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation_Numerics"))]
    pub fn DrawCircleWithBrush<P0, E0>(
        &self,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radius: f32,
        brush: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawCircleWithBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    centerpoint,
                    radius,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`*
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub fn DrawCircleAtCoordsWithBrush<P0, E0>(
        &self,
        x: f32,
        y: f32,
        radius: f32,
        brush: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawCircleAtCoordsWithBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    radius,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`, `"UI"`*
    #[cfg(all(feature = "Foundation_Numerics", feature = "UI"))]
    pub fn DrawCircleWithColor(
        &self,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radius: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawCircleWithColor)(
                    ::windows::core::Vtable::as_raw(this),
                    centerpoint,
                    radius,
                    color,
                )
                .ok()
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn DrawCircleAtCoordsWithColor(
        &self,
        x: f32,
        y: f32,
        radius: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawCircleAtCoordsWithColor)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    radius,
                    color,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Foundation_Numerics"`*
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation_Numerics"))]
    pub fn DrawCircleWithBrushAndStrokeWidth<P0, E0>(
        &self,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radius: f32,
        brush: P0,
        strokewidth: f32,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawCircleWithBrushAndStrokeWidth)(
                    ::windows::core::Vtable::as_raw(this),
                    centerpoint,
                    radius,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    strokewidth,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`*
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub fn DrawCircleAtCoordsWithBrushAndStrokeWidth<P0, E0>(
        &self,
        x: f32,
        y: f32,
        radius: f32,
        brush: P0,
        strokewidth: f32,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawCircleAtCoordsWithBrushAndStrokeWidth)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    radius,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    strokewidth,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`, `"UI"`*
    #[cfg(all(feature = "Foundation_Numerics", feature = "UI"))]
    pub fn DrawCircleWithColorAndStrokeWidth(
        &self,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radius: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawCircleWithColorAndStrokeWidth)(
                    ::windows::core::Vtable::as_raw(this),
                    centerpoint,
                    radius,
                    color,
                    strokewidth,
                )
                .ok()
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn DrawCircleAtCoordsWithColorAndStrokeWidth(
        &self,
        x: f32,
        y: f32,
        radius: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawCircleAtCoordsWithColorAndStrokeWidth)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    radius,
                    color,
                    strokewidth,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Geometry"`, `"Foundation_Numerics"`*
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics"
        )
    )]
    pub fn DrawCircleWithBrushAndStrokeWidthAndStrokeStyle<P0, E0>(
        &self,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radius: f32,
        brush: P0,
        strokewidth: f32,
        strokestyle: &Geometry::CanvasStrokeStyle,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawCircleWithBrushAndStrokeWidthAndStrokeStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    centerpoint,
                    radius,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Geometry"`*
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub fn DrawCircleAtCoordsWithBrushAndStrokeWidthAndStrokeStyle<P0, E0>(
        &self,
        x: f32,
        y: f32,
        radius: f32,
        brush: P0,
        strokewidth: f32,
        strokestyle: &Geometry::CanvasStrokeStyle,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawCircleAtCoordsWithBrushAndStrokeWidthAndStrokeStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    radius,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`, `"Foundation_Numerics"`, `"UI"`*
    #[cfg(
        all(
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics",
            feature = "UI"
        )
    )]
    pub fn DrawCircleWithColorAndStrokeWidthAndStrokeStyle(
        &self,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radius: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
        strokestyle: &Geometry::CanvasStrokeStyle,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawCircleWithColorAndStrokeWidthAndStrokeStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    centerpoint,
                    radius,
                    color,
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`, `"UI"`*
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "UI"))]
    pub fn DrawCircleAtCoordsWithColorAndStrokeWidthAndStrokeStyle(
        &self,
        x: f32,
        y: f32,
        radius: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
        strokestyle: &Geometry::CanvasStrokeStyle,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawCircleAtCoordsWithColorAndStrokeWidthAndStrokeStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    radius,
                    color,
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Foundation_Numerics"`*
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation_Numerics"))]
    pub fn FillCircleWithBrush<P0, E0>(
        &self,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radius: f32,
        brush: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .FillCircleWithBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    centerpoint,
                    radius,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`*
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub fn FillCircleAtCoordsWithBrush<P0, E0>(
        &self,
        x: f32,
        y: f32,
        radius: f32,
        brush: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .FillCircleAtCoordsWithBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    radius,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`, `"UI"`*
    #[cfg(all(feature = "Foundation_Numerics", feature = "UI"))]
    pub fn FillCircleWithColor(
        &self,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radius: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .FillCircleWithColor)(
                    ::windows::core::Vtable::as_raw(this),
                    centerpoint,
                    radius,
                    color,
                )
                .ok()
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn FillCircleAtCoordsWithColor(
        &self,
        x: f32,
        y: f32,
        radius: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .FillCircleAtCoordsWithColor)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    radius,
                    color,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`, `"UI"`*
    #[cfg(all(feature = "Foundation_Numerics", feature = "UI"))]
    pub fn DrawTextAtPointWithColor(
        &self,
        text: &::windows::core::HSTRING,
        point: ::windows::Foundation::Numerics::Vector2,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawTextAtPointWithColor)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(text),
                    point,
                    color,
                )
                .ok()
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn DrawTextAtPointCoordsWithColor(
        &self,
        text: &::windows::core::HSTRING,
        x: f32,
        y: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawTextAtPointCoordsWithColor)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(text),
                    x,
                    y,
                    color,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Text"`, `"Foundation_Numerics"`*
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Text",
            feature = "Foundation_Numerics"
        )
    )]
    pub fn DrawTextAtPointWithBrushAndFormat<P0, E0>(
        &self,
        text: &::windows::core::HSTRING,
        point: ::windows::Foundation::Numerics::Vector2,
        brush: P0,
        format: &Text::CanvasTextFormat,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawTextAtPointWithBrushAndFormat)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(text),
                    point,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    ::core::mem::transmute_copy(format),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Text"`, `"Foundation"`*
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Text",
            feature = "Foundation"
        )
    )]
    pub fn DrawTextAtRectWithBrushAndFormat<P0, E0>(
        &self,
        text: &::windows::core::HSTRING,
        rectangle: ::windows::Foundation::Rect,
        brush: P0,
        format: &Text::CanvasTextFormat,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawTextAtRectWithBrushAndFormat)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(text),
                    rectangle,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    ::core::mem::transmute_copy(format),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Text"`*
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Text"))]
    pub fn DrawTextAtPointCoordsWithBrushAndFormat<P0, E0>(
        &self,
        text: &::windows::core::HSTRING,
        x: f32,
        y: f32,
        brush: P0,
        format: &Text::CanvasTextFormat,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawTextAtPointCoordsWithBrushAndFormat)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(text),
                    x,
                    y,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    ::core::mem::transmute_copy(format),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Text"`*
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Text"))]
    pub fn DrawTextAtRectCoordsWithBrushAndFormat<P0, E0>(
        &self,
        text: &::windows::core::HSTRING,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        brush: P0,
        format: &Text::CanvasTextFormat,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawTextAtRectCoordsWithBrushAndFormat)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(text),
                    x,
                    y,
                    w,
                    h,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    ::core::mem::transmute_copy(format),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Text"`, `"Foundation_Numerics"`, `"UI"`*
    #[cfg(
        all(
            feature = "Graphics_Canvas_Text",
            feature = "Foundation_Numerics",
            feature = "UI"
        )
    )]
    pub fn DrawTextAtPointWithColorAndFormat(
        &self,
        text: &::windows::core::HSTRING,
        point: ::windows::Foundation::Numerics::Vector2,
        color: ::windows::UI::Color,
        format: &Text::CanvasTextFormat,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawTextAtPointWithColorAndFormat)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(text),
                    point,
                    color,
                    ::core::mem::transmute_copy(format),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Text"`, `"Foundation"`, `"UI"`*
    #[cfg(all(feature = "Graphics_Canvas_Text", feature = "Foundation", feature = "UI"))]
    pub fn DrawTextAtRectWithColorAndFormat(
        &self,
        text: &::windows::core::HSTRING,
        rectangle: ::windows::Foundation::Rect,
        color: ::windows::UI::Color,
        format: &Text::CanvasTextFormat,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawTextAtRectWithColorAndFormat)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(text),
                    rectangle,
                    color,
                    ::core::mem::transmute_copy(format),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Text"`, `"UI"`*
    #[cfg(all(feature = "Graphics_Canvas_Text", feature = "UI"))]
    pub fn DrawTextAtPointCoordsWithColorAndFormat(
        &self,
        text: &::windows::core::HSTRING,
        x: f32,
        y: f32,
        color: ::windows::UI::Color,
        format: &Text::CanvasTextFormat,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawTextAtPointCoordsWithColorAndFormat)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(text),
                    x,
                    y,
                    color,
                    ::core::mem::transmute_copy(format),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Text"`, `"UI"`*
    #[cfg(all(feature = "Graphics_Canvas_Text", feature = "UI"))]
    pub fn DrawTextAtRectCoordsWithColorAndFormat(
        &self,
        text: &::windows::core::HSTRING,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: ::windows::UI::Color,
        format: &Text::CanvasTextFormat,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawTextAtRectCoordsWithColorAndFormat)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(text),
                    x,
                    y,
                    w,
                    h,
                    color,
                    ::core::mem::transmute_copy(format),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Geometry"`, `"Foundation_Numerics"`*
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics"
        )
    )]
    pub fn DrawGeometryWithBrush<P0, E0>(
        &self,
        geometry: &Geometry::CanvasGeometry,
        offset: ::windows::Foundation::Numerics::Vector2,
        brush: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawGeometryWithBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    offset,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`, `"Foundation_Numerics"`, `"UI"`*
    #[cfg(
        all(
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics",
            feature = "UI"
        )
    )]
    pub fn DrawGeometryWithColor(
        &self,
        geometry: &Geometry::CanvasGeometry,
        offset: ::windows::Foundation::Numerics::Vector2,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawGeometryWithColor)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    offset,
                    color,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Geometry"`*
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub fn DrawGeometryAtCoordsWithBrush<P0, E0>(
        &self,
        geometry: &Geometry::CanvasGeometry,
        x: f32,
        y: f32,
        brush: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawGeometryAtCoordsWithBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    x,
                    y,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`, `"UI"`*
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "UI"))]
    pub fn DrawGeometryAtCoordsWithColor(
        &self,
        geometry: &Geometry::CanvasGeometry,
        x: f32,
        y: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawGeometryAtCoordsWithColor)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    x,
                    y,
                    color,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Geometry"`*
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub fn DrawGeometryAtOriginWithBrush<P0, E0>(
        &self,
        geometry: &Geometry::CanvasGeometry,
        brush: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawGeometryAtOriginWithBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    brush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`, `"UI"`*
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "UI"))]
    pub fn DrawGeometryAtOriginWithColor(
        &self,
        geometry: &Geometry::CanvasGeometry,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawGeometryAtOriginWithColor)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    color,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Geometry"`, `"Foundation_Numerics"`*
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics"
        )
    )]
    pub fn DrawGeometryWithBrushAndStrokeWidth<P0, E0>(
        &self,
        geometry: &Geometry::CanvasGeometry,
        offset: ::windows::Foundation::Numerics::Vector2,
        brush: P0,
        strokewidth: f32,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawGeometryWithBrushAndStrokeWidth)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    offset,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    strokewidth,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`, `"Foundation_Numerics"`, `"UI"`*
    #[cfg(
        all(
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics",
            feature = "UI"
        )
    )]
    pub fn DrawGeometryWithColorAndStrokeWidth(
        &self,
        geometry: &Geometry::CanvasGeometry,
        offset: ::windows::Foundation::Numerics::Vector2,
        color: ::windows::UI::Color,
        strokewidth: f32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawGeometryWithColorAndStrokeWidth)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    offset,
                    color,
                    strokewidth,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Geometry"`*
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub fn DrawGeometryAtCoordsWithBrushAndStrokeWidth<P0, E0>(
        &self,
        geometry: &Geometry::CanvasGeometry,
        x: f32,
        y: f32,
        brush: P0,
        strokewidth: f32,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawGeometryAtCoordsWithBrushAndStrokeWidth)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    x,
                    y,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    strokewidth,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`, `"UI"`*
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "UI"))]
    pub fn DrawGeometryAtCoordsWithColorAndStrokeWidth(
        &self,
        geometry: &Geometry::CanvasGeometry,
        x: f32,
        y: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawGeometryAtCoordsWithColorAndStrokeWidth)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    x,
                    y,
                    color,
                    strokewidth,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Geometry"`*
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub fn DrawGeometryAtOriginWithBrushAndStrokeWidth<P0, E0>(
        &self,
        geometry: &Geometry::CanvasGeometry,
        brush: P0,
        strokewidth: f32,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawGeometryAtOriginWithBrushAndStrokeWidth)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    strokewidth,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`, `"UI"`*
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "UI"))]
    pub fn DrawGeometryAtOriginWithColorAndStrokeWidth(
        &self,
        geometry: &Geometry::CanvasGeometry,
        color: ::windows::UI::Color,
        strokewidth: f32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawGeometryAtOriginWithColorAndStrokeWidth)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    color,
                    strokewidth,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Geometry"`, `"Foundation_Numerics"`*
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics"
        )
    )]
    pub fn DrawGeometryWithBrushAndStrokeWidthAndStrokeStyle<P0, E0>(
        &self,
        geometry: &Geometry::CanvasGeometry,
        offset: ::windows::Foundation::Numerics::Vector2,
        brush: P0,
        strokewidth: f32,
        strokestyle: &Geometry::CanvasStrokeStyle,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawGeometryWithBrushAndStrokeWidthAndStrokeStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    offset,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`, `"Foundation_Numerics"`, `"UI"`*
    #[cfg(
        all(
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics",
            feature = "UI"
        )
    )]
    pub fn DrawGeometryWithColorAndStrokeWidthAndStrokeStyle(
        &self,
        geometry: &Geometry::CanvasGeometry,
        offset: ::windows::Foundation::Numerics::Vector2,
        color: ::windows::UI::Color,
        strokewidth: f32,
        strokestyle: &Geometry::CanvasStrokeStyle,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawGeometryWithColorAndStrokeWidthAndStrokeStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    offset,
                    color,
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Geometry"`*
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub fn DrawGeometryAtCoordsWithBrushAndStrokeWidthAndStrokeStyle<P0, E0>(
        &self,
        geometry: &Geometry::CanvasGeometry,
        x: f32,
        y: f32,
        brush: P0,
        strokewidth: f32,
        strokestyle: &Geometry::CanvasStrokeStyle,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawGeometryAtCoordsWithBrushAndStrokeWidthAndStrokeStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    x,
                    y,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`, `"UI"`*
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "UI"))]
    pub fn DrawGeometryAtCoordsWithColorAndStrokeWidthAndStrokeStyle(
        &self,
        geometry: &Geometry::CanvasGeometry,
        x: f32,
        y: f32,
        color: ::windows::UI::Color,
        strokewidth: f32,
        strokestyle: &Geometry::CanvasStrokeStyle,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawGeometryAtCoordsWithColorAndStrokeWidthAndStrokeStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    x,
                    y,
                    color,
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Geometry"`*
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub fn DrawGeometryAtOriginWithBrushAndStrokeWidthAndStrokeStyle<P0, E0>(
        &self,
        geometry: &Geometry::CanvasGeometry,
        brush: P0,
        strokewidth: f32,
        strokestyle: &Geometry::CanvasStrokeStyle,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawGeometryAtOriginWithBrushAndStrokeWidthAndStrokeStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`, `"UI"`*
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "UI"))]
    pub fn DrawGeometryAtOriginWithColorAndStrokeWidthAndStrokeStyle(
        &self,
        geometry: &Geometry::CanvasGeometry,
        color: ::windows::UI::Color,
        strokewidth: f32,
        strokestyle: &Geometry::CanvasStrokeStyle,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawGeometryAtOriginWithColorAndStrokeWidthAndStrokeStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    color,
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Geometry"`, `"Foundation_Numerics"`*
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics"
        )
    )]
    pub fn FillGeometryWithBrush<P0, E0>(
        &self,
        geometry: &Geometry::CanvasGeometry,
        offset: ::windows::Foundation::Numerics::Vector2,
        brush: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .FillGeometryWithBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    offset,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Geometry"`, `"Foundation_Numerics"`*
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics"
        )
    )]
    pub fn FillGeometryWithBrushAndOpacityBrush<P0, E0, P1, E1>(
        &self,
        geometry: &Geometry::CanvasGeometry,
        offset: ::windows::Foundation::Numerics::Vector2,
        brush: P0,
        opacitybrush: P1,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .FillGeometryWithBrushAndOpacityBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    offset,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    opacitybrush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`, `"Foundation_Numerics"`, `"UI"`*
    #[cfg(
        all(
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics",
            feature = "UI"
        )
    )]
    pub fn FillGeometryWithColor(
        &self,
        geometry: &Geometry::CanvasGeometry,
        offset: ::windows::Foundation::Numerics::Vector2,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .FillGeometryWithColor)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    offset,
                    color,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Geometry"`*
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub fn FillGeometryAtCoordsWithBrush<P0, E0>(
        &self,
        geometry: &Geometry::CanvasGeometry,
        x: f32,
        y: f32,
        brush: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .FillGeometryAtCoordsWithBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    x,
                    y,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Geometry"`*
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub fn FillGeometryAtCoordsWithBrushAndOpacityBrush<P0, E0, P1, E1>(
        &self,
        geometry: &Geometry::CanvasGeometry,
        x: f32,
        y: f32,
        brush: P0,
        opacitybrush: P1,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .FillGeometryAtCoordsWithBrushAndOpacityBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    x,
                    y,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    opacitybrush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`, `"UI"`*
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "UI"))]
    pub fn FillGeometryAtCoordsWithColor(
        &self,
        geometry: &Geometry::CanvasGeometry,
        x: f32,
        y: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .FillGeometryAtCoordsWithColor)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    x,
                    y,
                    color,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Geometry"`*
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub fn FillGeometryAtOriginWithBrush<P0, E0>(
        &self,
        geometry: &Geometry::CanvasGeometry,
        brush: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .FillGeometryAtOriginWithBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    brush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Geometry"`*
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub fn FillGeometryAtOriginWithBrushAndOpacityBrush<P0, E0, P1, E1>(
        &self,
        geometry: &Geometry::CanvasGeometry,
        brush: P0,
        opacitybrush: P1,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .FillGeometryAtOriginWithBrushAndOpacityBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    opacitybrush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`, `"UI"`*
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "UI"))]
    pub fn FillGeometryAtOriginWithColor(
        &self,
        geometry: &Geometry::CanvasGeometry,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .FillGeometryAtOriginWithColor)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    color,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Geometry"`, `"Foundation_Numerics"`*
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics"
        )
    )]
    pub fn DrawCachedGeometryWithBrush<P0, E0>(
        &self,
        geometry: &Geometry::CanvasCachedGeometry,
        offset: ::windows::Foundation::Numerics::Vector2,
        brush: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawCachedGeometryWithBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    offset,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`, `"Foundation_Numerics"`, `"UI"`*
    #[cfg(
        all(
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics",
            feature = "UI"
        )
    )]
    pub fn DrawCachedGeometryWithColor(
        &self,
        geometry: &Geometry::CanvasCachedGeometry,
        offset: ::windows::Foundation::Numerics::Vector2,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawCachedGeometryWithColor)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    offset,
                    color,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Geometry"`*
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub fn DrawCachedGeometryAtCoordsWithBrush<P0, E0>(
        &self,
        geometry: &Geometry::CanvasCachedGeometry,
        x: f32,
        y: f32,
        brush: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawCachedGeometryAtCoordsWithBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    x,
                    y,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`, `"UI"`*
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "UI"))]
    pub fn DrawCachedGeometryAtCoordsWithColor(
        &self,
        geometry: &Geometry::CanvasCachedGeometry,
        x: f32,
        y: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawCachedGeometryAtCoordsWithColor)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    x,
                    y,
                    color,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Geometry"`*
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub fn DrawCachedGeometryAtOriginWithBrush<P0, E0>(
        &self,
        geometry: &Geometry::CanvasCachedGeometry,
        brush: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawCachedGeometryAtOriginWithBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    brush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`, `"UI"`*
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "UI"))]
    pub fn DrawCachedGeometryAtOriginWithColor(
        &self,
        geometry: &Geometry::CanvasCachedGeometry,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawCachedGeometryAtOriginWithColor)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    color,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Text"`, `"Foundation_Numerics"`*
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Text",
            feature = "Foundation_Numerics"
        )
    )]
    pub fn DrawTextLayoutWithBrush<P0, E0>(
        &self,
        textlayout: &Text::CanvasTextLayout,
        point: ::windows::Foundation::Numerics::Vector2,
        brush: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawTextLayoutWithBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(textlayout),
                    point,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Text"`*
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Text"))]
    pub fn DrawTextLayoutAtCoordsWithBrush<P0, E0>(
        &self,
        textlayout: &Text::CanvasTextLayout,
        x: f32,
        y: f32,
        brush: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawTextLayoutAtCoordsWithBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(textlayout),
                    x,
                    y,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Text"`, `"Foundation_Numerics"`, `"UI"`*
    #[cfg(
        all(
            feature = "Graphics_Canvas_Text",
            feature = "Foundation_Numerics",
            feature = "UI"
        )
    )]
    pub fn DrawTextLayoutWithColor(
        &self,
        textlayout: &Text::CanvasTextLayout,
        point: ::windows::Foundation::Numerics::Vector2,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawTextLayoutWithColor)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(textlayout),
                    point,
                    color,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Text"`, `"UI"`*
    #[cfg(all(feature = "Graphics_Canvas_Text", feature = "UI"))]
    pub fn DrawTextLayoutAtCoordsWithColor(
        &self,
        textlayout: &Text::CanvasTextLayout,
        x: f32,
        y: f32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawTextLayoutAtCoordsWithColor)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(textlayout),
                    x,
                    y,
                    color,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Collections"`, `"UI_Input_Inking"`*
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Input_Inking"))]
    pub fn DrawInk<P0, E0>(&self, inkstrokes: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<
                ::windows::Foundation::Collections::IIterable::<
                    ::windows::UI::Input::Inking::InkStroke,
                >,
            >,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawInk)(
                    ::windows::core::Vtable::as_raw(this),
                    inkstrokes.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Collections"`, `"UI_Input_Inking"`*
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Input_Inking"))]
    pub fn DrawInkWithHighContrast<P0, E0>(
        &self,
        inkstrokes: P0,
        highcontrast: bool,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<
                ::windows::Foundation::Collections::IIterable::<
                    ::windows::UI::Input::Inking::InkStroke,
                >,
            >,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawInkWithHighContrast)(
                    ::windows::core::Vtable::as_raw(this),
                    inkstrokes.try_into().map_err(|e| e.into())?.abi(),
                    highcontrast,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`*
    #[cfg(feature = "Graphics_Canvas_Geometry")]
    pub fn DrawGradientMeshAtOrigin(
        &self,
        gradientmesh: &Geometry::CanvasGradientMesh,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawGradientMeshAtOrigin)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(gradientmesh),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`, `"Foundation_Numerics"`*
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "Foundation_Numerics"))]
    pub fn DrawGradientMesh(
        &self,
        gradientmesh: &Geometry::CanvasGradientMesh,
        point: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawGradientMesh)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(gradientmesh),
                    point,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`*
    #[cfg(feature = "Graphics_Canvas_Geometry")]
    pub fn DrawGradientMeshAtCoords(
        &self,
        gradientmesh: &Geometry::CanvasGradientMesh,
        x: f32,
        y: f32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawGradientMeshAtCoords)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(gradientmesh),
                    x,
                    y,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Svg"`, `"Foundation"`*
    #[cfg(all(feature = "Graphics_Canvas_Svg", feature = "Foundation"))]
    pub fn DrawSvgAtOrigin(
        &self,
        svgdocument: &Svg::CanvasSvgDocument,
        viewportsize: ::windows::Foundation::Size,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawSvgAtOrigin)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(svgdocument),
                    viewportsize,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Svg"`, `"Foundation_Numerics"`*
    #[cfg(all(feature = "Graphics_Canvas_Svg", feature = "Foundation_Numerics"))]
    pub fn DrawSvgAtPoint(
        &self,
        svgdocument: &Svg::CanvasSvgDocument,
        viewportsize: ::windows::Foundation::Size,
        point: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawSvgAtPoint)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(svgdocument),
                    viewportsize,
                    point,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Svg"`, `"Foundation"`*
    #[cfg(all(feature = "Graphics_Canvas_Svg", feature = "Foundation"))]
    pub fn DrawSvgAtCoords(
        &self,
        svgdocument: &Svg::CanvasSvgDocument,
        viewportsize: ::windows::Foundation::Size,
        x: f32,
        y: f32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawSvgAtCoords)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(svgdocument),
                    viewportsize,
                    x,
                    y,
                )
                .ok()
        }
    }
    pub fn Antialiasing(&self) -> ::windows::core::Result<CanvasAntialiasing> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Antialiasing)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetAntialiasing(
        &self,
        value: CanvasAntialiasing,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetAntialiasing)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn Blend(&self) -> ::windows::core::Result<CanvasBlend> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Blend)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn SetBlend(&self, value: CanvasBlend) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetBlend)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Text"`*
    #[cfg(feature = "Graphics_Canvas_Text")]
    pub fn TextAntialiasing(
        &self,
    ) -> ::windows::core::Result<Text::CanvasTextAntialiasing> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .TextAntialiasing)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Graphics_Canvas_Text"`*
    #[cfg(feature = "Graphics_Canvas_Text")]
    pub fn SetTextAntialiasing(
        &self,
        value: Text::CanvasTextAntialiasing,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetTextAntialiasing)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Text"`*
    #[cfg(feature = "Graphics_Canvas_Text")]
    pub fn TextRenderingParameters(
        &self,
    ) -> ::windows::core::Result<Text::CanvasTextRenderingParameters> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .TextRenderingParameters)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Graphics_Canvas_Text"`*
    #[cfg(feature = "Graphics_Canvas_Text")]
    pub fn SetTextRenderingParameters(
        &self,
        value: &Text::CanvasTextRenderingParameters,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetTextRenderingParameters)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(value),
                )
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
    pub fn Units(&self) -> ::windows::core::Result<CanvasUnits> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Units)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn SetUnits(&self, value: CanvasUnits) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetUnits)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn EffectBufferPrecision(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::IReference::<CanvasBufferPrecision>,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .EffectBufferPrecision)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn SetEffectBufferPrecision<P0, E0>(
        &self,
        value: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<
                ::windows::Foundation::IReference::<CanvasBufferPrecision>,
            >,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetEffectBufferPrecision)(
                    ::windows::core::Vtable::as_raw(this),
                    value.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Imaging"`*
    #[cfg(feature = "Graphics_Imaging")]
    pub fn EffectTileSize(
        &self,
    ) -> ::windows::core::Result<::windows::Graphics::Imaging::BitmapSize> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .EffectTileSize)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Graphics_Imaging"`*
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetEffectTileSize(
        &self,
        value: ::windows::Graphics::Imaging::BitmapSize,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetEffectTileSize)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn CreateLayerWithOpacity(
        &self,
        opacity: f32,
    ) -> ::windows::core::Result<CanvasActiveLayer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateLayerWithOpacity)(
                    ::windows::core::Vtable::as_raw(this),
                    opacity,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`*
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub fn CreateLayerWithOpacityBrush<P0, E0>(
        &self,
        opacitybrush: P0,
    ) -> ::windows::core::Result<CanvasActiveLayer>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateLayerWithOpacityBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    opacitybrush.try_into().map_err(|e| e.into())?.abi(),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn CreateLayerWithOpacityAndClipRectangle(
        &self,
        opacity: f32,
        cliprectangle: ::windows::Foundation::Rect,
    ) -> ::windows::core::Result<CanvasActiveLayer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateLayerWithOpacityAndClipRectangle)(
                    ::windows::core::Vtable::as_raw(this),
                    opacity,
                    cliprectangle,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Foundation"`*
    #[cfg(all(feature = "Graphics_Canvas_Brushes", feature = "Foundation"))]
    pub fn CreateLayerWithOpacityBrushAndClipRectangle<P0, E0>(
        &self,
        opacitybrush: P0,
        cliprectangle: ::windows::Foundation::Rect,
    ) -> ::windows::core::Result<CanvasActiveLayer>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateLayerWithOpacityBrushAndClipRectangle)(
                    ::windows::core::Vtable::as_raw(this),
                    opacitybrush.try_into().map_err(|e| e.into())?.abi(),
                    cliprectangle,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`*
    #[cfg(feature = "Graphics_Canvas_Geometry")]
    pub fn CreateLayerWithOpacityAndClipGeometry(
        &self,
        opacity: f32,
        clipgeometry: &Geometry::CanvasGeometry,
    ) -> ::windows::core::Result<CanvasActiveLayer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateLayerWithOpacityAndClipGeometry)(
                    ::windows::core::Vtable::as_raw(this),
                    opacity,
                    ::core::mem::transmute_copy(clipgeometry),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Geometry"`*
    #[cfg(
        all(feature = "Graphics_Canvas_Brushes", feature = "Graphics_Canvas_Geometry")
    )]
    pub fn CreateLayerWithOpacityBrushAndClipGeometry<P0, E0>(
        &self,
        opacitybrush: P0,
        clipgeometry: &Geometry::CanvasGeometry,
    ) -> ::windows::core::Result<CanvasActiveLayer>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateLayerWithOpacityBrushAndClipGeometry)(
                    ::windows::core::Vtable::as_raw(this),
                    opacitybrush.try_into().map_err(|e| e.into())?.abi(),
                    ::core::mem::transmute_copy(clipgeometry),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Graphics_Canvas_Geometry"`, `"Foundation_Numerics"`*
    #[cfg(all(feature = "Graphics_Canvas_Geometry", feature = "Foundation_Numerics"))]
    pub fn CreateLayerWithOpacityAndClipGeometryAndTransform(
        &self,
        opacity: f32,
        clipgeometry: &Geometry::CanvasGeometry,
        geometrytransform: ::windows::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::core::Result<CanvasActiveLayer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateLayerWithOpacityAndClipGeometryAndTransform)(
                    ::windows::core::Vtable::as_raw(this),
                    opacity,
                    ::core::mem::transmute_copy(clipgeometry),
                    geometrytransform,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Geometry"`, `"Foundation_Numerics"`*
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics"
        )
    )]
    pub fn CreateLayerWithOpacityBrushAndClipGeometryAndTransform<P0, E0>(
        &self,
        opacitybrush: P0,
        clipgeometry: &Geometry::CanvasGeometry,
        geometrytransform: ::windows::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::core::Result<CanvasActiveLayer>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateLayerWithOpacityBrushAndClipGeometryAndTransform)(
                    ::windows::core::Vtable::as_raw(this),
                    opacitybrush.try_into().map_err(|e| e.into())?.abi(),
                    ::core::mem::transmute_copy(clipgeometry),
                    geometrytransform,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Geometry"`, `"Foundation_Numerics"`*
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Geometry",
            feature = "Foundation_Numerics"
        )
    )]
    pub fn CreateLayerWithAllOptions<P0, E0>(
        &self,
        opacity: f32,
        opacitybrush: P0,
        cliprectangle: ::windows::Foundation::Rect,
        clipgeometry: &Geometry::CanvasGeometry,
        geometrytransform: ::windows::Foundation::Numerics::Matrix3x2,
        options: CanvasLayerOptions,
    ) -> ::windows::core::Result<CanvasActiveLayer>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateLayerWithAllOptions)(
                    ::windows::core::Vtable::as_raw(this),
                    opacity,
                    opacitybrush.try_into().map_err(|e| e.into())?.abi(),
                    cliprectangle,
                    ::core::mem::transmute_copy(clipgeometry),
                    geometrytransform,
                    options,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Text"`, `"Foundation_Numerics"`*
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Text",
            feature = "Foundation_Numerics"
        )
    )]
    pub fn DrawGlyphRun<P0, E0>(
        &self,
        point: ::windows::Foundation::Numerics::Vector2,
        fontface: &Text::CanvasFontFace,
        fontsize: f32,
        glyphs: &[Text::CanvasGlyph],
        issideways: bool,
        bidilevel: u32,
        brush: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawGlyphRun)(
                    ::windows::core::Vtable::as_raw(this),
                    point,
                    ::core::mem::transmute_copy(fontface),
                    fontsize,
                    glyphs.len() as u32,
                    glyphs.as_ptr(),
                    issideways,
                    bidilevel,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Text"`, `"Foundation_Numerics"`*
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Text",
            feature = "Foundation_Numerics"
        )
    )]
    pub fn DrawGlyphRunWithMeasuringMode<P0, E0>(
        &self,
        point: ::windows::Foundation::Numerics::Vector2,
        fontface: &Text::CanvasFontFace,
        fontsize: f32,
        glyphs: &[Text::CanvasGlyph],
        issideways: bool,
        bidilevel: u32,
        brush: P0,
        measuringmode: Text::CanvasTextMeasuringMode,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawGlyphRunWithMeasuringMode)(
                    ::windows::core::Vtable::as_raw(this),
                    point,
                    ::core::mem::transmute_copy(fontface),
                    fontsize,
                    glyphs.len() as u32,
                    glyphs.as_ptr(),
                    issideways,
                    bidilevel,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    measuringmode,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`, `"Graphics_Canvas_Text"`, `"Foundation_Numerics"`*
    #[cfg(
        all(
            feature = "Graphics_Canvas_Brushes",
            feature = "Graphics_Canvas_Text",
            feature = "Foundation_Numerics"
        )
    )]
    pub fn DrawGlyphRunWithMeasuringModeAndDescription<P0, E0>(
        &self,
        point: ::windows::Foundation::Numerics::Vector2,
        fontface: &Text::CanvasFontFace,
        fontsize: f32,
        glyphs: &[Text::CanvasGlyph],
        issideways: bool,
        bidilevel: u32,
        brush: P0,
        measuringmode: Text::CanvasTextMeasuringMode,
        localename: &::windows::core::HSTRING,
        textstring: &::windows::core::HSTRING,
        clustermapindices: &[i32],
        textposition: u32,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawGlyphRunWithMeasuringModeAndDescription)(
                    ::windows::core::Vtable::as_raw(this),
                    point,
                    ::core::mem::transmute_copy(fontface),
                    fontsize,
                    glyphs.len() as u32,
                    glyphs.as_ptr(),
                    issideways,
                    bidilevel,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                    measuringmode,
                    ::core::mem::transmute_copy(localename),
                    ::core::mem::transmute_copy(textstring),
                    clustermapindices.len() as u32,
                    clustermapindices.as_ptr(),
                    textposition,
                )
                .ok()
        }
    }
    pub fn CreateSpriteBatch(&self) -> ::windows::core::Result<CanvasSpriteBatch> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateSpriteBatch)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn CreateSpriteBatchWithSortMode(
        &self,
        sortmode: CanvasSpriteSortMode,
    ) -> ::windows::core::Result<CanvasSpriteBatch> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateSpriteBatchWithSortMode)(
                    ::windows::core::Vtable::as_raw(this),
                    sortmode,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn CreateSpriteBatchWithSortModeAndInterpolation(
        &self,
        sortmode: CanvasSpriteSortMode,
        interpolation: CanvasImageInterpolation,
    ) -> ::windows::core::Result<CanvasSpriteBatch> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateSpriteBatchWithSortModeAndInterpolation)(
                    ::windows::core::Vtable::as_raw(this),
                    sortmode,
                    interpolation,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn CreateSpriteBatchWithSortModeAndInterpolationAndOptions(
        &self,
        sortmode: CanvasSpriteSortMode,
        interpolation: CanvasImageInterpolation,
        options: CanvasSpriteOptions,
    ) -> ::windows::core::Result<CanvasSpriteBatch> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateSpriteBatchWithSortModeAndInterpolationAndOptions)(
                    ::windows::core::Vtable::as_raw(this),
                    sortmode,
                    interpolation,
                    options,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn Device(&self) -> ::windows::core::Result<CanvasDevice> {
        let this = &::windows::core::Interface::cast::<ICanvasResourceCreator>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Device)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn Dpi(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<
            ICanvasResourceCreatorWithDpi,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Dpi)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn ConvertPixelsToDips(&self, pixels: i32) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<
            ICanvasResourceCreatorWithDpi,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ConvertPixelsToDips)(
                    ::windows::core::Vtable::as_raw(this),
                    pixels,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn ConvertDipsToPixels(
        &self,
        dips: f32,
        dpirounding: CanvasDpiRounding,
    ) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<
            ICanvasResourceCreatorWithDpi,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ConvertDipsToPixels)(
                    ::windows::core::Vtable::as_raw(this),
                    dips,
                    dpirounding,
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
impl ::core::clone::Clone for CanvasDrawingSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasDrawingSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasDrawingSession {}
impl ::core::fmt::Debug for CanvasDrawingSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasDrawingSession").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasDrawingSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.CanvasDrawingSession;{f60afd09-e623-4be0-b750-578aa920b1db})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasDrawingSession {
    type Vtable = ICanvasDrawingSession_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasDrawingSession {
    const IID: ::windows::core::GUID = <ICanvasDrawingSession as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasDrawingSession {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.CanvasDrawingSession";
}
::windows::core::interface_hierarchy!(
    CanvasDrawingSession,::windows::core::IUnknown,::windows::core::IInspectable
);
impl ::core::convert::TryFrom<CanvasDrawingSession> for ICanvasResourceCreator {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasDrawingSession) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CanvasDrawingSession> for ICanvasResourceCreator {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasDrawingSession) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&CanvasDrawingSession>
for ::windows::core::InParam<ICanvasResourceCreator> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasDrawingSession) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<CanvasDrawingSession> for ICanvasResourceCreatorWithDpi {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasDrawingSession) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CanvasDrawingSession> for ICanvasResourceCreatorWithDpi {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasDrawingSession) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&CanvasDrawingSession>
for ::windows::core::InParam<ICanvasResourceCreatorWithDpi> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasDrawingSession) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasDrawingSession>
for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasDrawingSession) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasDrawingSession>
for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasDrawingSession) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasDrawingSession>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasDrawingSession) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CanvasDrawingSession {}
unsafe impl ::core::marker::Sync for CanvasDrawingSession {}
///*Required features: `"Graphics_Canvas"`*
pub struct CanvasImage;
impl CanvasImage {
    ///*Required features: `"Foundation"`, `"Storage_Streams"`*
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SaveAsync<P0, E0, P1, E1, P2, E2>(
        image: P0,
        sourcerectangle: ::windows::Foundation::Rect,
        dpi: f32,
        resourcecreator: P1,
        stream: P2,
        fileformat: CanvasBitmapFileFormat,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<ICanvasImage>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
        P2: ::std::convert::TryInto<
            ::windows::core::InParam<::windows::Storage::Streams::IRandomAccessStream>,
            Error = E2,
        >,
        E2: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasImageStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .SaveAsync)(
                    ::windows::core::Vtable::as_raw(this),
                    image.try_into().map_err(|e| e.into())?.abi(),
                    sourcerectangle,
                    dpi,
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    stream.try_into().map_err(|e| e.into())?.abi(),
                    fileformat,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation"`, `"Storage_Streams"`*
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SaveWithQualityAsync<P0, E0, P1, E1, P2, E2>(
        image: P0,
        sourcerectangle: ::windows::Foundation::Rect,
        dpi: f32,
        resourcecreator: P1,
        stream: P2,
        fileformat: CanvasBitmapFileFormat,
        quality: f32,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<ICanvasImage>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
        P2: ::std::convert::TryInto<
            ::windows::core::InParam<::windows::Storage::Streams::IRandomAccessStream>,
            Error = E2,
        >,
        E2: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasImageStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .SaveWithQualityAsync)(
                    ::windows::core::Vtable::as_raw(this),
                    image.try_into().map_err(|e| e.into())?.abi(),
                    sourcerectangle,
                    dpi,
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    stream.try_into().map_err(|e| e.into())?.abi(),
                    fileformat,
                    quality,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation"`, `"Storage_Streams"`*
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SaveWithQualityAndBufferPrecisionAsync<P0, E0, P1, E1, P2, E2>(
        image: P0,
        sourcerectangle: ::windows::Foundation::Rect,
        dpi: f32,
        resourcecreator: P1,
        stream: P2,
        fileformat: CanvasBitmapFileFormat,
        quality: f32,
        bufferprecision: CanvasBufferPrecision,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<ICanvasImage>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
        P2: ::std::convert::TryInto<
            ::windows::core::InParam<::windows::Storage::Streams::IRandomAccessStream>,
            Error = E2,
        >,
        E2: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasImageStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .SaveWithQualityAndBufferPrecisionAsync)(
                    ::windows::core::Vtable::as_raw(this),
                    image.try_into().map_err(|e| e.into())?.abi(),
                    sourcerectangle,
                    dpi,
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    stream.try_into().map_err(|e| e.into())?.abi(),
                    fileformat,
                    quality,
                    bufferprecision,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Graphics_Canvas_Effects"`, `"Foundation"`*
    #[cfg(all(feature = "Graphics_Canvas_Effects", feature = "Foundation"))]
    pub fn ComputeHistogram<P0, E0, P1, E1>(
        image: P0,
        sourcerectangle: ::windows::Foundation::Rect,
        resourcecreator: P1,
        channelselect: Effects::EffectChannelSelect,
        numberofbins: i32,
    ) -> ::windows::core::Result<::windows::core::Array<f32>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<ICanvasImage>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasImageStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ComputeHistogram)(
                    ::windows::core::Vtable::as_raw(this),
                    image.try_into().map_err(|e| e.into())?.abi(),
                    sourcerectangle,
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    channelselect,
                    numberofbins,
                    ::windows::core::Array::<
                        f32,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        })
    }
    pub fn IsHistogramSupported(device: &CanvasDevice) -> ::windows::core::Result<bool> {
        Self::ICanvasImageStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .IsHistogramSupported)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(device),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICanvasImageStatics<
        R,
        F: FnOnce(&ICanvasImageStatics) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<CanvasImage, ICanvasImageStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for CanvasImage {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.CanvasImage";
}
///*Required features: `"Graphics_Canvas"`*
#[repr(transparent)]
pub struct CanvasLock(::windows::core::IUnknown);
impl CanvasLock {
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
impl ::core::clone::Clone for CanvasLock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasLock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasLock {}
impl ::core::fmt::Debug for CanvasLock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasLock").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasLock {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.CanvasLock;{7a0e8498-fba9-4fb0-aa8c-6a48b5ee3e4f})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasLock {
    type Vtable = ICanvasLock_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasLock {
    const IID: ::windows::core::GUID = <ICanvasLock as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasLock {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.CanvasLock";
}
::windows::core::interface_hierarchy!(
    CanvasLock,::windows::core::IUnknown,::windows::core::IInspectable
);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasLock> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasLock) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasLock> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasLock) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasLock>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasLock) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CanvasLock {}
unsafe impl ::core::marker::Sync for CanvasLock {}
///*Required features: `"Graphics_Canvas"`*
#[repr(transparent)]
pub struct CanvasRenderTarget(::windows::core::IUnknown);
impl CanvasRenderTarget {
    ///*Required features: `"Graphics_Imaging"`*
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SizeInPixels(
        &self,
    ) -> ::windows::core::Result<::windows::Graphics::Imaging::BitmapSize> {
        let this = &::windows::core::Interface::cast::<ICanvasBitmap>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .SizeInPixels)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn Size(&self) -> ::windows::core::Result<::windows::Foundation::Size> {
        let this = &::windows::core::Interface::cast::<ICanvasBitmap>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn Bounds(&self) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = &::windows::core::Interface::cast::<ICanvasBitmap>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Bounds)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"Graphics_DirectX"`*
    #[cfg(feature = "Graphics_DirectX")]
    pub fn Format(
        &self,
    ) -> ::windows::core::Result<::windows::Graphics::DirectX::DirectXPixelFormat> {
        let this = &::windows::core::Interface::cast::<ICanvasBitmap>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Format)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn AlphaMode(&self) -> ::windows::core::Result<CanvasAlphaMode> {
        let this = &::windows::core::Interface::cast::<ICanvasBitmap>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .AlphaMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn SaveToFileAsync(
        &self,
        filename: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<ICanvasBitmap>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .SaveToFileAsync)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(filename),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn SaveToFileWithBitmapFileFormatAsync(
        &self,
        filename: &::windows::core::HSTRING,
        fileformat: CanvasBitmapFileFormat,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<ICanvasBitmap>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .SaveToFileWithBitmapFileFormatAsync)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(filename),
                    fileformat,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn SaveToFileWithBitmapFileFormatAndQualityAsync(
        &self,
        filename: &::windows::core::HSTRING,
        fileformat: CanvasBitmapFileFormat,
        quality: f32,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<ICanvasBitmap>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .SaveToFileWithBitmapFileFormatAndQualityAsync)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(filename),
                    fileformat,
                    quality,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`, `"Storage_Streams"`*
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SaveToStreamAsync<P0, E0>(
        &self,
        stream: P0,
        fileformat: CanvasBitmapFileFormat,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<::windows::Storage::Streams::IRandomAccessStream>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<ICanvasBitmap>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .SaveToStreamAsync)(
                    ::windows::core::Vtable::as_raw(this),
                    stream.try_into().map_err(|e| e.into())?.abi(),
                    fileformat,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`, `"Storage_Streams"`*
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SaveToStreamWithQualityAsync<P0, E0>(
        &self,
        stream: P0,
        fileformat: CanvasBitmapFileFormat,
        quality: f32,
    ) -> ::windows::core::Result<::windows::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<::windows::Storage::Streams::IRandomAccessStream>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<ICanvasBitmap>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .SaveToStreamWithQualityAsync)(
                    ::windows::core::Vtable::as_raw(this),
                    stream.try_into().map_err(|e| e.into())?.abi(),
                    fileformat,
                    quality,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn GetPixelBytes(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = &::windows::core::Interface::cast::<ICanvasBitmap>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetPixelBytes)(
                    ::windows::core::Vtable::as_raw(this),
                    ::windows::core::Array::<
                        u8,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn GetPixelBytesWithSubrectangle(
        &self,
        left: i32,
        top: i32,
        width: i32,
        height: i32,
    ) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = &::windows::core::Interface::cast::<ICanvasBitmap>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetPixelBytesWithSubrectangle)(
                    ::windows::core::Vtable::as_raw(this),
                    left,
                    top,
                    width,
                    height,
                    ::windows::core::Array::<
                        u8,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    ///*Required features: `"Storage_Streams"`*
    #[cfg(feature = "Storage_Streams")]
    pub fn GetPixelBytesWithBuffer<P0, E0>(
        &self,
        buffer: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<::windows::Storage::Streams::IBuffer>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<ICanvasBitmap>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .GetPixelBytesWithBuffer)(
                    ::windows::core::Vtable::as_raw(this),
                    buffer.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Storage_Streams"`*
    #[cfg(feature = "Storage_Streams")]
    pub fn GetPixelBytesWithBufferAndSubrectangle<P0, E0>(
        &self,
        buffer: P0,
        left: i32,
        top: i32,
        width: i32,
        height: i32,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<::windows::Storage::Streams::IBuffer>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<ICanvasBitmap>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .GetPixelBytesWithBufferAndSubrectangle)(
                    ::windows::core::Vtable::as_raw(this),
                    buffer.try_into().map_err(|e| e.into())?.abi(),
                    left,
                    top,
                    width,
                    height,
                )
                .ok()
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn GetPixelColors(
        &self,
    ) -> ::windows::core::Result<::windows::core::Array<::windows::UI::Color>> {
        let this = &::windows::core::Interface::cast::<ICanvasBitmap>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetPixelColors)(
                    ::windows::core::Vtable::as_raw(this),
                    ::windows::core::Array::<
                        ::windows::UI::Color,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn GetPixelColorsWithSubrectangle(
        &self,
        left: i32,
        top: i32,
        width: i32,
        height: i32,
    ) -> ::windows::core::Result<::windows::core::Array<::windows::UI::Color>> {
        let this = &::windows::core::Interface::cast::<ICanvasBitmap>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetPixelColorsWithSubrectangle)(
                    ::windows::core::Vtable::as_raw(this),
                    left,
                    top,
                    width,
                    height,
                    ::windows::core::Array::<
                        ::windows::UI::Color,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn SetPixelBytes(&self, valueelements: &[u8]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICanvasBitmap>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetPixelBytes)(
                    ::windows::core::Vtable::as_raw(this),
                    valueelements.len() as u32,
                    valueelements.as_ptr(),
                )
                .ok()
        }
    }
    pub fn SetPixelBytesWithSubrectangle(
        &self,
        valueelements: &[u8],
        left: i32,
        top: i32,
        width: i32,
        height: i32,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICanvasBitmap>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetPixelBytesWithSubrectangle)(
                    ::windows::core::Vtable::as_raw(this),
                    valueelements.len() as u32,
                    valueelements.as_ptr(),
                    left,
                    top,
                    width,
                    height,
                )
                .ok()
        }
    }
    ///*Required features: `"Storage_Streams"`*
    #[cfg(feature = "Storage_Streams")]
    pub fn SetPixelBytesWithBuffer<P0, E0>(
        &self,
        buffer: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<::windows::Storage::Streams::IBuffer>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<ICanvasBitmap>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetPixelBytesWithBuffer)(
                    ::windows::core::Vtable::as_raw(this),
                    buffer.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Storage_Streams"`*
    #[cfg(feature = "Storage_Streams")]
    pub fn SetPixelBytesWithBufferAndSubrectangle<P0, E0>(
        &self,
        buffer: P0,
        left: i32,
        top: i32,
        width: i32,
        height: i32,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<::windows::Storage::Streams::IBuffer>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<ICanvasBitmap>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetPixelBytesWithBufferAndSubrectangle)(
                    ::windows::core::Vtable::as_raw(this),
                    buffer.try_into().map_err(|e| e.into())?.abi(),
                    left,
                    top,
                    width,
                    height,
                )
                .ok()
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn SetPixelColors(
        &self,
        valueelements: &[::windows::UI::Color],
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICanvasBitmap>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetPixelColors)(
                    ::windows::core::Vtable::as_raw(this),
                    valueelements.len() as u32,
                    valueelements.as_ptr(),
                )
                .ok()
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn SetPixelColorsWithSubrectangle(
        &self,
        valueelements: &[::windows::UI::Color],
        left: i32,
        top: i32,
        width: i32,
        height: i32,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICanvasBitmap>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetPixelColorsWithSubrectangle)(
                    ::windows::core::Vtable::as_raw(this),
                    valueelements.len() as u32,
                    valueelements.as_ptr(),
                    left,
                    top,
                    width,
                    height,
                )
                .ok()
        }
    }
    pub fn CopyPixelsFromBitmap<P0>(
        &self,
        otherbitmap: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<CanvasBitmap>>,
    {
        let this = &::windows::core::Interface::cast::<ICanvasBitmap>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .CopyPixelsFromBitmap)(
                    ::windows::core::Vtable::as_raw(this),
                    otherbitmap.into().abi(),
                )
                .ok()
        }
    }
    pub fn CopyPixelsFromBitmapWithDestPoint<P0>(
        &self,
        otherbitmap: P0,
        destx: i32,
        desty: i32,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<CanvasBitmap>>,
    {
        let this = &::windows::core::Interface::cast::<ICanvasBitmap>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .CopyPixelsFromBitmapWithDestPoint)(
                    ::windows::core::Vtable::as_raw(this),
                    otherbitmap.into().abi(),
                    destx,
                    desty,
                )
                .ok()
        }
    }
    pub fn CopyPixelsFromBitmapWithDestPointAndSourceRect<P0>(
        &self,
        otherbitmap: P0,
        destx: i32,
        desty: i32,
        sourcerectleft: i32,
        sourcerecttop: i32,
        sourcerectwidth: i32,
        sourcerectheight: i32,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<CanvasBitmap>>,
    {
        let this = &::windows::core::Interface::cast::<ICanvasBitmap>(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .CopyPixelsFromBitmapWithDestPointAndSourceRect)(
                    ::windows::core::Vtable::as_raw(this),
                    otherbitmap.into().abi(),
                    destx,
                    desty,
                    sourcerectleft,
                    sourcerecttop,
                    sourcerectwidth,
                    sourcerectheight,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn GetBounds<P0, E0>(
        &self,
        resourcecreator: P0,
    ) -> ::windows::core::Result<::windows::Foundation::Rect>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<ICanvasImage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetBounds)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn GetBoundsWithTransform<P0, E0>(
        &self,
        resourcecreator: P0,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::core::Result<::windows::Foundation::Rect>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<ICanvasImage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetBoundsWithTransform)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    transform,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn CreateDrawingSession(&self) -> ::windows::core::Result<CanvasDrawingSession> {
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
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn CreateWithSize<P0, E0>(
        resourcecreator: P0,
        size: ::windows::Foundation::Size,
    ) -> ::windows::core::Result<CanvasRenderTarget>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreatorWithDpi>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasRenderTargetFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateWithSize)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    size,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    pub fn CreateWithWidthAndHeight<P0, E0>(
        resourcecreator: P0,
        width: f32,
        height: f32,
    ) -> ::windows::core::Result<CanvasRenderTarget>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreatorWithDpi>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasRenderTargetFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateWithWidthAndHeight)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    width,
                    height,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    pub fn CreateWithWidthAndHeightAndDpi<P0, E0>(
        resourcecreator: P0,
        width: f32,
        height: f32,
        dpi: f32,
    ) -> ::windows::core::Result<CanvasRenderTarget>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasRenderTargetFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateWithWidthAndHeightAndDpi)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    width,
                    height,
                    dpi,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Graphics_DirectX"`*
    #[cfg(feature = "Graphics_DirectX")]
    pub fn CreateWithWidthAndHeightAndDpiAndFormatAndAlpha<P0, E0>(
        resourcecreator: P0,
        width: f32,
        height: f32,
        dpi: f32,
        format: ::windows::Graphics::DirectX::DirectXPixelFormat,
        alpha: CanvasAlphaMode,
    ) -> ::windows::core::Result<CanvasRenderTarget>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasRenderTargetFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateWithWidthAndHeightAndDpiAndFormatAndAlpha)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    width,
                    height,
                    dpi,
                    format,
                    alpha,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Graphics_DirectX_Direct3D11"`*
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CreateFromDirect3D11Surface<P0, E0, P1, E1>(
        resourcecreator: P0,
        surface: P1,
    ) -> ::windows::core::Result<CanvasRenderTarget>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<
                ::windows::Graphics::DirectX::Direct3D11::IDirect3DSurface,
            >,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasRenderTargetStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateFromDirect3D11Surface)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    surface.try_into().map_err(|e| e.into())?.abi(),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Graphics_DirectX_Direct3D11"`*
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CreateFromDirect3D11SurfaceWithDpi<P0, E0, P1, E1>(
        resourcecreator: P0,
        surface: P1,
        dpi: f32,
    ) -> ::windows::core::Result<CanvasRenderTarget>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<
                ::windows::Graphics::DirectX::Direct3D11::IDirect3DSurface,
            >,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasRenderTargetStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateFromDirect3D11SurfaceWithDpi)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    surface.try_into().map_err(|e| e.into())?.abi(),
                    dpi,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Graphics_DirectX_Direct3D11"`*
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn CreateFromDirect3D11SurfaceWithDpiAndAlpha<P0, E0, P1, E1>(
        resourcecreator: P0,
        surface: P1,
        dpi: f32,
        alpha: CanvasAlphaMode,
    ) -> ::windows::core::Result<CanvasRenderTarget>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<
                ::windows::Graphics::DirectX::Direct3D11::IDirect3DSurface,
            >,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasRenderTargetStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateFromDirect3D11SurfaceWithDpiAndAlpha)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    surface.try_into().map_err(|e| e.into())?.abi(),
                    dpi,
                    alpha,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    pub fn Device(&self) -> ::windows::core::Result<CanvasDevice> {
        let this = &::windows::core::Interface::cast::<ICanvasResourceCreator>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Device)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn Dpi(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<
            ICanvasResourceCreatorWithDpi,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Dpi)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn ConvertPixelsToDips(&self, pixels: i32) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<
            ICanvasResourceCreatorWithDpi,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ConvertPixelsToDips)(
                    ::windows::core::Vtable::as_raw(this),
                    pixels,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn ConvertDipsToPixels(
        &self,
        dips: f32,
        dpirounding: CanvasDpiRounding,
    ) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<
            ICanvasResourceCreatorWithDpi,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ConvertDipsToPixels)(
                    ::windows::core::Vtable::as_raw(this),
                    dips,
                    dpirounding,
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
    ///*Required features: `"UI_Xaml"`*
    #[cfg(feature = "UI_Xaml")]
    pub fn GetValue(
        &self,
        dp: &::windows::UI::Xaml::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<
            ::windows::UI::Xaml::IDependencyObject,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetValue)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(dp),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"UI_Xaml"`*
    #[cfg(feature = "UI_Xaml")]
    pub fn SetValue<P0>(
        &self,
        dp: &::windows::UI::Xaml::DependencyProperty,
        value: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<::windows::core::IInspectable>,
        >,
    {
        let this = &::windows::core::Interface::cast::<
            ::windows::UI::Xaml::IDependencyObject,
        >(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetValue)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(dp),
                    value.into().abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"UI_Xaml"`*
    #[cfg(feature = "UI_Xaml")]
    pub fn ClearValue(
        &self,
        dp: &::windows::UI::Xaml::DependencyProperty,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<
            ::windows::UI::Xaml::IDependencyObject,
        >(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .ClearValue)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(dp),
                )
                .ok()
        }
    }
    ///*Required features: `"UI_Xaml"`*
    #[cfg(feature = "UI_Xaml")]
    pub fn ReadLocalValue(
        &self,
        dp: &::windows::UI::Xaml::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<
            ::windows::UI::Xaml::IDependencyObject,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ReadLocalValue)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(dp),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"UI_Xaml"`*
    #[cfg(feature = "UI_Xaml")]
    pub fn GetAnimationBaseValue(
        &self,
        dp: &::windows::UI::Xaml::DependencyProperty,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<
            ::windows::UI::Xaml::IDependencyObject,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetAnimationBaseValue)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(dp),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"UI_Core"`, `"UI_Xaml"`*
    #[cfg(all(feature = "UI_Core", feature = "UI_Xaml"))]
    pub fn Dispatcher(
        &self,
    ) -> ::windows::core::Result<::windows::UI::Core::CoreDispatcher> {
        let this = &::windows::core::Interface::cast::<
            ::windows::UI::Xaml::IDependencyObject,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Dispatcher)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"UI_Xaml"`*
    #[cfg(feature = "UI_Xaml")]
    pub fn RegisterPropertyChangedCallback(
        &self,
        dp: &::windows::UI::Xaml::DependencyProperty,
        callback: &::windows::UI::Xaml::DependencyPropertyChangedCallback,
    ) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<
            ::windows::UI::Xaml::IDependencyObject2,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .RegisterPropertyChangedCallback)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(dp),
                    ::core::mem::transmute_copy(callback),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"UI_Xaml"`*
    #[cfg(feature = "UI_Xaml")]
    pub fn UnregisterPropertyChangedCallback(
        &self,
        dp: &::windows::UI::Xaml::DependencyProperty,
        token: i64,
    ) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<
            ::windows::UI::Xaml::IDependencyObject2,
        >(self)?;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .UnregisterPropertyChangedCallback)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(dp),
                    token,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_DirectX_Direct3D11"`*
    #[cfg(feature = "Graphics_DirectX_Direct3D11")]
    pub fn Description(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Graphics::DirectX::Direct3D11::Direct3DSurfaceDescription,
    > {
        let this = &::windows::core::Interface::cast::<
            ::windows::Graphics::DirectX::Direct3D11::IDirect3DSurface,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Description)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn ICanvasRenderTargetFactory<
        R,
        F: FnOnce(&ICanvasRenderTargetFactory) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasRenderTarget,
            ICanvasRenderTargetFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICanvasRenderTargetStatics<
        R,
        F: FnOnce(&ICanvasRenderTargetStatics) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasRenderTarget,
            ICanvasRenderTargetStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CanvasRenderTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasRenderTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasRenderTarget {}
impl ::core::fmt::Debug for CanvasRenderTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasRenderTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasRenderTarget {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.CanvasRenderTarget;{2d4c7349-9a32-41b9-b3cc-caf1b7e1099b})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasRenderTarget {
    type Vtable = ICanvasRenderTarget_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasRenderTarget {
    const IID: ::windows::core::GUID = <ICanvasRenderTarget as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasRenderTarget {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.CanvasRenderTarget";
}
::windows::core::interface_hierarchy!(
    CanvasRenderTarget,::windows::core::IUnknown,::windows::core::IInspectable
);
impl ::core::convert::TryFrom<CanvasRenderTarget> for ICanvasImage {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasRenderTarget) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CanvasRenderTarget> for ICanvasImage {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasRenderTarget) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&CanvasRenderTarget>
for ::windows::core::InParam<ICanvasImage> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasRenderTarget) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<CanvasRenderTarget> for ICanvasResourceCreator {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasRenderTarget) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CanvasRenderTarget> for ICanvasResourceCreator {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasRenderTarget) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&CanvasRenderTarget>
for ::windows::core::InParam<ICanvasResourceCreator> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasRenderTarget) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<CanvasRenderTarget> for ICanvasResourceCreatorWithDpi {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasRenderTarget) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CanvasRenderTarget> for ICanvasResourceCreatorWithDpi {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasRenderTarget) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&CanvasRenderTarget>
for ::windows::core::InParam<ICanvasResourceCreatorWithDpi> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasRenderTarget) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasRenderTarget> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasRenderTarget) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasRenderTarget> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasRenderTarget) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasRenderTarget>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasRenderTarget) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Graphics_DirectX_Direct3D11")]
impl ::core::convert::TryFrom<CanvasRenderTarget>
for ::windows::Graphics::DirectX::Direct3D11::IDirect3DSurface {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasRenderTarget) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Graphics_DirectX_Direct3D11")]
impl ::core::convert::TryFrom<&CanvasRenderTarget>
for ::windows::Graphics::DirectX::Direct3D11::IDirect3DSurface {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasRenderTarget) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Graphics_DirectX_Direct3D11")]
impl ::core::convert::TryFrom<&CanvasRenderTarget>
for ::windows::core::InParam<
    ::windows::Graphics::DirectX::Direct3D11::IDirect3DSurface,
> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasRenderTarget) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Graphics_Effects")]
impl ::core::convert::TryFrom<CanvasRenderTarget>
for ::windows::Graphics::Effects::IGraphicsEffectSource {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasRenderTarget) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Graphics_Effects")]
impl ::core::convert::TryFrom<&CanvasRenderTarget>
for ::windows::Graphics::Effects::IGraphicsEffectSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasRenderTarget) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Graphics_Effects")]
impl ::core::convert::TryFrom<&CanvasRenderTarget>
for ::windows::core::InParam<::windows::Graphics::Effects::IGraphicsEffectSource> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasRenderTarget) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::From<CanvasRenderTarget> for CanvasBitmap {
    fn from(value: CanvasRenderTarget) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CanvasRenderTarget> for CanvasBitmap {
    fn from(value: &CanvasRenderTarget) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl ::core::convert::From<&CanvasRenderTarget>
for ::windows::core::InParam<CanvasBitmap> {
    fn from(value: &CanvasRenderTarget) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
#[cfg(feature = "UI_Xaml")]
impl ::core::convert::From<CanvasRenderTarget>
for ::windows::UI::Xaml::DependencyObject {
    fn from(value: CanvasRenderTarget) -> Self {
        ::core::convert::From::from(&value)
    }
}
#[cfg(feature = "UI_Xaml")]
impl ::core::convert::From<&CanvasRenderTarget>
for ::windows::UI::Xaml::DependencyObject {
    fn from(value: &CanvasRenderTarget) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
#[cfg(feature = "UI_Xaml")]
impl ::core::convert::From<&CanvasRenderTarget>
for ::windows::core::InParam<::windows::UI::Xaml::DependencyObject> {
    fn from(value: &CanvasRenderTarget) -> Self {
        ::windows::core::InParam::owned(value.into())
    }
}
unsafe impl ::core::marker::Send for CanvasRenderTarget {}
unsafe impl ::core::marker::Sync for CanvasRenderTarget {}
///*Required features: `"Graphics_Canvas"`*
#[repr(transparent)]
pub struct CanvasSpriteBatch(::windows::core::IUnknown);
impl CanvasSpriteBatch {
    pub fn Device(&self) -> ::windows::core::Result<CanvasDevice> {
        let this = &::windows::core::Interface::cast::<ICanvasResourceCreator>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Device)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn Dpi(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<
            ICanvasResourceCreatorWithDpi,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Dpi)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn ConvertPixelsToDips(&self, pixels: i32) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<
            ICanvasResourceCreatorWithDpi,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ConvertPixelsToDips)(
                    ::windows::core::Vtable::as_raw(this),
                    pixels,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn ConvertDipsToPixels(
        &self,
        dips: f32,
        dpirounding: CanvasDpiRounding,
    ) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<
            ICanvasResourceCreatorWithDpi,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ConvertDipsToPixels)(
                    ::windows::core::Vtable::as_raw(this),
                    dips,
                    dpirounding,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn DrawToRect<P0>(
        &self,
        bitmap: P0,
        destrect: ::windows::Foundation::Rect,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<CanvasBitmap>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawToRect)(
                    ::windows::core::Vtable::as_raw(this),
                    bitmap.into().abi(),
                    destrect,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DrawAtOffset<P0>(
        &self,
        bitmap: P0,
        offset: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<CanvasBitmap>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawAtOffset)(
                    ::windows::core::Vtable::as_raw(this),
                    bitmap.into().abi(),
                    offset,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DrawWithTransform<P0>(
        &self,
        bitmap: P0,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<CanvasBitmap>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawWithTransform)(
                    ::windows::core::Vtable::as_raw(this),
                    bitmap.into().abi(),
                    transform,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DrawToRectWithTint<P0>(
        &self,
        bitmap: P0,
        destrect: ::windows::Foundation::Rect,
        tint: ::windows::Foundation::Numerics::Vector4,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<CanvasBitmap>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawToRectWithTint)(
                    ::windows::core::Vtable::as_raw(this),
                    bitmap.into().abi(),
                    destrect,
                    tint,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DrawAtOffsetWithTint<P0>(
        &self,
        bitmap: P0,
        offset: ::windows::Foundation::Numerics::Vector2,
        tint: ::windows::Foundation::Numerics::Vector4,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<CanvasBitmap>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawAtOffsetWithTint)(
                    ::windows::core::Vtable::as_raw(this),
                    bitmap.into().abi(),
                    offset,
                    tint,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DrawWithTransformAndTint<P0>(
        &self,
        bitmap: P0,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        tint: ::windows::Foundation::Numerics::Vector4,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<CanvasBitmap>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawWithTransformAndTint)(
                    ::windows::core::Vtable::as_raw(this),
                    bitmap.into().abi(),
                    transform,
                    tint,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DrawToRectWithTintAndFlip<P0>(
        &self,
        bitmap: P0,
        destrect: ::windows::Foundation::Rect,
        tint: ::windows::Foundation::Numerics::Vector4,
        flip: CanvasSpriteFlip,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<CanvasBitmap>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawToRectWithTintAndFlip)(
                    ::windows::core::Vtable::as_raw(this),
                    bitmap.into().abi(),
                    destrect,
                    tint,
                    flip,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DrawWithTransformAndTintAndFlip<P0>(
        &self,
        bitmap: P0,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        tint: ::windows::Foundation::Numerics::Vector4,
        flip: CanvasSpriteFlip,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<CanvasBitmap>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawWithTransformAndTintAndFlip)(
                    ::windows::core::Vtable::as_raw(this),
                    bitmap.into().abi(),
                    transform,
                    tint,
                    flip,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DrawAtOffsetWithTintAndTransform<P0>(
        &self,
        bitmap: P0,
        offset: ::windows::Foundation::Numerics::Vector2,
        tint: ::windows::Foundation::Numerics::Vector4,
        origin: ::windows::Foundation::Numerics::Vector2,
        rotation: f32,
        scale: ::windows::Foundation::Numerics::Vector2,
        flip: CanvasSpriteFlip,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<CanvasBitmap>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawAtOffsetWithTintAndTransform)(
                    ::windows::core::Vtable::as_raw(this),
                    bitmap.into().abi(),
                    offset,
                    tint,
                    origin,
                    rotation,
                    scale,
                    flip,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn DrawFromSpriteSheetToRect<P0>(
        &self,
        bitmap: P0,
        destrect: ::windows::Foundation::Rect,
        sourcerect: ::windows::Foundation::Rect,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<CanvasBitmap>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawFromSpriteSheetToRect)(
                    ::windows::core::Vtable::as_raw(this),
                    bitmap.into().abi(),
                    destrect,
                    sourcerect,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DrawFromSpriteSheetAtOffset<P0>(
        &self,
        bitmap: P0,
        offset: ::windows::Foundation::Numerics::Vector2,
        sourcerect: ::windows::Foundation::Rect,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<CanvasBitmap>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawFromSpriteSheetAtOffset)(
                    ::windows::core::Vtable::as_raw(this),
                    bitmap.into().abi(),
                    offset,
                    sourcerect,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DrawFromSpriteSheetWithTransform<P0>(
        &self,
        bitmap: P0,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        sourcerect: ::windows::Foundation::Rect,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<CanvasBitmap>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawFromSpriteSheetWithTransform)(
                    ::windows::core::Vtable::as_raw(this),
                    bitmap.into().abi(),
                    transform,
                    sourcerect,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DrawFromSpriteSheetToRectWithTint<P0>(
        &self,
        bitmap: P0,
        destrect: ::windows::Foundation::Rect,
        sourcerect: ::windows::Foundation::Rect,
        tint: ::windows::Foundation::Numerics::Vector4,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<CanvasBitmap>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawFromSpriteSheetToRectWithTint)(
                    ::windows::core::Vtable::as_raw(this),
                    bitmap.into().abi(),
                    destrect,
                    sourcerect,
                    tint,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DrawFromSpriteSheetAtOffsetWithTint<P0>(
        &self,
        bitmap: P0,
        offset: ::windows::Foundation::Numerics::Vector2,
        sourcerect: ::windows::Foundation::Rect,
        tint: ::windows::Foundation::Numerics::Vector4,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<CanvasBitmap>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawFromSpriteSheetAtOffsetWithTint)(
                    ::windows::core::Vtable::as_raw(this),
                    bitmap.into().abi(),
                    offset,
                    sourcerect,
                    tint,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DrawFromSpriteSheetWithTransformAndTint<P0>(
        &self,
        bitmap: P0,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        sourcerect: ::windows::Foundation::Rect,
        tint: ::windows::Foundation::Numerics::Vector4,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<CanvasBitmap>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawFromSpriteSheetWithTransformAndTint)(
                    ::windows::core::Vtable::as_raw(this),
                    bitmap.into().abi(),
                    transform,
                    sourcerect,
                    tint,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DrawFromSpriteSheetToRectWithTintAndFlip<P0>(
        &self,
        bitmap: P0,
        destrect: ::windows::Foundation::Rect,
        sourcerect: ::windows::Foundation::Rect,
        tint: ::windows::Foundation::Numerics::Vector4,
        flip: CanvasSpriteFlip,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<CanvasBitmap>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawFromSpriteSheetToRectWithTintAndFlip)(
                    ::windows::core::Vtable::as_raw(this),
                    bitmap.into().abi(),
                    destrect,
                    sourcerect,
                    tint,
                    flip,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DrawFromSpriteSheetWithTransformAndTintAndFlip<P0>(
        &self,
        bitmap: P0,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        sourcerect: ::windows::Foundation::Rect,
        tint: ::windows::Foundation::Numerics::Vector4,
        flip: CanvasSpriteFlip,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<CanvasBitmap>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawFromSpriteSheetWithTransformAndTintAndFlip)(
                    ::windows::core::Vtable::as_raw(this),
                    bitmap.into().abi(),
                    transform,
                    sourcerect,
                    tint,
                    flip,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DrawFromSpriteSheetAtOffsetWithTintAndTransform<P0>(
        &self,
        bitmap: P0,
        offset: ::windows::Foundation::Numerics::Vector2,
        sourcerect: ::windows::Foundation::Rect,
        tint: ::windows::Foundation::Numerics::Vector4,
        origin: ::windows::Foundation::Numerics::Vector2,
        rotation: f32,
        scale: ::windows::Foundation::Numerics::Vector2,
        flip: CanvasSpriteFlip,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<CanvasBitmap>>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawFromSpriteSheetAtOffsetWithTintAndTransform)(
                    ::windows::core::Vtable::as_raw(this),
                    bitmap.into().abi(),
                    offset,
                    sourcerect,
                    tint,
                    origin,
                    rotation,
                    scale,
                    flip,
                )
                .ok()
        }
    }
    pub fn IsSupported(device: &CanvasDevice) -> ::windows::core::Result<bool> {
        Self::ICanvasSpriteBatchStatics(|this| unsafe {
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
    pub fn ICanvasSpriteBatchStatics<
        R,
        F: FnOnce(&ICanvasSpriteBatchStatics) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasSpriteBatch,
            ICanvasSpriteBatchStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CanvasSpriteBatch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasSpriteBatch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasSpriteBatch {}
impl ::core::fmt::Debug for CanvasSpriteBatch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasSpriteBatch").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasSpriteBatch {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.CanvasSpriteBatch;{a065dce4-a7f2-4df4-8405-ea9e3a215bf8})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasSpriteBatch {
    type Vtable = ICanvasSpriteBatch_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasSpriteBatch {
    const IID: ::windows::core::GUID = <ICanvasSpriteBatch as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasSpriteBatch {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.CanvasSpriteBatch";
}
::windows::core::interface_hierarchy!(
    CanvasSpriteBatch,::windows::core::IUnknown,::windows::core::IInspectable
);
impl ::core::convert::TryFrom<CanvasSpriteBatch> for ICanvasResourceCreator {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasSpriteBatch) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CanvasSpriteBatch> for ICanvasResourceCreator {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSpriteBatch) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&CanvasSpriteBatch>
for ::windows::core::InParam<ICanvasResourceCreator> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSpriteBatch) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<CanvasSpriteBatch> for ICanvasResourceCreatorWithDpi {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasSpriteBatch) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CanvasSpriteBatch> for ICanvasResourceCreatorWithDpi {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSpriteBatch) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&CanvasSpriteBatch>
for ::windows::core::InParam<ICanvasResourceCreatorWithDpi> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSpriteBatch) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasSpriteBatch> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasSpriteBatch) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasSpriteBatch> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSpriteBatch) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasSpriteBatch>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSpriteBatch) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CanvasSpriteBatch {}
unsafe impl ::core::marker::Sync for CanvasSpriteBatch {}
///*Required features: `"Graphics_Canvas"`*
#[repr(transparent)]
pub struct CanvasSwapChain(::windows::core::IUnknown);
impl CanvasSwapChain {
    pub fn Device(&self) -> ::windows::core::Result<CanvasDevice> {
        let this = &::windows::core::Interface::cast::<ICanvasResourceCreator>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Device)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn Dpi(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<
            ICanvasResourceCreatorWithDpi,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Dpi)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn ConvertPixelsToDips(&self, pixels: i32) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<
            ICanvasResourceCreatorWithDpi,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ConvertPixelsToDips)(
                    ::windows::core::Vtable::as_raw(this),
                    pixels,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn ConvertDipsToPixels(
        &self,
        dips: f32,
        dpirounding: CanvasDpiRounding,
    ) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<
            ICanvasResourceCreatorWithDpi,
        >(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ConvertDipsToPixels)(
                    ::windows::core::Vtable::as_raw(this),
                    dips,
                    dpirounding,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn Present(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .Present)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    pub fn PresentWithSyncInterval(
        &self,
        syncinterval: i32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .PresentWithSyncInterval)(
                    ::windows::core::Vtable::as_raw(this),
                    syncinterval,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn ResizeBuffersWithSize(
        &self,
        newsize: ::windows::Foundation::Size,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .ResizeBuffersWithSize)(::windows::core::Vtable::as_raw(this), newsize)
                .ok()
        }
    }
    pub fn ResizeBuffersWithWidthAndHeight(
        &self,
        newwidth: f32,
        newheight: f32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .ResizeBuffersWithWidthAndHeight)(
                    ::windows::core::Vtable::as_raw(this),
                    newwidth,
                    newheight,
                )
                .ok()
        }
    }
    pub fn ResizeBuffersWithWidthAndHeightAndDpi(
        &self,
        newwidth: f32,
        newheight: f32,
        newdpi: f32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .ResizeBuffersWithWidthAndHeightAndDpi)(
                    ::windows::core::Vtable::as_raw(this),
                    newwidth,
                    newheight,
                    newdpi,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_DirectX"`*
    #[cfg(feature = "Graphics_DirectX")]
    pub fn ResizeBuffersWithAllOptions(
        &self,
        newwidth: f32,
        newheight: f32,
        newdpi: f32,
        newformat: ::windows::Graphics::DirectX::DirectXPixelFormat,
        buffercount: i32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .ResizeBuffersWithAllOptions)(
                    ::windows::core::Vtable::as_raw(this),
                    newwidth,
                    newheight,
                    newdpi,
                    newformat,
                    buffercount,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn Size(&self) -> ::windows::core::Result<::windows::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"Graphics_Imaging"`*
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SizeInPixels(
        &self,
    ) -> ::windows::core::Result<::windows::Graphics::Imaging::BitmapSize> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .SizeInPixels)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Graphics_DirectX"`*
    #[cfg(feature = "Graphics_DirectX")]
    pub fn Format(
        &self,
    ) -> ::windows::core::Result<::windows::Graphics::DirectX::DirectXPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Format)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn BufferCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .BufferCount)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn AlphaMode(&self) -> ::windows::core::Result<CanvasAlphaMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .AlphaMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn Rotation(&self) -> ::windows::core::Result<CanvasSwapChainRotation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Rotation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn SetRotation(
        &self,
        value: CanvasSwapChainRotation,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetRotation)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn SourceSize(&self) -> ::windows::core::Result<::windows::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .SourceSize)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn SetSourceSize(
        &self,
        value: ::windows::Foundation::Size,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetSourceSize)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TransformMatrix(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Matrix3x2> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .TransformMatrix)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetTransformMatrix(
        &self,
        value: ::windows::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetTransformMatrix)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn CreateDrawingSession(
        &self,
        clearcolor: ::windows::UI::Color,
    ) -> ::windows::core::Result<CanvasDrawingSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateDrawingSession)(
                    ::windows::core::Vtable::as_raw(this),
                    clearcolor,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn WaitForVerticalBlank(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .WaitForVerticalBlank)(::windows::core::Vtable::as_raw(this))
                .ok()
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn CreateWithSize<P0, E0>(
        resourcecreator: P0,
        size: ::windows::Foundation::Size,
    ) -> ::windows::core::Result<CanvasSwapChain>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreatorWithDpi>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasSwapChainFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateWithSize)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    size,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    pub fn CreateWithWidthAndHeight<P0, E0>(
        resourcecreator: P0,
        width: f32,
        height: f32,
    ) -> ::windows::core::Result<CanvasSwapChain>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreatorWithDpi>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasSwapChainFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateWithWidthAndHeight)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    width,
                    height,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    pub fn CreateWithWidthAndHeightAndDpi<P0, E0>(
        resourcecreator: P0,
        width: f32,
        height: f32,
        dpi: f32,
    ) -> ::windows::core::Result<CanvasSwapChain>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasSwapChainFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateWithWidthAndHeightAndDpi)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    width,
                    height,
                    dpi,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Graphics_DirectX"`*
    #[cfg(feature = "Graphics_DirectX")]
    pub fn CreateWithAllOptions<P0, E0>(
        resourcecreator: P0,
        width: f32,
        height: f32,
        dpi: f32,
        format: ::windows::Graphics::DirectX::DirectXPixelFormat,
        buffercount: i32,
        alphamode: CanvasAlphaMode,
    ) -> ::windows::core::Result<CanvasSwapChain>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasSwapChainFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateWithAllOptions)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    width,
                    height,
                    dpi,
                    format,
                    buffercount,
                    alphamode,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"UI_Core"`*
    #[cfg(feature = "UI_Core")]
    pub fn CreateForCoreWindowWithDpi<P0, E0>(
        resourcecreator: P0,
        corewindow: &::windows::UI::Core::CoreWindow,
        dpi: f32,
    ) -> ::windows::core::Result<CanvasSwapChain>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasSwapChainStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateForCoreWindowWithDpi)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    ::core::mem::transmute_copy(corewindow),
                    dpi,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Graphics_DirectX"`, `"UI_Core"`*
    #[cfg(all(feature = "Graphics_DirectX", feature = "UI_Core"))]
    pub fn CreateForCoreWindowWithAllOptions<P0, E0>(
        resourcecreator: P0,
        corewindow: &::windows::UI::Core::CoreWindow,
        width: f32,
        height: f32,
        dpi: f32,
        format: ::windows::Graphics::DirectX::DirectXPixelFormat,
        buffercount: i32,
    ) -> ::windows::core::Result<CanvasSwapChain>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasSwapChainStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateForCoreWindowWithAllOptions)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    ::core::mem::transmute_copy(corewindow),
                    width,
                    height,
                    dpi,
                    format,
                    buffercount,
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
    pub fn ICanvasSwapChainFactory<
        R,
        F: FnOnce(&ICanvasSwapChainFactory) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasSwapChain,
            ICanvasSwapChainFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICanvasSwapChainStatics<
        R,
        F: FnOnce(&ICanvasSwapChainStatics) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasSwapChain,
            ICanvasSwapChainStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CanvasSwapChain {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasSwapChain {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasSwapChain {}
impl ::core::fmt::Debug for CanvasSwapChain {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasSwapChain").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasSwapChain {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.CanvasSwapChain;{882e3c3a-5725-409c-9e76-f80b3bacf1b4})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasSwapChain {
    type Vtable = ICanvasSwapChain_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasSwapChain {
    const IID: ::windows::core::GUID = <ICanvasSwapChain as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasSwapChain {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.CanvasSwapChain";
}
::windows::core::interface_hierarchy!(
    CanvasSwapChain,::windows::core::IUnknown,::windows::core::IInspectable
);
impl ::core::convert::TryFrom<CanvasSwapChain> for ICanvasResourceCreator {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasSwapChain) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CanvasSwapChain> for ICanvasResourceCreator {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSwapChain) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&CanvasSwapChain>
for ::windows::core::InParam<ICanvasResourceCreator> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSwapChain) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<CanvasSwapChain> for ICanvasResourceCreatorWithDpi {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasSwapChain) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CanvasSwapChain> for ICanvasResourceCreatorWithDpi {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSwapChain) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&CanvasSwapChain>
for ::windows::core::InParam<ICanvasResourceCreatorWithDpi> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSwapChain) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasSwapChain> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasSwapChain) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasSwapChain> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSwapChain) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasSwapChain>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasSwapChain) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CanvasSwapChain {}
unsafe impl ::core::marker::Sync for CanvasSwapChain {}
///*Required features: `"Graphics_Canvas"`*
#[repr(transparent)]
pub struct CanvasVirtualBitmap(::windows::core::IUnknown);
impl CanvasVirtualBitmap {
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn GetBounds<P0, E0>(
        &self,
        resourcecreator: P0,
    ) -> ::windows::core::Result<::windows::Foundation::Rect>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<ICanvasImage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetBounds)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn GetBoundsWithTransform<P0, E0>(
        &self,
        resourcecreator: P0,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::core::Result<::windows::Foundation::Rect>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<ICanvasImage>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetBoundsWithTransform)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    transform,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn Device(&self) -> ::windows::core::Result<CanvasDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Device)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn IsCachedOnDemand(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .IsCachedOnDemand)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Graphics_Imaging"`*
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SizeInPixels(
        &self,
    ) -> ::windows::core::Result<::windows::Graphics::Imaging::BitmapSize> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .SizeInPixels)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn Size(&self) -> ::windows::core::Result<::windows::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn Bounds(&self) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Bounds)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn LoadAsyncFromFileName<P0, E0>(
        resourcecreator: P0,
        filename: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<
        ::windows::Foundation::IAsyncOperation::<CanvasVirtualBitmap>,
    >
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasVirtualBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LoadAsyncFromFileName)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    ::core::mem::transmute_copy(filename),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn LoadAsyncFromFileNameWithOptions<P0, E0>(
        resourcecreator: P0,
        filename: &::windows::core::HSTRING,
        options: CanvasVirtualBitmapOptions,
    ) -> ::windows::core::Result<
        ::windows::Foundation::IAsyncOperation::<CanvasVirtualBitmap>,
    >
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasVirtualBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LoadAsyncFromFileNameWithOptions)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    ::core::mem::transmute_copy(filename),
                    options,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn LoadAsyncFromFileNameWithOptionsAndAlpha<P0, E0>(
        resourcecreator: P0,
        filename: &::windows::core::HSTRING,
        options: CanvasVirtualBitmapOptions,
        alpha: CanvasAlphaMode,
    ) -> ::windows::core::Result<
        ::windows::Foundation::IAsyncOperation::<CanvasVirtualBitmap>,
    >
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasVirtualBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LoadAsyncFromFileNameWithOptionsAndAlpha)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    ::core::mem::transmute_copy(filename),
                    options,
                    alpha,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn LoadAsyncFromUri<P0, E0>(
        resourcecreator: P0,
        uri: &::windows::Foundation::Uri,
    ) -> ::windows::core::Result<
        ::windows::Foundation::IAsyncOperation::<CanvasVirtualBitmap>,
    >
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasVirtualBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LoadAsyncFromUri)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    ::core::mem::transmute_copy(uri),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn LoadAsyncFromUriWithOptions<P0, E0>(
        resourcecreator: P0,
        uri: &::windows::Foundation::Uri,
        options: CanvasVirtualBitmapOptions,
    ) -> ::windows::core::Result<
        ::windows::Foundation::IAsyncOperation::<CanvasVirtualBitmap>,
    >
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasVirtualBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LoadAsyncFromUriWithOptions)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    ::core::mem::transmute_copy(uri),
                    options,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn LoadAsyncFromUriWithOptionsAndAlpha<P0, E0>(
        resourcecreator: P0,
        uri: &::windows::Foundation::Uri,
        options: CanvasVirtualBitmapOptions,
        alpha: CanvasAlphaMode,
    ) -> ::windows::core::Result<
        ::windows::Foundation::IAsyncOperation::<CanvasVirtualBitmap>,
    >
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasVirtualBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LoadAsyncFromUriWithOptionsAndAlpha)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    ::core::mem::transmute_copy(uri),
                    options,
                    alpha,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation"`, `"Storage_Streams"`*
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn LoadAsyncFromStream<P0, E0, P1, E1>(
        resourcecreator: P0,
        stream: P1,
    ) -> ::windows::core::Result<
        ::windows::Foundation::IAsyncOperation::<CanvasVirtualBitmap>,
    >
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<::windows::Storage::Streams::IRandomAccessStream>,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasVirtualBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LoadAsyncFromStream)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    stream.try_into().map_err(|e| e.into())?.abi(),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation"`, `"Storage_Streams"`*
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn LoadAsyncFromStreamWithOptions<P0, E0, P1, E1>(
        resourcecreator: P0,
        stream: P1,
        options: CanvasVirtualBitmapOptions,
    ) -> ::windows::core::Result<
        ::windows::Foundation::IAsyncOperation::<CanvasVirtualBitmap>,
    >
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<::windows::Storage::Streams::IRandomAccessStream>,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasVirtualBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LoadAsyncFromStreamWithOptions)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    stream.try_into().map_err(|e| e.into())?.abi(),
                    options,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation"`, `"Storage_Streams"`*
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn LoadAsyncFromStreamWithOptionsAndAlpha<P0, E0, P1, E1>(
        resourcecreator: P0,
        stream: P1,
        options: CanvasVirtualBitmapOptions,
        alpha: CanvasAlphaMode,
    ) -> ::windows::core::Result<
        ::windows::Foundation::IAsyncOperation::<CanvasVirtualBitmap>,
    >
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<::windows::Storage::Streams::IRandomAccessStream>,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasVirtualBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LoadAsyncFromStreamWithOptionsAndAlpha)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    stream.try_into().map_err(|e| e.into())?.abi(),
                    options,
                    alpha,
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
    pub fn ICanvasVirtualBitmapStatics<
        R,
        F: FnOnce(&ICanvasVirtualBitmapStatics) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasVirtualBitmap,
            ICanvasVirtualBitmapStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CanvasVirtualBitmap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasVirtualBitmap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasVirtualBitmap {}
impl ::core::fmt::Debug for CanvasVirtualBitmap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasVirtualBitmap").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasVirtualBitmap {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.CanvasVirtualBitmap;{707d8bb0-05f9-484c-9ee2-179e0681c8a7})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasVirtualBitmap {
    type Vtable = ICanvasVirtualBitmap_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasVirtualBitmap {
    const IID: ::windows::core::GUID = <ICanvasVirtualBitmap as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasVirtualBitmap {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.CanvasVirtualBitmap";
}
::windows::core::interface_hierarchy!(
    CanvasVirtualBitmap,::windows::core::IUnknown,::windows::core::IInspectable
);
impl ::core::convert::TryFrom<CanvasVirtualBitmap> for ICanvasImage {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasVirtualBitmap) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CanvasVirtualBitmap> for ICanvasImage {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasVirtualBitmap) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&CanvasVirtualBitmap>
for ::windows::core::InParam<ICanvasImage> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasVirtualBitmap) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasVirtualBitmap> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasVirtualBitmap) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasVirtualBitmap>
for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasVirtualBitmap) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasVirtualBitmap>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasVirtualBitmap) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Graphics_Effects")]
impl ::core::convert::TryFrom<CanvasVirtualBitmap>
for ::windows::Graphics::Effects::IGraphicsEffectSource {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasVirtualBitmap) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Graphics_Effects")]
impl ::core::convert::TryFrom<&CanvasVirtualBitmap>
for ::windows::Graphics::Effects::IGraphicsEffectSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasVirtualBitmap) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Graphics_Effects")]
impl ::core::convert::TryFrom<&CanvasVirtualBitmap>
for ::windows::core::InParam<::windows::Graphics::Effects::IGraphicsEffectSource> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasVirtualBitmap) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CanvasVirtualBitmap {}
unsafe impl ::core::marker::Sync for CanvasVirtualBitmap {}
///*Required features: `"Graphics_Canvas"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasAlphaMode(pub i32);
impl CanvasAlphaMode {
    pub const Premultiplied: Self = Self(0i32);
    pub const Straight: Self = Self(1i32);
    pub const Ignore: Self = Self(2i32);
}
impl ::core::marker::Copy for CanvasAlphaMode {}
impl ::core::clone::Clone for CanvasAlphaMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasAlphaMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasAlphaMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasAlphaMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasAlphaMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasAlphaMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.CanvasAlphaMode;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasAntialiasing(pub i32);
impl CanvasAntialiasing {
    pub const Antialiased: Self = Self(0i32);
    pub const Aliased: Self = Self(1i32);
}
impl ::core::marker::Copy for CanvasAntialiasing {}
impl ::core::clone::Clone for CanvasAntialiasing {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasAntialiasing {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasAntialiasing {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasAntialiasing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasAntialiasing").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasAntialiasing {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.CanvasAntialiasing;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasBitmapFileFormat(pub i32);
impl CanvasBitmapFileFormat {
    pub const Auto: Self = Self(0i32);
    pub const Bmp: Self = Self(1i32);
    pub const Png: Self = Self(2i32);
    pub const Jpeg: Self = Self(3i32);
    pub const Tiff: Self = Self(4i32);
    pub const Gif: Self = Self(5i32);
    pub const JpegXR: Self = Self(6i32);
}
impl ::core::marker::Copy for CanvasBitmapFileFormat {}
impl ::core::clone::Clone for CanvasBitmapFileFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasBitmapFileFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasBitmapFileFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasBitmapFileFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasBitmapFileFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasBitmapFileFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.CanvasBitmapFileFormat;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasBlend(pub i32);
impl CanvasBlend {
    pub const SourceOver: Self = Self(0i32);
    pub const Copy: Self = Self(1i32);
    pub const Min: Self = Self(2i32);
    pub const Add: Self = Self(3i32);
}
impl ::core::marker::Copy for CanvasBlend {}
impl ::core::clone::Clone for CanvasBlend {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasBlend {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasBlend {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasBlend {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasBlend").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasBlend {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.CanvasBlend;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasBufferPrecision(pub i32);
impl CanvasBufferPrecision {
    pub const Precision8UIntNormalized: Self = Self(0i32);
    pub const Precision8UIntNormalizedSrgb: Self = Self(1i32);
    pub const Precision16UIntNormalized: Self = Self(2i32);
    pub const Precision16Float: Self = Self(3i32);
    pub const Precision32Float: Self = Self(4i32);
}
impl ::core::marker::Copy for CanvasBufferPrecision {}
impl ::core::clone::Clone for CanvasBufferPrecision {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasBufferPrecision {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasBufferPrecision {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasBufferPrecision {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasBufferPrecision").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasBufferPrecision {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.CanvasBufferPrecision;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasColorSpace(pub i32);
impl CanvasColorSpace {
    pub const Custom: Self = Self(0i32);
    pub const Srgb: Self = Self(1i32);
    pub const ScRgb: Self = Self(2i32);
}
impl ::core::marker::Copy for CanvasColorSpace {}
impl ::core::clone::Clone for CanvasColorSpace {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasColorSpace {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasColorSpace {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasColorSpace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasColorSpace").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasColorSpace {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.CanvasColorSpace;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasComposite(pub i32);
impl CanvasComposite {
    pub const SourceOver: Self = Self(0i32);
    pub const DestinationOver: Self = Self(1i32);
    pub const SourceIn: Self = Self(2i32);
    pub const DestinationIn: Self = Self(3i32);
    pub const SourceOut: Self = Self(4i32);
    pub const DestinationOut: Self = Self(5i32);
    pub const SourceAtop: Self = Self(6i32);
    pub const DestinationAtop: Self = Self(7i32);
    pub const Xor: Self = Self(8i32);
    pub const Add: Self = Self(9i32);
    pub const Copy: Self = Self(10i32);
    pub const BoundedCopy: Self = Self(11i32);
    pub const MaskInvert: Self = Self(12i32);
}
impl ::core::marker::Copy for CanvasComposite {}
impl ::core::clone::Clone for CanvasComposite {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasComposite {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasComposite {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasComposite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasComposite").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasComposite {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.CanvasComposite;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasDebugLevel(pub i32);
impl CanvasDebugLevel {
    pub const None: Self = Self(0i32);
    pub const Error: Self = Self(1i32);
    pub const Warning: Self = Self(2i32);
    pub const Information: Self = Self(3i32);
}
impl ::core::marker::Copy for CanvasDebugLevel {}
impl ::core::clone::Clone for CanvasDebugLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasDebugLevel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasDebugLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasDebugLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasDebugLevel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasDebugLevel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.CanvasDebugLevel;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasDpiRounding(pub i32);
impl CanvasDpiRounding {
    pub const Floor: Self = Self(0i32);
    pub const Round: Self = Self(1i32);
    pub const Ceiling: Self = Self(2i32);
}
impl ::core::marker::Copy for CanvasDpiRounding {}
impl ::core::clone::Clone for CanvasDpiRounding {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasDpiRounding {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasDpiRounding {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasDpiRounding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasDpiRounding").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasDpiRounding {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.CanvasDpiRounding;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasEdgeBehavior(pub i32);
impl CanvasEdgeBehavior {
    pub const Clamp: Self = Self(0i32);
    pub const Wrap: Self = Self(1i32);
    pub const Mirror: Self = Self(2i32);
}
impl ::core::marker::Copy for CanvasEdgeBehavior {}
impl ::core::clone::Clone for CanvasEdgeBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasEdgeBehavior {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasEdgeBehavior {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasEdgeBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasEdgeBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasEdgeBehavior {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.CanvasEdgeBehavior;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasImageInterpolation(pub i32);
impl CanvasImageInterpolation {
    pub const NearestNeighbor: Self = Self(0i32);
    pub const Linear: Self = Self(1i32);
    pub const Cubic: Self = Self(2i32);
    pub const MultiSampleLinear: Self = Self(3i32);
    pub const Anisotropic: Self = Self(4i32);
    pub const HighQualityCubic: Self = Self(5i32);
}
impl ::core::marker::Copy for CanvasImageInterpolation {}
impl ::core::clone::Clone for CanvasImageInterpolation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasImageInterpolation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasImageInterpolation {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasImageInterpolation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasImageInterpolation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasImageInterpolation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.CanvasImageInterpolation;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasLayerOptions(pub u32);
impl CanvasLayerOptions {
    pub const None: Self = Self(0u32);
    pub const InitializeFromBackground: Self = Self(1u32);
    pub const IgnoreAlpha: Self = Self(2u32);
}
impl ::core::marker::Copy for CanvasLayerOptions {}
impl ::core::clone::Clone for CanvasLayerOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasLayerOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasLayerOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasLayerOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasLayerOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CanvasLayerOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CanvasLayerOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CanvasLayerOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CanvasLayerOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CanvasLayerOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasLayerOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.CanvasLayerOptions;u4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasSpriteFlip(pub u32);
impl CanvasSpriteFlip {
    pub const None: Self = Self(0u32);
    pub const Horizontal: Self = Self(1u32);
    pub const Vertical: Self = Self(2u32);
    pub const Both: Self = Self(3u32);
}
impl ::core::marker::Copy for CanvasSpriteFlip {}
impl ::core::clone::Clone for CanvasSpriteFlip {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasSpriteFlip {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasSpriteFlip {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasSpriteFlip {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasSpriteFlip").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CanvasSpriteFlip {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CanvasSpriteFlip {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CanvasSpriteFlip {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CanvasSpriteFlip {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CanvasSpriteFlip {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasSpriteFlip {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.CanvasSpriteFlip;u4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasSpriteOptions(pub u32);
impl CanvasSpriteOptions {
    pub const None: Self = Self(0u32);
    pub const ClampToSourceRect: Self = Self(1u32);
}
impl ::core::marker::Copy for CanvasSpriteOptions {}
impl ::core::clone::Clone for CanvasSpriteOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasSpriteOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasSpriteOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasSpriteOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasSpriteOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CanvasSpriteOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CanvasSpriteOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CanvasSpriteOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CanvasSpriteOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CanvasSpriteOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasSpriteOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.CanvasSpriteOptions;u4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasSpriteSortMode(pub i32);
impl CanvasSpriteSortMode {
    pub const None: Self = Self(0i32);
    pub const Bitmap: Self = Self(1i32);
}
impl ::core::marker::Copy for CanvasSpriteSortMode {}
impl ::core::clone::Clone for CanvasSpriteSortMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasSpriteSortMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasSpriteSortMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasSpriteSortMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasSpriteSortMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasSpriteSortMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.CanvasSpriteSortMode;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasSwapChainRotation(pub i32);
impl CanvasSwapChainRotation {
    pub const None: Self = Self(0i32);
    pub const Rotate90: Self = Self(1i32);
    pub const Rotate180: Self = Self(2i32);
    pub const Rotate270: Self = Self(3i32);
}
impl ::core::marker::Copy for CanvasSwapChainRotation {}
impl ::core::clone::Clone for CanvasSwapChainRotation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasSwapChainRotation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasSwapChainRotation {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasSwapChainRotation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasSwapChainRotation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasSwapChainRotation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.CanvasSwapChainRotation;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasUnits(pub i32);
impl CanvasUnits {
    pub const Dips: Self = Self(0i32);
    pub const Pixels: Self = Self(1i32);
}
impl ::core::marker::Copy for CanvasUnits {}
impl ::core::clone::Clone for CanvasUnits {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasUnits {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasUnits {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasUnits {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasUnits").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasUnits {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.CanvasUnits;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasVirtualBitmapOptions(pub i32);
impl CanvasVirtualBitmapOptions {
    pub const None: Self = Self(0i32);
    pub const ReleaseSource: Self = Self(1i32);
    pub const CacheOnDemand: Self = Self(2i32);
}
impl ::core::marker::Copy for CanvasVirtualBitmapOptions {}
impl ::core::clone::Clone for CanvasVirtualBitmapOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasVirtualBitmapOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasVirtualBitmapOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasVirtualBitmapOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasVirtualBitmapOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasVirtualBitmapOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.CanvasVirtualBitmapOptions;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
