use std::fmt::Display;

use super::global_attributes::AriaRole;
use crate::NumberOrString;
use strum::Display;

/// An enum defining the different attribute keys for SVG elements. Each variant takes a tuple
/// that represents the valid values for the attributes.
#[derive(Debug, Display)]
#[strum(serialize_all = "kebab-case")]
pub enum SVGAttributes<'a> {
    Color(&'a str),
    Height(SvgLength),
    Id(&'a str),
    Lang(&'a str),
    Max(NumberOrString<'a>), // TODO: define this type: https://developer.mozilla.org/en-US/docs/Web/SVG/Content_type#clock-value
    Media(&'a str),
    Method(&'a str),
    Min(NumberOrString<'a>),
    Name(&'a str),
    // Note: implementing css properties is unneccessary since style tags are already handled by Yew and Stylist
    // Style: CSSProperties
    Target(&'a str),
    Type(&'a str),
    Width(SvgLength),
    // Other HTML properties supported by SVG elements in browsers
    Role(AriaRole),
    TabIndex(i64),
    CrossOrigin(CrossOrigin),
    // SVG Specific attributes
    AccentHeight(f64),
    Accumulate(Accumulate),
    Additive(Additive),
    AlignmentBaseline(AlignmentBaseline),
    #[strum(serialize = "allowReorder")]
    AllowReorder(AllowReorder),
    Alphabetic(NumberOrString<'a>), // TODO: make a data type that can better restrict the string to scientific notation
    Amplitude(NumberOrString<'a>), // TODO: make a data type that can better restrict the string to scientific notation
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

/// An enum representing the different options for the `cross-origin` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/crossorigin>
#[derive(Debug, Display)]
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
#[derive(Debug, Display)]
#[strum(serialize_all = "lowercase")]
pub enum Accumulate {
    None,
    Sum,
}

/// An enum representing the different options for the `additive` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/additive>
#[derive(Debug, Display)]
#[strum(serialize_all = "lowercase")]
pub enum Additive {
    Replace,
    Sum,
}

/// An enum representing the different options for the `alignment-baseline` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/alignment-baseline>
#[derive(Debug, Display)]
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
#[derive(Debug, Display)]
#[strum(serialize_all = "kebab-case")]
pub enum AllowReorder {
    No,
    Yes,
}

/// An enum representing the different options for the `arabic-form` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/arabic-form>
#[derive(Debug, Display)]
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
#[derive(Debug, Display)]
#[strum(serialize_all = "lowercase")]
pub enum ClipRule {
    Nonzero,
    Evenodd,
    Inherit,
}

/// An enum representing the different options for the `clipPathUnits` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/clipPathUnits>
#[derive(Debug, Display)]
#[strum(serialize_all = "camelCase")]
pub enum ClipPathUnits {
    UserSpaceOnUse,
    ObjectBoundingBox,
}

/// An enum representing the different options for the `color-interpolation-filters` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/color-interpolation-filters>
#[derive(Debug, Display)]
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
#[derive(Debug, Display)]
#[strum(serialize_all = "lowercase")]
pub enum FillRule {
    Nonzero,
    Evenodd,
    Inherit,
}

/// An enum representing the different options for the `focusable` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/focusable>
#[derive(Debug, Display)]
#[strum(serialize_all = "lowercase")]
pub enum Focusable {
    True,
    False,
    Auto,
}

/// An enum representing the different options for the `stroke-linecap` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stroke-linecap>
#[derive(Debug, Display)]
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
#[derive(Debug, Display)]
#[strum(serialize_all = "lowercase")]
pub enum StrokeLinejoin {
    Miter,
    Round,
    Bevel,
    Inherit,
}

/// An enum representing the different options for the `stroke-linejoin` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/height>
#[derive(Debug, Display)]
#[strum(serialize_all = "lowercase")]
pub enum SvgHeight {
    Auto,
    Length(SvgLength),
}

#[derive(Debug)]
pub struct SvgLength {
    value: f64,
    unit: SvgLengthUnit,
}

impl SvgLength {
    pub fn new(value: f64, unit: SvgLengthUnit) -> Self {
        Self { value, unit }
    }
}

impl Display for SvgLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.unit)
    }
}

/// An enum defining the different options for length units for svg elements.
#[derive(Debug, Display)]
#[strum(serialize_all = "lowercase")]
pub enum SvgLengthUnit {
    Em,
    Ex,
    Px,
    In,
    Cm,
    Mm,
    Pt,
    Pc,
    #[strum(serialize = "%")]
    Percent,
}
