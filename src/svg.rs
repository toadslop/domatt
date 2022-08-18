use std::ops::Deref;

use crate::Attribute;
use crate::NumberOrString;
use crate::{
    anchor::AnchorAttribute, area::AreaAttribute, audio::AudioAttribute, base::BaseAttribute,
    blockquote::BlockQuoteAttribute, button::ButtonAttribute, canvas::CanvasAttribute,
    col::ColAttribute, colgroup::ColGroupAttribute, data::DataAttribute, details::DetailsAttribute,
};
use strum::AsRefStr;
use strum::Display;

pub trait SvgAttribute: Attribute {}

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/color>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", String)]
pub struct Color(String);
crate::add_impls!(Color);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/height>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", SvgLength)]
pub struct Height(String);
crate::add_impls!(Height);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/id>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", String)]
pub struct Id(String);
crate::add_impls!(Id);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/lang>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", String)]
pub struct Lang(String);
crate::add_impls!(Lang);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/max>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Max(String);
crate::add_impls!(Max);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/media>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", String)]
pub struct Media(String);
crate::add_impls!(Media);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/method>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", String)]
pub struct Method(String);
crate::add_impls!(Method);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/min>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", String)]
pub struct Min(String);
crate::add_impls!(Min);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/name>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", String)]
pub struct Name(String);
crate::add_impls!(Name);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/target>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", String)]
pub struct Target(String);
crate::add_impls!(Target);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/type>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", String)]
pub struct Type(String);
crate::add_impls!(Type);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/width>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", SvgLength)]
pub struct Width(String);
crate::add_impls!(Width);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/accumulate>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", AccumulateOption)]
pub struct Accumulate(AccumulateOption);
crate::add_impls!(Accumulate);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/additive>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", AdditiveOption)]
pub struct Additive(AdditiveOption);
crate::add_impls!(Additive);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/alignment-baseline>"
#[derive(Debug, Attribute)]
#[attribute("kebab-case", AlignmentBaselineOption)]
pub struct AlignmentBaseline(AlignmentBaselineOption);
crate::add_impls!(AlignmentBaseline);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/amplitude>"
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct Amplitude(String);
crate::add_impls!(Amplitude);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/attributeName>"
#[derive(Debug, Attribute)]
#[attribute("camelCase", String)]
pub struct AttributeName(String);
crate::add_impls!(AttributeName);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/attributeName>"
#[derive(Debug, Attribute)]
#[attribute("kebab-case", bool)]
pub struct AutoReverse(String);
crate::add_impls!(AutoReverse);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/azimuth>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Azimuth(String);
crate::add_impls!(Azimuth);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/baseFrequency>"
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct BaseFrequency(String);
crate::add_impls!(BaseFrequency);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/baseline-shift>"
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct BaselineShift(String);
crate::add_impls!(BaselineShift);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/begin>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Begin(String);
crate::add_impls!(Begin);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/bias>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Bias(String);
crate::add_impls!(Bias);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/calcMode>"
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct CalcMode(String);
crate::add_impls!(CalcMode);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/clipPath>"
#[derive(Debug, Attribute)]
#[attribute("camelCase", String)]
pub struct ClipPath(String);
crate::add_impls!(ClipPath);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/clipPathUnits>"
#[derive(Debug, Attribute)]
#[attribute("camelCase", ClipPathUnitsOption)]
pub struct ClipPathUnits(ClipPathUnitsOption);
crate::add_impls!(ClipPathUnits);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/clip-rule>"
#[derive(Debug, Attribute)]
#[attribute("kebab-case", ClipRuleOption)]
pub struct ClipRule(ClipRuleOption);
crate::add_impls!(ClipRule);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/color-interpolation>"
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct ColorInterpolation(String);
crate::add_impls!(ColorInterpolation);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/color-interpolation-filters>"
#[derive(Debug, Attribute)]
#[attribute("kebab-case", ColorInterpolationFiltersOption)]
pub struct ColorInterpolationFilters(ColorInterpolationFiltersOption);
crate::add_impls!(ColorInterpolationFilters);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/color-rendering>"
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct ColorRendering(String);
crate::add_impls!(ColorRendering);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/cursor>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Cursor(String);
crate::add_impls!(Cursor);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/cx>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Cx(String);
crate::add_impls!(Cx);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/cy>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Cy(String);
crate::add_impls!(Cy);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/d>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", String)]
pub struct D(String);
crate::add_impls!(D);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/decelerate>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Decelerate(String);
crate::add_impls!(Decelerate);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/diffuseConstant>"
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct DiffuseConstant(String);
crate::add_impls!(DiffuseConstant);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/direction>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Direction(String);
crate::add_impls!(Direction);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/display>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Display(String);
crate::add_impls!(Display);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/divisor>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Divisor(String);
crate::add_impls!(Divisor);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/dominant-baseline>"
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct DominantBaseline(String);
crate::add_impls!(DominantBaseline);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/dur>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Dur(String);
crate::add_impls!(Dur);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/dx>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Dx(String);
crate::add_impls!(Dx);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/dy>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Dy(String);
crate::add_impls!(Dy);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/edgeMode>"
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct EdgeMode(String);
crate::add_impls!(EdgeMode);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/elevation>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Elevation(String);
crate::add_impls!(Elevation);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/end>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct End(String);
crate::add_impls!(End);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/exponent>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Exponent(String);
crate::add_impls!(Exponent);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fill>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", String)]
pub struct Fill(String);
crate::add_impls!(Fill);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fill-opacity>"
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct FillOpacity(String);
crate::add_impls!(FillOpacity);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fill-rule>"
#[derive(Debug, Attribute)]
#[attribute("kebab-case", FillRuleOption)]
pub struct FillRule(FillRuleOption);
crate::add_impls!(FillRule);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/filter>"
#[derive(Debug, Attribute)]
#[attribute("lowercase", String)]
pub struct Filter(String);
crate::add_impls!(Filter);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/filterUnits>"
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct FilterUnits(String);
crate::add_impls!(FilterUnits);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/flood-color>"
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct FloodColor(String);
crate::add_impls!(FloodColor);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/flood-opacity>"
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct FloodOpacity(String);
crate::add_impls!(FloodOpacity);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/font-family>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", String)]
pub struct FontFamily(String);
crate::add_impls!(FontFamily);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/font-size>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct FontSize(String);
crate::add_impls!(FontSize);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/font-size-adjust>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct FontSizeAdjust(String);
crate::add_impls!(FontSizeAdjust);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/font-stretch>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct FontStretch(String);
crate::add_impls!(FontStretch);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/font-style>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct FontStyle(String);
crate::add_impls!(FontStyle);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/font-variant>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct FontVariant(String);
crate::add_impls!(FontVariant);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/font-weight>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct FontWeight(String);
crate::add_impls!(FontWeight);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fr>
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Fr(String);
crate::add_impls!(Fr);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/from>
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct From(String);
crate::add_impls!(From);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fx>
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Fx(String);
crate::add_impls!(Fx);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fy>
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Fy(String);
crate::add_impls!(Fy);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/gradientTransform>
#[derive(Debug, Attribute)]
#[attribute("camelCase", String)]
pub struct GradientTransform(String);
crate::add_impls!(GradientTransform);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/gradientUnits>
#[derive(Debug, Attribute)]
#[attribute("camelCase", String)]
pub struct GradientUnits(String);
crate::add_impls!(GradientUnits);

