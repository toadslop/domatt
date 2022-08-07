use super::html_attributes::AriaRole;
use crate::{Attribute, NumberOrString};
use std::fmt::Display;
use strum::Display;

/// An enum defining the different attribute keys for SVG elements. Each variant takes a tuple
/// that represents the valid values for the attributes.
#[derive(Debug, Clone, Display)]
#[strum(serialize_all = "kebab-case")]
pub enum SVGAttributes<'a> {
    // Note: class is already handled well by Yew, so we are disabling it here.
    // Class(String),
    Color(&'a str),
    Height(NumberOrString<'a>),
    Id(&'a str),
    Lang(&'a str),
    Max(NumberOrString<'a>),
    Media(&'a str),
    Method(&'a str),
    Min(NumberOrString<'a>),
    Name(&'a str),
    // Note: implementing css properties is unneccessary since style tags are already handled by Yew and Stylist
    // Style: CSSProperties
    Target(&'a str),
    Type(&'a str),
    Width(NumberOrString<'a>),
    // Other HTML properties supported by SVG elements in browsers
    Role(&'a AriaRole),
    TabIndex(i64),
    CrossOrigin(&'a CrossOrigin),
    // SVG Specific attributes
    AccentHeight(NumberOrString<'a>),
    Accumulate(&'a Accumulate),
    Additive(&'a Additive),
    AlignmentBaseline(&'a AlignmentBaseline),
    #[strum(serialize = "allowReorder")]
    AllowReorder(&'a AllowReorder),
    Alphabetic(NumberOrString<'a>),
    Amplitude(NumberOrString<'a>),
    ArabicForm(&'a ArabicForm),
    Ascent(NumberOrString<'a>),
    #[strum(serialize = "attributeName")]
    AttributeName(&'a str),
    #[strum(serialize = "attributeType")]
    AttributeType(&'a str),
    AutoReverse(bool),
    Azimuth(NumberOrString<'a>),
    #[strum(serialize = "baseFrequency")]
    BaseFrequency(NumberOrString<'a>),
    BaselineShift(NumberOrString<'a>),
    #[strum(serialize = "baseProfile")]
    BaseProfile(NumberOrString<'a>),
    Bbox(NumberOrString<'a>),
    Begin(NumberOrString<'a>),
    Bias(NumberOrString<'a>),
    By(NumberOrString<'a>),
    CalcMode(NumberOrString<'a>),
    CapHeight(NumberOrString<'a>),
    Clip(NumberOrString<'a>),
    #[strum(serialize = "clipPath")]
    ClipPath(&'a str),
    #[strum(serialize = "clipPathUnits")]
    ClipPathUnits(ClipPathUnits),
    ClipRule(ClipRule),
    ColorInterpolation(NumberOrString<'a>),
    ColorInterpolationFilters(ColorInterpolationFilters),
    ColorProfile(NumberOrString<'a>),
    ColorRendering(NumberOrString<'a>),
    #[strum(serialize = "contentScriptType")]
    ContentScriptType(NumberOrString<'a>),
    #[strum(serialize = "contentStyleType")]
    ContentStyleType(NumberOrString<'a>),
    Cursor(NumberOrString<'a>),
    Cx(NumberOrString<'a>),
    Cy(NumberOrString<'a>),
    D(&'a str),
    Decelerate(NumberOrString<'a>),
    Descent(NumberOrString<'a>),
    #[strum(serialize = "diffuseConstant")]
    DiffuseConstant(NumberOrString<'a>),
    Direction(NumberOrString<'a>),
    Display(NumberOrString<'a>),
    Divisor(NumberOrString<'a>),
    DominantBaseline(NumberOrString<'a>),
    Dur(NumberOrString<'a>),
    Dx(NumberOrString<'a>),
    Dy(NumberOrString<'a>),
    #[strum(serialize = "edgeMode")]
    EdgeMode(NumberOrString<'a>),
    Elevation(NumberOrString<'a>),
    EnableBackground(NumberOrString<'a>),
    End(NumberOrString<'a>),
    Exponent(NumberOrString<'a>),
    #[strum(serialize = "externalResourcesRequired")]
    ExternalResourcesRequired(bool),
    Fill(&'a str),
    FillOpacity(NumberOrString<'a>),
    FillRule(FillRule),
    Filter(&'a str),
    #[strum(serialize = "filterRes")]
    FilterRes(NumberOrString<'a>),
    #[strum(serialize = "filterUnits")]
    FilterUnits(NumberOrString<'a>),
    FloodColor(NumberOrString<'a>),
    FloodOpacity(NumberOrString<'a>),
    Focusable(Focusable),
    FontFamily(&'a str),
    FontSize(NumberOrString<'a>),
    FontSizeAdjust(NumberOrString<'a>),
    FontStretch(NumberOrString<'a>),
    FontStyle(NumberOrString<'a>),
    FontVariant(NumberOrString<'a>),
    FontWeight(NumberOrString<'a>),
    Format(NumberOrString<'a>),
    Fr(NumberOrString<'a>),
    From(NumberOrString<'a>),
    Fx(NumberOrString<'a>),
    Fy(NumberOrString<'a>),
    G1(NumberOrString<'a>),
    G2(NumberOrString<'a>),
    GlyphName(NumberOrString<'a>),
    GlyphOrientationHorizontal(NumberOrString<'a>),
    GlyphOrientationVertical(NumberOrString<'a>),
    #[strum(serialize = "glyphRef")]
    GlyphRef(NumberOrString<'a>),
    #[strum(serialize = "gradientTransform")]
    GradientTransform(String),
    #[strum(serialize = "gradientUnits")]
    GradientUnits(&'a str),
    Hanging(NumberOrString<'a>),
    #[strum(serialize = "horizAdvX")]
    HorizAdvX(NumberOrString<'a>),
    HorizOriginX(NumberOrString<'a>),
    Href(&'a str),
    Ideographic(NumberOrString<'a>),
    ImageRendering(NumberOrString<'a>),
    In2(NumberOrString<'a>),
    In(&'a str),
    Intercept(NumberOrString<'a>),
    K1(NumberOrString<'a>),
    K2(NumberOrString<'a>),
    K3(NumberOrString<'a>),
    K4(NumberOrString<'a>),
    K(NumberOrString<'a>),
    #[strum(serialize = "kernelMatrix")]
    KernelMatrix(NumberOrString<'a>),
    #[strum(serialize = "kernelUnitLength")]
    KernelUnitLength(NumberOrString<'a>),
    Kerning(NumberOrString<'a>),
    #[strum(serialize = "keyPoints")]
    KeyPoints(NumberOrString<'a>),
    #[strum(serialize = "keySplines")]
    KeySplines(NumberOrString<'a>),
    #[strum(serialize = "keyTimes")]
    KeyTimes(NumberOrString<'a>),
    #[strum(serialize = "lengthAdjust")]
    LengthAdjust(NumberOrString<'a>),
    LetterSpacing(NumberOrString<'a>),
    LightingColor(NumberOrString<'a>),
    #[strum(serialize = "limitingConeAngle")]
    LimitingConeAngle(NumberOrString<'a>),
    Local(NumberOrString<'a>),
    MarkerEnd(&'a str),
    #[strum(serialize = "markerHeight")]
    MarkerHeight(NumberOrString<'a>),
    MarkerMid(&'a str),
    MarkerStart(&'a str),
    #[strum(serialize = "markerUnits")]
    MarkerUnits(NumberOrString<'a>),
    #[strum(serialize = "markerWidth")]
    MarkerWidth(NumberOrString<'a>),
    Mask(&'a str),
    #[strum(serialize = "maskContentUnits")]
    MaskContentUnits(NumberOrString<'a>),
    #[strum(serialize = "maskUnits")]
    MaskUnits(NumberOrString<'a>),
    Mathematical(NumberOrString<'a>),
    Mode(NumberOrString<'a>),
    #[strum(serialize = "numOctaves")]
    NumOctaves(NumberOrString<'a>),
    Offset(NumberOrString<'a>),
    Opacity(NumberOrString<'a>),
    Operator(NumberOrString<'a>),
    Order(NumberOrString<'a>),
    Orient(NumberOrString<'a>),
    Orientation(NumberOrString<'a>),
    Origin(NumberOrString<'a>),
    Overflow(NumberOrString<'a>),
    OverlinePosition(NumberOrString<'a>),
    PverlineThickness(NumberOrString<'a>),
    PaintOrder(NumberOrString<'a>),
    #[strum(serialize = "panose-1")]
    Panose1(NumberOrString<'a>),
    Path(&'a str),
    #[strum(serialize = "pathLength")]
    PathLength(NumberOrString<'a>),
    #[strum(serialize = "patternContentUnits")]
    PatternContentUnits(&'a str),
    #[strum(serialize = "patternTransform")]
    PatternTransform(NumberOrString<'a>),
    #[strum(serialize = "patternUnits")]
    PatternUnits(&'a str),
    PointerEvents(NumberOrString<'a>),
    Points(&'a str),
    #[strum(serialize = "pointsAtX")]
    PointsAtX(NumberOrString<'a>),
    #[strum(serialize = "pointsAtY")]
    PointsAtY(NumberOrString<'a>),
    #[strum(serialize = "pointsAtZ")]
    PointsAtZ(NumberOrString<'a>),
    #[strum(serialize = "preserveAlpha")]
    PreserveAlpha(bool),
    #[strum(serialize = "preserveAspectRatio")]
    OreserveAspectRatio(&'a str),
    #[strum(serialize = "primitiveUnits")]
    PrimitiveUnits(NumberOrString<'a>),
    R(NumberOrString<'a>),
    Radius(NumberOrString<'a>),
    #[strum(serialize = "refX")]
    RefX(NumberOrString<'a>),
    #[strum(serialize = "refY")]
    RefY(NumberOrString<'a>),
    RenderingIntent(NumberOrString<'a>),
    #[strum(serialize = "repeatCount")]
    RepeatCount(NumberOrString<'a>),
    #[strum(serialize = "repeatDur")]
    RepeatDur(NumberOrString<'a>),
    #[strum(serialize = "requiredExtensions")]
    RequiredExtensions(NumberOrString<'a>),
    #[strum(serialize = "requiredFeatures")]
    RequiredFeatures(NumberOrString<'a>),
    Restart(NumberOrString<'a>),
    Result(&'a str),
    Rotate(NumberOrString<'a>),
    Rx(NumberOrString<'a>),
    Ry(NumberOrString<'a>),
    Scale(NumberOrString<'a>),
    Seed(NumberOrString<'a>),
    ShapeRendering(NumberOrString<'a>),
    Slope(NumberOrString<'a>),
    Spacing(NumberOrString<'a>),
    #[strum(serialize = "requiredFeatures")]
    SpecularConstant(NumberOrString<'a>),
    #[strum(serialize = "specularExponent")]
    SpecularExponent(NumberOrString<'a>),
    Speed(NumberOrString<'a>),
    #[strum(serialize = "spreadMethod")]
    SpreadMethod(&'a str),
    #[strum(serialize = "startOffset")]
    StartOffset(NumberOrString<'a>),
    #[strum(serialize = "stdDeviation")]
    StdDeviation(NumberOrString<'a>),
    Stemh(NumberOrString<'a>),
    Stemv(NumberOrString<'a>),
    #[strum(serialize = "stitchTiles")]
    StitchTiles(NumberOrString<'a>),
    StopColor(&'a str),
    StopOpacity(NumberOrString<'a>),
    StrikethroughPosition(NumberOrString<'a>),
    StrikethroughThickness(NumberOrString<'a>),
    String(NumberOrString<'a>),
    Stroke(&'a str),
    StrokeDasharray(NumberOrString<'a>),
    StrokeDashoffset(NumberOrString<'a>),
    StrokeLinecap(StrokeLinecap),
    StrokeLinejoin(StrokeLinejoin),
    StrokeMiterlimit(NumberOrString<'a>),
    StrokeOpacity(NumberOrString<'a>),
    StrokeWidth(NumberOrString<'a>),
    #[strum(serialize = "surfaceScale")]
    SurfaceScale(NumberOrString<'a>),
    #[strum(serialize = "systemLanguage")]
    SystemLanguage(NumberOrString<'a>),
    #[strum(serialize = "tableValues")]
    TableValues(NumberOrString<'a>),
    #[strum(serialize = "targetX")]
    TargetX(NumberOrString<'a>),
    #[strum(serialize = "targetY")]
    TargetY(NumberOrString<'a>),
    TextAnchor(String),
    TextDecoration(NumberOrString<'a>),
    #[strum(serialize = "textLength")]
    TextLength(NumberOrString<'a>),
    TextRendering(NumberOrString<'a>),
    To(NumberOrString<'a>),
    Transform(&'a str),
    U1(NumberOrString<'a>),
    U2(NumberOrString<'a>),
    EnderlinePosition(NumberOrString<'a>),
    UnderlineThickness(NumberOrString<'a>),
    Unicode(NumberOrString<'a>),
    UnicodeBidi(NumberOrString<'a>),
    UnicodeRange(NumberOrString<'a>),
    UnitsPerEm(NumberOrString<'a>),
    VAlphabetic(NumberOrString<'a>),
    Values(&'a str),
    VectorEffect(NumberOrString<'a>),
    Version(&'a str),
    VertAdvY(NumberOrString<'a>),
    VertOriginX(NumberOrString<'a>),
    VertOriginY(NumberOrString<'a>),
    VHanging(NumberOrString<'a>),
    VIdeographic(NumberOrString<'a>),
    #[strum(serialize = "viewBox")]
    SiewBox(&'a str),
    #[strum(serialize = "viewTarget")]
    ViewTarget(NumberOrString<'a>),
    Visibility(NumberOrString<'a>),
    VMathematical(NumberOrString<'a>),
    Widths(NumberOrString<'a>),
    WordSpacing(NumberOrString<'a>),
    WritingMode(NumberOrString<'a>),
    #[strum(serialize = "x1")]
    X1(NumberOrString<'a>),
    #[strum(serialize = "x2")]
    X2(NumberOrString<'a>),
    X(NumberOrString<'a>),
    #[strum(serialize = "xChannelSelector")]
    XChannelSelector(&'a str),
    XHeight(NumberOrString<'a>),
    #[strum(serialize = "xlink:actuate")]
    XlinkActuate(&'a str),
    #[strum(serialize = "xlink:arcrole")]
    XlinkArcrole(&'a str),
    #[strum(serialize = "xlink:href")]
    XlinkHref(&'a str),
    #[strum(serialize = "xlink:role")]
    XlinkRole(&'a str),
    #[strum(serialize = "xlink:show")]
    XlinkShow(&'a str),
    #[strum(serialize = "xlink:title")]
    XlinkTitle(&'a str),
    #[strum(serialize = "xlink:type")]
    XlinkType(&'a str),
    #[strum(serialize = "xml:base")]
    XmlBase(&'a str),
    #[strum(serialize = "xml:lang")]
    XmlLang(&'a str),
    Xmlns(&'a str),
    #[strum(serialize = "xmlns:xlink")]
    XmlnsXlink(&'a str),
    #[strum(serialize = "xml:space")]
    XmlSpace(&'a str),
    #[strum(serialize = "y1")]
    Y1(NumberOrString<'a>),
    #[strum(serialize = "y2")]
    Y2(NumberOrString<'a>),
    Y(NumberOrString<'a>),
    #[strum(serialize = "yChannelSelector ")]
    YChannelSelector(&'a str),
    Z(NumberOrString<'a>),
    #[strum(serialize = "zoomAndPan")]
    ZoomAndPan(&'a str),
}

impl<'a> Attribute for SVGAttributes<'a> {
    fn get_key(&self) -> String {
        self.to_string()
    }

    fn get_val(&self) -> Option<String> {
        match self {
            SVGAttributes::Color(val) => Some(val.to_string()),
            SVGAttributes::Height(val) => Some(val.to_string()),
            SVGAttributes::Id(val) => Some(val.to_string()),
            SVGAttributes::Lang(val) => Some(val.to_string()),
            SVGAttributes::Max(val) => Some(val.to_string()),
            SVGAttributes::Media(val) => Some(val.to_string()),
            SVGAttributes::Method(val) => Some(val.to_string()),
            SVGAttributes::Min(val) => Some(val.to_string()),
            SVGAttributes::Name(val) => Some(val.to_string()),
            SVGAttributes::Target(val) => Some(val.to_string()),
            SVGAttributes::Type(val) => Some(val.to_string()),
            SVGAttributes::Width(val) => Some(val.to_string()),
            SVGAttributes::Role(val) => Some(val.to_string()),
            SVGAttributes::TabIndex(val) => Some(val.to_string()),
            SVGAttributes::CrossOrigin(val) => Some(val.to_string()),
            SVGAttributes::AccentHeight(val) => Some(val.to_string()),
            SVGAttributes::Accumulate(val) => Some(val.to_string()),
            SVGAttributes::Additive(val) => Some(val.to_string()),
            SVGAttributes::AlignmentBaseline(val) => Some(val.to_string()),
            SVGAttributes::AllowReorder(val) => Some(val.to_string()),
            SVGAttributes::Alphabetic(val) => Some(val.to_string()),
            SVGAttributes::Amplitude(val) => Some(val.to_string()),
            SVGAttributes::ArabicForm(val) => Some(val.to_string()),
            SVGAttributes::Ascent(val) => Some(val.to_string()),
            SVGAttributes::AttributeName(val) => Some(val.to_string()),
            SVGAttributes::AttributeType(val) => Some(val.to_string()),
            SVGAttributes::AutoReverse(val) => Some(val.to_string()),
            SVGAttributes::Azimuth(val) => Some(val.to_string()),
            SVGAttributes::BaseFrequency(val) => Some(val.to_string()),
            SVGAttributes::BaselineShift(val) => Some(val.to_string()),
            SVGAttributes::BaseProfile(val) => Some(val.to_string()),
            SVGAttributes::Bbox(val) => Some(val.to_string()),
            SVGAttributes::Begin(val) => Some(val.to_string()),
            SVGAttributes::Bias(val) => Some(val.to_string()),
            SVGAttributes::By(val) => Some(val.to_string()),
            SVGAttributes::CalcMode(val) => Some(val.to_string()),
            SVGAttributes::CapHeight(val) => Some(val.to_string()),
            SVGAttributes::Clip(val) => Some(val.to_string()),
            SVGAttributes::ClipPath(val) => Some(val.to_string()),
            SVGAttributes::ClipPathUnits(val) => Some(val.to_string()),
            SVGAttributes::ClipRule(val) => Some(val.to_string()),
            SVGAttributes::ColorInterpolation(val) => Some(val.to_string()),
            SVGAttributes::ColorInterpolationFilters(val) => Some(val.to_string()),
            SVGAttributes::ColorProfile(val) => Some(val.to_string()),
            SVGAttributes::ColorRendering(val) => Some(val.to_string()),
            SVGAttributes::ContentScriptType(val) => Some(val.to_string()),
            SVGAttributes::ContentStyleType(val) => Some(val.to_string()),
            SVGAttributes::Cursor(val) => Some(val.to_string()),
            SVGAttributes::Cx(val) => Some(val.to_string()),
            SVGAttributes::Cy(val) => Some(val.to_string()),
            SVGAttributes::D(val) => Some(val.to_string()),
            SVGAttributes::Decelerate(val) => Some(val.to_string()),
            SVGAttributes::Descent(val) => Some(val.to_string()),
            SVGAttributes::DiffuseConstant(val) => Some(val.to_string()),
            SVGAttributes::Direction(val) => Some(val.to_string()),
            SVGAttributes::Display(val) => Some(val.to_string()),
            SVGAttributes::Divisor(val) => Some(val.to_string()),
            SVGAttributes::DominantBaseline(val) => Some(val.to_string()),
            SVGAttributes::Dur(val) => Some(val.to_string()),
            SVGAttributes::Dx(val) => Some(val.to_string()),
            SVGAttributes::Dy(val) => Some(val.to_string()),
            SVGAttributes::EdgeMode(val) => Some(val.to_string()),
            SVGAttributes::Elevation(val) => Some(val.to_string()),
            SVGAttributes::EnableBackground(val) => Some(val.to_string()),
            SVGAttributes::End(val) => Some(val.to_string()),
            SVGAttributes::Exponent(val) => Some(val.to_string()),
            SVGAttributes::ExternalResourcesRequired(val) => Some(val.to_string()),
            SVGAttributes::Fill(val) => Some(val.to_string()),
            SVGAttributes::FillOpacity(val) => Some(val.to_string()),
            SVGAttributes::FillRule(val) => Some(val.to_string()),
            SVGAttributes::Filter(val) => Some(val.to_string()),
            SVGAttributes::FilterRes(val) => Some(val.to_string()),
            SVGAttributes::FilterUnits(val) => Some(val.to_string()),
            SVGAttributes::FloodColor(val) => Some(val.to_string()),
            SVGAttributes::FloodOpacity(val) => Some(val.to_string()),
            SVGAttributes::Focusable(val) => Some(val.to_string()),
            SVGAttributes::FontFamily(val) => Some(val.to_string()),
            SVGAttributes::FontSize(val) => Some(val.to_string()),
            SVGAttributes::FontSizeAdjust(val) => Some(val.to_string()),
            SVGAttributes::FontStretch(val) => Some(val.to_string()),
            SVGAttributes::FontStyle(val) => Some(val.to_string()),
            SVGAttributes::FontVariant(val) => Some(val.to_string()),
            SVGAttributes::FontWeight(val) => Some(val.to_string()),
            SVGAttributes::Format(val) => Some(val.to_string()),
            SVGAttributes::Fr(val) => Some(val.to_string()),
            SVGAttributes::From(val) => Some(val.to_string()),
            SVGAttributes::Fx(val) => Some(val.to_string()),
            SVGAttributes::Fy(val) => Some(val.to_string()),
            SVGAttributes::G1(val) => Some(val.to_string()),
            SVGAttributes::G2(val) => Some(val.to_string()),
            SVGAttributes::GlyphName(val) => Some(val.to_string()),
            SVGAttributes::GlyphOrientationHorizontal(val) => Some(val.to_string()),
            SVGAttributes::GlyphOrientationVertical(val) => Some(val.to_string()),
            SVGAttributes::GlyphRef(val) => Some(val.to_string()),
            SVGAttributes::GradientTransform(val) => Some(val.to_string()),
            SVGAttributes::GradientUnits(val) => Some(val.to_string()),
            SVGAttributes::Hanging(val) => Some(val.to_string()),
            SVGAttributes::HorizAdvX(val) => Some(val.to_string()),
            SVGAttributes::HorizOriginX(val) => Some(val.to_string()),
            SVGAttributes::Href(val) => Some(val.to_string()),
            SVGAttributes::Ideographic(val) => Some(val.to_string()),
            SVGAttributes::ImageRendering(val) => Some(val.to_string()),
            SVGAttributes::In2(val) => Some(val.to_string()),
            SVGAttributes::In(val) => Some(val.to_string()),
            SVGAttributes::Intercept(val) => Some(val.to_string()),
            SVGAttributes::K1(val) => Some(val.to_string()),
            SVGAttributes::K2(val) => Some(val.to_string()),
            SVGAttributes::K3(val) => Some(val.to_string()),
            SVGAttributes::K4(val) => Some(val.to_string()),
            SVGAttributes::K(val) => Some(val.to_string()),
            SVGAttributes::KernelMatrix(val) => Some(val.to_string()),
            SVGAttributes::KernelUnitLength(val) => Some(val.to_string()),
            SVGAttributes::Kerning(val) => Some(val.to_string()),
            SVGAttributes::KeyPoints(val) => Some(val.to_string()),
            SVGAttributes::KeySplines(val) => Some(val.to_string()),
            SVGAttributes::KeyTimes(val) => Some(val.to_string()),
            SVGAttributes::LengthAdjust(val) => Some(val.to_string()),
            SVGAttributes::LetterSpacing(val) => Some(val.to_string()),
            SVGAttributes::LightingColor(val) => Some(val.to_string()),
            SVGAttributes::LimitingConeAngle(val) => Some(val.to_string()),
            SVGAttributes::Local(val) => Some(val.to_string()),
            SVGAttributes::MarkerEnd(val) => Some(val.to_string()),
            SVGAttributes::MarkerHeight(val) => Some(val.to_string()),
            SVGAttributes::MarkerMid(val) => Some(val.to_string()),
            SVGAttributes::MarkerStart(val) => Some(val.to_string()),
            SVGAttributes::MarkerUnits(val) => Some(val.to_string()),
            SVGAttributes::MarkerWidth(val) => Some(val.to_string()),
            SVGAttributes::Mask(val) => Some(val.to_string()),
            SVGAttributes::MaskContentUnits(val) => Some(val.to_string()),
            SVGAttributes::MaskUnits(val) => Some(val.to_string()),
            SVGAttributes::Mathematical(val) => Some(val.to_string()),
            SVGAttributes::Mode(val) => Some(val.to_string()),
            SVGAttributes::NumOctaves(val) => Some(val.to_string()),
            SVGAttributes::Offset(val) => Some(val.to_string()),
            SVGAttributes::Opacity(val) => Some(val.to_string()),
            SVGAttributes::Operator(val) => Some(val.to_string()),
            SVGAttributes::Order(val) => Some(val.to_string()),
            SVGAttributes::Orient(val) => Some(val.to_string()),
            SVGAttributes::Orientation(val) => Some(val.to_string()),
            SVGAttributes::Origin(val) => Some(val.to_string()),
            SVGAttributes::Overflow(val) => Some(val.to_string()),
            SVGAttributes::OverlinePosition(val) => Some(val.to_string()),
            SVGAttributes::PverlineThickness(val) => Some(val.to_string()),
            SVGAttributes::PaintOrder(val) => Some(val.to_string()),
            SVGAttributes::Panose1(val) => Some(val.to_string()),
            SVGAttributes::Path(val) => Some(val.to_string()),
            SVGAttributes::PathLength(val) => Some(val.to_string()),
            SVGAttributes::PatternContentUnits(val) => Some(val.to_string()),
            SVGAttributes::PatternTransform(val) => Some(val.to_string()),
            SVGAttributes::PatternUnits(val) => Some(val.to_string()),
            SVGAttributes::PointerEvents(val) => Some(val.to_string()),
            SVGAttributes::Points(val) => Some(val.to_string()),
            SVGAttributes::PointsAtX(val) => Some(val.to_string()),
            SVGAttributes::PointsAtY(val) => Some(val.to_string()),
            SVGAttributes::PointsAtZ(val) => Some(val.to_string()),
            SVGAttributes::PreserveAlpha(val) => Some(val.to_string()),
            SVGAttributes::OreserveAspectRatio(val) => Some(val.to_string()),
            SVGAttributes::PrimitiveUnits(val) => Some(val.to_string()),
            SVGAttributes::R(val) => Some(val.to_string()),
            SVGAttributes::Radius(val) => Some(val.to_string()),
            SVGAttributes::RefX(val) => Some(val.to_string()),
            SVGAttributes::RefY(val) => Some(val.to_string()),
            SVGAttributes::RenderingIntent(val) => Some(val.to_string()),
            SVGAttributes::RepeatCount(val) => Some(val.to_string()),
            SVGAttributes::RepeatDur(val) => Some(val.to_string()),
            SVGAttributes::RequiredExtensions(val) => Some(val.to_string()),
            SVGAttributes::RequiredFeatures(val) => Some(val.to_string()),
            SVGAttributes::Restart(val) => Some(val.to_string()),
            SVGAttributes::Result(val) => Some(val.to_string()),
            SVGAttributes::Rotate(val) => Some(val.to_string()),
            SVGAttributes::Rx(val) => Some(val.to_string()),
            SVGAttributes::Ry(val) => Some(val.to_string()),
            SVGAttributes::Scale(val) => Some(val.to_string()),
            SVGAttributes::Seed(val) => Some(val.to_string()),
            SVGAttributes::ShapeRendering(val) => Some(val.to_string()),
            SVGAttributes::Slope(val) => Some(val.to_string()),
            SVGAttributes::Spacing(val) => Some(val.to_string()),
            SVGAttributes::SpecularConstant(val) => Some(val.to_string()),
            SVGAttributes::SpecularExponent(val) => Some(val.to_string()),
            SVGAttributes::Speed(val) => Some(val.to_string()),
            SVGAttributes::SpreadMethod(val) => Some(val.to_string()),
            SVGAttributes::StartOffset(val) => Some(val.to_string()),
            SVGAttributes::StdDeviation(val) => Some(val.to_string()),
            SVGAttributes::Stemh(val) => Some(val.to_string()),
            SVGAttributes::Stemv(val) => Some(val.to_string()),
            SVGAttributes::StitchTiles(val) => Some(val.to_string()),
            SVGAttributes::StopColor(val) => Some(val.to_string()),
            SVGAttributes::StopOpacity(val) => Some(val.to_string()),
            SVGAttributes::StrikethroughPosition(val) => Some(val.to_string()),
            SVGAttributes::StrikethroughThickness(val) => Some(val.to_string()),
            SVGAttributes::String(val) => Some(val.to_string()),
            SVGAttributes::Stroke(val) => Some(val.to_string()),
            SVGAttributes::StrokeDasharray(val) => Some(val.to_string()),
            SVGAttributes::StrokeDashoffset(val) => Some(val.to_string()),
            SVGAttributes::StrokeLinecap(val) => Some(val.to_string()),
            SVGAttributes::StrokeLinejoin(val) => Some(val.to_string()),
            SVGAttributes::StrokeMiterlimit(val) => Some(val.to_string()),
            SVGAttributes::StrokeOpacity(val) => Some(val.to_string()),
            SVGAttributes::StrokeWidth(val) => Some(val.to_string()),
            SVGAttributes::SurfaceScale(val) => Some(val.to_string()),
            SVGAttributes::SystemLanguage(val) => Some(val.to_string()),
            SVGAttributes::TableValues(val) => Some(val.to_string()),
            SVGAttributes::TargetX(val) => Some(val.to_string()),
            SVGAttributes::TargetY(val) => Some(val.to_string()),
            SVGAttributes::TextAnchor(val) => Some(val.to_string()),
            SVGAttributes::TextDecoration(val) => Some(val.to_string()),
            SVGAttributes::TextLength(val) => Some(val.to_string()),
            SVGAttributes::TextRendering(val) => Some(val.to_string()),
            SVGAttributes::To(val) => Some(val.to_string()),
            SVGAttributes::Transform(val) => Some(val.to_string()),
            SVGAttributes::U1(val) => Some(val.to_string()),
            SVGAttributes::U2(val) => Some(val.to_string()),
            SVGAttributes::EnderlinePosition(val) => Some(val.to_string()),
            SVGAttributes::UnderlineThickness(val) => Some(val.to_string()),
            SVGAttributes::Unicode(val) => Some(val.to_string()),
            SVGAttributes::UnicodeBidi(val) => Some(val.to_string()),
            SVGAttributes::UnicodeRange(val) => Some(val.to_string()),
            SVGAttributes::UnitsPerEm(val) => Some(val.to_string()),
            SVGAttributes::VAlphabetic(val) => Some(val.to_string()),
            SVGAttributes::Values(val) => Some(val.to_string()),
            SVGAttributes::VectorEffect(val) => Some(val.to_string()),
            SVGAttributes::Version(val) => Some(val.to_string()),
            SVGAttributes::VertAdvY(val) => Some(val.to_string()),
            SVGAttributes::VertOriginX(val) => Some(val.to_string()),
            SVGAttributes::VertOriginY(val) => Some(val.to_string()),
            SVGAttributes::VHanging(val) => Some(val.to_string()),
            SVGAttributes::VIdeographic(val) => Some(val.to_string()),
            SVGAttributes::SiewBox(val) => Some(val.to_string()),
            SVGAttributes::ViewTarget(val) => Some(val.to_string()),
            SVGAttributes::Visibility(val) => Some(val.to_string()),
            SVGAttributes::VMathematical(val) => Some(val.to_string()),
            SVGAttributes::Widths(val) => Some(val.to_string()),
            SVGAttributes::WordSpacing(val) => Some(val.to_string()),
            SVGAttributes::WritingMode(val) => Some(val.to_string()),
            SVGAttributes::X1(val) => Some(val.to_string()),
            SVGAttributes::X2(val) => Some(val.to_string()),
            SVGAttributes::X(val) => Some(val.to_string()),
            SVGAttributes::XChannelSelector(val) => Some(val.to_string()),
            SVGAttributes::XHeight(val) => Some(val.to_string()),
            SVGAttributes::XlinkActuate(val) => Some(val.to_string()),
            SVGAttributes::XlinkArcrole(val) => Some(val.to_string()),
            SVGAttributes::XlinkHref(val) => Some(val.to_string()),
            SVGAttributes::XlinkRole(val) => Some(val.to_string()),
            SVGAttributes::XlinkShow(val) => Some(val.to_string()),
            SVGAttributes::XlinkTitle(val) => Some(val.to_string()),
            SVGAttributes::XlinkType(val) => Some(val.to_string()),
            SVGAttributes::XmlBase(val) => Some(val.to_string()),
            SVGAttributes::XmlLang(val) => Some(val.to_string()),
            SVGAttributes::Xmlns(val) => Some(val.to_string()),
            SVGAttributes::XmlnsXlink(val) => Some(val.to_string()),
            SVGAttributes::XmlSpace(val) => Some(val.to_string()),
            SVGAttributes::Y1(val) => Some(val.to_string()),
            SVGAttributes::Y2(val) => Some(val.to_string()),
            SVGAttributes::Y(val) => Some(val.to_string()),
            SVGAttributes::YChannelSelector(val) => Some(val.to_string()),
            SVGAttributes::Z(val) => Some(val.to_string()),
            SVGAttributes::ZoomAndPan(val) => Some(val.to_string()),
        }
    }
}

/// An enum representing the different options for the `cross-origin` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/crossorigin>
#[derive(Debug, Clone, Display, Eq, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum CrossOrigin {
    Anonymous,
    UseCredentials,
    #[strum(serialize = "\"\"")] // TODO: add backslash once the interet comes back
    Blank,
}

/// An enum representing the different options for the `accumulate` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/accumulate>
#[derive(Debug, Clone, Display, Eq, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum Accumulate {
    None,
    Sum,
}

/// An enum representing the different options for the `additive` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/additive>
#[derive(Debug, Clone, Display, Eq, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum Additive {
    Replace,
    Sum,
}

/// An enum representing the different options for the `alignment-baseline` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/alignment-baseline>
#[derive(Debug, Clone, Display, Eq, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum AlignmentBaseline {
    Auto,
    Baseline,
    BeforeEdge,
    TextBeforeEdge,
    Middle,
    Central,
    AfterEdge,
    TextAfterEdge,
    Ideographic,
    Alphabetic,
    Hanging,
    Mathematical,
    Inherit,
}

/// An enum representing the different options for the `allow-reorder` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/allow-reorder>
#[derive(Debug, Clone, Display, Eq, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum AllowReorder {
    No,
    Yes,
}

/// An enum representing the different options for the `arabic-form` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/arabic-form>
#[derive(Debug, Clone, Display, Eq, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum ArabicForm {
    Initial,
    Medial,
    Terminal,
    Isolated,
}

/// An enum representing the different options for the `clip-rule` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/clip-rule>
#[derive(Debug, Clone, Display, Eq, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum ClipRule {
    Nonzero,
    Evenodd,
    Inherit,
}

/// An enum representing the different options for the `clipPathUnits` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/clipPathUnits>
#[derive(Debug, Clone, Display, Eq, PartialEq)]
#[strum(serialize_all = "camelCase")]
pub enum ClipPathUnits {
    UserSpaceOnUse,
    ObjectBoundingBox,
}

/// An enum representing the different options for the `color-interpolation-filters` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/color-interpolation-filters>
#[derive(Debug, Clone, Display, Eq, PartialEq)]
#[strum(serialize_all = "camelCase")]
pub enum ColorInterpolationFilters {
    Auto,
    #[strum(serialize = "sRGB")]
    SRGB,
    #[strum(serialize = "linearRGB")]
    LinearRGB,
    Inherit,
}

/// An enum representing the different options for the `fill-rule` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fill-rule>
#[derive(Debug, Clone, Display, Eq, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum FillRule {
    Nonzero,
    Evenodd,
    Inherit,
}

/// An enum representing the different options for the `focusable` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/focusable>
#[derive(Debug, Clone, Display, Eq, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum Focusable {
    True,
    False,
    Auto,
}

/// An enum representing the different options for the `stroke-linecap` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stroke-linecap>
#[derive(Debug, Clone, Display, Eq, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum StrokeLinecap {
    Butt,
    Round,
    Square,
    Inherit,
}

/// An enum representing the different options for the `stroke-linejoin` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stroke-linejoin>
#[derive(Debug, Clone, Display, Eq, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum StrokeLinejoin {
    Miter,
    Round,
    Bevel,
    Inherit,
}
