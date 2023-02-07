#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasCachedGeometry(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasCachedGeometry {
    type Vtable = ICanvasCachedGeometry_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasCachedGeometry {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xba6cb114_e1a1_448d_ab7c_8d2b92674119,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasCachedGeometry_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Device: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasCachedGeometryStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasCachedGeometryStatics {
    type Vtable = ICanvasCachedGeometryStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasCachedGeometryStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x80ba1060_a9d7_41ba_9372_ec3fc1744e5d,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasCachedGeometryStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateFill: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateFillWithFlatteningTolerance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        flatteningtolerance: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateStroke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        strokewidth: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateStrokeWithStrokeStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateStrokeWithStrokeStyleAndFlatteningTolerance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
        flatteningtolerance: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasGeometry(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasGeometry {
    type Vtable = ICanvasGeometry_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasGeometry {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x74ea89fa_c87c_4d0d_9057_2743b8db67ee,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasGeometry_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub CombineWith: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        othergeometry: *mut ::core::ffi::c_void,
        othergeometrytransform: ::windows::Foundation::Numerics::Matrix3x2,
        combine: CanvasGeometryCombine,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CombineWith: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CombineWithUsingFlatteningTolerance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        othergeometry: *mut ::core::ffi::c_void,
        othergeometrytransform: ::windows::Foundation::Numerics::Matrix3x2,
        combine: CanvasGeometryCombine,
        flatteningtolerance: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CombineWithUsingFlatteningTolerance: usize,
    pub Stroke: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        strokewidth: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub StrokeWithStrokeStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub StrokeWithAllOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        flatteningtolerance: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    StrokeWithAllOptions: usize,
    pub Outline: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub OutlineWithTransformAndFlatteningTolerance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        flatteningtolerance: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    OutlineWithTransformAndFlatteningTolerance: usize,
    pub Simplify: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        simplification: CanvasGeometrySimplification,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub SimplifyWithTransformAndFlatteningTolerance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        simplification: CanvasGeometrySimplification,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        flatteningtolerance: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SimplifyWithTransformAndFlatteningTolerance: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Transform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Transform: usize,
    pub CompareWith: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        othergeometry: *mut ::core::ffi::c_void,
        result__: *mut CanvasGeometryRelation,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub CompareWithUsingTransformAndFlatteningTolerance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        othergeometry: *mut ::core::ffi::c_void,
        othergeometrytransform: ::windows::Foundation::Numerics::Matrix3x2,
        flatteningtolerance: f32,
        result__: *mut CanvasGeometryRelation,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CompareWithUsingTransformAndFlatteningTolerance: usize,
    pub ComputeArea: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub ComputeAreaWithTransformAndFlatteningTolerance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        flatteningtolerance: f32,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ComputeAreaWithTransformAndFlatteningTolerance: usize,
    pub ComputePathLength: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub ComputePathLengthWithTransformAndFlatteningTolerance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        flatteningtolerance: f32,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ComputePathLengthWithTransformAndFlatteningTolerance: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub ComputePointOnPath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        distance: f32,
        result__: *mut ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ComputePointOnPath: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub ComputePointOnPathWithTangent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        distance: f32,
        tangent: *mut ::windows::Foundation::Numerics::Vector2,
        result__: *mut ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ComputePointOnPathWithTangent: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub ComputePointOnPathWithTransformAndFlatteningToleranceAndTangent: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        length: f32,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        flatteningtolerance: f32,
        tangent: *mut ::windows::Foundation::Numerics::Vector2,
        result__: *mut ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ComputePointOnPathWithTransformAndFlatteningToleranceAndTangent: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub FillContainsPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        point: ::windows::Foundation::Numerics::Vector2,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    FillContainsPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub FillContainsPointWithTransformAndFlatteningTolerance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        point: ::windows::Foundation::Numerics::Vector2,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        flatteningtolerance: f32,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    FillContainsPointWithTransformAndFlatteningTolerance: usize,
    #[cfg(feature = "Foundation")]
    pub ComputeBounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ComputeBounds: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub ComputeBoundsWithTransform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ComputeBoundsWithTransform: usize,
    #[cfg(feature = "Foundation")]
    pub ComputeStrokeBounds: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        strokewidth: f32,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ComputeStrokeBounds: usize,
    #[cfg(feature = "Foundation")]
    pub ComputeStrokeBoundsWithStrokeStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ComputeStrokeBoundsWithStrokeStyle: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub ComputeStrokeBoundsWithAllOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        flatteningtolerance: f32,
        result__: *mut ::windows::Foundation::Rect,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ComputeStrokeBoundsWithAllOptions: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub StrokeContainsPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        point: ::windows::Foundation::Numerics::Vector2,
        strokewidth: f32,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    StrokeContainsPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub StrokeContainsPointWithStrokeStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        point: ::windows::Foundation::Numerics::Vector2,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    StrokeContainsPointWithStrokeStyle: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub StrokeContainsPointWithAllOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        point: ::windows::Foundation::Numerics::Vector2,
        strokewidth: f32,
        strokestyle: *mut ::core::ffi::c_void,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        flatteningtolerance: f32,
        result__: *mut bool,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    StrokeContainsPointWithAllOptions: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub Tessellate: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut CanvasTriangleVertices,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Tessellate: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub TessellateWithTransformAndFlatteningTolerance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        flatteningtolerance: f32,
        result_size__: *mut u32,
        result__: *mut *mut CanvasTriangleVertices,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    TessellateWithTransformAndFlatteningTolerance: usize,
    pub SendPathTo: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        streamreader: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub Device: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasGeometryStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasGeometryStatics {
    type Vtable = ICanvasGeometryStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasGeometryStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xd94e33cf_cd59_46f2_8df4_55066aabfd56,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasGeometryStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateRectangle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        rect: ::windows::Foundation::Rect,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateRectangle: usize,
    pub CreateRectangleAtCoords: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateRoundedRectangle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        rect: ::windows::Foundation::Rect,
        radiusx: f32,
        radiusy: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateRoundedRectangle: usize,
    pub CreateRoundedRectangleAtCoords: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radiusx: f32,
        radiusy: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateEllipse: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radiusx: f32,
        radiusy: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateEllipse: usize,
    pub CreateEllipseAtCoords: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        radiusx: f32,
        radiusy: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateCircle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radius: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateCircle: usize,
    pub CreateCircleAtCoords: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
        radius: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreatePath: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        pathbuilder: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreatePolygon: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        points_array_size: u32,
        points: *const ::windows::Foundation::Numerics::Vector2,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreatePolygon: usize,
    pub CreateGroup: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        geometries_array_size: u32,
        geometries: *const *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    pub CreateGroupWithFilledRegionDetermination: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        geometries_array_size: u32,
        geometries: *const *mut ::core::ffi::c_void,
        filledregiondetermination: CanvasFilledRegionDetermination,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Canvas_Text")]
    pub CreateText: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        textlayout: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Canvas_Text"))]
    CreateText: usize,
    #[cfg(all(feature = "Graphics_Canvas_Text", feature = "Foundation_Numerics"))]
    pub CreateGlyphRun: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        point: ::windows::Foundation::Numerics::Vector2,
        fontface: *mut ::core::ffi::c_void,
        fontsize: f32,
        glyphs_array_size: u32,
        glyphs: *const super::Text::CanvasGlyph,
        issideways: bool,
        bidilevel: u32,
        measuringmode: super::Text::CanvasTextMeasuringMode,
        glyphorientation: super::Text::CanvasGlyphOrientation,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Graphics_Canvas_Text", feature = "Foundation_Numerics")))]
    CreateGlyphRun: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Input_Inking"))]
    pub CreateInk: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        inkstrokes: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "UI_Input_Inking")))]
    CreateInk: usize,
    #[cfg(
        all(
            feature = "Foundation_Collections",
            feature = "Foundation_Numerics",
            feature = "UI_Input_Inking"
        )
    )]
    pub CreateInkWithTransformAndFlatteningTolerance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        inkstrokes: *mut ::core::ffi::c_void,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        flatteningtolerance: f32,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(
        not(
            all(
                feature = "Foundation_Collections",
                feature = "Foundation_Numerics",
                feature = "UI_Input_Inking"
            )
        )
    )]
    CreateInkWithTransformAndFlatteningTolerance: usize,
    pub ComputeFlatteningTolerance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dpi: f32,
        maximumzoomfactor: f32,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub ComputeFlatteningToleranceWithTransform: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        dpi: f32,
        maximumzoomfactor: f32,
        expectedgeometrytransform: ::windows::Foundation::Numerics::Matrix3x2,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    ComputeFlatteningToleranceWithTransform: usize,
    pub DefaultFlatteningTolerance: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasGradientMesh(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasGradientMesh {
    type Vtable = ICanvasGradientMesh_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasGradientMesh {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x6bfc2bf1_0a7a_449c_a7ef_6706321b0c1a,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasGradientMesh_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Patches: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut CanvasGradientMeshPatch,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Patches: usize,
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
    pub Device: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasGradientMeshFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasGradientMeshFactory {
    type Vtable = ICanvasGradientMeshFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasGradientMeshFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x4756492d_251e_421d_834d_87ec260d5e4d,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasGradientMeshFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        patchElements_array_size: u32,
        patchelements: *const CanvasGradientMeshPatch,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasGradientMeshStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasGradientMeshStatics {
    type Vtable = ICanvasGradientMeshStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasGradientMeshStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x44027640_3eab_4199_aa3b_644890d0123d,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasGradientMeshStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateCoonsPatch: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        points_array_size: u32,
        points: *const ::windows::Foundation::Numerics::Vector2,
        colors_array_size: u32,
        colors: *const ::windows::Foundation::Numerics::Vector4,
        edges_array_size: u32,
        edges: *const CanvasGradientMeshPatchEdge,
        result__: *mut CanvasGradientMeshPatch,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateCoonsPatch: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub CreateTensorPatch: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        points_array_size: u32,
        points: *const ::windows::Foundation::Numerics::Vector2,
        colors_array_size: u32,
        colors: *const ::windows::Foundation::Numerics::Vector4,
        edges_array_size: u32,
        edges: *const CanvasGradientMeshPatchEdge,
        result__: *mut CanvasGradientMeshPatch,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    CreateTensorPatch: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasPathBuilder(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasPathBuilder {
    type Vtable = ICanvasPathBuilder_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasPathBuilder {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xbcf5822f_8127_4e5c_96b8_29983b915541,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasPathBuilder_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub BeginFigureWithFigureFill: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        startpoint: ::windows::Foundation::Numerics::Vector2,
        figurefill: CanvasFigureFill,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    BeginFigureWithFigureFill: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub BeginFigure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        startpoint: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    BeginFigure: usize,
    pub BeginFigureAtCoordsWithFigureFill: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        startx: f32,
        starty: f32,
        figurefill: CanvasFigureFill,
    ) -> ::windows::core::HRESULT,
    pub BeginFigureAtCoords: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        startx: f32,
        starty: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub AddArcToPoint: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        endpoint: ::windows::Foundation::Numerics::Vector2,
        radiusx: f32,
        radiusy: f32,
        rotationangle: f32,
        sweepdirection: CanvasSweepDirection,
        arcsize: CanvasArcSize,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AddArcToPoint: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub AddArcAroundEllipse: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radiusx: f32,
        radiusy: f32,
        startangle: f32,
        sweepangle: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AddArcAroundEllipse: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub AddCubicBezier: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        controlpoint1: ::windows::Foundation::Numerics::Vector2,
        controlpoint2: ::windows::Foundation::Numerics::Vector2,
        endpoint: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AddCubicBezier: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub AddLine: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        endpoint: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AddLine: usize,
    pub AddLineWithCoords: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        x: f32,
        y: f32,
    ) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub AddQuadraticBezier: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        controlpoint: ::windows::Foundation::Numerics::Vector2,
        endpoint: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AddQuadraticBezier: usize,
    pub SetFilledRegionDetermination: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        filledregiondetermination: CanvasFilledRegionDetermination,
    ) -> ::windows::core::HRESULT,
    pub SetSegmentOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        figuresegmentoptions: CanvasFigureSegmentOptions,
    ) -> ::windows::core::HRESULT,
    pub EndFigure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        figureloop: CanvasFigureLoop,
    ) -> ::windows::core::HRESULT,
    pub AddGeometry: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        geometry: *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasPathBuilderFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasPathBuilderFactory {
    type Vtable = ICanvasPathBuilderFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasPathBuilderFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xac2bee14_efd1_4343_8e53_ba62153d8966,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasPathBuilderFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        resourcecreator: *mut ::core::ffi::c_void,
        result__: *mut *mut ::core::ffi::c_void,
    ) -> ::windows::core::HRESULT,
}
///*Required features: `"Graphics_Canvas_Geometry"`*
#[repr(transparent)]
pub struct ICanvasPathReceiver(::windows::core::IUnknown);
impl ICanvasPathReceiver {
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn BeginFigure(
        &self,
        startpoint: ::windows::Foundation::Numerics::Vector2,
        figurefill: CanvasFigureFill,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .BeginFigure)(
                    ::windows::core::Vtable::as_raw(this),
                    startpoint,
                    figurefill,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AddArc(
        &self,
        endpoint: ::windows::Foundation::Numerics::Vector2,
        radiusx: f32,
        radiusy: f32,
        rotationangle: f32,
        sweepdirection: CanvasSweepDirection,
        arcsize: CanvasArcSize,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .AddArc)(
                    ::windows::core::Vtable::as_raw(this),
                    endpoint,
                    radiusx,
                    radiusy,
                    rotationangle,
                    sweepdirection,
                    arcsize,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AddCubicBezier(
        &self,
        controlpoint1: ::windows::Foundation::Numerics::Vector2,
        controlpoint2: ::windows::Foundation::Numerics::Vector2,
        endpoint: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .AddCubicBezier)(
                    ::windows::core::Vtable::as_raw(this),
                    controlpoint1,
                    controlpoint2,
                    endpoint,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AddLine(
        &self,
        endpoint: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .AddLine)(::windows::core::Vtable::as_raw(this), endpoint)
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AddQuadraticBezier(
        &self,
        controlpoint: ::windows::Foundation::Numerics::Vector2,
        endpoint: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .AddQuadraticBezier)(
                    ::windows::core::Vtable::as_raw(this),
                    controlpoint,
                    endpoint,
                )
                .ok()
        }
    }
    pub fn SetFilledRegionDetermination(
        &self,
        filledregiondetermination: CanvasFilledRegionDetermination,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetFilledRegionDetermination)(
                    ::windows::core::Vtable::as_raw(this),
                    filledregiondetermination,
                )
                .ok()
        }
    }
    pub fn SetSegmentOptions(
        &self,
        figuresegmentoptions: CanvasFigureSegmentOptions,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetSegmentOptions)(
                    ::windows::core::Vtable::as_raw(this),
                    figuresegmentoptions,
                )
                .ok()
        }
    }
    pub fn EndFigure(
        &self,
        figureloop: CanvasFigureLoop,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .EndFigure)(::windows::core::Vtable::as_raw(this), figureloop)
                .ok()
        }
    }
}
::windows::core::interface_hierarchy!(
    ICanvasPathReceiver, ::windows::core::IUnknown, ::windows::core::IInspectable
);
impl ::core::clone::Clone for ICanvasPathReceiver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICanvasPathReceiver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICanvasPathReceiver {}
impl ::core::fmt::Debug for ICanvasPathReceiver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICanvasPathReceiver").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICanvasPathReceiver {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"{70e65373-7fb3-4645-8b6d-f616d1b9a9d7}",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for ICanvasPathReceiver {
    type Vtable = ICanvasPathReceiver_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasPathReceiver {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0x70e65373_7fb3_4645_8b6d_f616d1b9a9d7,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasPathReceiver_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub BeginFigure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        startpoint: ::windows::Foundation::Numerics::Vector2,
        figurefill: CanvasFigureFill,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    BeginFigure: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub AddArc: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        endpoint: ::windows::Foundation::Numerics::Vector2,
        radiusx: f32,
        radiusy: f32,
        rotationangle: f32,
        sweepdirection: CanvasSweepDirection,
        arcsize: CanvasArcSize,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AddArc: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub AddCubicBezier: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        controlpoint1: ::windows::Foundation::Numerics::Vector2,
        controlpoint2: ::windows::Foundation::Numerics::Vector2,
        endpoint: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AddCubicBezier: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub AddLine: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        endpoint: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AddLine: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub AddQuadraticBezier: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        controlpoint: ::windows::Foundation::Numerics::Vector2,
        endpoint: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    AddQuadraticBezier: usize,
    pub SetFilledRegionDetermination: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        filledregiondetermination: CanvasFilledRegionDetermination,
    ) -> ::windows::core::HRESULT,
    pub SetSegmentOptions: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        figuresegmentoptions: CanvasFigureSegmentOptions,
    ) -> ::windows::core::HRESULT,
    pub EndFigure: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        figureloop: CanvasFigureLoop,
    ) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICanvasStrokeStyle(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ICanvasStrokeStyle {
    type Vtable = ICanvasStrokeStyle_Vtbl;
}
unsafe impl ::windows::core::Interface for ICanvasStrokeStyle {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(
        0xfd3e1cd2_6019_40a1_b315_267eef6c2aeb,
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanvasStrokeStyle_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub StartCap: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasCapStyle,
    ) -> ::windows::core::HRESULT,
    pub SetStartCap: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CanvasCapStyle,
    ) -> ::windows::core::HRESULT,
    pub EndCap: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasCapStyle,
    ) -> ::windows::core::HRESULT,
    pub SetEndCap: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CanvasCapStyle,
    ) -> ::windows::core::HRESULT,
    pub DashCap: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasCapStyle,
    ) -> ::windows::core::HRESULT,
    pub SetDashCap: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CanvasCapStyle,
    ) -> ::windows::core::HRESULT,
    pub LineJoin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasLineJoin,
    ) -> ::windows::core::HRESULT,
    pub SetLineJoin: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CanvasLineJoin,
    ) -> ::windows::core::HRESULT,
    pub MiterLimit: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetMiterLimit: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub DashStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasDashStyle,
    ) -> ::windows::core::HRESULT,
    pub SetDashStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CanvasDashStyle,
    ) -> ::windows::core::HRESULT,
    pub DashOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetDashOffset: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: f32,
    ) -> ::windows::core::HRESULT,
    pub CustomDashStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result_size__: *mut u32,
        result__: *mut *mut f32,
    ) -> ::windows::core::HRESULT,
    pub SetCustomDashStyle: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        valueElements_array_size: u32,
        valueelements: *const f32,
    ) -> ::windows::core::HRESULT,
    pub TransformBehavior: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        result__: *mut CanvasStrokeTransformBehavior,
    ) -> ::windows::core::HRESULT,
    pub SetTransformBehavior: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        value: CanvasStrokeTransformBehavior,
    ) -> ::windows::core::HRESULT,
}
///*Required features: `"Graphics_Canvas_Geometry"`*
#[repr(transparent)]
pub struct CanvasCachedGeometry(::windows::core::IUnknown);
impl CanvasCachedGeometry {
    pub fn Device(&self) -> ::windows::core::Result<super::CanvasDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Device)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn CreateFill(
        geometry: &CanvasGeometry,
    ) -> ::windows::core::Result<CanvasCachedGeometry> {
        Self::ICanvasCachedGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateFill)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    pub fn CreateFillWithFlatteningTolerance(
        geometry: &CanvasGeometry,
        flatteningtolerance: f32,
    ) -> ::windows::core::Result<CanvasCachedGeometry> {
        Self::ICanvasCachedGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateFillWithFlatteningTolerance)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    flatteningtolerance,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    pub fn CreateStroke(
        geometry: &CanvasGeometry,
        strokewidth: f32,
    ) -> ::windows::core::Result<CanvasCachedGeometry> {
        Self::ICanvasCachedGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateStroke)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    strokewidth,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    pub fn CreateStrokeWithStrokeStyle(
        geometry: &CanvasGeometry,
        strokewidth: f32,
        strokestyle: &CanvasStrokeStyle,
    ) -> ::windows::core::Result<CanvasCachedGeometry> {
        Self::ICanvasCachedGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateStrokeWithStrokeStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    pub fn CreateStrokeWithStrokeStyleAndFlatteningTolerance(
        geometry: &CanvasGeometry,
        strokewidth: f32,
        strokestyle: &CanvasStrokeStyle,
        flatteningtolerance: f32,
    ) -> ::windows::core::Result<CanvasCachedGeometry> {
        Self::ICanvasCachedGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateStrokeWithStrokeStyleAndFlatteningTolerance)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                    flatteningtolerance,
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
    pub fn ICanvasCachedGeometryStatics<
        R,
        F: FnOnce(&ICanvasCachedGeometryStatics) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasCachedGeometry,
            ICanvasCachedGeometryStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CanvasCachedGeometry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasCachedGeometry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasCachedGeometry {}
impl ::core::fmt::Debug for CanvasCachedGeometry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasCachedGeometry").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasCachedGeometry {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.Geometry.CanvasCachedGeometry;{ba6cb114-e1a1-448d-ab7c-8d2b92674119})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasCachedGeometry {
    type Vtable = ICanvasCachedGeometry_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasCachedGeometry {
    const IID: ::windows::core::GUID = <ICanvasCachedGeometry as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasCachedGeometry {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Geometry.CanvasCachedGeometry";
}
::windows::core::interface_hierarchy!(
    CanvasCachedGeometry,::windows::core::IUnknown,::windows::core::IInspectable
);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasCachedGeometry>
for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasCachedGeometry) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasCachedGeometry>
for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasCachedGeometry) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasCachedGeometry>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasCachedGeometry) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CanvasCachedGeometry {}
unsafe impl ::core::marker::Sync for CanvasCachedGeometry {}
///*Required features: `"Graphics_Canvas_Geometry"`*
#[repr(transparent)]
pub struct CanvasGeometry(::windows::core::IUnknown);
impl CanvasGeometry {
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CombineWith(
        &self,
        othergeometry: &CanvasGeometry,
        othergeometrytransform: ::windows::Foundation::Numerics::Matrix3x2,
        combine: CanvasGeometryCombine,
    ) -> ::windows::core::Result<CanvasGeometry> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CombineWith)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(othergeometry),
                    othergeometrytransform,
                    combine,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CombineWithUsingFlatteningTolerance(
        &self,
        othergeometry: &CanvasGeometry,
        othergeometrytransform: ::windows::Foundation::Numerics::Matrix3x2,
        combine: CanvasGeometryCombine,
        flatteningtolerance: f32,
    ) -> ::windows::core::Result<CanvasGeometry> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CombineWithUsingFlatteningTolerance)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(othergeometry),
                    othergeometrytransform,
                    combine,
                    flatteningtolerance,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn Stroke(&self, strokewidth: f32) -> ::windows::core::Result<CanvasGeometry> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Stroke)(
                    ::windows::core::Vtable::as_raw(this),
                    strokewidth,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn StrokeWithStrokeStyle(
        &self,
        strokewidth: f32,
        strokestyle: &CanvasStrokeStyle,
    ) -> ::windows::core::Result<CanvasGeometry> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .StrokeWithStrokeStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn StrokeWithAllOptions(
        &self,
        strokewidth: f32,
        strokestyle: &CanvasStrokeStyle,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        flatteningtolerance: f32,
    ) -> ::windows::core::Result<CanvasGeometry> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .StrokeWithAllOptions)(
                    ::windows::core::Vtable::as_raw(this),
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                    transform,
                    flatteningtolerance,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn Outline(&self) -> ::windows::core::Result<CanvasGeometry> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Outline)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn OutlineWithTransformAndFlatteningTolerance(
        &self,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        flatteningtolerance: f32,
    ) -> ::windows::core::Result<CanvasGeometry> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .OutlineWithTransformAndFlatteningTolerance)(
                    ::windows::core::Vtable::as_raw(this),
                    transform,
                    flatteningtolerance,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn Simplify(
        &self,
        simplification: CanvasGeometrySimplification,
    ) -> ::windows::core::Result<CanvasGeometry> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Simplify)(
                    ::windows::core::Vtable::as_raw(this),
                    simplification,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SimplifyWithTransformAndFlatteningTolerance(
        &self,
        simplification: CanvasGeometrySimplification,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        flatteningtolerance: f32,
    ) -> ::windows::core::Result<CanvasGeometry> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .SimplifyWithTransformAndFlatteningTolerance)(
                    ::windows::core::Vtable::as_raw(this),
                    simplification,
                    transform,
                    flatteningtolerance,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Transform(
        &self,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::core::Result<CanvasGeometry> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Transform)(
                    ::windows::core::Vtable::as_raw(this),
                    transform,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn CompareWith(
        &self,
        othergeometry: &CanvasGeometry,
    ) -> ::windows::core::Result<CanvasGeometryRelation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CompareWith)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(othergeometry),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CompareWithUsingTransformAndFlatteningTolerance(
        &self,
        othergeometry: &CanvasGeometry,
        othergeometrytransform: ::windows::Foundation::Numerics::Matrix3x2,
        flatteningtolerance: f32,
    ) -> ::windows::core::Result<CanvasGeometryRelation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CompareWithUsingTransformAndFlatteningTolerance)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(othergeometry),
                    othergeometrytransform,
                    flatteningtolerance,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn ComputeArea(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ComputeArea)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn ComputeAreaWithTransformAndFlatteningTolerance(
        &self,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        flatteningtolerance: f32,
    ) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ComputeAreaWithTransformAndFlatteningTolerance)(
                    ::windows::core::Vtable::as_raw(this),
                    transform,
                    flatteningtolerance,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn ComputePathLength(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ComputePathLength)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn ComputePathLengthWithTransformAndFlatteningTolerance(
        &self,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        flatteningtolerance: f32,
    ) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ComputePathLengthWithTransformAndFlatteningTolerance)(
                    ::windows::core::Vtable::as_raw(this),
                    transform,
                    flatteningtolerance,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn ComputePointOnPath(
        &self,
        distance: f32,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ComputePointOnPath)(
                    ::windows::core::Vtable::as_raw(this),
                    distance,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn ComputePointOnPathWithTangent(
        &self,
        distance: f32,
        tangent: &mut ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ComputePointOnPathWithTangent)(
                    ::windows::core::Vtable::as_raw(this),
                    distance,
                    tangent,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn ComputePointOnPathWithTransformAndFlatteningToleranceAndTangent(
        &self,
        length: f32,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        flatteningtolerance: f32,
        tangent: &mut ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::Result<::windows::Foundation::Numerics::Vector2> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ComputePointOnPathWithTransformAndFlatteningToleranceAndTangent)(
                    ::windows::core::Vtable::as_raw(this),
                    length,
                    transform,
                    flatteningtolerance,
                    tangent,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn FillContainsPoint(
        &self,
        point: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .FillContainsPoint)(
                    ::windows::core::Vtable::as_raw(this),
                    point,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn FillContainsPointWithTransformAndFlatteningTolerance(
        &self,
        point: ::windows::Foundation::Numerics::Vector2,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        flatteningtolerance: f32,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .FillContainsPointWithTransformAndFlatteningTolerance)(
                    ::windows::core::Vtable::as_raw(this),
                    point,
                    transform,
                    flatteningtolerance,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn ComputeBounds(&self) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ComputeBounds)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn ComputeBoundsWithTransform(
        &self,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ComputeBoundsWithTransform)(
                    ::windows::core::Vtable::as_raw(this),
                    transform,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn ComputeStrokeBounds(
        &self,
        strokewidth: f32,
    ) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ComputeStrokeBounds)(
                    ::windows::core::Vtable::as_raw(this),
                    strokewidth,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn ComputeStrokeBoundsWithStrokeStyle(
        &self,
        strokewidth: f32,
        strokestyle: &CanvasStrokeStyle,
    ) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ComputeStrokeBoundsWithStrokeStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn ComputeStrokeBoundsWithAllOptions(
        &self,
        strokewidth: f32,
        strokestyle: &CanvasStrokeStyle,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        flatteningtolerance: f32,
    ) -> ::windows::core::Result<::windows::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ComputeStrokeBoundsWithAllOptions)(
                    ::windows::core::Vtable::as_raw(this),
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                    transform,
                    flatteningtolerance,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn StrokeContainsPoint(
        &self,
        point: ::windows::Foundation::Numerics::Vector2,
        strokewidth: f32,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .StrokeContainsPoint)(
                    ::windows::core::Vtable::as_raw(this),
                    point,
                    strokewidth,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn StrokeContainsPointWithStrokeStyle(
        &self,
        point: ::windows::Foundation::Numerics::Vector2,
        strokewidth: f32,
        strokestyle: &CanvasStrokeStyle,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .StrokeContainsPointWithStrokeStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    point,
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn StrokeContainsPointWithAllOptions(
        &self,
        point: ::windows::Foundation::Numerics::Vector2,
        strokewidth: f32,
        strokestyle: &CanvasStrokeStyle,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        flatteningtolerance: f32,
    ) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .StrokeContainsPointWithAllOptions)(
                    ::windows::core::Vtable::as_raw(this),
                    point,
                    strokewidth,
                    ::core::mem::transmute_copy(strokestyle),
                    transform,
                    flatteningtolerance,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Tessellate(
        &self,
    ) -> ::windows::core::Result<::windows::core::Array<CanvasTriangleVertices>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Tessellate)(
                    ::windows::core::Vtable::as_raw(this),
                    ::windows::core::Array::<
                        CanvasTriangleVertices,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn TessellateWithTransformAndFlatteningTolerance(
        &self,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        flatteningtolerance: f32,
    ) -> ::windows::core::Result<::windows::core::Array<CanvasTriangleVertices>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .TessellateWithTransformAndFlatteningTolerance)(
                    ::windows::core::Vtable::as_raw(this),
                    transform,
                    flatteningtolerance,
                    ::windows::core::Array::<
                        CanvasTriangleVertices,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn SendPathTo<P0, E0>(&self, streamreader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<ICanvasPathReceiver>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SendPathTo)(
                    ::windows::core::Vtable::as_raw(this),
                    streamreader.try_into().map_err(|e| e.into())?.abi(),
                )
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
    pub fn CreateRectangle<P0, E0>(
        resourcecreator: P0,
        rect: ::windows::Foundation::Rect,
    ) -> ::windows::core::Result<CanvasGeometry>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateRectangle)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    rect,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    pub fn CreateRectangleAtCoords<P0, E0>(
        resourcecreator: P0,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    ) -> ::windows::core::Result<CanvasGeometry>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateRectangleAtCoords)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    x,
                    y,
                    w,
                    h,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation"`*
    #[cfg(feature = "Foundation")]
    pub fn CreateRoundedRectangle<P0, E0>(
        resourcecreator: P0,
        rect: ::windows::Foundation::Rect,
        radiusx: f32,
        radiusy: f32,
    ) -> ::windows::core::Result<CanvasGeometry>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateRoundedRectangle)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    rect,
                    radiusx,
                    radiusy,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    pub fn CreateRoundedRectangleAtCoords<P0, E0>(
        resourcecreator: P0,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        radiusx: f32,
        radiusy: f32,
    ) -> ::windows::core::Result<CanvasGeometry>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateRoundedRectangleAtCoords)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    x,
                    y,
                    w,
                    h,
                    radiusx,
                    radiusy,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateEllipse<P0, E0>(
        resourcecreator: P0,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radiusx: f32,
        radiusy: f32,
    ) -> ::windows::core::Result<CanvasGeometry>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateEllipse)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    centerpoint,
                    radiusx,
                    radiusy,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    pub fn CreateEllipseAtCoords<P0, E0>(
        resourcecreator: P0,
        x: f32,
        y: f32,
        radiusx: f32,
        radiusy: f32,
    ) -> ::windows::core::Result<CanvasGeometry>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateEllipseAtCoords)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    x,
                    y,
                    radiusx,
                    radiusy,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateCircle<P0, E0>(
        resourcecreator: P0,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radius: f32,
    ) -> ::windows::core::Result<CanvasGeometry>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateCircle)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    centerpoint,
                    radius,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    pub fn CreateCircleAtCoords<P0, E0>(
        resourcecreator: P0,
        x: f32,
        y: f32,
        radius: f32,
    ) -> ::windows::core::Result<CanvasGeometry>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateCircleAtCoords)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    x,
                    y,
                    radius,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    pub fn CreatePath(
        pathbuilder: &CanvasPathBuilder,
    ) -> ::windows::core::Result<CanvasGeometry> {
        Self::ICanvasGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreatePath)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(pathbuilder),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreatePolygon<P0, E0>(
        resourcecreator: P0,
        points: &[::windows::Foundation::Numerics::Vector2],
    ) -> ::windows::core::Result<CanvasGeometry>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreatePolygon)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    points.len() as u32,
                    points.as_ptr(),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    pub fn CreateGroup<P0, E0>(
        resourcecreator: P0,
        geometries: &[::core::option::Option<CanvasGeometry>],
    ) -> ::windows::core::Result<CanvasGeometry>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateGroup)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    geometries.len() as u32,
                    ::core::mem::transmute(geometries.as_ptr()),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    pub fn CreateGroupWithFilledRegionDetermination<P0, E0>(
        resourcecreator: P0,
        geometries: &[::core::option::Option<CanvasGeometry>],
        filledregiondetermination: CanvasFilledRegionDetermination,
    ) -> ::windows::core::Result<CanvasGeometry>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateGroupWithFilledRegionDetermination)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    geometries.len() as u32,
                    ::core::mem::transmute(geometries.as_ptr()),
                    filledregiondetermination,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Graphics_Canvas_Text"`*
    #[cfg(feature = "Graphics_Canvas_Text")]
    pub fn CreateText(
        textlayout: &super::Text::CanvasTextLayout,
    ) -> ::windows::core::Result<CanvasGeometry> {
        Self::ICanvasGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateText)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(textlayout),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Graphics_Canvas_Text"`, `"Foundation_Numerics"`*
    #[cfg(all(feature = "Graphics_Canvas_Text", feature = "Foundation_Numerics"))]
    pub fn CreateGlyphRun<P0, E0>(
        resourcecreator: P0,
        point: ::windows::Foundation::Numerics::Vector2,
        fontface: &super::Text::CanvasFontFace,
        fontsize: f32,
        glyphs: &[super::Text::CanvasGlyph],
        issideways: bool,
        bidilevel: u32,
        measuringmode: super::Text::CanvasTextMeasuringMode,
        glyphorientation: super::Text::CanvasGlyphOrientation,
    ) -> ::windows::core::Result<CanvasGeometry>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateGlyphRun)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    point,
                    ::core::mem::transmute_copy(fontface),
                    fontsize,
                    glyphs.len() as u32,
                    glyphs.as_ptr(),
                    issideways,
                    bidilevel,
                    measuringmode,
                    glyphorientation,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation_Collections"`, `"UI_Input_Inking"`*
    #[cfg(all(feature = "Foundation_Collections", feature = "UI_Input_Inking"))]
    pub fn CreateInk<P0, E0, P1, E1>(
        resourcecreator: P0,
        inkstrokes: P1,
    ) -> ::windows::core::Result<CanvasGeometry>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<
                ::windows::Foundation::Collections::IIterable::<
                    ::windows::UI::Input::Inking::InkStroke,
                >,
            >,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateInk)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    inkstrokes.try_into().map_err(|e| e.into())?.abi(),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation_Collections"`, `"Foundation_Numerics"`, `"UI_Input_Inking"`*
    #[cfg(
        all(
            feature = "Foundation_Collections",
            feature = "Foundation_Numerics",
            feature = "UI_Input_Inking"
        )
    )]
    pub fn CreateInkWithTransformAndFlatteningTolerance<P0, E0, P1, E1>(
        resourcecreator: P0,
        inkstrokes: P1,
        transform: ::windows::Foundation::Numerics::Matrix3x2,
        flatteningtolerance: f32,
    ) -> ::windows::core::Result<CanvasGeometry>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<
            ::windows::core::InParam<
                ::windows::Foundation::Collections::IIterable::<
                    ::windows::UI::Input::Inking::InkStroke,
                >,
            >,
            Error = E1,
        >,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateInkWithTransformAndFlatteningTolerance)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    inkstrokes.try_into().map_err(|e| e.into())?.abi(),
                    transform,
                    flatteningtolerance,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    pub fn ComputeFlatteningTolerance(
        dpi: f32,
        maximumzoomfactor: f32,
    ) -> ::windows::core::Result<f32> {
        Self::ICanvasGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ComputeFlatteningTolerance)(
                    ::windows::core::Vtable::as_raw(this),
                    dpi,
                    maximumzoomfactor,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn ComputeFlatteningToleranceWithTransform(
        dpi: f32,
        maximumzoomfactor: f32,
        expectedgeometrytransform: ::windows::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::core::Result<f32> {
        Self::ICanvasGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .ComputeFlatteningToleranceWithTransform)(
                    ::windows::core::Vtable::as_raw(this),
                    dpi,
                    maximumzoomfactor,
                    expectedgeometrytransform,
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    pub fn DefaultFlatteningTolerance() -> ::windows::core::Result<f32> {
        Self::ICanvasGeometryStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .DefaultFlatteningTolerance)(
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
    pub fn ICanvasGeometryStatics<
        R,
        F: FnOnce(&ICanvasGeometryStatics) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasGeometry,
            ICanvasGeometryStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CanvasGeometry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasGeometry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasGeometry {}
impl ::core::fmt::Debug for CanvasGeometry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasGeometry").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasGeometry {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.Geometry.CanvasGeometry;{74ea89fa-c87c-4d0d-9057-2743b8db67ee})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasGeometry {
    type Vtable = ICanvasGeometry_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasGeometry {
    const IID: ::windows::core::GUID = <ICanvasGeometry as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasGeometry {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Geometry.CanvasGeometry";
}
::windows::core::interface_hierarchy!(
    CanvasGeometry,::windows::core::IUnknown,::windows::core::IInspectable
);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasGeometry> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasGeometry) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasGeometry> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasGeometry) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasGeometry>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasGeometry) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Graphics")]
impl ::core::convert::TryFrom<CanvasGeometry>
for ::windows::Graphics::IGeometrySource2D {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasGeometry) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Graphics")]
impl ::core::convert::TryFrom<&CanvasGeometry>
for ::windows::Graphics::IGeometrySource2D {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasGeometry) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Graphics")]
impl ::core::convert::TryFrom<&CanvasGeometry>
for ::windows::core::InParam<::windows::Graphics::IGeometrySource2D> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasGeometry) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CanvasGeometry {}
unsafe impl ::core::marker::Sync for CanvasGeometry {}
///*Required features: `"Graphics_Canvas_Geometry"`*
#[repr(transparent)]
pub struct CanvasGradientMesh(::windows::core::IUnknown);
impl CanvasGradientMesh {
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Patches(
        &self,
    ) -> ::windows::core::Result<::windows::core::Array<CanvasGradientMeshPatch>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Patches)(
                    ::windows::core::Vtable::as_raw(this),
                    ::windows::core::Array::<
                        CanvasGradientMeshPatch,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
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
            ::windows::core::InParam<super::ICanvasResourceCreator>,
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
            ::windows::core::InParam<super::ICanvasResourceCreator>,
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
    pub fn Device(&self) -> ::windows::core::Result<super::CanvasDevice> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Device)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Create<P0, E0>(
        resourcecreator: P0,
        patchelements: &[CanvasGradientMeshPatch],
    ) -> ::windows::core::Result<CanvasGradientMesh>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasGradientMeshFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .Create)(
                    ::windows::core::Vtable::as_raw(this),
                    resourcecreator.try_into().map_err(|e| e.into())?.abi(),
                    patchelements.len() as u32,
                    patchelements.as_ptr(),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateCoonsPatch(
        points: &[::windows::Foundation::Numerics::Vector2],
        colors: &[::windows::Foundation::Numerics::Vector4],
        edges: &[CanvasGradientMeshPatchEdge],
    ) -> ::windows::core::Result<CanvasGradientMeshPatch> {
        Self::ICanvasGradientMeshStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateCoonsPatch)(
                    ::windows::core::Vtable::as_raw(this),
                    points.len() as u32,
                    points.as_ptr(),
                    colors.len() as u32,
                    colors.as_ptr(),
                    edges.len() as u32,
                    edges.as_ptr(),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        })
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn CreateTensorPatch(
        points: &[::windows::Foundation::Numerics::Vector2],
        colors: &[::windows::Foundation::Numerics::Vector4],
        edges: &[CanvasGradientMeshPatchEdge],
    ) -> ::windows::core::Result<CanvasGradientMeshPatch> {
        Self::ICanvasGradientMeshStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CreateTensorPatch)(
                    ::windows::core::Vtable::as_raw(this),
                    points.len() as u32,
                    points.as_ptr(),
                    colors.len() as u32,
                    colors.as_ptr(),
                    edges.len() as u32,
                    edges.as_ptr(),
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
    pub fn ICanvasGradientMeshFactory<
        R,
        F: FnOnce(&ICanvasGradientMeshFactory) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasGradientMesh,
            ICanvasGradientMeshFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ICanvasGradientMeshStatics<
        R,
        F: FnOnce(&ICanvasGradientMeshStatics) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasGradientMesh,
            ICanvasGradientMeshStatics,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CanvasGradientMesh {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasGradientMesh {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasGradientMesh {}
impl ::core::fmt::Debug for CanvasGradientMesh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasGradientMesh").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasGradientMesh {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.Geometry.CanvasGradientMesh;{6bfc2bf1-0a7a-449c-a7ef-6706321b0c1a})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasGradientMesh {
    type Vtable = ICanvasGradientMesh_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasGradientMesh {
    const IID: ::windows::core::GUID = <ICanvasGradientMesh as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasGradientMesh {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Geometry.CanvasGradientMesh";
}
::windows::core::interface_hierarchy!(
    CanvasGradientMesh,::windows::core::IUnknown,::windows::core::IInspectable
);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasGradientMesh> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasGradientMesh) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasGradientMesh> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasGradientMesh) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasGradientMesh>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasGradientMesh) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CanvasGradientMesh {}
unsafe impl ::core::marker::Sync for CanvasGradientMesh {}
///*Required features: `"Graphics_Canvas_Geometry"`*
#[repr(transparent)]
pub struct CanvasPathBuilder(::windows::core::IUnknown);
impl CanvasPathBuilder {
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn BeginFigureWithFigureFill(
        &self,
        startpoint: ::windows::Foundation::Numerics::Vector2,
        figurefill: CanvasFigureFill,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .BeginFigureWithFigureFill)(
                    ::windows::core::Vtable::as_raw(this),
                    startpoint,
                    figurefill,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn BeginFigure(
        &self,
        startpoint: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .BeginFigure)(::windows::core::Vtable::as_raw(this), startpoint)
                .ok()
        }
    }
    pub fn BeginFigureAtCoordsWithFigureFill(
        &self,
        startx: f32,
        starty: f32,
        figurefill: CanvasFigureFill,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .BeginFigureAtCoordsWithFigureFill)(
                    ::windows::core::Vtable::as_raw(this),
                    startx,
                    starty,
                    figurefill,
                )
                .ok()
        }
    }
    pub fn BeginFigureAtCoords(
        &self,
        startx: f32,
        starty: f32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .BeginFigureAtCoords)(
                    ::windows::core::Vtable::as_raw(this),
                    startx,
                    starty,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AddArcToPoint(
        &self,
        endpoint: ::windows::Foundation::Numerics::Vector2,
        radiusx: f32,
        radiusy: f32,
        rotationangle: f32,
        sweepdirection: CanvasSweepDirection,
        arcsize: CanvasArcSize,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .AddArcToPoint)(
                    ::windows::core::Vtable::as_raw(this),
                    endpoint,
                    radiusx,
                    radiusy,
                    rotationangle,
                    sweepdirection,
                    arcsize,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AddArcAroundEllipse(
        &self,
        centerpoint: ::windows::Foundation::Numerics::Vector2,
        radiusx: f32,
        radiusy: f32,
        startangle: f32,
        sweepangle: f32,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .AddArcAroundEllipse)(
                    ::windows::core::Vtable::as_raw(this),
                    centerpoint,
                    radiusx,
                    radiusy,
                    startangle,
                    sweepangle,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AddCubicBezier(
        &self,
        controlpoint1: ::windows::Foundation::Numerics::Vector2,
        controlpoint2: ::windows::Foundation::Numerics::Vector2,
        endpoint: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .AddCubicBezier)(
                    ::windows::core::Vtable::as_raw(this),
                    controlpoint1,
                    controlpoint2,
                    endpoint,
                )
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AddLine(
        &self,
        endpoint: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .AddLine)(::windows::core::Vtable::as_raw(this), endpoint)
                .ok()
        }
    }
    pub fn AddLineWithCoords(&self, x: f32, y: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .AddLineWithCoords)(::windows::core::Vtable::as_raw(this), x, y)
                .ok()
        }
    }
    ///*Required features: `"Foundation_Numerics"`*
    #[cfg(feature = "Foundation_Numerics")]
    pub fn AddQuadraticBezier(
        &self,
        controlpoint: ::windows::Foundation::Numerics::Vector2,
        endpoint: ::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .AddQuadraticBezier)(
                    ::windows::core::Vtable::as_raw(this),
                    controlpoint,
                    endpoint,
                )
                .ok()
        }
    }
    pub fn SetFilledRegionDetermination(
        &self,
        filledregiondetermination: CanvasFilledRegionDetermination,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetFilledRegionDetermination)(
                    ::windows::core::Vtable::as_raw(this),
                    filledregiondetermination,
                )
                .ok()
        }
    }
    pub fn SetSegmentOptions(
        &self,
        figuresegmentoptions: CanvasFigureSegmentOptions,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetSegmentOptions)(
                    ::windows::core::Vtable::as_raw(this),
                    figuresegmentoptions,
                )
                .ok()
        }
    }
    pub fn EndFigure(
        &self,
        figureloop: CanvasFigureLoop,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .EndFigure)(::windows::core::Vtable::as_raw(this), figureloop)
                .ok()
        }
    }
    pub fn AddGeometry(&self, geometry: &CanvasGeometry) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .AddGeometry)(
                    ::windows::core::Vtable::as_raw(this),
                    ::core::mem::transmute_copy(geometry),
                )
                .ok()
        }
    }
    pub fn Create<P0, E0>(
        resourcecreator: P0,
    ) -> ::windows::core::Result<CanvasPathBuilder>
    where
        P0: ::std::convert::TryInto<
            ::windows::core::InParam<super::ICanvasResourceCreator>,
            Error = E0,
        >,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ICanvasPathBuilderFactory(|this| unsafe {
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
    pub fn ICanvasPathBuilderFactory<
        R,
        F: FnOnce(&ICanvasPathBuilderFactory) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasPathBuilder,
            ICanvasPathBuilderFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for CanvasPathBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasPathBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasPathBuilder {}
impl ::core::fmt::Debug for CanvasPathBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasPathBuilder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasPathBuilder {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.Geometry.CanvasPathBuilder;{bcf5822f-8127-4e5c-96b8-29983b915541})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasPathBuilder {
    type Vtable = ICanvasPathBuilder_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasPathBuilder {
    const IID: ::windows::core::GUID = <ICanvasPathBuilder as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasPathBuilder {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Geometry.CanvasPathBuilder";
}
::windows::core::interface_hierarchy!(
    CanvasPathBuilder,::windows::core::IUnknown,::windows::core::IInspectable
);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasPathBuilder> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasPathBuilder) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasPathBuilder> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasPathBuilder) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasPathBuilder>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasPathBuilder) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CanvasPathBuilder {}
unsafe impl ::core::marker::Sync for CanvasPathBuilder {}
///*Required features: `"Graphics_Canvas_Geometry"`*
#[repr(transparent)]
pub struct CanvasStrokeStyle(::windows::core::IUnknown);
impl CanvasStrokeStyle {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>,
    >(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<
            CanvasStrokeStyle,
            ::windows::core::IGenericFactory,
        > = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn StartCap(&self) -> ::windows::core::Result<CanvasCapStyle> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .StartCap)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn SetStartCap(&self, value: CanvasCapStyle) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetStartCap)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn EndCap(&self) -> ::windows::core::Result<CanvasCapStyle> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .EndCap)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn SetEndCap(&self, value: CanvasCapStyle) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetEndCap)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn DashCap(&self) -> ::windows::core::Result<CanvasCapStyle> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .DashCap)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn SetDashCap(&self, value: CanvasCapStyle) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetDashCap)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn LineJoin(&self) -> ::windows::core::Result<CanvasLineJoin> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .LineJoin)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn SetLineJoin(&self, value: CanvasLineJoin) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetLineJoin)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn MiterLimit(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .MiterLimit)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetMiterLimit(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetMiterLimit)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn DashStyle(&self) -> ::windows::core::Result<CanvasDashStyle> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .DashStyle)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr())
                .from_abi(result__)
        }
    }
    pub fn SetDashStyle(&self, value: CanvasDashStyle) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetDashStyle)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn DashOffset(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .DashOffset)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetDashOffset(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetDashOffset)(::windows::core::Vtable::as_raw(this), value)
                .ok()
        }
    }
    pub fn CustomDashStyle(
        &self,
    ) -> ::windows::core::Result<::windows::core::Array<f32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .CustomDashStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    ::windows::core::Array::<
                        f32,
                    >::set_abi_len(result__.assume_init_mut()),
                    result__.as_mut_ptr() as *mut _ as _,
                )
                .and_then(|| result__.assume_init())
        }
    }
    pub fn SetCustomDashStyle(
        &self,
        valueelements: &[f32],
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetCustomDashStyle)(
                    ::windows::core::Vtable::as_raw(this),
                    valueelements.len() as u32,
                    valueelements.as_ptr(),
                )
                .ok()
        }
    }
    pub fn TransformBehavior(
        &self,
    ) -> ::windows::core::Result<CanvasStrokeTransformBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this)
                .TransformBehavior)(
                    ::windows::core::Vtable::as_raw(this),
                    result__.as_mut_ptr(),
                )
                .from_abi(result__)
        }
    }
    pub fn SetTransformBehavior(
        &self,
        value: CanvasStrokeTransformBehavior,
    ) -> ::windows::core::Result<()> {
        let this = self;
        unsafe {
            (::windows::core::Vtable::vtable(this)
                .SetTransformBehavior)(::windows::core::Vtable::as_raw(this), value)
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
impl ::core::clone::Clone for CanvasStrokeStyle {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CanvasStrokeStyle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CanvasStrokeStyle {}
impl ::core::fmt::Debug for CanvasStrokeStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasStrokeStyle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasStrokeStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"rc(Microsoft.Graphics.Canvas.Geometry.CanvasStrokeStyle;{fd3e1cd2-6019-40a1-b315-267eef6c2aeb})",
    );
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for CanvasStrokeStyle {
    type Vtable = ICanvasStrokeStyle_Vtbl;
}
unsafe impl ::windows::core::Interface for CanvasStrokeStyle {
    const IID: ::windows::core::GUID = <ICanvasStrokeStyle as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CanvasStrokeStyle {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Geometry.CanvasStrokeStyle";
}
::windows::core::interface_hierarchy!(
    CanvasStrokeStyle,::windows::core::IUnknown,::windows::core::IInspectable
);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CanvasStrokeStyle> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CanvasStrokeStyle) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasStrokeStyle> for ::windows::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasStrokeStyle) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CanvasStrokeStyle>
for ::windows::core::InParam<::windows::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &CanvasStrokeStyle) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for CanvasStrokeStyle {}
unsafe impl ::core::marker::Sync for CanvasStrokeStyle {}
///*Required features: `"Graphics_Canvas_Geometry"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasArcSize(pub i32);
impl CanvasArcSize {
    pub const Small: Self = Self(0i32);
    pub const Large: Self = Self(1i32);
}
impl ::core::marker::Copy for CanvasArcSize {}
impl ::core::clone::Clone for CanvasArcSize {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasArcSize {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasArcSize {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasArcSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasArcSize").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasArcSize {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Geometry.CanvasArcSize;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Geometry"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasCapStyle(pub i32);
impl CanvasCapStyle {
    pub const Flat: Self = Self(0i32);
    pub const Square: Self = Self(1i32);
    pub const Round: Self = Self(2i32);
    pub const Triangle: Self = Self(3i32);
}
impl ::core::marker::Copy for CanvasCapStyle {}
impl ::core::clone::Clone for CanvasCapStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasCapStyle {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasCapStyle {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasCapStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasCapStyle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasCapStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Geometry.CanvasCapStyle;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Geometry"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasDashStyle(pub i32);
impl CanvasDashStyle {
    pub const Solid: Self = Self(0i32);
    pub const Dash: Self = Self(1i32);
    pub const Dot: Self = Self(2i32);
    pub const DashDot: Self = Self(3i32);
    pub const DashDotDot: Self = Self(4i32);
}
impl ::core::marker::Copy for CanvasDashStyle {}
impl ::core::clone::Clone for CanvasDashStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasDashStyle {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasDashStyle {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasDashStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasDashStyle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasDashStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Geometry.CanvasDashStyle;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Geometry"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasFigureFill(pub i32);
impl CanvasFigureFill {
    pub const Default: Self = Self(0i32);
    pub const DoesNotAffectFills: Self = Self(1i32);
}
impl ::core::marker::Copy for CanvasFigureFill {}
impl ::core::clone::Clone for CanvasFigureFill {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasFigureFill {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasFigureFill {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasFigureFill {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasFigureFill").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasFigureFill {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Geometry.CanvasFigureFill;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Geometry"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasFigureLoop(pub i32);
impl CanvasFigureLoop {
    pub const Open: Self = Self(0i32);
    pub const Closed: Self = Self(1i32);
}
impl ::core::marker::Copy for CanvasFigureLoop {}
impl ::core::clone::Clone for CanvasFigureLoop {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasFigureLoop {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasFigureLoop {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasFigureLoop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasFigureLoop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasFigureLoop {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Geometry.CanvasFigureLoop;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Geometry"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasFigureSegmentOptions(pub u32);
impl CanvasFigureSegmentOptions {
    pub const None: Self = Self(0u32);
    pub const ForceUnstroked: Self = Self(1u32);
    pub const ForceRoundLineJoin: Self = Self(2u32);
}
impl ::core::marker::Copy for CanvasFigureSegmentOptions {}
impl ::core::clone::Clone for CanvasFigureSegmentOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasFigureSegmentOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasFigureSegmentOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasFigureSegmentOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasFigureSegmentOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CanvasFigureSegmentOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CanvasFigureSegmentOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CanvasFigureSegmentOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CanvasFigureSegmentOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CanvasFigureSegmentOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasFigureSegmentOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Geometry.CanvasFigureSegmentOptions;u4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Geometry"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasFilledRegionDetermination(pub i32);
impl CanvasFilledRegionDetermination {
    pub const Alternate: Self = Self(0i32);
    pub const Winding: Self = Self(1i32);
}
impl ::core::marker::Copy for CanvasFilledRegionDetermination {}
impl ::core::clone::Clone for CanvasFilledRegionDetermination {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasFilledRegionDetermination {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasFilledRegionDetermination {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasFilledRegionDetermination {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasFilledRegionDetermination").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasFilledRegionDetermination {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Geometry.CanvasFilledRegionDetermination;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Geometry"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasGeometryCombine(pub i32);
impl CanvasGeometryCombine {
    pub const Union: Self = Self(0i32);
    pub const Intersect: Self = Self(1i32);
    pub const Xor: Self = Self(2i32);
    pub const Exclude: Self = Self(3i32);
}
impl ::core::marker::Copy for CanvasGeometryCombine {}
impl ::core::clone::Clone for CanvasGeometryCombine {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasGeometryCombine {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasGeometryCombine {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasGeometryCombine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasGeometryCombine").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasGeometryCombine {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Geometry.CanvasGeometryCombine;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Geometry"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasGeometryRelation(pub i32);
impl CanvasGeometryRelation {
    pub const Disjoint: Self = Self(0i32);
    pub const Contained: Self = Self(1i32);
    pub const Contains: Self = Self(2i32);
    pub const Overlap: Self = Self(3i32);
}
impl ::core::marker::Copy for CanvasGeometryRelation {}
impl ::core::clone::Clone for CanvasGeometryRelation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasGeometryRelation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasGeometryRelation {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasGeometryRelation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasGeometryRelation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasGeometryRelation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Geometry.CanvasGeometryRelation;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Geometry"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasGeometrySimplification(pub i32);
impl CanvasGeometrySimplification {
    pub const CubicsAndLines: Self = Self(0i32);
    pub const Lines: Self = Self(1i32);
}
impl ::core::marker::Copy for CanvasGeometrySimplification {}
impl ::core::clone::Clone for CanvasGeometrySimplification {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasGeometrySimplification {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasGeometrySimplification {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasGeometrySimplification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasGeometrySimplification").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasGeometrySimplification {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Geometry.CanvasGeometrySimplification;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Geometry"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasGradientMeshPatchEdge(pub i32);
impl CanvasGradientMeshPatchEdge {
    pub const Aliased: Self = Self(0i32);
    pub const Antialiased: Self = Self(1i32);
    pub const AliasedAndInflated: Self = Self(2i32);
}
impl ::core::marker::Copy for CanvasGradientMeshPatchEdge {}
impl ::core::clone::Clone for CanvasGradientMeshPatchEdge {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasGradientMeshPatchEdge {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasGradientMeshPatchEdge {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasGradientMeshPatchEdge {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasGradientMeshPatchEdge").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasGradientMeshPatchEdge {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Geometry.CanvasGradientMeshPatchEdge;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Geometry"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasLineJoin(pub i32);
impl CanvasLineJoin {
    pub const Miter: Self = Self(0i32);
    pub const Bevel: Self = Self(1i32);
    pub const Round: Self = Self(2i32);
    pub const MiterOrBevel: Self = Self(3i32);
}
impl ::core::marker::Copy for CanvasLineJoin {}
impl ::core::clone::Clone for CanvasLineJoin {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasLineJoin {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasLineJoin {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasLineJoin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasLineJoin").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasLineJoin {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Geometry.CanvasLineJoin;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Geometry"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasStrokeTransformBehavior(pub i32);
impl CanvasStrokeTransformBehavior {
    pub const Normal: Self = Self(0i32);
    pub const Fixed: Self = Self(1i32);
    pub const Hairline: Self = Self(2i32);
}
impl ::core::marker::Copy for CanvasStrokeTransformBehavior {}
impl ::core::clone::Clone for CanvasStrokeTransformBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasStrokeTransformBehavior {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasStrokeTransformBehavior {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasStrokeTransformBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasStrokeTransformBehavior").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasStrokeTransformBehavior {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Geometry.CanvasStrokeTransformBehavior;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
///*Required features: `"Graphics_Canvas_Geometry"`*
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CanvasSweepDirection(pub i32);
impl CanvasSweepDirection {
    pub const CounterClockwise: Self = Self(0i32);
    pub const Clockwise: Self = Self(1i32);
}
impl ::core::marker::Copy for CanvasSweepDirection {}
impl ::core::clone::Clone for CanvasSweepDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CanvasSweepDirection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CanvasSweepDirection {
    type Abi = Self;
}
impl ::core::fmt::Debug for CanvasSweepDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CanvasSweepDirection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CanvasSweepDirection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"enum(Microsoft.Graphics.Canvas.Geometry.CanvasSweepDirection;i4)",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
///*Required features: `"Graphics_Canvas_Geometry"`, `"Foundation_Numerics"`*
#[cfg(feature = "Foundation_Numerics")]
pub struct CanvasGradientMeshPatch {
    pub Point00: ::windows::Foundation::Numerics::Vector2,
    pub Point01: ::windows::Foundation::Numerics::Vector2,
    pub Point02: ::windows::Foundation::Numerics::Vector2,
    pub Point03: ::windows::Foundation::Numerics::Vector2,
    pub Point10: ::windows::Foundation::Numerics::Vector2,
    pub Point11: ::windows::Foundation::Numerics::Vector2,
    pub Point12: ::windows::Foundation::Numerics::Vector2,
    pub Point13: ::windows::Foundation::Numerics::Vector2,
    pub Point20: ::windows::Foundation::Numerics::Vector2,
    pub Point21: ::windows::Foundation::Numerics::Vector2,
    pub Point22: ::windows::Foundation::Numerics::Vector2,
    pub Point23: ::windows::Foundation::Numerics::Vector2,
    pub Point30: ::windows::Foundation::Numerics::Vector2,
    pub Point31: ::windows::Foundation::Numerics::Vector2,
    pub Point32: ::windows::Foundation::Numerics::Vector2,
    pub Point33: ::windows::Foundation::Numerics::Vector2,
    pub Color00: ::windows::Foundation::Numerics::Vector4,
    pub Color03: ::windows::Foundation::Numerics::Vector4,
    pub Color30: ::windows::Foundation::Numerics::Vector4,
    pub Color33: ::windows::Foundation::Numerics::Vector4,
    pub Edge00To03: CanvasGradientMeshPatchEdge,
    pub Edge03To33: CanvasGradientMeshPatchEdge,
    pub Edge33To30: CanvasGradientMeshPatchEdge,
    pub Edge30To00: CanvasGradientMeshPatchEdge,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for CanvasGradientMeshPatch {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for CanvasGradientMeshPatch {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for CanvasGradientMeshPatch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CanvasGradientMeshPatch")
            .field("Point00", &self.Point00)
            .field("Point01", &self.Point01)
            .field("Point02", &self.Point02)
            .field("Point03", &self.Point03)
            .field("Point10", &self.Point10)
            .field("Point11", &self.Point11)
            .field("Point12", &self.Point12)
            .field("Point13", &self.Point13)
            .field("Point20", &self.Point20)
            .field("Point21", &self.Point21)
            .field("Point22", &self.Point22)
            .field("Point23", &self.Point23)
            .field("Point30", &self.Point30)
            .field("Point31", &self.Point31)
            .field("Point32", &self.Point32)
            .field("Point33", &self.Point33)
            .field("Color00", &self.Color00)
            .field("Color03", &self.Color03)
            .field("Color30", &self.Color30)
            .field("Color33", &self.Color33)
            .field("Edge00To03", &self.Edge00To03)
            .field("Edge03To33", &self.Edge03To33)
            .field("Edge33To30", &self.Edge33To30)
            .field("Edge30To00", &self.Edge30To00)
            .finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::Abi for CanvasGradientMeshPatch {
    type Abi = Self;
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::RuntimeType for CanvasGradientMeshPatch {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.Graphics.Canvas.Geometry.CanvasGradientMeshPatch;struct(Windows.Foundation.Numerics.Vector2;f4;f4);struct(Windows.Foundation.Numerics.Vector2;f4;f4);struct(Windows.Foundation.Numerics.Vector2;f4;f4);struct(Windows.Foundation.Numerics.Vector2;f4;f4);struct(Windows.Foundation.Numerics.Vector2;f4;f4);struct(Windows.Foundation.Numerics.Vector2;f4;f4);struct(Windows.Foundation.Numerics.Vector2;f4;f4);struct(Windows.Foundation.Numerics.Vector2;f4;f4);struct(Windows.Foundation.Numerics.Vector2;f4;f4);struct(Windows.Foundation.Numerics.Vector2;f4;f4);struct(Windows.Foundation.Numerics.Vector2;f4;f4);struct(Windows.Foundation.Numerics.Vector2;f4;f4);struct(Windows.Foundation.Numerics.Vector2;f4;f4);struct(Windows.Foundation.Numerics.Vector2;f4;f4);struct(Windows.Foundation.Numerics.Vector2;f4;f4);struct(Windows.Foundation.Numerics.Vector2;f4;f4);struct(Windows.Foundation.Numerics.Vector4;f4;f4;f4;f4);struct(Windows.Foundation.Numerics.Vector4;f4;f4;f4;f4);struct(Windows.Foundation.Numerics.Vector4;f4;f4;f4;f4);struct(Windows.Foundation.Numerics.Vector4;f4;f4;f4;f4);enum(Microsoft.Graphics.Canvas.Geometry.CanvasGradientMeshPatchEdge;i4);enum(Microsoft.Graphics.Canvas.Geometry.CanvasGradientMeshPatchEdge;i4);enum(Microsoft.Graphics.Canvas.Geometry.CanvasGradientMeshPatchEdge;i4);enum(Microsoft.Graphics.Canvas.Geometry.CanvasGradientMeshPatchEdge;i4))",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for CanvasGradientMeshPatch {
    fn eq(&self, other: &Self) -> bool {
        self.Point00 == other.Point00 && self.Point01 == other.Point01
            && self.Point02 == other.Point02 && self.Point03 == other.Point03
            && self.Point10 == other.Point10 && self.Point11 == other.Point11
            && self.Point12 == other.Point12 && self.Point13 == other.Point13
            && self.Point20 == other.Point20 && self.Point21 == other.Point21
            && self.Point22 == other.Point22 && self.Point23 == other.Point23
            && self.Point30 == other.Point30 && self.Point31 == other.Point31
            && self.Point32 == other.Point32 && self.Point33 == other.Point33
            && self.Color00 == other.Color00 && self.Color03 == other.Color03
            && self.Color30 == other.Color30 && self.Color33 == other.Color33
            && self.Edge00To03 == other.Edge00To03 && self.Edge03To33 == other.Edge03To33
            && self.Edge33To30 == other.Edge33To30 && self.Edge30To00 == other.Edge30To00
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for CanvasGradientMeshPatch {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for CanvasGradientMeshPatch {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
///*Required features: `"Graphics_Canvas_Geometry"`, `"Foundation_Numerics"`*
#[cfg(feature = "Foundation_Numerics")]
pub struct CanvasTriangleVertices {
    pub Vertex1: ::windows::Foundation::Numerics::Vector2,
    pub Vertex2: ::windows::Foundation::Numerics::Vector2,
    pub Vertex3: ::windows::Foundation::Numerics::Vector2,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for CanvasTriangleVertices {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for CanvasTriangleVertices {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::fmt::Debug for CanvasTriangleVertices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CanvasTriangleVertices")
            .field("Vertex1", &self.Vertex1)
            .field("Vertex2", &self.Vertex2)
            .field("Vertex3", &self.Vertex3)
            .finish()
    }
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::Abi for CanvasTriangleVertices {
    type Abi = Self;
}
#[cfg(feature = "Foundation_Numerics")]
unsafe impl ::windows::core::RuntimeType for CanvasTriangleVertices {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(
        b"struct(Microsoft.Graphics.Canvas.Geometry.CanvasTriangleVertices;struct(Windows.Foundation.Numerics.Vector2;f4;f4);struct(Windows.Foundation.Numerics.Vector2;f4;f4);struct(Windows.Foundation.Numerics.Vector2;f4;f4))",
    );
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::PartialEq for CanvasTriangleVertices {
    fn eq(&self, other: &Self) -> bool {
        self.Vertex1 == other.Vertex1 && self.Vertex2 == other.Vertex2
            && self.Vertex3 == other.Vertex3
    }
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::cmp::Eq for CanvasTriangleVertices {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::default::Default for CanvasTriangleVertices {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