pub use crate::anchor::Href;
impl SvgAttribute for Href {}

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/image-rendering>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", String)]
pub struct ImageRendering(String);
crate::add_impls!(ImageRendering);

/// An enum defining the different attribute keys for SVG elements. Each variant takes a tuple
/// that represents the valid values for the attributes.
#[derive(Debug, Display)]
#[strum(serialize_all = "kebab-case")]
pub enum SVGAttributes<'a> {
    In2(NumberOrString),
    In(&'a str),
    Intercept(NumberOrString),
    K1(NumberOrString),
    K2(NumberOrString),
    K3(NumberOrString),
    K4(NumberOrString),
    K(NumberOrString),
    #[strum(serialize = "kernelMatrix")]
    KernelMatrix(NumberOrString),
    #[strum(serialize = "kernelUnitLength")]
    KernelUnitLength(NumberOrString),
    Kerning(NumberOrString),
    #[strum(serialize = "keyPoints")]
    KeyPoints(NumberOrString),
    #[strum(serialize = "keySplines")]
    KeySplines(NumberOrString),
    #[strum(serialize = "keyTimes")]
    KeyTimes(NumberOrString),
    #[strum(serialize = "lengthAdjust")]
    LengthAdjust(NumberOrString),
    LetterSpacing(NumberOrString),
    LightingColor(NumberOrString),
    #[strum(serialize = "limitingConeAngle")]
    LimitingConeAngle(NumberOrString),
    Local(NumberOrString),
    MarkerEnd(&'a str),
    #[strum(serialize = "markerHeight")]
    MarkerHeight(NumberOrString),
    MarkerMid(&'a str),
    MarkerStart(&'a str),
    #[strum(serialize = "markerUnits")]
    MarkerUnits(NumberOrString),
    #[strum(serialize = "markerWidth")]
    MarkerWidth(NumberOrString),
    Mask(&'a str),
    #[strum(serialize = "maskContentUnits")]
    MaskContentUnits(NumberOrString),
    #[strum(serialize = "maskUnits")]
    MaskUnits(NumberOrString),
    Mathematical(NumberOrString),
    Mode(NumberOrString),
    #[strum(serialize = "numOctaves")]
    NumOctaves(NumberOrString),
    Offset(NumberOrString),
    Opacity(NumberOrString),
    Operator(NumberOrString),
    Order(NumberOrString),
    Orient(NumberOrString),
    Orientation(NumberOrString),
    Origin(NumberOrString),
    Overflow(NumberOrString),
    OverlinePosition(NumberOrString),
    PverlineThickness(NumberOrString),
    PaintOrder(NumberOrString),
    #[strum(serialize = "panose-1")]
    Panose1(NumberOrString),
    Path(&'a str),
    #[strum(serialize = "pathLength")]
    PathLength(NumberOrString),
    #[strum(serialize = "patternContentUnits")]
    PatternContentUnits(&'a str),
    #[strum(serialize = "patternTransform")]
    PatternTransform(NumberOrString),
    #[strum(serialize = "patternUnits")]
    PatternUnits(&'a str),
    PointerEvents(NumberOrString),
    Points(&'a str),
    #[strum(serialize = "pointsAtX")]
    PointsAtX(NumberOrString),
    #[strum(serialize = "pointsAtY")]
    PointsAtY(NumberOrString),
    #[strum(serialize = "pointsAtZ")]
    PointsAtZ(NumberOrString),
    #[strum(serialize = "preserveAlpha")]
    PreserveAlpha(bool),
    #[strum(serialize = "preserveAspectRatio")]
    OreserveAspectRatio(&'a str),
    #[strum(serialize = "primitiveUnits")]
    PrimitiveUnits(NumberOrString),
    R(NumberOrString),
    Radius(NumberOrString),
    #[strum(serialize = "refX")]
    RefX(NumberOrString),
    #[strum(serialize = "refY")]
    RefY(NumberOrString),
    RenderingIntent(NumberOrString),
    #[strum(serialize = "repeatCount")]
    RepeatCount(NumberOrString),
    #[strum(serialize = "repeatDur")]
    RepeatDur(NumberOrString),
    #[strum(serialize = "requiredExtensions")]
    RequiredExtensions(NumberOrString),
    #[strum(serialize = "requiredFeatures")]
    RequiredFeatures(NumberOrString),
    Restart(NumberOrString),
    Result(&'a str),
    Rotate(NumberOrString),
    Rx(NumberOrString),
    Ry(NumberOrString),
    Scale(NumberOrString),
    Seed(NumberOrString),
    ShapeRendering(NumberOrString),
    Slope(NumberOrString),
    Spacing(NumberOrString),
    #[strum(serialize = "requiredFeatures")]
    SpecularConstant(NumberOrString),
    #[strum(serialize = "specularExponent")]
    SpecularExponent(NumberOrString),
    Speed(NumberOrString),
    #[strum(serialize = "spreadMethod")]
    SpreadMethod(&'a str),
    #[strum(serialize = "startOffset")]
    StartOffset(NumberOrString),
    #[strum(serialize = "stdDeviation")]
    StdDeviation(NumberOrString),
    Stemh(NumberOrString),
    Stemv(NumberOrString),
    #[strum(serialize = "stitchTiles")]
    StitchTiles(NumberOrString),
    StopColor(&'a str),
    StopOpacity(NumberOrString),
    StrikethroughPosition(NumberOrString),
    StrikethroughThickness(NumberOrString),
    String(NumberOrString),
    Stroke(&'a str),
    StrokeDasharray(NumberOrString),
    StrokeDashoffset(NumberOrString),
    StrokeLinecap(StrokeLinecap),
    StrokeLinejoin(StrokeLinejoin),
    StrokeMiterlimit(NumberOrString),
    StrokeOpacity(NumberOrString),
    StrokeWidth(NumberOrString),
    #[strum(serialize = "surfaceScale")]
    SurfaceScale(NumberOrString),
    #[strum(serialize = "systemLanguage")]
    SystemLanguage(NumberOrString),
    #[strum(serialize = "tableValues")]
    TableValues(NumberOrString),
    #[strum(serialize = "targetX")]
    TargetX(NumberOrString),
    #[strum(serialize = "targetY")]
    TargetY(NumberOrString),
    TextAnchor(String),
    TextDecoration(NumberOrString),
    #[strum(serialize = "textLength")]
    TextLength(NumberOrString),
    TextRendering(NumberOrString),
    To(NumberOrString),
    Transform(&'a str),
    U1(NumberOrString),
    U2(NumberOrString),
    EnderlinePosition(NumberOrString),
    UnderlineThickness(NumberOrString),
    Unicode(NumberOrString),
    UnicodeBidi(NumberOrString),
    UnicodeRange(NumberOrString),
    UnitsPerEm(NumberOrString),
    VAlphabetic(NumberOrString),
    Values(&'a str),
    VectorEffect(NumberOrString),
    Version(&'a str),
    VertAdvY(NumberOrString),
    VertOriginX(NumberOrString),
    VertOriginY(NumberOrString),
    VHanging(NumberOrString),
    VIdeographic(NumberOrString),
    #[strum(serialize = "viewBox")]
    SiewBox(&'a str),
    #[strum(serialize = "viewTarget")]
    ViewTarget(NumberOrString),
    Visibility(NumberOrString),
    VMathematical(NumberOrString),
    Widths(NumberOrString),
    WordSpacing(NumberOrString),
    WritingMode(NumberOrString),
    #[strum(serialize = "x1")]
    X1(NumberOrString),
    #[strum(serialize = "x2")]
    X2(NumberOrString),
    X(NumberOrString),
    #[strum(serialize = "xChannelSelector")]
    XChannelSelector(&'a str),
    XHeight(NumberOrString),
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
    Y1(NumberOrString),
    #[strum(serialize = "y2")]
    Y2(NumberOrString),
    Y(NumberOrString),
    #[strum(serialize = "yChannelSelector ")]
    YChannelSelector(&'a str),
    Z(NumberOrString),
    #[strum(serialize = "zoomAndPan")]
    ZoomAndPan(&'a str),
}

/// An enum representing the different options for the `cross-origin` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/crossorigin>
#[derive(Debug, AsRefStr)]
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
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum AccumulateOption {
    None,
    Sum,
}

/// An enum representing the different options for the `additive` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/additive>
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum AdditiveOption {
    Replace,
    Sum,
}

/// An enum representing the different options for the `alignment-baseline` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/alignment-baseline>
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum AlignmentBaselineOption {
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
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum AllowReorder {
    No,
    Yes,
}

/// An enum representing the different options for the `clip-rule` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/clip-rule>
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum ClipRuleOption {
    Nonzero,
    Evenodd,
    Inherit,
}

/// An enum representing the different options for the `clipPathUnits` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/clipPathUnits>
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "camelCase")]
pub enum ClipPathUnitsOption {
    UserSpaceOnUse,
    ObjectBoundingBox,
}

/// An enum representing the different options for the `color-interpolation-filters` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/color-interpolation-filters>
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "camelCase")]
pub enum ColorInterpolationFiltersOption {
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
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum FillRuleOption {
    Nonzero,
    Evenodd,
    Inherit,
}

/// An enum representing the different options for the `focusable` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/focusable>
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum FocusableOption {
    True,
    False,
    Auto,
}

/// An enum representing the different options for the `stroke-linecap` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stroke-linecap>
#[derive(Debug, AsRefStr)]
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
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum StrokeLinejoin {
    Miter,
    Round,
    Bevel,
    Inherit,
}

// /// An enum representing the different options for the `stroke-linejoin` attribute.
// ///
// /// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/height>
// #[derive(Debug, Display)]
// #[strum(serialize_all = "lowercase")]
// pub enum SvgHeightOption {
//     Auto,
//     Length(SvgLength),
// }

#[derive(Debug)]
pub struct SvgLength(String);

impl SvgLength {
    pub fn new(value: f64, unit: SvgLengthUnit) -> Self {
        let value = value.to_string();
        let unit = unit.as_ref();
        let mut as_string = String::with_capacity(value.len() + unit.len());
        as_string.push_str(value.as_str());
        as_string.push_str(unit);
        Self(as_string)
    }
}

impl Deref for SvgLength {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// An enum defining the different options for length units for svg elements.
#[derive(Debug, AsRefStr)]
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
