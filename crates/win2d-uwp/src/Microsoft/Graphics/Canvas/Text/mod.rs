#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasFontFace(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasFontFace {
    type Vtable = ICanvasFontFace_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasFontFace {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x5199d129_4ef9_4dee_b74c_4dc910201a7f,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasFontFace_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetRecommendedRenderingMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        fontsize: f32,
        dpi: f32,
        measuringmode: CanvasTextMeasuringMode,
        renderingparameters: *mut ::core::ffi::c_void,
        result__: *mut CanvasTextRenderingMode,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub GetRecommendedRenderingModeWithAllOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        fontsize: f32,
        dpi: f32,
        measuringmode: CanvasTextMeasuringMode,
        renderingparameters: *mut ::core::ffi::c_void,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        issideways: bool,
        outlinethreshold: super::CanvasAntialiasing,
        result__: *mut CanvasTextRenderingMode,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetRecommendedRenderingModeWithAllOptions: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub GetRecommendedGridFit: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        fontsize: f32,
        dpi: f32,
        measuringmode: CanvasTextMeasuringMode,
        renderingparameters: *mut ::core::ffi::c_void,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        issideways: bool,
        outlinethreshold: super::CanvasAntialiasing,
        result__: *mut CanvasTextGridFit,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetRecommendedGridFit: usize,
    #[cfg(feature = "Foundation")]
    pub GlyphBox: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GlyphBox: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SubscriptPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SubscriptPosition: usize,
    #[cfg(feature = "Foundation")]
    pub SubscriptSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Size,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SubscriptSize: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SuperscriptPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SuperscriptPosition: usize,
    #[cfg(feature = "Foundation")]
    pub SuperscriptSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Size,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SuperscriptSize: usize,
    pub HasTypographicMetrics: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub Ascent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub Descent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub LineGap: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub CapHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub LowercaseLetterHeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub UnderlinePosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub UnderlineThickness: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub StrikethroughPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub StrikethroughThickness: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub CaretSlopeRise: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub CaretSlopeRun: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub CaretOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub UnicodeRanges: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut CanvasUnicodeRange,
    ) -> ::windows::core::HRESULT,
    pub IsMonospaced: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub GetVerticalGlyphVariants: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        inputElements_array_size: u32,
        inputelements: *const i32,
        result_size__: *mut u32,
        result__: *mut *mut i32,
    ) -> ::windows::core::HRESULT,
    pub HasVerticalGlyphVariants: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub FileFormatType: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasFontFileFormatType,
    ) -> ::windows::core::HRESULT,
    pub Simulations: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasFontSimulations,
    ) -> ::windows::core::HRESULT,
    pub IsSymbolFont: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub GlyphCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    pub GetGlyphIndices: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        inputElements_array_size: u32,
        inputelements: *const u32,
        result_size__: *mut u32,
        result__: *mut *mut i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetGlyphMetrics: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        inputElements_array_size: u32,
        inputelements: *const i32,
        issideways: bool,
        result_size__: *mut u32,
        result__: *mut *mut CanvasGlyphMetrics,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetGlyphMetrics: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub GetGdiCompatibleGlyphMetrics: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        fontsize: f32,
        dpi: f32,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        usegdinatural: bool,
        inputElements_array_size: u32,
        inputelements: *const i32,
        issideways: bool,
        result_size__: *mut u32,
        result__: *mut *mut CanvasGlyphMetrics,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetGdiCompatibleGlyphMetrics: usize,
    #[cfg(feature = "UI_Text")]
    pub Weight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Text::FontWeight,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    Weight: usize,
    #[cfg(feature = "UI_Text")]
    pub Stretch: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    Stretch: usize,
    #[cfg(feature = "UI_Text")]
    pub Style: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    Style: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FamilyNames: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FamilyNames: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FaceNames: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FaceNames: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetInformationalStrings: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        fontinformation: CanvasFontInformation,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetInformationalStrings: usize,
    pub HasCharacter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        unicodevalue: u32,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub GetGlyphRunBounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        drawingsession: *mut ::core::ffi::c_void,
        point: ::windows::Foundation::Numerics::Vector2,
        fontsize: f32,
        glyphs_array_size: u32,
        glyphs: *const CanvasGlyph,
        issideways: bool,
        bidilevel: u32,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetGlyphRunBounds: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub GetGlyphRunBoundsWithMeasuringMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        drawingsession: *mut ::core::ffi::c_void,
        point: ::windows::Foundation::Numerics::Vector2,
        fontsize: f32,
        glyphs_array_size: u32,
        glyphs: *const CanvasGlyph,
        issideways: bool,
        bidilevel: u32,
        measuringmode: CanvasTextMeasuringMode,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetGlyphRunBoundsWithMeasuringMode: usize,
    pub Panose: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut u8,
    ) -> ::windows::core::HRESULT,
    pub GetSupportedTypographicFeatureNames: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        script: CanvasAnalyzedScript,
        result_size__: *mut u32,
        result__: *mut *mut CanvasTypographyFeatureName,
    ) -> ::windows::core::HRESULT,
    pub GetSupportedTypographicFeatureNamesWithLocale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        script: CanvasAnalyzedScript,
        locale: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut CanvasTypographyFeatureName,
    ) -> ::windows::core::HRESULT,
    pub GetTypographicFeatureGlyphSupport: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        script: CanvasAnalyzedScript,
        typographicfeaturename: CanvasTypographyFeatureName,
        glyphsElements_array_size: u32,
        glyphselements: *const CanvasGlyph,
        result_size__: *mut u32,
        result__: *mut *mut bool,
    ) -> ::windows::core::HRESULT,
    pub GetTypographicFeatureGlyphSupportWithLocale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        script: CanvasAnalyzedScript,
        typographicfeaturename: CanvasTypographyFeatureName,
        glyphsElements_array_size: u32,
        glyphselements: *const CanvasGlyph,
        locale: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut bool,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasFontSet(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasFontSet {
    type Vtable = ICanvasFontSet_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasFontSet {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x0a5bfb92_1f3c_459f_9d7e_a6289dd093c0,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasFontSet_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Fonts: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Fonts: usize,
    pub TryFindFontFace: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        fontface: *mut ::core::ffi::c_void,
        index: *mut i32,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub GetMatchingFontsFromProperties: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        propertyElements_array_size: u32,
        propertyelements: *const ::std::mem::ManuallyDrop<CanvasFontProperty>,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Text")]
    pub GetMatchingFontsFromWwsFamily: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        familyname: *mut ::core::ffi::c_void,
        weight: ::windows::UI::Text::FontWeight,
        stretch: ::windows::UI::Text::FontStretch,
        style: ::windows::UI::Text::FontStyle,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    GetMatchingFontsFromWwsFamily: usize,
    pub CountFontsMatchingProperty: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        property: ::std::mem::ManuallyDrop<CanvasFontProperty>,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPropertyValuesFromIndex: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        fontindex: u32,
        propertyidentifier: CanvasFontPropertyIdentifier,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPropertyValuesFromIndex: usize,
    pub GetPropertyValuesFromIdentifier: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        propertyidentifier: CanvasFontPropertyIdentifier,
        preferredlocalenames: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut ::std::mem::ManuallyDrop<CanvasFontProperty>,
    ) -> ::windows::core::HRESULT,
    pub GetPropertyValues: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        propertyidentifier: CanvasFontPropertyIdentifier,
        result_size__: *mut u32,
        result__: *mut *mut ::std::mem::ManuallyDrop<CanvasFontProperty>,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasFontSetFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasFontSetFactory {
    type Vtable = ICanvasFontSetFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasFontSetFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x3c9c9bda_70f9_4ff9_aab2_3b42923286ee,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasFontSetFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        uri: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasFontSetStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasFontSetStatics {
    type Vtable = ICanvasFontSetStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasFontSetStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x5f4275ce_bcfa_48c5_9e67_fbe9866d4924,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasFontSetStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetSystemFontSet: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasNumberSubstitution(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasNumberSubstitution {
    type Vtable = ICanvasNumberSubstitution_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasNumberSubstitution {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xc81a67ad_0639_4f8f_878b_d937f8a14293,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasNumberSubstitution_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasNumberSubstitutionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasNumberSubstitutionFactory {
    type Vtable = ICanvasNumberSubstitutionFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasNumberSubstitutionFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x7496a822_c781_4eb0_aafb_c078e7fa8e24,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasNumberSubstitutionFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        method: CanvasNumberSubstitutionMethod,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateWithLocaleAndIgnoreOverrides: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        method: CanvasNumberSubstitutionMethod,
        localename: *mut ::core::ffi::c_void,
        ignoreenvironmentoverrides: bool,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasScaledFont(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasScaledFont {
    type Vtable = ICanvasScaledFont_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasScaledFont {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xbbc4f8d2_eb2b_45f1_ac2a_cfc1f598bae3,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasScaledFont_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub FontFace: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub ScaleFactor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasTextAnalyzer(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasTextAnalyzer {
    type Vtable = ICanvasTextAnalyzer_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasTextAnalyzer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x4298f3d1_645b_40e3_b91b_81986d767fc0,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasTextAnalyzer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFontsUsingSystemFontSet: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        textformat: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFontsUsingSystemFontSet: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFonts: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        textformat: *mut ::core::ffi::c_void,
        fontset: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFonts: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetBidi: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetBidi: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetBidiWithLocale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        locale: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetBidiWithLocale: usize,
    pub GetBreakpoints: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut CanvasAnalyzedBreakpoint,
    ) -> ::windows::core::HRESULT,
    pub GetBreakpointsWithLocale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        locale: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut CanvasAnalyzedBreakpoint,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetNumberSubstitutions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetNumberSubstitutions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetScript: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetScript: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetScriptWithLocale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        locale: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetScriptWithLocale: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetGlyphOrientations: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetGlyphOrientations: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetGlyphOrientationsWithLocale: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        locale: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetGlyphOrientationsWithLocale: usize,
    pub GetScriptProperties: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        analyzedscript: CanvasAnalyzedScript,
        result__: *mut ::std::mem::ManuallyDrop<CanvasScriptProperties>,
    ) -> ::windows::core::HRESULT,
    pub GetGlyphs: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterrange: CanvasCharacterRange,
        fontface: *mut ::core::ffi::c_void,
        fontsize: f32,
        issideways: bool,
        isrighttoleft: bool,
        script: CanvasAnalyzedScript,
        result_size__: *mut u32,
        result__: *mut *mut CanvasGlyph,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetGlyphsWithAllOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterrange: CanvasCharacterRange,
        fontface: *mut ::core::ffi::c_void,
        fontsize: f32,
        issideways: bool,
        isrighttoleft: bool,
        script: CanvasAnalyzedScript,
        locale: *mut ::core::ffi::c_void,
        numbersubstitution: *mut ::core::ffi::c_void,
        typographyranges: *mut ::core::ffi::c_void,
        clusterMapIndicesElements_array_size: *mut u32,
        clustermapindiceselements: *mut *mut i32,
        isShapedAloneGlyphsElements_array_size: *mut u32,
        isshapedaloneglyphselements: *mut *mut bool,
        glyphShapingResultsElements_array_size: *mut u32,
        glyphshapingresultselements: *mut *mut CanvasGlyphShaping,
        result_size__: *mut u32,
        result__: *mut *mut CanvasGlyph,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetGlyphsWithAllOptions: usize,
    pub GetJustificationOpportunities: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterrange: CanvasCharacterRange,
        fontface: *mut ::core::ffi::c_void,
        fontsize: f32,
        script: CanvasAnalyzedScript,
        clusterMapIndicesElements_array_size: u32,
        clustermapindiceselements: *const i32,
        glyphShapingResultsElements_array_size: u32,
        glyphshapingresultselements: *const CanvasGlyphShaping,
        result_size__: *mut u32,
        result__: *mut *mut CanvasJustificationOpportunity,
    ) -> ::windows::core::HRESULT,
    pub ApplyJustificationOpportunities: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        linewidth: f32,
        justificationOpportunitiesElements_array_size: u32,
        justificationopportunitieselements: *const CanvasJustificationOpportunity,
        sourceGlyphsElements_array_size: u32,
        sourceglyphselements: *const CanvasGlyph,
        result_size__: *mut u32,
        result__: *mut *mut CanvasGlyph,
    ) -> ::windows::core::HRESULT,
    pub AddGlyphsAfterJustification: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        fontface: *mut ::core::ffi::c_void,
        fontsize: f32,
        script: CanvasAnalyzedScript,
        clusterMapIndicesElements_array_size: u32,
        clustermapindiceselements: *const i32,
        originalGlyphsElements_array_size: u32,
        originalglyphselements: *const CanvasGlyph,
        justifiedGlyphsElements_array_size: u32,
        justifiedglyphselements: *const CanvasGlyph,
        glyphShapingResultsElements_array_size: u32,
        glyphshapingresultselements: *const CanvasGlyphShaping,
        result_size__: *mut u32,
        result__: *mut *mut CanvasGlyph,
    ) -> ::windows::core::HRESULT,
    pub AddGlyphsAfterJustificationWithClusterMap: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        fontface: *mut ::core::ffi::c_void,
        fontsize: f32,
        script: CanvasAnalyzedScript,
        clusterMapIndicesElements_array_size: u32,
        clustermapindiceselements: *const i32,
        originalGlyphsElements_array_size: u32,
        originalglyphselements: *const CanvasGlyph,
        justifiedGlyphsElements_array_size: u32,
        justifiedglyphselements: *const CanvasGlyph,
        glyphShapingResultsElements_array_size: u32,
        glyphshapingresultselements: *const CanvasGlyphShaping,
        outputClusterMapIndicesElements_array_size: *mut u32,
        outputclustermapindiceselements: *mut *mut i32,
        result_size__: *mut u32,
        result__: *mut *mut CanvasGlyph,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasTextAnalyzerFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasTextAnalyzerFactory {
    type Vtable = ICanvasTextAnalyzerFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasTextAnalyzerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x521e433f_f698_44c0_8d7f_fe374fe539e1,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasTextAnalyzerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        text: *mut ::core::ffi::c_void,
        textdirection: CanvasTextDirection,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateWithNumberSubstitutionAndVerticalGlyphOrientationAndBidiLevel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        text: *mut ::core::ffi::c_void,
        textdirection: CanvasTextDirection,
        numbersubstitution: *mut ::core::ffi::c_void,
        verticalglyphorientation: CanvasVerticalGlyphOrientation,
        bidilevel: u32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateWithOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        text: *mut ::core::ffi::c_void,
        textdirection: CanvasTextDirection,
        options: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
pub struct ICanvasTextAnalyzerOptions(::windows::core::IUnknown);
impl ICanvasTextAnalyzerOptions {
    pub fn GetLocaleName(
        &self,
        characterindex: i32,
        charactercount: &mut i32,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetLocaleName)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    charactercount,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn GetNumberSubstitution(
        &self,
        characterindex: i32,
        charactercount: &mut i32,
    ) -> ::windows::core::Result<CanvasNumberSubstitution> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetNumberSubstitution)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    charactercount,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn GetVerticalGlyphOrientation(
        &self,
        characterindex: i32,
        charactercount: &mut i32,
    ) -> ::windows::core::Result<CanvasVerticalGlyphOrientation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetVerticalGlyphOrientation)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    charactercount,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn GetBidiLevel(
        &self,
        characterindex: i32,
        charactercount: &mut i32,
    ) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetBidiLevel)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    charactercount,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(
    ICanvasTextAnalyzerOptions, ::windows::core::IUnknown, ::windows::core::IInspectable
);
impl ::core::clone::Clone for ICanvasTextAnalyzerOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICanvasTextAnalyzerOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICanvasTextAnalyzerOptions {}
impl ::core::fmt::Debug for ICanvasTextAnalyzerOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICanvasTextAnalyzerOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICanvasTextAnalyzerOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"{31f2406a-8c5f-4e12-8bd6-cfbbc7214d02}",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ICanvasTextAnalyzerOptions {
    type Vtable = ICanvasTextAnalyzerOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasTextAnalyzerOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x31f2406a_8c5f_4e12_8bd6_cfbbc7214d02,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasTextAnalyzerOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetLocaleName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        charactercount: *mut i32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetNumberSubstitution: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        charactercount: *mut i32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetVerticalGlyphOrientation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        charactercount: *mut i32,
        result__: *mut CanvasVerticalGlyphOrientation,
    ) -> ::windows::core::HRESULT,
    pub GetBidiLevel: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        charactercount: *mut i32,
        result__: *mut u32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasTextFormat(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasTextFormat {
    type Vtable = ICanvasTextFormat_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasTextFormat {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xaf61bfdc_eabb_4d38_ba1b_afb340612d33,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasTextFormat_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Direction: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasTextDirection,
    ) -> ::windows::core::HRESULT,
    pub SetDirection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CanvasTextDirection,
    ) -> ::windows::core::HRESULT,
    pub FontFamily: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetFontFamily: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub FontSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetFontSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Text")]
    pub FontStretch: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontStretch: usize,
    #[cfg(feature = "UI_Text")]
    pub SetFontStretch: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetFontStretch: usize,
    #[cfg(feature = "UI_Text")]
    pub FontStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontStyle: usize,
    #[cfg(feature = "UI_Text")]
    pub SetFontStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetFontStyle: usize,
    #[cfg(feature = "UI_Text")]
    pub FontWeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Text::FontWeight,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    FontWeight: usize,
    #[cfg(feature = "UI_Text")]
    pub SetFontWeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::UI::Text::FontWeight,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetFontWeight: usize,
    pub IncrementalTabStop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetIncrementalTabStop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub LineSpacing: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetLineSpacing: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub LineSpacingBaseline: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetLineSpacingBaseline: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub LocaleName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetLocaleName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub VerticalAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasVerticalAlignment,
    ) -> ::windows::core::HRESULT,
    pub SetVerticalAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CanvasVerticalAlignment,
    ) -> ::windows::core::HRESULT,
    pub HorizontalAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasHorizontalAlignment,
    ) -> ::windows::core::HRESULT,
    pub SetHorizontalAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CanvasHorizontalAlignment,
    ) -> ::windows::core::HRESULT,
    pub TrimmingGranularity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasTextTrimmingGranularity,
    ) -> ::windows::core::HRESULT,
    pub SetTrimmingGranularity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CanvasTextTrimmingGranularity,
    ) -> ::windows::core::HRESULT,
    pub TrimmingDelimiter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetTrimmingDelimiter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub TrimmingDelimiterCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub SetTrimmingDelimiterCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows::core::HRESULT,
    pub WordWrapping: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasWordWrapping,
    ) -> ::windows::core::HRESULT,
    pub SetWordWrapping: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CanvasWordWrapping,
    ) -> ::windows::core::HRESULT,
    pub Options: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasDrawTextOptions,
    ) -> ::windows::core::HRESULT,
    pub SetOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CanvasDrawTextOptions,
    ) -> ::windows::core::HRESULT,
    pub VerticalGlyphOrientation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasVerticalGlyphOrientation,
    ) -> ::windows::core::HRESULT,
    pub SetVerticalGlyphOrientation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CanvasVerticalGlyphOrientation,
    ) -> ::windows::core::HRESULT,
    pub OpticalAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasOpticalAlignment,
    ) -> ::windows::core::HRESULT,
    pub SetOpticalAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CanvasOpticalAlignment,
    ) -> ::windows::core::HRESULT,
    pub LastLineWrapping: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetLastLineWrapping: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub LineSpacingMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasLineSpacingMode,
    ) -> ::windows::core::HRESULT,
    pub SetLineSpacingMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CanvasLineSpacingMode,
    ) -> ::windows::core::HRESULT,
    pub TrimmingSign: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasTrimmingSign,
    ) -> ::windows::core::HRESULT,
    pub SetTrimmingSign: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CanvasTrimmingSign,
    ) -> ::windows::core::HRESULT,
    pub CustomTrimmingSign: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetCustomTrimmingSign: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasTextFormatStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasTextFormatStatics {
    type Vtable = ICanvasTextFormatStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasTextFormatStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x8a927515_33fc_4c92_a6aa_94a8f29c140b,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasTextFormatStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetSystemFontFamilies: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSystemFontFamiliesFromLocaleList: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        localelist: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSystemFontFamiliesFromLocaleList: usize,
}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
pub struct ICanvasTextInlineObject(::windows::core::IUnknown);
impl ICanvasTextInlineObject {
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Draw<P0, E0, P1>(
        &self,
        textrenderer: P0,
        point: ::windows::Foundation::Numerics::Vector2,
        issideways: bool,
        isrighttoleft: bool,
        brush: P1,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasTextRenderer>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::Into<
            ::windows::core::InParam<::windows::core::IInspectable>,
        >,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .Draw)(
                    ::windows::core::Vtable::as_raw(this),
                    textrenderer.try_into().map_err(|e| e.into())?.abi(),
                    point,
                    issideways,
                    isrighttoleft,
                    brush.into().abi(),
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
    pub fn Baseline(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Baseline)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn SupportsSideways(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .SupportsSideways)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn DrawBounds(&self) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .DrawBounds)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn BreakBefore(&self) -> ::windows::core::Result<CanvasLineBreakCondition> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .BreakBefore)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn BreakAfter(&self) -> ::windows::core::Result<CanvasLineBreakCondition> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .BreakAfter)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(
    ICanvasTextInlineObject, ::windows::core::IUnknown, ::windows::core::IInspectable
);
impl ::core::clone::Clone for ICanvasTextInlineObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICanvasTextInlineObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICanvasTextInlineObject {}
impl ::core::fmt::Debug for ICanvasTextInlineObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICanvasTextInlineObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICanvasTextInlineObject {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"{7a89ee99-ce2a-47fa-9dd2-0a6825f6053f}",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ICanvasTextInlineObject {
    type Vtable = ICanvasTextInlineObject_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasTextInlineObject {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x7a89ee99_ce2a_47fa_9dd2_0a6825f6053f,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasTextInlineObject_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Draw: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        textrenderer: *mut ::core::ffi::c_void,
        point: ::windows::Foundation::Numerics::Vector2,
        issideways: bool,
        isrighttoleft: bool,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Draw: usize,
    #[cfg(feature = "Foundation")]
    pub Size: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Size,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Size: usize,
    pub Baseline: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SupportsSideways: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DrawBounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DrawBounds: usize,
    pub BreakBefore: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasLineBreakCondition,
    ) -> ::windows::core::HRESULT,
    pub BreakAfter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasLineBreakCondition,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasTextLayout(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasTextLayout {
    type Vtable = ICanvasTextLayout_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasTextLayout {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xbae63e54_48ae_4446_a2c7_b6ef93806c20,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasTextLayout_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetFormatChangeIndices: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut i32,
    ) -> ::windows::core::HRESULT,
    pub Direction: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasTextDirection,
    ) -> ::windows::core::HRESULT,
    pub SetDirection: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CanvasTextDirection,
    ) -> ::windows::core::HRESULT,
    pub DefaultFontFamily: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub DefaultFontSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Text")]
    pub DefaultFontStretch: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    DefaultFontStretch: usize,
    #[cfg(feature = "UI_Text")]
    pub DefaultFontStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    DefaultFontStyle: usize,
    #[cfg(feature = "UI_Text")]
    pub DefaultFontWeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::UI::Text::FontWeight,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    DefaultFontWeight: usize,
    pub IncrementalTabStop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetIncrementalTabStop: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub LineSpacing: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetLineSpacing: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub LineSpacingBaseline: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetLineSpacingBaseline: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub DefaultLocaleName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub VerticalAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasVerticalAlignment,
    ) -> ::windows::core::HRESULT,
    pub SetVerticalAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CanvasVerticalAlignment,
    ) -> ::windows::core::HRESULT,
    pub HorizontalAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasHorizontalAlignment,
    ) -> ::windows::core::HRESULT,
    pub SetHorizontalAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CanvasHorizontalAlignment,
    ) -> ::windows::core::HRESULT,
    pub TrimmingGranularity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasTextTrimmingGranularity,
    ) -> ::windows::core::HRESULT,
    pub SetTrimmingGranularity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CanvasTextTrimmingGranularity,
    ) -> ::windows::core::HRESULT,
    pub TrimmingDelimiter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetTrimmingDelimiter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub TrimmingDelimiterCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub SetTrimmingDelimiterCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: i32,
    ) -> ::windows::core::HRESULT,
    pub WordWrapping: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasWordWrapping,
    ) -> ::windows::core::HRESULT,
    pub SetWordWrapping: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CanvasWordWrapping,
    ) -> ::windows::core::HRESULT,
    pub Options: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasDrawTextOptions,
    ) -> ::windows::core::HRESULT,
    pub SetOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CanvasDrawTextOptions,
    ) -> ::windows::core::HRESULT,
    pub LineSpacingMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasLineSpacingMode,
    ) -> ::windows::core::HRESULT,
    pub SetLineSpacingMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CanvasLineSpacingMode,
    ) -> ::windows::core::HRESULT,
    pub TrimmingSign: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasTrimmingSign,
    ) -> ::windows::core::HRESULT,
    pub SetTrimmingSign: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CanvasTrimmingSign,
    ) -> ::windows::core::HRESULT,
    pub CustomTrimmingSign: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetCustomTrimmingSign: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RequestedSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Size,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestedSize: usize,
    #[cfg(feature = "Foundation")]
    pub SetRequestedSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: ::windows::Foundation::Size,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRequestedSize: usize,
    pub GetMinimumLineLength: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub GetBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Brushes"))]
    GetBrush: usize,
    pub GetCustomBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetFontFamily: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetFontSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Text")]
    pub GetFontStretch: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        result__: *mut ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    GetFontStretch: usize,
    #[cfg(feature = "UI_Text")]
    pub GetFontStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        result__: *mut ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    GetFontStyle: usize,
    #[cfg(feature = "UI_Text")]
    pub GetFontWeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        result__: *mut ::windows::UI::Text::FontWeight,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    GetFontWeight: usize,
    pub GetLocaleName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetStrikethrough: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub GetUnderline: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub GetInlineObject: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI")]
    pub SetColor: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        charactercount: i32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColor: usize,
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub SetBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        charactercount: i32,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Brushes"))]
    SetBrush: usize,
    pub SetCustomBrush: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        charactercount: i32,
        brush: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetFontFamily: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        charactercount: i32,
        fontfamily: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetFontSize: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        charactercount: i32,
        fontsize: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Text")]
    pub SetFontStretch: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        charactercount: i32,
        fontstretch: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetFontStretch: usize,
    #[cfg(feature = "UI_Text")]
    pub SetFontStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        charactercount: i32,
        fontstyle: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetFontStyle: usize,
    #[cfg(feature = "UI_Text")]
    pub SetFontWeight: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        charactercount: i32,
        fontweight: ::windows::UI::Text::FontWeight,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Text"))]
    SetFontWeight: usize,
    pub SetLocaleName: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        charactercount: i32,
        name: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub SetStrikethrough: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        charactercount: i32,
        hasstrikethrough: bool,
    ) -> ::windows::core::HRESULT,
    pub SetUnderline: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        charactercount: i32,
        hasunderline: bool,
    ) -> ::windows::core::HRESULT,
    pub SetInlineObject: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        charactercount: i32,
        inlineobject: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub DrawToTextRenderer: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        textrenderer: *mut ::core::ffi::c_void,
        position: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DrawToTextRenderer: usize,
    pub DrawToTextRendererWithCoords: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        textrenderer: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
    ) -> ::windows::core::HRESULT,
    pub LineMetrics: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut CanvasLineMetrics,
    ) -> ::windows::core::HRESULT,
    pub ClusterMetrics: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut CanvasClusterMetrics,
    ) -> ::windows::core::HRESULT,
    pub SetTypography: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        charactercount: i32,
        typography: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub GetTypography: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LayoutBounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LayoutBounds: usize,
    #[cfg(feature = "Foundation")]
    pub LayoutBoundsIncludingTrailingWhitespace: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LayoutBoundsIncludingTrailingWhitespace: usize,
    pub LineCount: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    pub MaximumBidiReorderingDepth: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut i32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DrawBounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DrawBounds: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub HitTest: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        point: ::windows::Foundation::Numerics::Vector2,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    HitTest: usize,
    pub HitTestWithCoords: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub HitTestWithDescription: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        point: ::windows::Foundation::Numerics::Vector2,
        textlayoutregion: *mut CanvasTextLayoutRegion,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    HitTestWithDescription: usize,
    #[cfg(feature = "Foundation")]
    pub HitTestWithDescriptionAndCoords: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        textlayoutregion: *mut CanvasTextLayoutRegion,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HitTestWithDescriptionAndCoords: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub HitTestWithDescriptionAndTrailingSide: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        point: ::windows::Foundation::Numerics::Vector2,
        textlayoutregion: *mut CanvasTextLayoutRegion,
        trailingsideofcharacter: *mut bool,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    HitTestWithDescriptionAndTrailingSide: usize,
    #[cfg(feature = "Foundation")]
    pub HitTestWithDescriptionAndCoordsAndTrailingSide: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        textlayoutregion: *mut CanvasTextLayoutRegion,
        trailingsideofcharacter: *mut bool,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    HitTestWithDescriptionAndCoordsAndTrailingSide: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub GetCaretPosition: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        trailingsideofcharacter: bool,
        result__: *mut ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetCaretPosition: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub GetCaretPositionWithDescription: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        trailingsideofcharacter: bool,
        textlayoutregion: *mut CanvasTextLayoutRegion,
        result__: *mut ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetCaretPositionWithDescription: usize,
    #[cfg(feature = "Foundation")]
    pub GetCharacterRegions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        charactercount: i32,
        result_size__: *mut u32,
        result__: *mut *mut CanvasTextLayoutRegion,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCharacterRegions: usize,
    pub GetPairKerning: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetPairKerning: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        charactercount: i32,
        haspairkerning: bool,
    ) -> ::windows::core::HRESULT,
    pub GetLeadingCharacterSpacing: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub GetTrailingCharacterSpacing: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub GetMinimumCharacterAdvance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetCharacterSpacing: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        characterindex: i32,
        charactercount: i32,
        leadingspacing: f32,
        trailingspacing: f32,
        minimumadvance: f32,
    ) -> ::windows::core::HRESULT,
    pub VerticalGlyphOrientation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasVerticalGlyphOrientation,
    ) -> ::windows::core::HRESULT,
    pub SetVerticalGlyphOrientation: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CanvasVerticalGlyphOrientation,
    ) -> ::windows::core::HRESULT,
    pub OpticalAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasOpticalAlignment,
    ) -> ::windows::core::HRESULT,
    pub SetOpticalAlignment: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CanvasOpticalAlignment,
    ) -> ::windows::core::HRESULT,
    pub LastLineWrapping: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    pub SetLastLineWrapping: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: bool,
    ) -> ::windows::core::HRESULT,
    pub Device: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasTextLayoutFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasTextLayoutFactory {
    type Vtable = ICanvasTextLayoutFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasTextLayoutFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x9c1f7179_acd0_4680_93d5_95a6247e8f6b,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasTextLayoutFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        textstring: *mut ::core::ffi::c_void,
        textformat: *mut ::core::ffi::c_void,
        requestedwidth: f32,
        requestedheight: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasTextLayoutStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasTextLayoutStatics {
    type Vtable = ICanvasTextLayoutStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasTextLayoutStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x7f2b8ffd_6935_4f60_b409_6394a19c5ebc,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasTextLayoutStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub GetGlyphOrientationTransform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        glyphorientation: CanvasGlyphOrientation,
        issideways: bool,
        position: ::windows::Foundation::Numerics::Vector2,
        result__: *mut ::windows::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    GetGlyphOrientationTransform: usize,
}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
pub struct ICanvasTextRenderer(::windows::core::IUnknown);
impl ICanvasTextRenderer {
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DrawGlyphRun<P0>(
        &self,
        point: ::windows::Foundation::Numerics::Vector2,
        fontface: &CanvasFontFace,
        fontsize: f32,
        glyphs: &[CanvasGlyph],
        issideways: bool,
        bidilevel: u32,
        brush: P0,
        measuringmode: CanvasTextMeasuringMode,
        localename: &::windows::core::HSTRING,
        textstring: &::windows::core::HSTRING,
        clustermapindices: &[i32],
        characterindex: u32,
        glyphorientation: CanvasGlyphOrientation,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<::windows::core::IInspectable>,
        >,
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
                    brush.into().abi(),
                    measuringmode,
                    ::core::mem::transmute_copy(localename),
                    ::core::mem::transmute_copy(textstring),
                    clustermapindices.len() as u32,
                    clustermapindices.as_ptr(),
                    characterindex,
                    glyphorientation,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DrawStrikethrough<P0>(
        &self,
        point: ::windows::Foundation::Numerics::Vector2,
        strikethroughwidth: f32,
        strikethroughthickness: f32,
        strikethroughoffset: f32,
        textdirection: CanvasTextDirection,
        brush: P0,
        textmeasuringmode: CanvasTextMeasuringMode,
        localename: &::windows::core::HSTRING,
        glyphorientation: CanvasGlyphOrientation,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<::windows::core::IInspectable>,
        >,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawStrikethrough)(
                    ::windows::core::Vtable::as_raw(this),
                    point,
                    strikethroughwidth,
                    strikethroughthickness,
                    strikethroughoffset,
                    textdirection,
                    brush.into().abi(),
                    textmeasuringmode,
                    ::core::mem::transmute_copy(localename),
                    glyphorientation,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DrawUnderline<P0>(
        &self,
        point: ::windows::Foundation::Numerics::Vector2,
        underlinewidth: f32,
        underlinethickness: f32,
        underlineoffset: f32,
        runheight: f32,
        textdirection: CanvasTextDirection,
        brush: P0,
        textmeasuringmode: CanvasTextMeasuringMode,
        localename: &::windows::core::HSTRING,
        glyphorientation: CanvasGlyphOrientation,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<::windows::core::IInspectable>,
        >,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawUnderline)(
                    ::windows::core::Vtable::as_raw(this),
                    point,
                    underlinewidth,
                    underlinethickness,
                    underlineoffset,
                    runheight,
                    textdirection,
                    brush.into().abi(),
                    textmeasuringmode,
                    ::core::mem::transmute_copy(localename),
                    glyphorientation,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DrawInlineObject<P0, E0, P1>(
        &self,
        point: ::windows::Foundation::Numerics::Vector2,
        inlineobject: P0,
        issideways: bool,
        isrighttoleft: bool,
        brush: P1,
        glyphorientation: CanvasGlyphOrientation,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasTextInlineObject>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::Into<
            ::windows::core::InParam<::windows::core::IInspectable>,
        >,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawInlineObject)(
                    ::windows::core::Vtable::as_raw(this),
                    point,
                    inlineobject.try_into().map_err(|e| e.into())?.abi(),
                    issideways,
                    isrighttoleft,
                    brush.into().abi(),
                    glyphorientation,
                )
                .ok()
        }
    }
    pub fn PixelSnappingDisabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .PixelSnappingDisabled)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
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
    pub fn Dpi(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Dpi)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(
    ICanvasTextRenderer, ::windows::core::IUnknown, ::windows::core::IInspectable
);
impl ::core::clone::Clone for ICanvasTextRenderer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICanvasTextRenderer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICanvasTextRenderer {}
impl ::core::fmt::Debug for ICanvasTextRenderer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICanvasTextRenderer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICanvasTextRenderer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"{9aaeece5-8d09-4a64-b322-af030421b2e4}",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ICanvasTextRenderer {
    type Vtable = ICanvasTextRenderer_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasTextRenderer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x9aaeece5_8d09_4a64_b322_af030421b2e4,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasTextRenderer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub DrawGlyphRun: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        point: ::windows::Foundation::Numerics::Vector2,
        fontface: *mut ::core::ffi::c_void,
        fontsize: f32,
        glyphs_array_size: u32,
        glyphs: *const CanvasGlyph,
        issideways: bool,
        bidilevel: u32,
        brush: *mut ::core::ffi::c_void,
        measuringmode: CanvasTextMeasuringMode,
        localename: *mut ::core::ffi::c_void,
        textstring: *mut ::core::ffi::c_void,
        clusterMapIndices_array_size: u32,
        clustermapindices: *const i32,
        characterindex: u32,
        glyphorientation: CanvasGlyphOrientation,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DrawGlyphRun: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub DrawStrikethrough: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        point: ::windows::Foundation::Numerics::Vector2,
        strikethroughwidth: f32,
        strikethroughthickness: f32,
        strikethroughoffset: f32,
        textdirection: CanvasTextDirection,
        brush: *mut ::core::ffi::c_void,
        textmeasuringmode: CanvasTextMeasuringMode,
        localename: *mut ::core::ffi::c_void,
        glyphorientation: CanvasGlyphOrientation,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DrawStrikethrough: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub DrawUnderline: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        point: ::windows::Foundation::Numerics::Vector2,
        underlinewidth: f32,
        underlinethickness: f32,
        underlineoffset: f32,
        runheight: f32,
        textdirection: CanvasTextDirection,
        brush: *mut ::core::ffi::c_void,
        textmeasuringmode: CanvasTextMeasuringMode,
        localename: *mut ::core::ffi::c_void,
        glyphorientation: CanvasGlyphOrientation,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DrawUnderline: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub DrawInlineObject: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        point: ::windows::Foundation::Numerics::Vector2,
        inlineobject: *mut ::core::ffi::c_void,
        issideways: bool,
        isrighttoleft: bool,
        brush: *mut ::core::ffi::c_void,
        glyphorientation: CanvasGlyphOrientation,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    DrawInlineObject: usize,
    pub PixelSnappingDisabled: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Transform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Transform: usize,
    pub Dpi: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasTextRenderingParameters(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasTextRenderingParameters {
    type Vtable = ICanvasTextRenderingParameters_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasTextRenderingParameters {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xb20bf738_edb9_4eec_a12f_b6ae32e8ace6,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasTextRenderingParameters_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RenderingMode: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasTextRenderingMode,
    ) -> ::windows::core::HRESULT,
    pub GridFit: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasTextGridFit,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasTextRenderingParametersFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasTextRenderingParametersFactory {
    type Vtable = ICanvasTextRenderingParametersFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasTextRenderingParametersFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xd240ac25_4d23_4964_9d9a_db2fc8af185d,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasTextRenderingParametersFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        textrenderingmode: CanvasTextRenderingMode,
        gridfit: CanvasTextGridFit,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasTypography(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasTypography {
    type Vtable = ICanvasTypography_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasTypography {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xf15bc312_447f_44ed_8bec_7e40f4a4dfc8,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasTypography_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AddFeature: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        feature: CanvasTypographyFeature,
    ) -> ::windows::core::HRESULT,
    pub AddFeatureWithNameAndParameter: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        name: CanvasTypographyFeatureName,
        parameter: u32,
    ) -> ::windows::core::HRESULT,
    pub GetFeatures: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut CanvasTypographyFeature,
    ) -> ::windows::core::HRESULT,
}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
pub struct CanvasFontFace(::windows::core::IUnknown);
impl CanvasFontFace {
    pub fn GetRecommendedRenderingMode(
        &self,
        fontsize: f32,
        dpi: f32,
        measuringmode: CanvasTextMeasuringMode,
        renderingparameters: &CanvasTextRenderingParameters,
    ) -> ::windows::core::Result<CanvasTextRenderingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetRecommendedRenderingMode)(
                    ::windows::core::Vtable::as_raw(this),
                    fontsize,
                    dpi,
                    measuringmode,
                    ::core::mem::transmute_copy(renderingparameters),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn GetRecommendedRenderingModeWithAllOptions(
        &self,
        fontsize: f32,
        dpi: f32,
        measuringmode: CanvasTextMeasuringMode,
        renderingparameters: &CanvasTextRenderingParameters,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        issideways: bool,
        outlinethreshold: super::CanvasAntialiasing,
    ) -> ::windows::core::Result<CanvasTextRenderingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetRecommendedRenderingModeWithAllOptions)(
                    ::windows::core::Vtable::as_raw(this),
                    fontsize,
                    dpi,
                    measuringmode,
                    ::core::mem::transmute_copy(renderingparameters),
                    transform,
                    issideways,
                    outlinethreshold,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn GetRecommendedGridFit(
        &self,
        fontsize: f32,
        dpi: f32,
        measuringmode: CanvasTextMeasuringMode,
        renderingparameters: &CanvasTextRenderingParameters,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        issideways: bool,
        outlinethreshold: super::CanvasAntialiasing,
    ) -> ::windows::core::Result<CanvasTextGridFit> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetRecommendedGridFit)(
                    ::windows::core::Vtable::as_raw(this),
                    fontsize,
                    dpi,
                    measuringmode,
                    ::core::mem::transmute_copy(renderingparameters),
                    transform,
                    issideways,
                    outlinethreshold,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn GlyphBox(&self) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GlyphBox)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SubscriptPosition(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .SubscriptPosition)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn SubscriptSize(&self) -> ::windows::core::Result<::windows::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .SubscriptSize)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SuperscriptPosition(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .SuperscriptPosition)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn SuperscriptSize(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .SuperscriptSize)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn HasTypographicMetrics(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .HasTypographicMetrics)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn Ascent(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Ascent)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn Descent(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Descent)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn LineGap(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LineGap)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn CapHeight(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CapHeight)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn LowercaseLetterHeight(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LowercaseLetterHeight)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn UnderlinePosition(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .UnderlinePosition)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn UnderlineThickness(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .UnderlineThickness)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn StrikethroughPosition(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .StrikethroughPosition)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn StrikethroughThickness(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .StrikethroughThickness)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn CaretSlopeRise(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CaretSlopeRise)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn CaretSlopeRun(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CaretSlopeRun)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn CaretOffset(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CaretOffset)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn UnicodeRanges(
        &self,
    ) -> ::windows::core::Result<::windows::core::Array<CanvasUnicodeRange>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .UnicodeRanges)(
                    ::windows::core::Vtable::as_raw(this),
                    ::windows::core::Array::<
                        CanvasUnicodeRange,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn IsMonospaced(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .IsMonospaced)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn GetVerticalGlyphVariants(
        &self,
        inputelements: &[i32],
    ) -> ::windows::core::Result<::windows::core::Array<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetVerticalGlyphVariants)(
                    ::windows::core::Vtable::as_raw(this),
                    inputelements.len() as u32,
                    inputelements.as_ptr(),
                    ::windows::core::Array::<
                        i32,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn HasVerticalGlyphVariants(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .HasVerticalGlyphVariants)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn FileFormatType(&self) -> ::windows::core::Result<CanvasFontFileFormatType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .FileFormatType)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn Simulations(&self) -> ::windows::core::Result<CanvasFontSimulations> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Simulations)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn IsSymbolFont(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .IsSymbolFont)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn GlyphCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GlyphCount)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn GetGlyphIndices(
        &self,
        inputelements: &[u32],
    ) -> ::windows::core::Result<::windows::core::Array<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetGlyphIndices)(
                    ::windows::core::Vtable::as_raw(this),
                    inputelements.len() as u32,
                    inputelements.as_ptr(),
                    ::windows::core::Array::<
                        i32,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn GetGlyphMetrics(
        &self,
        inputelements: &[i32],
        issideways: bool,
    ) -> ::windows::core::Result<::windows::core::Array<CanvasGlyphMetrics>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetGlyphMetrics)(
                    ::windows::core::Vtable::as_raw(this),
                    inputelements.len() as u32,
                    inputelements.as_ptr(),
                    issideways,
                    ::windows::core::Array::<
                        CanvasGlyphMetrics,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn GetGdiCompatibleGlyphMetrics(
        &self,
        fontsize: f32,
        dpi: f32,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        usegdinatural: bool,
        inputelements: &[i32],
        issideways: bool,
    ) -> ::windows::core::Result<::windows::core::Array<CanvasGlyphMetrics>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetGdiCompatibleGlyphMetrics)(
                    ::windows::core::Vtable::as_raw(this),
                    fontsize,
                    dpi,
                    transform,
                    usegdinatural,
                    inputelements.len() as u32,
                    inputelements.as_ptr(),
                    issideways,
                    ::windows::core::Array::<
                        CanvasGlyphMetrics,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    ///*Required features: `"UI_Text"`*
    #[cfg(feature = "UI_Text")]
    pub fn Weight(&self) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Weight)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"UI_Text"`*
    #[cfg(feature = "UI_Text")]
    pub fn Stretch(&self) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Stretch)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"UI_Text"`*
    #[cfg(feature = "UI_Text")]
    pub fn Style(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Style)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Collections"`*
    #[cfg(feature = "Foundation_Collections")]
    pub fn FamilyNames(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMapView::<
            ::windows::core::HSTRING,
            ::windows::core::HSTRING,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .FamilyNames)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Collections"`*
    #[cfg(feature = "Foundation_Collections")]
    pub fn FaceNames(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMapView::<
            ::windows::core::HSTRING,
            ::windows::core::HSTRING,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .FaceNames)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Collections"`*
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetInformationalStrings(
        &self,
        fontinformation: CanvasFontInformation,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMapView::<
            ::windows::core::HSTRING,
            ::windows::core::HSTRING,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetInformationalStrings)(
                    ::windows::core::Vtable::as_raw(this),
                    fontinformation,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn HasCharacter(&self, unicodevalue: u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .HasCharacter)(
                    ::windows::core::Vtable::as_raw(this),
                    unicodevalue,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn GetGlyphRunBounds(
        &self,
        drawingsession: &super::CanvasDrawingSession,
        point: ::windows::Foundation::Numerics::Vector2,
        fontsize: f32,
        glyphs: &[CanvasGlyph],
        issideways: bool,
        bidilevel: u32,
    ) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetGlyphRunBounds)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(drawingsession),
                    point,
                    fontsize,
                    glyphs.len() as u32,
                    glyphs.as_ptr(),
                    issideways,
                    bidilevel,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn GetGlyphRunBoundsWithMeasuringMode(
        &self,
        drawingsession: &super::CanvasDrawingSession,
        point: ::windows::Foundation::Numerics::Vector2,
        fontsize: f32,
        glyphs: &[CanvasGlyph],
        issideways: bool,
        bidilevel: u32,
        measuringmode: CanvasTextMeasuringMode,
    ) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetGlyphRunBoundsWithMeasuringMode)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(drawingsession),
                    point,
                    fontsize,
                    glyphs.len() as u32,
                    glyphs.as_ptr(),
                    issideways,
                    bidilevel,
                    measuringmode,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn Panose(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Panose)(
                    ::windows::core::Vtable::as_raw(this),
                    ::windows::core::Array::<
                        u8,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn GetSupportedTypographicFeatureNames(
        &self,
        script: CanvasAnalyzedScript,
    ) -> ::windows::core::Result<::windows::core::Array<CanvasTypographyFeatureName>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetSupportedTypographicFeatureNames)(
                    ::windows::core::Vtable::as_raw(this),
                    script,
                    ::windows::core::Array::<
                        CanvasTypographyFeatureName,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn GetSupportedTypographicFeatureNamesWithLocale(
        &self,
        script: CanvasAnalyzedScript,
        locale: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::core::Array<CanvasTypographyFeatureName>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetSupportedTypographicFeatureNamesWithLocale)(
                    ::windows::core::Vtable::as_raw(this),
                    script,
                    ::core::mem::transmute_copy(locale),
                    ::windows::core::Array::<
                        CanvasTypographyFeatureName,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn GetTypographicFeatureGlyphSupport(
        &self,
        script: CanvasAnalyzedScript,
        typographicfeaturename: CanvasTypographyFeatureName,
        glyphselements: &[CanvasGlyph],
    ) -> ::windows::core::Result<::windows::core::Array<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetTypographicFeatureGlyphSupport)(
                    ::windows::core::Vtable::as_raw(this),
                    script,
                    typographicfeaturename,
                    glyphselements.len() as u32,
                    glyphselements.as_ptr(),
                    ::windows::core::Array::<
                        bool,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn GetTypographicFeatureGlyphSupportWithLocale(
        &self,
        script: CanvasAnalyzedScript,
        typographicfeaturename: CanvasTypographyFeatureName,
        glyphselements: &[CanvasGlyph],
        locale: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::core::Array<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetTypographicFeatureGlyphSupportWithLocale)(
                    ::windows::core::Vtable::as_raw(this),
                    script,
                    typographicfeaturename,
                    glyphselements.len() as u32,
                    glyphselements.as_ptr(),
                    ::core::mem::transmute_copy(locale),
                    ::windows::core::Array::<
                        bool,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
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
impl ::core::clone::Clone for CanvasFontFace {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasFontFace {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasFontFace {}
impl ::core::fmt::Debug for CanvasFontFace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasFontFace").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasFontFace {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.Text.CanvasFontFace;{5199d129-4ef9-4dee-b74c-4dc910201a7f})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasFontFace {
    type Vtable = ICanvasFontFace_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasFontFace {
    const IID: ::windows::core::GUID = <ICanvasFontFace as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasFontFace {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Text.CanvasFontFace";
}
::windows::core::interface_hierarchy!(
    CanvasFontFace,::windows::core::IUnknown,::windows::core::IInspectable
);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasFontFace> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasFontFace) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasFontFace> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasFontFace) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasFontFace>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasFontFace) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CanvasFontFace {}
unsafe impl ::core::marker::Sync for CanvasFontFace {}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
pub struct CanvasFontSet(::windows::core::IUnknown);
impl CanvasFontSet {
    ///*Required features: `"Foundation_Collections"`*
    #[cfg(feature = "Foundation_Collections")]
    pub fn Fonts(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IVectorView::<CanvasFontFace>,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Fonts)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn TryFindFontFace(
        &self,
        fontface: &CanvasFontFace,
        index: &mut i32,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .TryFindFontFace)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(fontface),
                    index,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn GetMatchingFontsFromProperties(
        &self,
        propertyelements: &[CanvasFontProperty],
    ) -> ::windows::core::Result<CanvasFontSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetMatchingFontsFromProperties)(
                    ::windows::core::Vtable::as_raw(this),
                    propertyelements.len() as u32,
                    ::core::mem::transmute(propertyelements.as_ptr()),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"UI_Text"`*
    #[cfg(feature = "UI_Text")]
    pub fn GetMatchingFontsFromWwsFamily(
        &self,
        familyname: &::windows::core::HSTRING,
        weight: ::windows::UI::Text::FontWeight,
        stretch: ::windows::UI::Text::FontStretch,
        style: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<CanvasFontSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetMatchingFontsFromWwsFamily)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(familyname),
                    weight,
                    stretch,
                    style,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn CountFontsMatchingProperty<P0>(
        &self,
        property: P0,
    ) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<CanvasFontProperty>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CountFontsMatchingProperty)(
                    ::windows::core::Vtable::as_raw(this),
                    property.into().abi(),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Collections"`*
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPropertyValuesFromIndex(
        &self,
        fontindex: u32,
        propertyidentifier: CanvasFontPropertyIdentifier,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IMapView::<
            ::windows::core::HSTRING,
            ::windows::core::HSTRING,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetPropertyValuesFromIndex)(
                    ::windows::core::Vtable::as_raw(this),
                    fontindex,
                    propertyidentifier,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn GetPropertyValuesFromIdentifier(
        &self,
        propertyidentifier: CanvasFontPropertyIdentifier,
        preferredlocalenames: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::core::Array<CanvasFontProperty>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetPropertyValuesFromIdentifier)(
                    ::windows::core::Vtable::as_raw(this),
                    propertyidentifier,
                    ::core::mem::transmute_copy(preferredlocalenames),
                    ::windows::core::Array::<
                        CanvasFontProperty,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn GetPropertyValues(
        &self,
        propertyidentifier: CanvasFontPropertyIdentifier,
    ) -> ::windows::core::Result<::windows::core::Array<CanvasFontProperty>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetPropertyValues)(
                    ::windows::core::Vtable::as_raw(this),
                    propertyidentifier,
                    ::windows::core::Array::<
                        CanvasFontProperty,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn Create(
        uri: &::windows::Foundation::Uri,
    ) -> ::windows::core::Result<CanvasFontSet> {
        Self::ICanvasFontSetFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Create)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(uri),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    pub fn GetSystemFontSet() -> ::windows::core::Result<CanvasFontSet> {
        Self::ICanvasFontSetStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetSystemFontSet)(
                    ::windows::core::Vtable::as_raw(this),
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
    pub fn ICanvasFontSetFactory<
        R,
        F: FnOnce(&ICanvasFontSetFactory) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasFontSet,
            ICanvasFontSetFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICanvasFontSetStatics<
        R,
        F: FnOnce(&ICanvasFontSetStatics) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasFontSet,
            ICanvasFontSetStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CanvasFontSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasFontSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasFontSet {}
impl ::core::fmt::Debug for CanvasFontSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasFontSet").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasFontSet {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.Text.CanvasFontSet;{0a5bfb92-1f3c-459f-9d7e-a6289dd093c0})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasFontSet {
    type Vtable = ICanvasFontSet_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasFontSet {
    const IID: ::windows::core::GUID = <ICanvasFontSet as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasFontSet {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Text.CanvasFontSet";
}
::windows::core::interface_hierarchy!(
    CanvasFontSet,::windows::core::IUnknown,::windows::core::IInspectable
);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasFontSet> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasFontSet) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasFontSet> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasFontSet) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasFontSet>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasFontSet) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CanvasFontSet {}
unsafe impl ::core::marker::Sync for CanvasFontSet {}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
pub struct CanvasNumberSubstitution(::windows::core::IUnknown);
impl CanvasNumberSubstitution {
    pub fn Create(
        method: CanvasNumberSubstitutionMethod,
    ) -> ::windows::core::Result<CanvasNumberSubstitution> {
        Self::ICanvasNumberSubstitutionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Create)(
                    ::windows::core::Vtable::as_raw(this),
                    method,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    pub fn CreateWithLocaleAndIgnoreOverrides(
        method: CanvasNumberSubstitutionMethod,
        localename: &::windows::core::HSTRING,
        ignoreenvironmentoverrides: bool,
    ) -> ::windows::core::Result<CanvasNumberSubstitution> {
        Self::ICanvasNumberSubstitutionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateWithLocaleAndIgnoreOverrides)(
                    ::windows::core::Vtable::as_raw(this),
                    method,
                    ::core::mem::transmute_copy(localename),
                    ignoreenvironmentoverrides,
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
    pub fn ICanvasNumberSubstitutionFactory<
        R,
        F: FnOnce(&ICanvasNumberSubstitutionFactory) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasNumberSubstitution,
            ICanvasNumberSubstitutionFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CanvasNumberSubstitution {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasNumberSubstitution {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasNumberSubstitution {}
impl ::core::fmt::Debug for CanvasNumberSubstitution {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasNumberSubstitution").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasNumberSubstitution {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.Text.CanvasNumberSubstitution;{c81a67ad-0639-4f8f-878b-d937f8a14293})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasNumberSubstitution {
    type Vtable = ICanvasNumberSubstitution_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasNumberSubstitution {
    const IID: ::windows::core::GUID = <ICanvasNumberSubstitution as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasNumberSubstitution {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Text.CanvasNumberSubstitution";
}
::windows::core::interface_hierarchy!(
    CanvasNumberSubstitution,::windows::core::IUnknown,::windows::core::IInspectable
);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasNumberSubstitution>
for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasNumberSubstitution) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasNumberSubstitution>
for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasNumberSubstitution) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasNumberSubstitution>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasNumberSubstitution) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CanvasNumberSubstitution {}
unsafe impl ::core::marker::Sync for CanvasNumberSubstitution {}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
pub struct CanvasScaledFont(::windows::core::IUnknown);
impl CanvasScaledFont {
    pub fn FontFace(&self) -> ::windows::core::Result<CanvasFontFace> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .FontFace)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn ScaleFactor(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ScaleFactor)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for CanvasScaledFont {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasScaledFont {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasScaledFont {}
impl ::core::fmt::Debug for CanvasScaledFont {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasScaledFont").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasScaledFont {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.Text.CanvasScaledFont;{bbc4f8d2-eb2b-45f1-ac2a-cfc1f598bae3})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasScaledFont {
    type Vtable = ICanvasScaledFont_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasScaledFont {
    const IID: ::windows::core::GUID = <ICanvasScaledFont as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasScaledFont {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Text.CanvasScaledFont";
}
::windows::core::interface_hierarchy!(
    CanvasScaledFont,::windows::core::IUnknown,::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for CanvasScaledFont {}
unsafe impl ::core::marker::Sync for CanvasScaledFont {}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
pub struct CanvasTextAnalyzer(::windows::core::IUnknown);
impl CanvasTextAnalyzer {
    ///*Required features: `"Foundation_Collections"`*
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFontsUsingSystemFontSet(
        &self,
        textformat: &CanvasTextFormat,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IVectorView::<
            ::windows::Foundation::Collections::IKeyValuePair::<
                CanvasCharacterRange,
                CanvasScaledFont,
            >,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetFontsUsingSystemFontSet)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(textformat),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Collections"`*
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFonts(
        &self,
        textformat: &CanvasTextFormat,
        fontset: &CanvasFontSet,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IVectorView::<
            ::windows::Foundation::Collections::IKeyValuePair::<
                CanvasCharacterRange,
                CanvasScaledFont,
            >,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetFonts)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(textformat),
                    ::core::mem::transmute_copy(fontset),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Collections"`*
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetBidi(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IVectorView::<
            ::windows::Foundation::Collections::IKeyValuePair::<
                CanvasCharacterRange,
                CanvasAnalyzedBidi,
            >,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetBidi)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Collections"`*
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetBidiWithLocale(
        &self,
        locale: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IVectorView::<
            ::windows::Foundation::Collections::IKeyValuePair::<
                CanvasCharacterRange,
                CanvasAnalyzedBidi,
            >,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetBidiWithLocale)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(locale),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn GetBreakpoints(
        &self,
    ) -> ::windows::core::Result<::windows::core::Array<CanvasAnalyzedBreakpoint>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetBreakpoints)(
                    ::windows::core::Vtable::as_raw(this),
                    ::windows::core::Array::<
                        CanvasAnalyzedBreakpoint,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn GetBreakpointsWithLocale(
        &self,
        locale: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<::windows::core::Array<CanvasAnalyzedBreakpoint>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetBreakpointsWithLocale)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(locale),
                    ::windows::core::Array::<
                        CanvasAnalyzedBreakpoint,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    ///*Required features: `"Foundation_Collections"`*
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetNumberSubstitutions(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IVectorView::<
            ::windows::Foundation::Collections::IKeyValuePair::<
                CanvasCharacterRange,
                CanvasNumberSubstitution,
            >,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetNumberSubstitutions)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Collections"`*
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetScript(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IVectorView::<
            ::windows::Foundation::Collections::IKeyValuePair::<
                CanvasCharacterRange,
                CanvasAnalyzedScript,
            >,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetScript)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Collections"`*
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetScriptWithLocale(
        &self,
        locale: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IVectorView::<
            ::windows::Foundation::Collections::IKeyValuePair::<
                CanvasCharacterRange,
                CanvasAnalyzedScript,
            >,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetScriptWithLocale)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(locale),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Collections"`*
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetGlyphOrientations(
        &self,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IVectorView::<
            ::windows::Foundation::Collections::IKeyValuePair::<
                CanvasCharacterRange,
                CanvasAnalyzedGlyphOrientation,
            >,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetGlyphOrientations)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Collections"`*
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetGlyphOrientationsWithLocale(
        &self,
        locale: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<
        ::windows::Foundation::Collections::IVectorView::<
            ::windows::Foundation::Collections::IKeyValuePair::<
                CanvasCharacterRange,
                CanvasAnalyzedGlyphOrientation,
            >,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetGlyphOrientationsWithLocale)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(locale),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn GetScriptProperties(
        &self,
        analyzedscript: CanvasAnalyzedScript,
    ) -> ::windows::core::Result<CanvasScriptProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetScriptProperties)(
                    ::windows::core::Vtable::as_raw(this),
                    analyzedscript,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn GetGlyphs(
        &self,
        characterrange: CanvasCharacterRange,
        fontface: &CanvasFontFace,
        fontsize: f32,
        issideways: bool,
        isrighttoleft: bool,
        script: CanvasAnalyzedScript,
    ) -> ::windows::core::Result<::windows::core::Array<CanvasGlyph>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetGlyphs)(
                    ::windows::core::Vtable::as_raw(this),
                    characterrange,
                    ::core::mem::transmute_copy(fontface),
                    fontsize,
                    issideways,
                    isrighttoleft,
                    script,
                    ::windows::core::Array::<
                        CanvasGlyph,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    ///*Required features: `"Foundation_Collections"`*
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetGlyphsWithAllOptions<P0, E0>(
        &self,
        characterrange: CanvasCharacterRange,
        fontface: &CanvasFontFace,
        fontsize: f32,
        issideways: bool,
        isrighttoleft: bool,
        script: CanvasAnalyzedScript,
        locale: &::windows::core::HSTRING,
        numbersubstitution: &CanvasNumberSubstitution,
        typographyranges: P0,
        clustermapindiceselements: &mut ::windows::core::Array<i32>,
        isshapedaloneglyphselements: &mut ::windows::core::Array<bool>,
        glyphshapingresultselements: &mut ::windows::core::Array<CanvasGlyphShaping>,
    ) -> ::windows::core::Result<::windows::core::Array<CanvasGlyph>>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<
                ::windows::Foundation::Collections::IVectorView::<
                    ::windows::Foundation::Collections::IKeyValuePair::<
                        CanvasCharacterRange,
                        CanvasTypography,
                    >,
                >,
            >,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetGlyphsWithAllOptions)(
                    ::windows::core::Vtable::as_raw(this),
                    characterrange,
                    ::core::mem::transmute_copy(fontface),
                    fontsize,
                    issideways,
                    isrighttoleft,
                    script,
                    ::core::mem::transmute_copy(locale),
                    ::core::mem::transmute_copy(numbersubstitution),
                    typographyranges.try_into().map_err(|e| e.into())?.abi(),
                    clustermapindiceselements.set_abi_len(),
                    clustermapindiceselements as *mut _ as _,
                    isshapedaloneglyphselements.set_abi_len(),
                    isshapedaloneglyphselements as *mut _ as _,
                    glyphshapingresultselements.set_abi_len(),
                    glyphshapingresultselements as *mut _ as _,
                    ::windows::core::Array::<
                        CanvasGlyph,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn GetJustificationOpportunities(
        &self,
        characterrange: CanvasCharacterRange,
        fontface: &CanvasFontFace,
        fontsize: f32,
        script: CanvasAnalyzedScript,
        clustermapindiceselements: &[i32],
        glyphshapingresultselements: &[CanvasGlyphShaping],
    ) -> ::windows::core::Result<
        ::windows::core::Array<CanvasJustificationOpportunity>,
    > {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetJustificationOpportunities)(
                    ::windows::core::Vtable::as_raw(this),
                    characterrange,
                    ::core::mem::transmute_copy(fontface),
                    fontsize,
                    script,
                    clustermapindiceselements.len() as u32,
                    clustermapindiceselements.as_ptr(),
                    glyphshapingresultselements.len() as u32,
                    glyphshapingresultselements.as_ptr(),
                    ::windows::core::Array::<
                        CanvasJustificationOpportunity,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn ApplyJustificationOpportunities(
        &self,
        linewidth: f32,
        justificationopportunitieselements: &[CanvasJustificationOpportunity],
        sourceglyphselements: &[CanvasGlyph],
    ) -> ::windows::core::Result<::windows::core::Array<CanvasGlyph>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ApplyJustificationOpportunities)(
                    ::windows::core::Vtable::as_raw(this),
                    linewidth,
                    justificationopportunitieselements.len() as u32,
                    justificationopportunitieselements.as_ptr(),
                    sourceglyphselements.len() as u32,
                    sourceglyphselements.as_ptr(),
                    ::windows::core::Array::<
                        CanvasGlyph,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn AddGlyphsAfterJustification(
        &self,
        fontface: &CanvasFontFace,
        fontsize: f32,
        script: CanvasAnalyzedScript,
        clustermapindiceselements: &[i32],
        originalglyphselements: &[CanvasGlyph],
        justifiedglyphselements: &[CanvasGlyph],
        glyphshapingresultselements: &[CanvasGlyphShaping],
    ) -> ::windows::core::Result<::windows::core::Array<CanvasGlyph>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .AddGlyphsAfterJustification)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(fontface),
                    fontsize,
                    script,
                    clustermapindiceselements.len() as u32,
                    clustermapindiceselements.as_ptr(),
                    originalglyphselements.len() as u32,
                    originalglyphselements.as_ptr(),
                    justifiedglyphselements.len() as u32,
                    justifiedglyphselements.as_ptr(),
                    glyphshapingresultselements.len() as u32,
                    glyphshapingresultselements.as_ptr(),
                    ::windows::core::Array::<
                        CanvasGlyph,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn AddGlyphsAfterJustificationWithClusterMap(
        &self,
        fontface: &CanvasFontFace,
        fontsize: f32,
        script: CanvasAnalyzedScript,
        clustermapindiceselements: &[i32],
        originalglyphselements: &[CanvasGlyph],
        justifiedglyphselements: &[CanvasGlyph],
        glyphshapingresultselements: &[CanvasGlyphShaping],
        outputclustermapindiceselements: &mut ::windows::core::Array<i32>,
    ) -> ::windows::core::Result<::windows::core::Array<CanvasGlyph>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .AddGlyphsAfterJustificationWithClusterMap)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(fontface),
                    fontsize,
                    script,
                    clustermapindiceselements.len() as u32,
                    clustermapindiceselements.as_ptr(),
                    originalglyphselements.len() as u32,
                    originalglyphselements.as_ptr(),
                    justifiedglyphselements.len() as u32,
                    justifiedglyphselements.as_ptr(),
                    glyphshapingresultselements.len() as u32,
                    glyphshapingresultselements.as_ptr(),
                    outputclustermapindiceselements.set_abi_len(),
                    outputclustermapindiceselements as *mut _ as _,
                    ::windows::core::Array::<
                        CanvasGlyph,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn Create(
        text: &::windows::core::HSTRING,
        textdirection: CanvasTextDirection,
    ) -> ::windows::core::Result<CanvasTextAnalyzer> {
        Self::ICanvasTextAnalyzerFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Create)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(text),
                    textdirection,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    pub fn CreateWithNumberSubstitutionAndVerticalGlyphOrientationAndBidiLevel(
        text: &::windows::core::HSTRING,
        textdirection: CanvasTextDirection,
        numbersubstitution: &CanvasNumberSubstitution,
        verticalglyphorientation: CanvasVerticalGlyphOrientation,
        bidilevel: u32,
    ) -> ::windows::core::Result<CanvasTextAnalyzer> {
        Self::ICanvasTextAnalyzerFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateWithNumberSubstitutionAndVerticalGlyphOrientationAndBidiLevel)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(text),
                    textdirection,
                    ::core::mem::transmute_copy(numbersubstitution),
                    verticalglyphorientation,
                    bidilevel,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    pub fn CreateWithOptions<P0, E0>(
        text: &::windows::core::HSTRING,
        textdirection: CanvasTextDirection,
        options: P0,
    ) -> ::windows::core::Result<CanvasTextAnalyzer>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasTextAnalyzerOptions>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasTextAnalyzerFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateWithOptions)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(text),
                    textdirection,
                    options.try_into().map_err(|e| e.into())?.abi(),
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
    pub fn ICanvasTextAnalyzerFactory<
        R,
        F: FnOnce(&ICanvasTextAnalyzerFactory) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasTextAnalyzer,
            ICanvasTextAnalyzerFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CanvasTextAnalyzer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasTextAnalyzer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasTextAnalyzer {}
impl ::core::fmt::Debug for CanvasTextAnalyzer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasTextAnalyzer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasTextAnalyzer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.Text.CanvasTextAnalyzer;{4298f3d1-645b-40e3-b91b-81986d767fc0})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasTextAnalyzer {
    type Vtable = ICanvasTextAnalyzer_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasTextAnalyzer {
    const IID: ::windows::core::GUID = <ICanvasTextAnalyzer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasTextAnalyzer {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Text.CanvasTextAnalyzer";
}
::windows::core::interface_hierarchy!(
    CanvasTextAnalyzer,::windows::core::IUnknown,::windows::core::IInspectable
);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasTextAnalyzer> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasTextAnalyzer) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasTextAnalyzer> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasTextAnalyzer) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasTextAnalyzer>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasTextAnalyzer) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CanvasTextAnalyzer {}
unsafe impl ::core::marker::Sync for CanvasTextAnalyzer {}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
pub struct CanvasTextFormat(::windows::core::IUnknown);
impl CanvasTextFormat {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasTextFormat,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Direction(&self) -> ::windows::core::Result<CanvasTextDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Direction)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn SetDirection(
        &self,
        value: CanvasTextDirection,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetDirection)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn FontFamily(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .FontFamily)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetFontFamily(
        &self,
        value: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetFontFamily)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(value),
                )
                .ok()
        }
    }
    pub fn FontSize(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .FontSize)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn SetFontSize(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetFontSize)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    ///*Required features: `"UI_Text"`*
    #[cfg(feature = "UI_Text")]
    pub fn FontStretch(
        &self,
    ) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .FontStretch)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"UI_Text"`*
    #[cfg(feature = "UI_Text")]
    pub fn SetFontStretch(
        &self,
        value: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetFontStretch)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    ///*Required features: `"UI_Text"`*
    #[cfg(feature = "UI_Text")]
    pub fn FontStyle(&self) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .FontStyle)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"UI_Text"`*
    #[cfg(feature = "UI_Text")]
    pub fn SetFontStyle(
        &self,
        value: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetFontStyle)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    ///*Required features: `"UI_Text"`*
    #[cfg(feature = "UI_Text")]
    pub fn FontWeight(
        &self,
    ) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .FontWeight)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"UI_Text"`*
    #[cfg(feature = "UI_Text")]
    pub fn SetFontWeight(
        &self,
        value: ::windows::UI::Text::FontWeight,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetFontWeight)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn IncrementalTabStop(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .IncrementalTabStop)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetIncrementalTabStop(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetIncrementalTabStop)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn LineSpacing(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LineSpacing)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetLineSpacing(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetLineSpacing)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn LineSpacingBaseline(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LineSpacingBaseline)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetLineSpacingBaseline(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetLineSpacingBaseline)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn LocaleName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LocaleName)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetLocaleName(
        &self,
        value: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetLocaleName)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(value),
                )
                .ok()
        }
    }
    pub fn VerticalAlignment(&self) -> ::windows::core::Result<CanvasVerticalAlignment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .VerticalAlignment)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetVerticalAlignment(
        &self,
        value: CanvasVerticalAlignment,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetVerticalAlignment)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn HorizontalAlignment(
        &self,
    ) -> ::windows::core::Result<CanvasHorizontalAlignment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .HorizontalAlignment)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetHorizontalAlignment(
        &self,
        value: CanvasHorizontalAlignment,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetHorizontalAlignment)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn TrimmingGranularity(
        &self,
    ) -> ::windows::core::Result<CanvasTextTrimmingGranularity> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .TrimmingGranularity)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetTrimmingGranularity(
        &self,
        value: CanvasTextTrimmingGranularity,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetTrimmingGranularity)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn TrimmingDelimiter(
        &self,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .TrimmingDelimiter)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetTrimmingDelimiter(
        &self,
        value: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetTrimmingDelimiter)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(value),
                )
                .ok()
        }
    }
    pub fn TrimmingDelimiterCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .TrimmingDelimiterCount)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetTrimmingDelimiterCount(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetTrimmingDelimiterCount)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn WordWrapping(&self) -> ::windows::core::Result<CanvasWordWrapping> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .WordWrapping)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetWordWrapping(
        &self,
        value: CanvasWordWrapping,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetWordWrapping)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn Options(&self) -> ::windows::core::Result<CanvasDrawTextOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Options)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn SetOptions(
        &self,
        value: CanvasDrawTextOptions,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetOptions)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn VerticalGlyphOrientation(
        &self,
    ) -> ::windows::core::Result<CanvasVerticalGlyphOrientation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .VerticalGlyphOrientation)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetVerticalGlyphOrientation(
        &self,
        value: CanvasVerticalGlyphOrientation,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetVerticalGlyphOrientation)(
                    ::windows::core::Vtable::as_raw(this),
                    value,
                )
                .ok()
        }
    }
    pub fn OpticalAlignment(&self) -> ::windows::core::Result<CanvasOpticalAlignment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .OpticalAlignment)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetOpticalAlignment(
        &self,
        value: CanvasOpticalAlignment,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetOpticalAlignment)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn LastLineWrapping(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LastLineWrapping)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetLastLineWrapping(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetLastLineWrapping)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn LineSpacingMode(&self) -> ::windows::core::Result<CanvasLineSpacingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LineSpacingMode)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetLineSpacingMode(
        &self,
        value: CanvasLineSpacingMode,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetLineSpacingMode)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn TrimmingSign(&self) -> ::windows::core::Result<CanvasTrimmingSign> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .TrimmingSign)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetTrimmingSign(
        &self,
        value: CanvasTrimmingSign,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetTrimmingSign)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn CustomTrimmingSign(
        &self,
    ) -> ::windows::core::Result<ICanvasTextInlineObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CustomTrimmingSign)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetCustomTrimmingSign<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasTextInlineObject>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetCustomTrimmingSign)(
                    ::windows::core::Vtable::as_raw(this),
                    value.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    pub fn GetSystemFontFamilies() -> ::windows::core::Result<
        ::windows::core::Array<::windows::core::HSTRING>,
    > {
        Self::ICanvasTextFormatStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetSystemFontFamilies)(
                    ::windows::core::Vtable::as_raw(this),
                    ::windows::core::Array::<
                        ::windows::core::HSTRING,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        })
    }
    ///*Required features: `"Foundation_Collections"`*
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSystemFontFamiliesFromLocaleList<P0, E0>(
        localelist: P0,
    ) -> ::windows::core::Result<::windows::core::Array<::windows::core::HSTRING>>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<
                ::windows::Foundation::Collections::IVectorView::<
                    ::windows::core::HSTRING,
                >,
            >,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasTextFormatStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetSystemFontFamiliesFromLocaleList)(
                    ::windows::core::Vtable::as_raw(this),
                    localelist.try_into().map_err(|e| e.into())?.abi(),
                    ::windows::core::Array::<
                        ::windows::core::HSTRING,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
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
    pub fn ICanvasTextFormatStatics<
        R,
        F: FnOnce(&ICanvasTextFormatStatics) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasTextFormat,
            ICanvasTextFormatStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CanvasTextFormat {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasTextFormat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasTextFormat {}
impl ::core::fmt::Debug for CanvasTextFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasTextFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasTextFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.Text.CanvasTextFormat;{af61bfdc-eabb-4d38-ba1b-afb340612d33})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasTextFormat {
    type Vtable = ICanvasTextFormat_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasTextFormat {
    const IID: ::windows::core::GUID = <ICanvasTextFormat as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasTextFormat {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Text.CanvasTextFormat";
}
::windows::core::interface_hierarchy!(
    CanvasTextFormat,::windows::core::IUnknown,::windows::core::IInspectable
);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasTextFormat> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasTextFormat) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasTextFormat> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasTextFormat) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasTextFormat>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasTextFormat) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CanvasTextFormat {}
unsafe impl ::core::marker::Sync for CanvasTextFormat {}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
pub struct CanvasTextLayout(::windows::core::IUnknown);
impl CanvasTextLayout {
    pub fn GetFormatChangeIndices(
        &self,
    ) -> ::windows::core::Result<::windows::core::Array<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetFormatChangeIndices)(
                    ::windows::core::Vtable::as_raw(this),
                    ::windows::core::Array::<
                        i32,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn Direction(&self) -> ::windows::core::Result<CanvasTextDirection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Direction)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn SetDirection(
        &self,
        value: CanvasTextDirection,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetDirection)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn DefaultFontFamily(
        &self,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .DefaultFontFamily)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn DefaultFontSize(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .DefaultFontSize)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"UI_Text"`*
    #[cfg(feature = "UI_Text")]
    pub fn DefaultFontStretch(
        &self,
    ) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .DefaultFontStretch)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"UI_Text"`*
    #[cfg(feature = "UI_Text")]
    pub fn DefaultFontStyle(
        &self,
    ) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .DefaultFontStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"UI_Text"`*
    #[cfg(feature = "UI_Text")]
    pub fn DefaultFontWeight(
        &self,
    ) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .DefaultFontWeight)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn IncrementalTabStop(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .IncrementalTabStop)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetIncrementalTabStop(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetIncrementalTabStop)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn LineSpacing(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LineSpacing)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetLineSpacing(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetLineSpacing)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn LineSpacingBaseline(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LineSpacingBaseline)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetLineSpacingBaseline(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetLineSpacingBaseline)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn DefaultLocaleName(
        &self,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .DefaultLocaleName)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn VerticalAlignment(&self) -> ::windows::core::Result<CanvasVerticalAlignment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .VerticalAlignment)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetVerticalAlignment(
        &self,
        value: CanvasVerticalAlignment,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetVerticalAlignment)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn HorizontalAlignment(
        &self,
    ) -> ::windows::core::Result<CanvasHorizontalAlignment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .HorizontalAlignment)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetHorizontalAlignment(
        &self,
        value: CanvasHorizontalAlignment,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetHorizontalAlignment)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn TrimmingGranularity(
        &self,
    ) -> ::windows::core::Result<CanvasTextTrimmingGranularity> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .TrimmingGranularity)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetTrimmingGranularity(
        &self,
        value: CanvasTextTrimmingGranularity,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetTrimmingGranularity)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn TrimmingDelimiter(
        &self,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .TrimmingDelimiter)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetTrimmingDelimiter(
        &self,
        value: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetTrimmingDelimiter)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(value),
                )
                .ok()
        }
    }
    pub fn TrimmingDelimiterCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .TrimmingDelimiterCount)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetTrimmingDelimiterCount(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetTrimmingDelimiterCount)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn WordWrapping(&self) -> ::windows::core::Result<CanvasWordWrapping> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .WordWrapping)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetWordWrapping(
        &self,
        value: CanvasWordWrapping,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetWordWrapping)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn Options(&self) -> ::windows::core::Result<CanvasDrawTextOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Options)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn SetOptions(
        &self,
        value: CanvasDrawTextOptions,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetOptions)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn LineSpacingMode(&self) -> ::windows::core::Result<CanvasLineSpacingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LineSpacingMode)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetLineSpacingMode(
        &self,
        value: CanvasLineSpacingMode,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetLineSpacingMode)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn TrimmingSign(&self) -> ::windows::core::Result<CanvasTrimmingSign> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .TrimmingSign)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetTrimmingSign(
        &self,
        value: CanvasTrimmingSign,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetTrimmingSign)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn CustomTrimmingSign(
        &self,
    ) -> ::windows::core::Result<ICanvasTextInlineObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CustomTrimmingSign)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetCustomTrimmingSign<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasTextInlineObject>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetCustomTrimmingSign)(
                    ::windows::core::Vtable::as_raw(this),
                    value.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn RequestedSize(&self) -> ::windows::core::Result<::windows::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .RequestedSize)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn SetRequestedSize(
        &self,
        value: ::windows::Foundation::Size,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetRequestedSize)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn GetMinimumLineLength(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetMinimumLineLength)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`*
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub fn GetBrush(
        &self,
        characterindex: i32,
    ) -> ::windows::core::Result<super::Brushes::ICanvasBrush> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn GetCustomBrush(
        &self,
        characterindex: i32,
    ) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetCustomBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn GetFontFamily(
        &self,
        characterindex: i32,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetFontFamily)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn GetFontSize(&self, characterindex: i32) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetFontSize)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"UI_Text"`*
    #[cfg(feature = "UI_Text")]
    pub fn GetFontStretch(
        &self,
        characterindex: i32,
    ) -> ::windows::core::Result<::windows::UI::Text::FontStretch> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetFontStretch)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"UI_Text"`*
    #[cfg(feature = "UI_Text")]
    pub fn GetFontStyle(
        &self,
        characterindex: i32,
    ) -> ::windows::core::Result<::windows::UI::Text::FontStyle> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetFontStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"UI_Text"`*
    #[cfg(feature = "UI_Text")]
    pub fn GetFontWeight(
        &self,
        characterindex: i32,
    ) -> ::windows::core::Result<::windows::UI::Text::FontWeight> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetFontWeight)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn GetLocaleName(
        &self,
        characterindex: i32,
    ) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetLocaleName)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn GetStrikethrough(
        &self,
        characterindex: i32,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetStrikethrough)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn GetUnderline(&self, characterindex: i32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetUnderline)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn GetInlineObject(
        &self,
        characterindex: i32,
    ) -> ::windows::core::Result<ICanvasTextInlineObject> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetInlineObject)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"UI"`*
    #[cfg(feature = "UI")]
    pub fn SetColor(
        &self,
        characterindex: i32,
        charactercount: i32,
        color: ::windows::UI::Color,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetColor)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    charactercount,
                    color,
                )
                .ok()
        }
    }
    ///*Required features: `"Graphics_Canvas_Brushes"`*
    #[cfg(feature = "Graphics_Canvas_Brushes")]
    pub fn SetBrush<P0, E0>(
        &self,
        characterindex: i32,
        charactercount: i32,
        brush: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::Brushes::ICanvasBrush>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    charactercount,
                    brush.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    pub fn SetCustomBrush<P0>(
        &self,
        characterindex: i32,
        charactercount: i32,
        brush: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<
            ::windows::core::InParam<::windows::core::IInspectable>,
        >,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetCustomBrush)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    charactercount,
                    brush.into().abi(),
                )
                .ok()
        }
    }
    pub fn SetFontFamily(
        &self,
        characterindex: i32,
        charactercount: i32,
        fontfamily: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetFontFamily)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    charactercount,
                    ::core::mem::transmute_copy(fontfamily),
                )
                .ok()
        }
    }
    pub fn SetFontSize(
        &self,
        characterindex: i32,
        charactercount: i32,
        fontsize: f32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetFontSize)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    charactercount,
                    fontsize,
                )
                .ok()
        }
    }
    ///*Required features: `"UI_Text"`*
    #[cfg(feature = "UI_Text")]
    pub fn SetFontStretch(
        &self,
        characterindex: i32,
        charactercount: i32,
        fontstretch: ::windows::UI::Text::FontStretch,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetFontStretch)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    charactercount,
                    fontstretch,
                )
                .ok()
        }
    }
    ///*Required features: `"UI_Text"`*
    #[cfg(feature = "UI_Text")]
    pub fn SetFontStyle(
        &self,
        characterindex: i32,
        charactercount: i32,
        fontstyle: ::windows::UI::Text::FontStyle,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetFontStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    charactercount,
                    fontstyle,
                )
                .ok()
        }
    }
    ///*Required features: `"UI_Text"`*
    #[cfg(feature = "UI_Text")]
    pub fn SetFontWeight(
        &self,
        characterindex: i32,
        charactercount: i32,
        fontweight: ::windows::UI::Text::FontWeight,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetFontWeight)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    charactercount,
                    fontweight,
                )
                .ok()
        }
    }
    pub fn SetLocaleName(
        &self,
        characterindex: i32,
        charactercount: i32,
        name: &::windows::core::HSTRING,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetLocaleName)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    charactercount,
                    ::core::mem::transmute_copy(name),
                )
                .ok()
        }
    }
    pub fn SetStrikethrough(
        &self,
        characterindex: i32,
        charactercount: i32,
        hasstrikethrough: bool,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetStrikethrough)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    charactercount,
                    hasstrikethrough,
                )
                .ok()
        }
    }
    pub fn SetUnderline(
        &self,
        characterindex: i32,
        charactercount: i32,
        hasunderline: bool,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetUnderline)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    charactercount,
                    hasunderline,
                )
                .ok()
        }
    }
    pub fn SetInlineObject<P0, E0>(
        &self,
        characterindex: i32,
        charactercount: i32,
        inlineobject: P0,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasTextInlineObject>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetInlineObject)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    charactercount,
                    inlineobject.try_into().map_err(|e| e.into())?.abi(),
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn DrawToTextRenderer<P0, E0>(
        &self,
        textrenderer: P0,
        position: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasTextRenderer>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawToTextRenderer)(
                    ::windows::core::Vtable::as_raw(this),
                    textrenderer.try_into().map_err(|e| e.into())?.abi(),
                    position,
                )
                .ok()
        }
    }
    pub fn DrawToTextRendererWithCoords<P0, E0>(
        &self,
        textrenderer: P0,
        x: f32,
        y: f32,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasTextRenderer>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .DrawToTextRendererWithCoords)(
                    ::windows::core::Vtable::as_raw(this),
                    textrenderer.try_into().map_err(|e| e.into())?.abi(),
                    x,
                    y,
                )
                .ok()
        }
    }
    pub fn LineMetrics(
        &self,
    ) -> ::windows::core::Result<::windows::core::Array<CanvasLineMetrics>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LineMetrics)(
                    ::windows::core::Vtable::as_raw(this),
                    ::windows::core::Array::<
                        CanvasLineMetrics,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn ClusterMetrics(
        &self,
    ) -> ::windows::core::Result<::windows::core::Array<CanvasClusterMetrics>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ClusterMetrics)(
                    ::windows::core::Vtable::as_raw(this),
                    ::windows::core::Array::<
                        CanvasClusterMetrics,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn SetTypography(
        &self,
        characterindex: i32,
        charactercount: i32,
        typography: &CanvasTypography,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetTypography)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    charactercount,
                    ::core::mem::transmute_copy(typography),
                )
                .ok()
        }
    }
    pub fn GetTypography(
        &self,
        characterindex: i32,
    ) -> ::windows::core::Result<CanvasTypography> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetTypography)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn LayoutBounds(&self) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LayoutBounds)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn LayoutBoundsIncludingTrailingWhitespace(
        &self,
    ) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LayoutBoundsIncludingTrailingWhitespace)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn LineCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LineCount)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn MaximumBidiReorderingDepth(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .MaximumBidiReorderingDepth)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn DrawBounds(&self) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .DrawBounds)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn HitTest(
        &self,
        point: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .HitTest)(
                    ::windows::core::Vtable::as_raw(this),
                    point,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn HitTestWithCoords(&self, x: f32, y: f32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .HitTestWithCoords)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn HitTestWithDescription(
        &self,
        point: ::windows::Foundation::Numerics::Vector2,
        textlayoutregion: &mut CanvasTextLayoutRegion,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .HitTestWithDescription)(
                    ::windows::core::Vtable::as_raw(this),
                    point,
                    textlayoutregion,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn HitTestWithDescriptionAndCoords(
        &self,
        x: f32,
        y: f32,
        textlayoutregion: &mut CanvasTextLayoutRegion,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .HitTestWithDescriptionAndCoords)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    textlayoutregion,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn HitTestWithDescriptionAndTrailingSide(
        &self,
        point: ::windows::Foundation::Numerics::Vector2,
        textlayoutregion: &mut CanvasTextLayoutRegion,
        trailingsideofcharacter: &mut bool,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .HitTestWithDescriptionAndTrailingSide)(
                    ::windows::core::Vtable::as_raw(this),
                    point,
                    textlayoutregion,
                    trailingsideofcharacter,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn HitTestWithDescriptionAndCoordsAndTrailingSide(
        &self,
        x: f32,
        y: f32,
        textlayoutregion: &mut CanvasTextLayoutRegion,
        trailingsideofcharacter: &mut bool,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .HitTestWithDescriptionAndCoordsAndTrailingSide)(
                    ::windows::core::Vtable::as_raw(this),
                    x,
                    y,
                    textlayoutregion,
                    trailingsideofcharacter,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn GetCaretPosition(
        &self,
        characterindex: i32,
        trailingsideofcharacter: bool,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetCaretPosition)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    trailingsideofcharacter,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn GetCaretPositionWithDescription(
        &self,
        characterindex: i32,
        trailingsideofcharacter: bool,
        textlayoutregion: &mut CanvasTextLayoutRegion,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetCaretPositionWithDescription)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    trailingsideofcharacter,
                    textlayoutregion,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn GetCharacterRegions(
        &self,
        characterindex: i32,
        charactercount: i32,
    ) -> ::windows::core::Result<::windows::core::Array<CanvasTextLayoutRegion>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetCharacterRegions)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    charactercount,
                    ::windows::core::Array::<
                        CanvasTextLayoutRegion,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn GetPairKerning(&self, characterindex: i32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetPairKerning)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetPairKerning(
        &self,
        characterindex: i32,
        charactercount: i32,
        haspairkerning: bool,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetPairKerning)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    charactercount,
                    haspairkerning,
                )
                .ok()
        }
    }
    pub fn GetLeadingCharacterSpacing(
        &self,
        characterindex: i32,
    ) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetLeadingCharacterSpacing)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn GetTrailingCharacterSpacing(
        &self,
        characterindex: i32,
    ) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetTrailingCharacterSpacing)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn GetMinimumCharacterAdvance(
        &self,
        characterindex: i32,
    ) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetMinimumCharacterAdvance)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetCharacterSpacing(
        &self,
        characterindex: i32,
        charactercount: i32,
        leadingspacing: f32,
        trailingspacing: f32,
        minimumadvance: f32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetCharacterSpacing)(
                    ::windows::core::Vtable::as_raw(this),
                    characterindex,
                    charactercount,
                    leadingspacing,
                    trailingspacing,
                    minimumadvance,
                )
                .ok()
        }
    }
    pub fn VerticalGlyphOrientation(
        &self,
    ) -> ::windows::core::Result<CanvasVerticalGlyphOrientation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .VerticalGlyphOrientation)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetVerticalGlyphOrientation(
        &self,
        value: CanvasVerticalGlyphOrientation,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetVerticalGlyphOrientation)(
                    ::windows::core::Vtable::as_raw(this),
                    value,
                )
                .ok()
        }
    }
    pub fn OpticalAlignment(&self) -> ::windows::core::Result<CanvasOpticalAlignment> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .OpticalAlignment)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetOpticalAlignment(
        &self,
        value: CanvasOpticalAlignment,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetOpticalAlignment)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn LastLineWrapping(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LastLineWrapping)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetLastLineWrapping(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetLastLineWrapping)(::windows::core::Vtable::as_raw(this), value)
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
    pub fn Create<P0, E0>(
        resourcecreator: P0,
        textstring: &::windows::core::HSTRING,
        textformat: &CanvasTextFormat,
        requestedwidth: f32,
        requestedheight: f32,
    ) -> ::windows::core::Result<CanvasTextLayout>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasTextLayoutFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Create)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    ::core::mem::transmute_copy(textstring),
                    ::core::mem::transmute_copy(textformat),
                    requestedwidth,
                    requestedheight,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn GetGlyphOrientationTransform(
        glyphorientation: CanvasGlyphOrientation,
        issideways: bool,
        position: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Matrix3x2> {
        Self::ICanvasTextLayoutStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetGlyphOrientationTransform)(
                    ::windows::core::Vtable::as_raw(this),
                    glyphorientation,
                    issideways,
                    position,
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
    pub fn ICanvasTextLayoutFactory<
        R,
        F: FnOnce(&ICanvasTextLayoutFactory) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasTextLayout,
            ICanvasTextLayoutFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICanvasTextLayoutStatics<
        R,
        F: FnOnce(&ICanvasTextLayoutStatics) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasTextLayout,
            ICanvasTextLayoutStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CanvasTextLayout {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasTextLayout {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasTextLayout {}
impl ::core::fmt::Debug for CanvasTextLayout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasTextLayout").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasTextLayout {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.Text.CanvasTextLayout;{bae63e54-48ae-4446-a2c7-b6ef93806c20})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasTextLayout {
    type Vtable = ICanvasTextLayout_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasTextLayout {
    const IID: ::windows::core::GUID = <ICanvasTextLayout as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasTextLayout {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Text.CanvasTextLayout";
}
::windows::core::interface_hierarchy!(
    CanvasTextLayout,::windows::core::IUnknown,::windows::core::IInspectable
);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasTextLayout> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasTextLayout) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasTextLayout> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasTextLayout) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasTextLayout>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasTextLayout) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CanvasTextLayout {}
unsafe impl ::core::marker::Sync for CanvasTextLayout {}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
pub struct CanvasTextRenderingParameters(::windows::core::IUnknown);
impl CanvasTextRenderingParameters {
    pub fn RenderingMode(&self) -> ::windows::core::Result<CanvasTextRenderingMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .RenderingMode)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn GridFit(&self) -> ::windows::core::Result<CanvasTextGridFit> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GridFit)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn Create(
        textrenderingmode: CanvasTextRenderingMode,
        gridfit: CanvasTextGridFit,
    ) -> ::windows::core::Result<CanvasTextRenderingParameters> {
        Self::ICanvasTextRenderingParametersFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Create)(
                    ::windows::core::Vtable::as_raw(this),
                    textrenderingmode,
                    gridfit,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICanvasTextRenderingParametersFactory<
        R,
        F: FnOnce(&ICanvasTextRenderingParametersFactory) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasTextRenderingParameters,
            ICanvasTextRenderingParametersFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CanvasTextRenderingParameters {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasTextRenderingParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasTextRenderingParameters {}
impl ::core::fmt::Debug for CanvasTextRenderingParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasTextRenderingParameters").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasTextRenderingParameters {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.Text.CanvasTextRenderingParameters;{b20bf738-edb9-4eec-a12f-b6ae32e8ace6})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasTextRenderingParameters {
    type Vtable = ICanvasTextRenderingParameters_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasTextRenderingParameters {
    const IID: ::windows::core::GUID = <ICanvasTextRenderingParameters as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasTextRenderingParameters {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Text.CanvasTextRenderingParameters";
}
::windows::core::interface_hierarchy!(
    CanvasTextRenderingParameters,::windows::core::IUnknown,::windows::core::IInspectable
);
unsafe impl ::core::marker::Send for CanvasTextRenderingParameters {}
unsafe impl ::core::marker::Sync for CanvasTextRenderingParameters {}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
pub struct CanvasTypography(::windows::core::IUnknown);
impl CanvasTypography {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasTypography,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn AddFeature(
        &self,
        feature: CanvasTypographyFeature,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .AddFeature)(::windows::core::Vtable::as_raw(this), feature)
                .ok()
        }
    }
    pub fn AddFeatureWithNameAndParameter(
        &self,
        name: CanvasTypographyFeatureName,
        parameter: u32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .AddFeatureWithNameAndParameter)(
                    ::windows::core::Vtable::as_raw(this),
                    name,
                    parameter,
                )
                .ok()
        }
    }
    pub fn GetFeatures(
        &self,
    ) -> ::windows::core::Result<::windows::core::Array<CanvasTypographyFeature>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .GetFeatures)(
                    ::windows::core::Vtable::as_raw(this),
                    ::windows::core::Array::<
                        CanvasTypographyFeature,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
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
impl ::core::clone::Clone for CanvasTypography {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasTypography {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasTypography {}
impl ::core::fmt::Debug for CanvasTypography {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasTypography").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasTypography {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.Text.CanvasTypography;{f15bc312-447f-44ed-8bec-7e40f4a4dfc8})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasTypography {
    type Vtable = ICanvasTypography_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasTypography {
    const IID: ::windows::core::GUID = <ICanvasTypography as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasTypography {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Text.CanvasTypography";
}
::windows::core::interface_hierarchy!(
    CanvasTypography,::windows::core::IUnknown,::windows::core::IInspectable
);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasTypography> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasTypography) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasTypography> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasTypography) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasTypography>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasTypography) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CanvasTypography {}
unsafe impl ::core::marker::Sync for CanvasTypography {}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasClusterProperties(pub u32);
impl CanvasClusterProperties {
    pub const None: Self = Self(0u32);
    pub const CanWrapLineAfter: Self = Self(1u32);
    pub const Whitespace: Self = Self(2u32);
    pub const Newline: Self = Self(4u32);
    pub const SoftHyphen: Self = Self(8u32);
    pub const RightToLeft: Self = Self(16u32);
}
impl ::core::marker::Copy for CanvasClusterProperties {}
impl ::core::clone::Clone for CanvasClusterProperties {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasClusterProperties {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasClusterProperties {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasClusterProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasClusterProperties").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CanvasClusterProperties {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CanvasClusterProperties {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CanvasClusterProperties {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CanvasClusterProperties {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CanvasClusterProperties {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasClusterProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Text.CanvasClusterProperties;u4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasDrawTextOptions(pub u32);
impl CanvasDrawTextOptions {
    pub const Default: Self = Self(0u32);
    pub const NoPixelSnap: Self = Self(1u32);
    pub const Clip: Self = Self(2u32);
    pub const EnableColorFont: Self = Self(4u32);
}
impl ::core::marker::Copy for CanvasDrawTextOptions {}
impl ::core::clone::Clone for CanvasDrawTextOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasDrawTextOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasDrawTextOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasDrawTextOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasDrawTextOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CanvasDrawTextOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CanvasDrawTextOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CanvasDrawTextOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CanvasDrawTextOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CanvasDrawTextOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasDrawTextOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Text.CanvasDrawTextOptions;u4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasFontFileFormatType(pub i32);
impl CanvasFontFileFormatType {
    pub const Cff: Self = Self(0i32);
    pub const TrueType: Self = Self(1i32);
    pub const TrueTypeCollection: Self = Self(2i32);
    pub const Type1: Self = Self(3i32);
    pub const Vector: Self = Self(4i32);
    pub const Bitmap: Self = Self(5i32);
    pub const Unknown: Self = Self(6i32);
    pub const RawCff: Self = Self(7i32);
}
impl ::core::marker::Copy for CanvasFontFileFormatType {}
impl ::core::clone::Clone for CanvasFontFileFormatType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasFontFileFormatType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasFontFileFormatType {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasFontFileFormatType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasFontFileFormatType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasFontFileFormatType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Text.CanvasFontFileFormatType;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasFontInformation(pub i32);
impl CanvasFontInformation {
    pub const None: Self = Self(0i32);
    pub const CopyrightNotice: Self = Self(1i32);
    pub const VersionStrings: Self = Self(2i32);
    pub const Trademark: Self = Self(3i32);
    pub const Manufacturer: Self = Self(4i32);
    pub const Designer: Self = Self(5i32);
    pub const DesignerUrl: Self = Self(6i32);
    pub const Description: Self = Self(7i32);
    pub const FontVendorUrl: Self = Self(8i32);
    pub const LicenseDescription: Self = Self(9i32);
    pub const LicenseInfoUrl: Self = Self(10i32);
    pub const Win32FamilyNames: Self = Self(11i32);
    pub const Win32SubfamilyNames: Self = Self(12i32);
    pub const PreferredFamilyNames: Self = Self(13i32);
    pub const PreferredSubfamilyNames: Self = Self(14i32);
    pub const SampleText: Self = Self(15i32);
    pub const FullName: Self = Self(16i32);
    pub const PostscriptName: Self = Self(17i32);
    pub const PostscriptCidName: Self = Self(18i32);
    pub const WwsFamilyName: Self = Self(19i32);
    pub const DesignScriptLanguageTag: Self = Self(20i32);
    pub const SupportedScriptLanguageTag: Self = Self(21i32);
}
impl ::core::marker::Copy for CanvasFontInformation {}
impl ::core::clone::Clone for CanvasFontInformation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasFontInformation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasFontInformation {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasFontInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasFontInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasFontInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Text.CanvasFontInformation;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasFontPropertyIdentifier(pub i32);
impl CanvasFontPropertyIdentifier {
    pub const None: Self = Self(0i32);
    pub const FamilyName: Self = Self(1i32);
    pub const PreferredFamilyName: Self = Self(2i32);
    pub const FaceName: Self = Self(3i32);
    pub const FullName: Self = Self(4i32);
    pub const Win32FamilyName: Self = Self(5i32);
    pub const PostscriptName: Self = Self(6i32);
    pub const DesignScriptLanguageTag: Self = Self(7i32);
    pub const SupportedScriptLanguageTag: Self = Self(8i32);
    pub const SemanticTag: Self = Self(9i32);
    pub const Weight: Self = Self(10i32);
    pub const Stretch: Self = Self(11i32);
    pub const Style: Self = Self(12i32);
    pub const Total: Self = Self(13i32);
}
impl ::core::marker::Copy for CanvasFontPropertyIdentifier {}
impl ::core::clone::Clone for CanvasFontPropertyIdentifier {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasFontPropertyIdentifier {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasFontPropertyIdentifier {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasFontPropertyIdentifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasFontPropertyIdentifier").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasFontPropertyIdentifier {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Text.CanvasFontPropertyIdentifier;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasFontSimulations(pub u32);
impl CanvasFontSimulations {
    pub const None: Self = Self(0u32);
    pub const Bold: Self = Self(1u32);
    pub const Oblique: Self = Self(2u32);
}
impl ::core::marker::Copy for CanvasFontSimulations {}
impl ::core::clone::Clone for CanvasFontSimulations {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasFontSimulations {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasFontSimulations {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasFontSimulations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasFontSimulations").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CanvasFontSimulations {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CanvasFontSimulations {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CanvasFontSimulations {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CanvasFontSimulations {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CanvasFontSimulations {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasFontSimulations {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Text.CanvasFontSimulations;u4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasGlyphJustification(pub i32);
impl CanvasGlyphJustification {
    pub const None: Self = Self(0i32);
    pub const ArabicBlank: Self = Self(1i32);
    pub const Character: Self = Self(2i32);
    pub const Blank: Self = Self(4i32);
    pub const ArabicNormal: Self = Self(7i32);
    pub const ArabicKashida: Self = Self(8i32);
    pub const ArabicAlef: Self = Self(9i32);
    pub const ArabicHa: Self = Self(10i32);
    pub const ArabicRa: Self = Self(11i32);
    pub const ArabicBa: Self = Self(12i32);
    pub const ArabicBara: Self = Self(13i32);
    pub const ArabicSeen: Self = Self(14i32);
    pub const ArabicSeenM: Self = Self(15i32);
}
impl ::core::marker::Copy for CanvasGlyphJustification {}
impl ::core::clone::Clone for CanvasGlyphJustification {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasGlyphJustification {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasGlyphJustification {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasGlyphJustification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasGlyphJustification").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasGlyphJustification {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Text.CanvasGlyphJustification;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasGlyphOrientation(pub i32);
impl CanvasGlyphOrientation {
    pub const Upright: Self = Self(0i32);
    pub const Clockwise90Degrees: Self = Self(1i32);
    pub const Clockwise180Degrees: Self = Self(2i32);
    pub const Clockwise270Degrees: Self = Self(3i32);
}
impl ::core::marker::Copy for CanvasGlyphOrientation {}
impl ::core::clone::Clone for CanvasGlyphOrientation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasGlyphOrientation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasGlyphOrientation {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasGlyphOrientation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasGlyphOrientation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasGlyphOrientation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Text.CanvasGlyphOrientation;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasHorizontalAlignment(pub i32);
impl CanvasHorizontalAlignment {
    pub const Left: Self = Self(0i32);
    pub const Right: Self = Self(1i32);
    pub const Center: Self = Self(2i32);
    pub const Justified: Self = Self(3i32);
}
impl ::core::marker::Copy for CanvasHorizontalAlignment {}
impl ::core::clone::Clone for CanvasHorizontalAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasHorizontalAlignment {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasHorizontalAlignment {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasHorizontalAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasHorizontalAlignment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasHorizontalAlignment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Text.CanvasHorizontalAlignment;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasLineBreakCondition(pub i32);
impl CanvasLineBreakCondition {
    pub const Neutral: Self = Self(0i32);
    pub const CanBreak: Self = Self(1i32);
    pub const CannotBreak: Self = Self(2i32);
    pub const MustBreak: Self = Self(3i32);
}
impl ::core::marker::Copy for CanvasLineBreakCondition {}
impl ::core::clone::Clone for CanvasLineBreakCondition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasLineBreakCondition {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasLineBreakCondition {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasLineBreakCondition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasLineBreakCondition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasLineBreakCondition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Text.CanvasLineBreakCondition;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasLineSpacingMode(pub i32);
impl CanvasLineSpacingMode {
    pub const Default: Self = Self(0i32);
    pub const Uniform: Self = Self(1i32);
    pub const Proportional: Self = Self(2i32);
}
impl ::core::marker::Copy for CanvasLineSpacingMode {}
impl ::core::clone::Clone for CanvasLineSpacingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasLineSpacingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasLineSpacingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasLineSpacingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasLineSpacingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasLineSpacingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Text.CanvasLineSpacingMode;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasNumberSubstitutionMethod(pub i32);
impl CanvasNumberSubstitutionMethod {
    pub const FromCulture: Self = Self(0i32);
    pub const Contextual: Self = Self(1i32);
    pub const Disabled: Self = Self(2i32);
    pub const National: Self = Self(3i32);
    pub const Traditional: Self = Self(4i32);
}
impl ::core::marker::Copy for CanvasNumberSubstitutionMethod {}
impl ::core::clone::Clone for CanvasNumberSubstitutionMethod {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasNumberSubstitutionMethod {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasNumberSubstitutionMethod {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasNumberSubstitutionMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasNumberSubstitutionMethod").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasNumberSubstitutionMethod {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Text.CanvasNumberSubstitutionMethod;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasOpticalAlignment(pub i32);
impl CanvasOpticalAlignment {
    pub const Default: Self = Self(0i32);
    pub const NoSideBearings: Self = Self(1i32);
}
impl ::core::marker::Copy for CanvasOpticalAlignment {}
impl ::core::clone::Clone for CanvasOpticalAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasOpticalAlignment {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasOpticalAlignment {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasOpticalAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasOpticalAlignment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasOpticalAlignment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Text.CanvasOpticalAlignment;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasScriptShape(pub i32);
impl CanvasScriptShape {
    pub const Default: Self = Self(0i32);
    pub const NoVisual: Self = Self(1i32);
}
impl ::core::marker::Copy for CanvasScriptShape {}
impl ::core::clone::Clone for CanvasScriptShape {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasScriptShape {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasScriptShape {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasScriptShape {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasScriptShape").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasScriptShape {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Text.CanvasScriptShape;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasTextAntialiasing(pub i32);
impl CanvasTextAntialiasing {
    pub const Auto: Self = Self(0i32);
    pub const ClearType: Self = Self(1i32);
    pub const Grayscale: Self = Self(2i32);
    pub const Aliased: Self = Self(3i32);
}
impl ::core::marker::Copy for CanvasTextAntialiasing {}
impl ::core::clone::Clone for CanvasTextAntialiasing {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasTextAntialiasing {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasTextAntialiasing {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasTextAntialiasing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasTextAntialiasing").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasTextAntialiasing {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Text.CanvasTextAntialiasing;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasTextDirection(pub i32);
impl CanvasTextDirection {
    pub const LeftToRightThenTopToBottom: Self = Self(0i32);
    pub const RightToLeftThenTopToBottom: Self = Self(1i32);
    pub const LeftToRightThenBottomToTop: Self = Self(2i32);
    pub const RightToLeftThenBottomToTop: Self = Self(3i32);
    pub const TopToBottomThenLeftToRight: Self = Self(4i32);
    pub const BottomToTopThenLeftToRight: Self = Self(5i32);
    pub const TopToBottomThenRightToLeft: Self = Self(6i32);
    pub const BottomToTopThenRightToLeft: Self = Self(7i32);
}
impl ::core::marker::Copy for CanvasTextDirection {}
impl ::core::clone::Clone for CanvasTextDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasTextDirection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasTextDirection {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasTextDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasTextDirection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasTextDirection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Text.CanvasTextDirection;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasTextGridFit(pub i32);
impl CanvasTextGridFit {
    pub const Default: Self = Self(0i32);
    pub const Disable: Self = Self(1i32);
    pub const Enable: Self = Self(2i32);
}
impl ::core::marker::Copy for CanvasTextGridFit {}
impl ::core::clone::Clone for CanvasTextGridFit {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasTextGridFit {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasTextGridFit {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasTextGridFit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasTextGridFit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasTextGridFit {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Text.CanvasTextGridFit;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasTextMeasuringMode(pub i32);
impl CanvasTextMeasuringMode {
    pub const Natural: Self = Self(0i32);
    pub const GdiClassic: Self = Self(1i32);
    pub const GdiNatural: Self = Self(2i32);
}
impl ::core::marker::Copy for CanvasTextMeasuringMode {}
impl ::core::clone::Clone for CanvasTextMeasuringMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasTextMeasuringMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasTextMeasuringMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasTextMeasuringMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasTextMeasuringMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasTextMeasuringMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Text.CanvasTextMeasuringMode;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasTextRenderingMode(pub i32);
impl CanvasTextRenderingMode {
    pub const Default: Self = Self(0i32);
    pub const Aliased: Self = Self(1i32);
    pub const GdiClassic: Self = Self(2i32);
    pub const GdiNatural: Self = Self(3i32);
    pub const Natural: Self = Self(4i32);
    pub const NaturalSymmetric: Self = Self(5i32);
    pub const Outline: Self = Self(6i32);
    pub const NaturalSymmetricDownsampled: Self = Self(7i32);
}
impl ::core::marker::Copy for CanvasTextRenderingMode {}
impl ::core::clone::Clone for CanvasTextRenderingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasTextRenderingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasTextRenderingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasTextRenderingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasTextRenderingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasTextRenderingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Text.CanvasTextRenderingMode;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasTextTrimmingGranularity(pub i32);
impl CanvasTextTrimmingGranularity {
    pub const None: Self = Self(0i32);
    pub const Character: Self = Self(1i32);
    pub const Word: Self = Self(2i32);
}
impl ::core::marker::Copy for CanvasTextTrimmingGranularity {}
impl ::core::clone::Clone for CanvasTextTrimmingGranularity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasTextTrimmingGranularity {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasTextTrimmingGranularity {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasTextTrimmingGranularity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasTextTrimmingGranularity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasTextTrimmingGranularity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Text.CanvasTextTrimmingGranularity;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasTrimmingSign(pub i32);
impl CanvasTrimmingSign {
    pub const None: Self = Self(0i32);
    pub const Ellipsis: Self = Self(1i32);
}
impl ::core::marker::Copy for CanvasTrimmingSign {}
impl ::core::clone::Clone for CanvasTrimmingSign {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasTrimmingSign {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasTrimmingSign {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasTrimmingSign {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasTrimmingSign").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasTrimmingSign {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Text.CanvasTrimmingSign;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasTypographyFeatureName(pub i32);
impl CanvasTypographyFeatureName {
    pub const None: Self = Self(0i32);
    pub const Default: Self = Self(1953261156i32);
    pub const VerticalWriting: Self = Self(1953654134i32);
    pub const VerticalAlternatesAndRotation: Self = Self(846492278i32);
    pub const AlternativeFractions: Self = Self(1668441697i32);
    pub const PetiteCapitalsFromCapitals: Self = Self(1668297315i32);
    pub const SmallCapitalsFromCapitals: Self = Self(1668493923i32);
    pub const ContextualAlternates: Self = Self(1953259875i32);
    pub const CaseSensitiveForms: Self = Self(1702060387i32);
    pub const GlyphCompositionDecomposition: Self = Self(1886217059i32);
    pub const ContextualLigatures: Self = Self(1734962275i32);
    pub const CapitalSpacing: Self = Self(1886613603i32);
    pub const ContextualSwash: Self = Self(1752658787i32);
    pub const CursivePositioning: Self = Self(1936880995i32);
    pub const DiscretionaryLigatures: Self = Self(1734962276i32);
    pub const ExpertForms: Self = Self(1953527909i32);
    pub const Fractions: Self = Self(1667330662i32);
    pub const FullWidth: Self = Self(1684633446i32);
    pub const HalfForms: Self = Self(1718378856i32);
    pub const HalantForms: Self = Self(1852596584i32);
    pub const AlternateHalfWidth: Self = Self(1953259880i32);
    pub const HistoricalForms: Self = Self(1953720680i32);
    pub const HorizontalKanaAlternates: Self = Self(1634626408i32);
    pub const HistoricalLigatures: Self = Self(1734962280i32);
    pub const HalfWidth: Self = Self(1684633448i32);
    pub const HojoKanjiForms: Self = Self(1869246312i32);
    pub const Jis04Forms: Self = Self(875589738i32);
    pub const Jis78Forms: Self = Self(943157354i32);
    pub const Jis83Forms: Self = Self(859336810i32);
    pub const Jis90Forms: Self = Self(809070698i32);
    pub const Kerning: Self = Self(1852990827i32);
    pub const StandardLigatures: Self = Self(1634167148i32);
    pub const LiningFigures: Self = Self(1836412524i32);
    pub const LocalizedForms: Self = Self(1818455916i32);
    pub const MarkPositioning: Self = Self(1802658157i32);
    pub const MathematicalGreek: Self = Self(1802659693i32);
    pub const MarkToMarkPositioning: Self = Self(1802333037i32);
    pub const AlternateAnnotationForms: Self = Self(1953259886i32);
    pub const NlcKanjiForms: Self = Self(1801677934i32);
    pub const OldStyleFigures: Self = Self(1836412527i32);
    pub const Ordinals: Self = Self(1852076655i32);
    pub const ProportionalAlternateWidth: Self = Self(1953259888i32);
    pub const PetiteCapitals: Self = Self(1885430640i32);
    pub const ProportionalFigures: Self = Self(1836412528i32);
    pub const ProportionalWidths: Self = Self(1684633456i32);
    pub const QuarterWidths: Self = Self(1684633457i32);
    pub const RequiredLigatures: Self = Self(1734962290i32);
    pub const RubyNotationForms: Self = Self(2036495730i32);
    pub const StylisticAlternates: Self = Self(1953259891i32);
    pub const ScientificInferiors: Self = Self(1718511987i32);
    pub const SmallCapitals: Self = Self(1885564275i32);
    pub const SimplifiedForms: Self = Self(1819307379i32);
    pub const StylisticSet1: Self = Self(825258867i32);
    pub const StylisticSet2: Self = Self(842036083i32);
    pub const StylisticSet3: Self = Self(858813299i32);
    pub const StylisticSet4: Self = Self(875590515i32);
    pub const StylisticSet5: Self = Self(892367731i32);
    pub const StylisticSet6: Self = Self(909144947i32);
    pub const StylisticSet7: Self = Self(925922163i32);
    pub const StylisticSet8: Self = Self(942699379i32);
    pub const StylisticSet9: Self = Self(959476595i32);
    pub const StylisticSet10: Self = Self(808547187i32);
    pub const StylisticSet11: Self = Self(825324403i32);
    pub const StylisticSet12: Self = Self(842101619i32);
    pub const StylisticSet13: Self = Self(858878835i32);
    pub const StylisticSet14: Self = Self(875656051i32);
    pub const StylisticSet15: Self = Self(892433267i32);
    pub const StylisticSet16: Self = Self(909210483i32);
    pub const StylisticSet17: Self = Self(925987699i32);
    pub const StylisticSet18: Self = Self(942764915i32);
    pub const StylisticSet19: Self = Self(959542131i32);
    pub const StylisticSet20: Self = Self(808612723i32);
    pub const Subscript: Self = Self(1935832435i32);
    pub const Superscript: Self = Self(1936749939i32);
    pub const Swash: Self = Self(1752397683i32);
    pub const Titling: Self = Self(1819568500i32);
    pub const TraditionalNameForms: Self = Self(1835101812i32);
    pub const TabularFigures: Self = Self(1836412532i32);
    pub const TraditionalForms: Self = Self(1684107892i32);
    pub const ThirdWidths: Self = Self(1684633460i32);
    pub const Unicase: Self = Self(1667853941i32);
    pub const SlashedZero: Self = Self(1869768058i32);
}
impl ::core::marker::Copy for CanvasTypographyFeatureName {}
impl ::core::clone::Clone for CanvasTypographyFeatureName {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasTypographyFeatureName {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasTypographyFeatureName {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasTypographyFeatureName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasTypographyFeatureName").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasTypographyFeatureName {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Text.CanvasTypographyFeatureName;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasVerticalAlignment(pub i32);
impl CanvasVerticalAlignment {
    pub const Top: Self = Self(0i32);
    pub const Bottom: Self = Self(1i32);
    pub const Center: Self = Self(2i32);
}
impl ::core::marker::Copy for CanvasVerticalAlignment {}
impl ::core::clone::Clone for CanvasVerticalAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasVerticalAlignment {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasVerticalAlignment {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasVerticalAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasVerticalAlignment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasVerticalAlignment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Text.CanvasVerticalAlignment;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasVerticalGlyphOrientation(pub i32);
impl CanvasVerticalGlyphOrientation {
    pub const Default: Self = Self(0i32);
    pub const Stacked: Self = Self(1i32);
}
impl ::core::marker::Copy for CanvasVerticalGlyphOrientation {}
impl ::core::clone::Clone for CanvasVerticalGlyphOrientation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasVerticalGlyphOrientation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasVerticalGlyphOrientation {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasVerticalGlyphOrientation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasVerticalGlyphOrientation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasVerticalGlyphOrientation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Text.CanvasVerticalGlyphOrientation;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Text"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasWordWrapping(pub i32);
impl CanvasWordWrapping {
    pub const Wrap: Self = Self(0i32);
    pub const NoWrap: Self = Self(1i32);
    pub const EmergencyBreak: Self = Self(2i32);
    pub const WholeWord: Self = Self(3i32);
    pub const Character: Self = Self(4i32);
}
impl ::core::marker::Copy for CanvasWordWrapping {}
impl ::core::clone::Clone for CanvasWordWrapping {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasWordWrapping {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasWordWrapping {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasWordWrapping {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasWordWrapping").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasWordWrapping {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Text.CanvasWordWrapping;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
///*Required features: `"Graphics_Canvas_Text"`*
pub struct CanvasAnalyzedBidi {
    pub ExplicitLevel: u32,
    pub ResolvedLevel: u32,
}
impl ::core::marker::Copy for CanvasAnalyzedBidi {}
impl ::core::clone::Clone for CanvasAnalyzedBidi {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CanvasAnalyzedBidi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CanvasAnalyzedBidi")
            .field("ExplicitLevel", &self.ExplicitLevel)
            .field("ResolvedLevel", &self.ResolvedLevel)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for CanvasAnalyzedBidi {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CanvasAnalyzedBidi {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.Graphics.Canvas.Text.CanvasAnalyzedBidi;u4;u4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for CanvasAnalyzedBidi {
    fn eq(&self, other: &Self) -> bool {
        self.ExplicitLevel == other.ExplicitLevel
            && self.ResolvedLevel == other.ResolvedLevel
    }
}
impl ::core::cmp::Eq for CanvasAnalyzedBidi {}
impl ::core::default::Default for CanvasAnalyzedBidi {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
///*Required features: `"Graphics_Canvas_Text"`*
pub struct CanvasAnalyzedBreakpoint {
    pub BreakBefore: CanvasLineBreakCondition,
    pub BreakAfter: CanvasLineBreakCondition,
    pub IsWhitespace: bool,
    pub IsSoftHyphen: bool,
}
impl ::core::marker::Copy for CanvasAnalyzedBreakpoint {}
impl ::core::clone::Clone for CanvasAnalyzedBreakpoint {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CanvasAnalyzedBreakpoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CanvasAnalyzedBreakpoint")
            .field("BreakBefore", &self.BreakBefore)
            .field("BreakAfter", &self.BreakAfter)
            .field("IsWhitespace", &self.IsWhitespace)
            .field("IsSoftHyphen", &self.IsSoftHyphen)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for CanvasAnalyzedBreakpoint {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CanvasAnalyzedBreakpoint {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.Graphics.Canvas.Text.CanvasAnalyzedBreakpoint;enum(Microsoft.Graphics.Canvas.Text.CanvasLineBreakCondition;i4);enum(Microsoft.Graphics.Canvas.Text.CanvasLineBreakCondition;i4);b1;b1)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for CanvasAnalyzedBreakpoint {
    fn eq(&self, other: &Self) -> bool {
        self.BreakBefore == other.BreakBefore && self.BreakAfter == other.BreakAfter
            && self.IsWhitespace == other.IsWhitespace
            && self.IsSoftHyphen == other.IsSoftHyphen
    }
}
impl ::core::cmp::Eq for CanvasAnalyzedBreakpoint {}
impl ::core::default::Default for CanvasAnalyzedBreakpoint {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
///*Required features: `"Graphics_Canvas_Text"`*
pub struct CanvasAnalyzedGlyphOrientation {
    pub GlyphOrientation: CanvasGlyphOrientation,
    pub AdjustedBidiLevel: u32,
    pub IsSideways: bool,
    pub IsRightToLeft: bool,
}
impl ::core::marker::Copy for CanvasAnalyzedGlyphOrientation {}
impl ::core::clone::Clone for CanvasAnalyzedGlyphOrientation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CanvasAnalyzedGlyphOrientation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CanvasAnalyzedGlyphOrientation")
            .field("GlyphOrientation", &self.GlyphOrientation)
            .field("AdjustedBidiLevel", &self.AdjustedBidiLevel)
            .field("IsSideways", &self.IsSideways)
            .field("IsRightToLeft", &self.IsRightToLeft)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for CanvasAnalyzedGlyphOrientation {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CanvasAnalyzedGlyphOrientation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.Graphics.Canvas.Text.CanvasAnalyzedGlyphOrientation;enum(Microsoft.Graphics.Canvas.Text.CanvasGlyphOrientation;i4);u4;b1;b1)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for CanvasAnalyzedGlyphOrientation {
    fn eq(&self, other: &Self) -> bool {
        self.GlyphOrientation == other.GlyphOrientation
            && self.AdjustedBidiLevel == other.AdjustedBidiLevel
            && self.IsSideways == other.IsSideways
            && self.IsRightToLeft == other.IsRightToLeft
    }
}
impl ::core::cmp::Eq for CanvasAnalyzedGlyphOrientation {}
impl ::core::default::Default for CanvasAnalyzedGlyphOrientation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
///*Required features: `"Graphics_Canvas_Text"`*
pub struct CanvasAnalyzedScript {
    pub ScriptIdentifier: i32,
    pub Shape: CanvasScriptShape,
}
impl ::core::marker::Copy for CanvasAnalyzedScript {}
impl ::core::clone::Clone for CanvasAnalyzedScript {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CanvasAnalyzedScript {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CanvasAnalyzedScript")
            .field("ScriptIdentifier", &self.ScriptIdentifier)
            .field("Shape", &self.Shape)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for CanvasAnalyzedScript {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CanvasAnalyzedScript {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.Graphics.Canvas.Text.CanvasAnalyzedScript;i4;enum(Microsoft.Graphics.Canvas.Text.CanvasScriptShape;i4))",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for CanvasAnalyzedScript {
    fn eq(&self, other: &Self) -> bool {
        self.ScriptIdentifier == other.ScriptIdentifier && self.Shape == other.Shape
    }
}
impl ::core::cmp::Eq for CanvasAnalyzedScript {}
impl ::core::default::Default for CanvasAnalyzedScript {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
///*Required features: `"Graphics_Canvas_Text"`*
pub struct CanvasCharacterRange {
    pub CharacterIndex: i32,
    pub CharacterCount: i32,
}
impl ::core::marker::Copy for CanvasCharacterRange {}
impl ::core::clone::Clone for CanvasCharacterRange {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CanvasCharacterRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CanvasCharacterRange")
            .field("CharacterIndex", &self.CharacterIndex)
            .field("CharacterCount", &self.CharacterCount)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for CanvasCharacterRange {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CanvasCharacterRange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.Graphics.Canvas.Text.CanvasCharacterRange;i4;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for CanvasCharacterRange {
    fn eq(&self, other: &Self) -> bool {
        self.CharacterIndex == other.CharacterIndex
            && self.CharacterCount == other.CharacterCount
    }
}
impl ::core::cmp::Eq for CanvasCharacterRange {}
impl ::core::default::Default for CanvasCharacterRange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
///*Required features: `"Graphics_Canvas_Text"`*
pub struct CanvasClusterMetrics {
    pub CharacterCount: i32,
    pub Width: f32,
    pub Properties: CanvasClusterProperties,
}
impl ::core::marker::Copy for CanvasClusterMetrics {}
impl ::core::clone::Clone for CanvasClusterMetrics {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CanvasClusterMetrics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CanvasClusterMetrics")
            .field("CharacterCount", &self.CharacterCount)
            .field("Width", &self.Width)
            .field("Properties", &self.Properties)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for CanvasClusterMetrics {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CanvasClusterMetrics {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.Graphics.Canvas.Text.CanvasClusterMetrics;i4;f4;enum(Microsoft.Graphics.Canvas.Text.CanvasClusterProperties;u4))",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for CanvasClusterMetrics {
    fn eq(&self, other: &Self) -> bool {
        self.CharacterCount == other.CharacterCount && self.Width == other.Width
            && self.Properties == other.Properties
    }
}
impl ::core::cmp::Eq for CanvasClusterMetrics {}
impl ::core::default::Default for CanvasClusterMetrics {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
///*Required features: `"Graphics_Canvas_Text"`*
pub struct CanvasFontProperty {
    pub Identifier: CanvasFontPropertyIdentifier,
    pub Value: ::windows::core::HSTRING,
    pub Locale: ::windows::core::HSTRING,
}
impl ::core::clone::Clone for CanvasFontProperty {
    fn clone(&self) -> Self {
        Self {
            Identifier: self.Identifier,
            Value: self.Value.clone(),
            Locale: self.Locale.clone(),
        }
    }
}
impl ::core::fmt::Debug for CanvasFontProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CanvasFontProperty")
            .field("Identifier", &self.Identifier)
            .field("Value", &self.Value)
            .field("Locale", &self.Locale)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for CanvasFontProperty {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
unsafe impl ::windows::core::RuntimeType for CanvasFontProperty {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.Graphics.Canvas.Text.CanvasFontProperty;enum(Microsoft.Graphics.Canvas.Text.CanvasFontPropertyIdentifier;i4);string;string)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(from.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasFontProperty {
    fn eq(&self, other: &Self) -> bool {
        self.Identifier == other.Identifier && self.Value == other.Value
            && self.Locale == other.Locale
    }
}
impl ::core::cmp::Eq for CanvasFontProperty {}
impl ::core::default::Default for CanvasFontProperty {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
///*Required features: `"Graphics_Canvas_Text"`*
pub struct CanvasGlyph {
    pub Index: i32,
    pub Advance: f32,
    pub AdvanceOffset: f32,
    pub AscenderOffset: f32,
}
impl ::core::marker::Copy for CanvasGlyph {}
impl ::core::clone::Clone for CanvasGlyph {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CanvasGlyph {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CanvasGlyph")
            .field("Index", &self.Index)
            .field("Advance", &self.Advance)
            .field("AdvanceOffset", &self.AdvanceOffset)
            .field("AscenderOffset", &self.AscenderOffset)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for CanvasGlyph {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CanvasGlyph {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.Graphics.Canvas.Text.CanvasGlyph;i4;f4;f4;f4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for CanvasGlyph {
    fn eq(&self, other: &Self) -> bool {
        self.Index == other.Index && self.Advance == other.Advance
            && self.AdvanceOffset == other.AdvanceOffset
            && self.AscenderOffset == other.AscenderOffset
    }
}
impl ::core::cmp::Eq for CanvasGlyph {}
impl ::core::default::Default for CanvasGlyph {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
///*Required features: `"Graphics_Canvas_Text"`, `"Foundation"`*
#[cfg(feature = "Foundation")]
pub struct CanvasGlyphMetrics {
    pub LeftSideBearing: f32,
    pub AdvanceWidth: f32,
    pub RightSideBearing: f32,
    pub TopSideBearing: f32,
    pub AdvanceHeight: f32,
    pub BottomSideBearing: f32,
    pub VerticalOrigin: f32,
    pub DrawBounds: ::windows::Foundation::Rect,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for CanvasGlyphMetrics {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for CanvasGlyphMetrics {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for CanvasGlyphMetrics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CanvasGlyphMetrics")
            .field("LeftSideBearing", &self.LeftSideBearing)
            .field("AdvanceWidth", &self.AdvanceWidth)
            .field("RightSideBearing", &self.RightSideBearing)
            .field("TopSideBearing", &self.TopSideBearing)
            .field("AdvanceHeight", &self.AdvanceHeight)
            .field("BottomSideBearing", &self.BottomSideBearing)
            .field("VerticalOrigin", &self.VerticalOrigin)
            .field("DrawBounds", &self.DrawBounds)
            .finish()
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Abi for CanvasGlyphMetrics {
    type Abi = Self;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for CanvasGlyphMetrics {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.Graphics.Canvas.Text.CanvasGlyphMetrics;f4;f4;f4;f4;f4;f4;f4;struct(Windows.Foundation.Rect;f4;f4;f4;f4))",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for CanvasGlyphMetrics {
    fn eq(&self, other: &Self) -> bool {
        self.LeftSideBearing == other.LeftSideBearing
            && self.AdvanceWidth == other.AdvanceWidth
            && self.RightSideBearing == other.RightSideBearing
            && self.TopSideBearing == other.TopSideBearing
            && self.AdvanceHeight == other.AdvanceHeight
            && self.BottomSideBearing == other.BottomSideBearing
            && self.VerticalOrigin == other.VerticalOrigin
            && self.DrawBounds == other.DrawBounds
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for CanvasGlyphMetrics {}
#[cfg(feature = "Foundation")]
impl ::core::default::Default for CanvasGlyphMetrics {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
///*Required features: `"Graphics_Canvas_Text"`*
pub struct CanvasGlyphShaping {
    pub Justification: CanvasGlyphJustification,
    pub IsClusterStart: bool,
    pub IsDiacritic: bool,
    pub IsZeroWidthSpace: bool,
}
impl ::core::marker::Copy for CanvasGlyphShaping {}
impl ::core::clone::Clone for CanvasGlyphShaping {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CanvasGlyphShaping {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CanvasGlyphShaping")
            .field("Justification", &self.Justification)
            .field("IsClusterStart", &self.IsClusterStart)
            .field("IsDiacritic", &self.IsDiacritic)
            .field("IsZeroWidthSpace", &self.IsZeroWidthSpace)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for CanvasGlyphShaping {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CanvasGlyphShaping {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.Graphics.Canvas.Text.CanvasGlyphShaping;enum(Microsoft.Graphics.Canvas.Text.CanvasGlyphJustification;i4);b1;b1;b1)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for CanvasGlyphShaping {
    fn eq(&self, other: &Self) -> bool {
        self.Justification == other.Justification
            && self.IsClusterStart == other.IsClusterStart
            && self.IsDiacritic == other.IsDiacritic
            && self.IsZeroWidthSpace == other.IsZeroWidthSpace
    }
}
impl ::core::cmp::Eq for CanvasGlyphShaping {}
impl ::core::default::Default for CanvasGlyphShaping {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
///*Required features: `"Graphics_Canvas_Text"`*
pub struct CanvasJustificationOpportunity {
    pub ExpansionMinimum: f32,
    pub ExpansionMaximum: f32,
    pub CompressionMaximum: f32,
    pub ExpansionPriority: u8,
    pub CompressionPriority: u8,
    pub AllowResidualExpansion: bool,
    pub AllowResidualCompression: bool,
    pub ApplyToLeadingEdge: bool,
    pub ApplyToTrailingEdge: bool,
}
impl ::core::marker::Copy for CanvasJustificationOpportunity {}
impl ::core::clone::Clone for CanvasJustificationOpportunity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CanvasJustificationOpportunity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CanvasJustificationOpportunity")
            .field("ExpansionMinimum", &self.ExpansionMinimum)
            .field("ExpansionMaximum", &self.ExpansionMaximum)
            .field("CompressionMaximum", &self.CompressionMaximum)
            .field("ExpansionPriority", &self.ExpansionPriority)
            .field("CompressionPriority", &self.CompressionPriority)
            .field("AllowResidualExpansion", &self.AllowResidualExpansion)
            .field("AllowResidualCompression", &self.AllowResidualCompression)
            .field("ApplyToLeadingEdge", &self.ApplyToLeadingEdge)
            .field("ApplyToTrailingEdge", &self.ApplyToTrailingEdge)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for CanvasJustificationOpportunity {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CanvasJustificationOpportunity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.Graphics.Canvas.Text.CanvasJustificationOpportunity;f4;f4;f4;u1;u1;b1;b1;b1;b1)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for CanvasJustificationOpportunity {
    fn eq(&self, other: &Self) -> bool {
        self.ExpansionMinimum == other.ExpansionMinimum
            && self.ExpansionMaximum == other.ExpansionMaximum
            && self.CompressionMaximum == other.CompressionMaximum
            && self.ExpansionPriority == other.ExpansionPriority
            && self.CompressionPriority == other.CompressionPriority
            && self.AllowResidualExpansion == other.AllowResidualExpansion
            && self.AllowResidualCompression == other.AllowResidualCompression
            && self.ApplyToLeadingEdge == other.ApplyToLeadingEdge
            && self.ApplyToTrailingEdge == other.ApplyToTrailingEdge
    }
}
impl ::core::cmp::Eq for CanvasJustificationOpportunity {}
impl ::core::default::Default for CanvasJustificationOpportunity {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
///*Required features: `"Graphics_Canvas_Text"`*
pub struct CanvasLineMetrics {
    pub CharacterCount: i32,
    pub TrailingWhitespaceCount: i32,
    pub TerminalNewlineCount: i32,
    pub Height: f32,
    pub Baseline: f32,
    pub IsTrimmed: bool,
    pub LeadingWhitespaceBefore: f32,
    pub LeadingWhitespaceAfter: f32,
}
impl ::core::marker::Copy for CanvasLineMetrics {}
impl ::core::clone::Clone for CanvasLineMetrics {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CanvasLineMetrics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CanvasLineMetrics")
            .field("CharacterCount", &self.CharacterCount)
            .field("TrailingWhitespaceCount", &self.TrailingWhitespaceCount)
            .field("TerminalNewlineCount", &self.TerminalNewlineCount)
            .field("Height", &self.Height)
            .field("Baseline", &self.Baseline)
            .field("IsTrimmed", &self.IsTrimmed)
            .field("LeadingWhitespaceBefore", &self.LeadingWhitespaceBefore)
            .field("LeadingWhitespaceAfter", &self.LeadingWhitespaceAfter)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for CanvasLineMetrics {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CanvasLineMetrics {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.Graphics.Canvas.Text.CanvasLineMetrics;i4;i4;i4;f4;f4;b1;f4;f4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for CanvasLineMetrics {
    fn eq(&self, other: &Self) -> bool {
        self.CharacterCount == other.CharacterCount
            && self.TrailingWhitespaceCount == other.TrailingWhitespaceCount
            && self.TerminalNewlineCount == other.TerminalNewlineCount
            && self.Height == other.Height && self.Baseline == other.Baseline
            && self.IsTrimmed == other.IsTrimmed
            && self.LeadingWhitespaceBefore == other.LeadingWhitespaceBefore
            && self.LeadingWhitespaceAfter == other.LeadingWhitespaceAfter
    }
}
impl ::core::cmp::Eq for CanvasLineMetrics {}
impl ::core::default::Default for CanvasLineMetrics {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
///*Required features: `"Graphics_Canvas_Text"`*
pub struct CanvasScriptProperties {
    pub IsoScriptCode: ::windows::core::HSTRING,
    pub IsoScriptNumber: i32,
    pub ClusterLookahead: i32,
    pub JustificationCharacter: ::windows::core::HSTRING,
    pub RestrictCaretToClusters: bool,
    pub UsesWordDividers: bool,
    pub IsDiscreteWriting: bool,
    pub IsBlockWriting: bool,
    pub IsDistributedWithinCluster: bool,
    pub IsConnectedWriting: bool,
    pub IsCursiveWriting: bool,
}
impl ::core::clone::Clone for CanvasScriptProperties {
    fn clone(&self) -> Self {
        Self {
            IsoScriptCode: self.IsoScriptCode.clone(),
            IsoScriptNumber: self.IsoScriptNumber,
            ClusterLookahead: self.ClusterLookahead,
            JustificationCharacter: self.JustificationCharacter.clone(),
            RestrictCaretToClusters: self.RestrictCaretToClusters,
            UsesWordDividers: self.UsesWordDividers,
            IsDiscreteWriting: self.IsDiscreteWriting,
            IsBlockWriting: self.IsBlockWriting,
            IsDistributedWithinCluster: self.IsDistributedWithinCluster,
            IsConnectedWriting: self.IsConnectedWriting,
            IsCursiveWriting: self.IsCursiveWriting,
        }
    }
}
impl ::core::fmt::Debug for CanvasScriptProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CanvasScriptProperties")
            .field("IsoScriptCode", &self.IsoScriptCode)
            .field("IsoScriptNumber", &self.IsoScriptNumber)
            .field("ClusterLookahead", &self.ClusterLookahead)
            .field("JustificationCharacter", &self.JustificationCharacter)
            .field("RestrictCaretToClusters", &self.RestrictCaretToClusters)
            .field("UsesWordDividers", &self.UsesWordDividers)
            .field("IsDiscreteWriting", &self.IsDiscreteWriting)
            .field("IsBlockWriting", &self.IsBlockWriting)
            .field("IsDistributedWithinCluster", &self.IsDistributedWithinCluster)
            .field("IsConnectedWriting", &self.IsConnectedWriting)
            .field("IsCursiveWriting", &self.IsCursiveWriting)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for CanvasScriptProperties {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
unsafe impl ::windows::core::RuntimeType for CanvasScriptProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.Graphics.Canvas.Text.CanvasScriptProperties;string;i4;i4;string;b1;b1;b1;b1;b1;b1;b1)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(from.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasScriptProperties {
    fn eq(&self, other: &Self) -> bool {
        self.IsoScriptCode == other.IsoScriptCode
            && self.IsoScriptNumber == other.IsoScriptNumber
            && self.ClusterLookahead == other.ClusterLookahead
            && self.JustificationCharacter == other.JustificationCharacter
            && self.RestrictCaretToClusters == other.RestrictCaretToClusters
            && self.UsesWordDividers == other.UsesWordDividers
            && self.IsDiscreteWriting == other.IsDiscreteWriting
            && self.IsBlockWriting == other.IsBlockWriting
            && self.IsDistributedWithinCluster == other.IsDistributedWithinCluster
            && self.IsConnectedWriting == other.IsConnectedWriting
            && self.IsCursiveWriting == other.IsCursiveWriting
    }
}
impl ::core::cmp::Eq for CanvasScriptProperties {}
impl ::core::default::Default for CanvasScriptProperties {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
///*Required features: `"Graphics_Canvas_Text"`, `"Foundation"`*
#[cfg(feature = "Foundation")]
pub struct CanvasTextLayoutRegion {
    pub CharacterIndex: i32,
    pub CharacterCount: i32,
    pub LayoutBounds: ::windows::Foundation::Rect,
}
#[cfg(feature = "Foundation")]
impl ::core::marker::Copy for CanvasTextLayoutRegion {}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for CanvasTextLayoutRegion {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for CanvasTextLayoutRegion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CanvasTextLayoutRegion")
            .field("CharacterIndex", &self.CharacterIndex)
            .field("CharacterCount", &self.CharacterCount)
            .field("LayoutBounds", &self.LayoutBounds)
            .finish()
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Abi for CanvasTextLayoutRegion {
    type Abi = Self;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for CanvasTextLayoutRegion {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.Graphics.Canvas.Text.CanvasTextLayoutRegion;i4;i4;struct(Windows.Foundation.Rect;f4;f4;f4;f4))",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for CanvasTextLayoutRegion {
    fn eq(&self, other: &Self) -> bool {
        self.CharacterIndex == other.CharacterIndex
            && self.CharacterCount == other.CharacterCount
            && self.LayoutBounds == other.LayoutBounds
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for CanvasTextLayoutRegion {}
#[cfg(feature = "Foundation")]
impl ::core::default::Default for CanvasTextLayoutRegion {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
///*Required features: `"Graphics_Canvas_Text"`*
pub struct CanvasTypographyFeature {
    pub Name: CanvasTypographyFeatureName,
    pub Parameter: u32,
}
impl ::core::marker::Copy for CanvasTypographyFeature {}
impl ::core::clone::Clone for CanvasTypographyFeature {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CanvasTypographyFeature {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CanvasTypographyFeature")
            .field("Name", &self.Name)
            .field("Parameter", &self.Parameter)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for CanvasTypographyFeature {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CanvasTypographyFeature {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.Graphics.Canvas.Text.CanvasTypographyFeature;enum(Microsoft.Graphics.Canvas.Text.CanvasTypographyFeatureName;i4);u4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for CanvasTypographyFeature {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Parameter == other.Parameter
    }
}
impl ::core::cmp::Eq for CanvasTypographyFeature {}
impl ::core::default::Default for CanvasTypographyFeature {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
///*Required features: `"Graphics_Canvas_Text"`*
pub struct CanvasUnicodeRange {
    pub First: u32,
    pub Last: u32,
}
impl ::core::marker::Copy for CanvasUnicodeRange {}
impl ::core::clone::Clone for CanvasUnicodeRange {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CanvasUnicodeRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CanvasUnicodeRange")
            .field("First", &self.First)
            .field("Last", &self.Last)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for CanvasUnicodeRange {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CanvasUnicodeRange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.Graphics.Canvas.Text.CanvasUnicodeRange;u4;u4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for CanvasUnicodeRange {
    fn eq(&self, other: &Self) -> bool {
        self.First == other.First && self.Last == other.Last
    }
}
impl ::core::cmp::Eq for CanvasUnicodeRange {}
impl ::core::default::Default for CanvasUnicodeRange {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
