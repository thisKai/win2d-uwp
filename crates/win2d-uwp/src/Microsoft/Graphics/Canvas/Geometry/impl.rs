///*Required features: `"Graphics_Canvas_Geometry"`, `"Foundation_Numerics"`, `"implement"`*
#[cfg(feature = "Foundation_Numerics")]
pub trait ICanvasPathReceiver_Impl: Sized {
    fn BeginFigure(
        &self,
        startpoint: &::windows::Foundation::Numerics::Vector2,
        figurefill: CanvasFigureFill,
    ) -> ::windows::core::Result<()>;
    fn AddArc(
        &self,
        endpoint: &::windows::Foundation::Numerics::Vector2,
        radiusx: f32,
        radiusy: f32,
        rotationangle: f32,
        sweepdirection: CanvasSweepDirection,
        arcsize: CanvasArcSize,
    ) -> ::windows::core::Result<()>;
    fn AddCubicBezier(
        &self,
        controlpoint1: &::windows::Foundation::Numerics::Vector2,
        controlpoint2: &::windows::Foundation::Numerics::Vector2,
        endpoint: &::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::Result<()>;
    fn AddLine(
        &self,
        endpoint: &::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::Result<()>;
    fn AddQuadraticBezier(
        &self,
        controlpoint: &::windows::Foundation::Numerics::Vector2,
        endpoint: &::windows::Foundation::Numerics::Vector2,
    ) -> ::windows::core::Result<()>;
    fn SetFilledRegionDetermination(
        &self,
        filledregiondetermination: CanvasFilledRegionDetermination,
    ) -> ::windows::core::Result<()>;
    fn SetSegmentOptions(
        &self,
        figuresegmentoptions: CanvasFigureSegmentOptions,
    ) -> ::windows::core::Result<()>;
    fn EndFigure(&self, figureloop: CanvasFigureLoop) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation_Numerics")]
impl ::windows::core::RuntimeName for ICanvasPathReceiver {
    const NAME: &'static str = "Microsoft.Graphics.Canvas.Geometry.ICanvasPathReceiver";
}
#[cfg(feature = "Foundation_Numerics")]
impl ICanvasPathReceiver_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: ICanvasPathReceiver_Impl,
        const OFFSET: isize,
    >() -> ICanvasPathReceiver_Vtbl {
        unsafe extern "system" fn BeginFigure<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasPathReceiver_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            startpoint: ::windows::Foundation::Numerics::Vector2,
            figurefill: CanvasFigureFill,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginFigure(::core::mem::transmute(&startpoint), figurefill).into()
        }
        unsafe extern "system" fn AddArc<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasPathReceiver_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            endpoint: ::windows::Foundation::Numerics::Vector2,
            radiusx: f32,
            radiusy: f32,
            rotationangle: f32,
            sweepdirection: CanvasSweepDirection,
            arcsize: CanvasArcSize,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddArc(
                    ::core::mem::transmute(&endpoint),
                    radiusx,
                    radiusy,
                    rotationangle,
                    sweepdirection,
                    arcsize,
                )
                .into()
        }
        unsafe extern "system" fn AddCubicBezier<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasPathReceiver_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            controlpoint1: ::windows::Foundation::Numerics::Vector2,
            controlpoint2: ::windows::Foundation::Numerics::Vector2,
            endpoint: ::windows::Foundation::Numerics::Vector2,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddCubicBezier(
                    ::core::mem::transmute(&controlpoint1),
                    ::core::mem::transmute(&controlpoint2),
                    ::core::mem::transmute(&endpoint),
                )
                .into()
        }
        unsafe extern "system" fn AddLine<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasPathReceiver_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            endpoint: ::windows::Foundation::Numerics::Vector2,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddLine(::core::mem::transmute(&endpoint)).into()
        }
        unsafe extern "system" fn AddQuadraticBezier<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasPathReceiver_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            controlpoint: ::windows::Foundation::Numerics::Vector2,
            endpoint: ::windows::Foundation::Numerics::Vector2,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddQuadraticBezier(
                    ::core::mem::transmute(&controlpoint),
                    ::core::mem::transmute(&endpoint),
                )
                .into()
        }
        unsafe extern "system" fn SetFilledRegionDetermination<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasPathReceiver_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            filledregiondetermination: CanvasFilledRegionDetermination,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFilledRegionDetermination(filledregiondetermination).into()
        }
        unsafe extern "system" fn SetSegmentOptions<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasPathReceiver_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            figuresegmentoptions: CanvasFigureSegmentOptions,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSegmentOptions(figuresegmentoptions).into()
        }
        unsafe extern "system" fn EndFigure<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: ICanvasPathReceiver_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            figureloop: CanvasFigureLoop,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndFigure(figureloop).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<
                Identity,
                ICanvasPathReceiver,
                OFFSET,
            >(),
            BeginFigure: BeginFigure::<Identity, Impl, OFFSET>,
            AddArc: AddArc::<Identity, Impl, OFFSET>,
            AddCubicBezier: AddCubicBezier::<Identity, Impl, OFFSET>,
            AddLine: AddLine::<Identity, Impl, OFFSET>,
            AddQuadraticBezier: AddQuadraticBezier::<Identity, Impl, OFFSET>,
            SetFilledRegionDetermination: SetFilledRegionDetermination::<
                Identity,
                Impl,
                OFFSET,
            >,
            SetSegmentOptions: SetSegmentOptions::<Identity, Impl, OFFSET>,
            EndFigure: EndFigure::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICanvasPathReceiver as ::windows::core::Interface>::IID
    }
}
