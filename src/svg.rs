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

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/in2>
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct In2(String);
crate::add_impls!(In2);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/in>
#[derive(Debug, Attribute)]
#[attribute("lowercase", String)]
pub struct In(String);
crate::add_impls!(In);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/intercept>
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Intercept(String);
crate::add_impls!(Intercept);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/k1>
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct K1(String);
crate::add_impls!(K1);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/k2>
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct K2(String);
crate::add_impls!(K2);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/k3>
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct K3(String);
crate::add_impls!(K3);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/k4>
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct K4(String);
crate::add_impls!(K4);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/k>
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct K(String);
crate::add_impls!(K);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/kernelMatrix>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct KernelMatrix(String);
crate::add_impls!(KernelMatrix);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/kernelUnitLength>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct KernelUnitLength(String);
crate::add_impls!(KernelUnitLength);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/kerning>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct Kerning(String);
crate::add_impls!(Kerning);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/keyPoints>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct KeyPoints(String);
crate::add_impls!(KeyPoints);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/keySplines>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct KeySplines(String);
crate::add_impls!(KeySplines);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/keyTimes>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct KeyTimes(String);
crate::add_impls!(KeyTimes);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/lengthAdjust>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct LengthAdjust(String);
crate::add_impls!(LengthAdjust);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/letter-spacing>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct LetterSpacing(String);
crate::add_impls!(LetterSpacing);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/lighting-color>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct LightingColor(String);
crate::add_impls!(LightingColor);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/limitingConeAngle>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct LimitingConeAngle(String);
crate::add_impls!(LimitingConeAngle);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/local>
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Local(String);
crate::add_impls!(Local);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/marker-end>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", String)]
pub struct MarkerEnd(String);
crate::add_impls!(MarkerEnd);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/markerHeight>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct MarkerHeight(String);
crate::add_impls!(MarkerHeight);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/marker-mid>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", String)]
pub struct MarkerMid(String);
crate::add_impls!(MarkerMid);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/marker-start>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", String)]
pub struct MarkerStart(String);
crate::add_impls!(MarkerStart);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/markerUnits>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct MarkerUnits(String);
crate::add_impls!(MarkerUnits);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/markerWidth>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct MarkerWidth(String);
crate::add_impls!(MarkerWidth);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/mask>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", String)]
pub struct Mask(String);
crate::add_impls!(Mask);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/maskContentUnits>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct MaskContentUnits(String);
crate::add_impls!(MaskContentUnits);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/maskUnits>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct MaskUnits(String);
crate::add_impls!(MaskUnits);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/mathematical>
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Mathematical(String);
crate::add_impls!(Mathematical);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/mode>
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Mode(String);
crate::add_impls!(Mode);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/numOctaves>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct NumOctaves(String);
crate::add_impls!(NumOctaves);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/offset>
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Offset(String);
crate::add_impls!(Offset);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/opacity>
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Opacity(String);
crate::add_impls!(Opacity);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/operator>
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Operator(String);
crate::add_impls!(Operator);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/order>
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Order(String);
crate::add_impls!(Order);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/orient>
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Orient(String);
crate::add_impls!(Orient);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/orientation>
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Orientation(String);
crate::add_impls!(Orientation);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/origin>
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Origin(String);
crate::add_impls!(Origin);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/overflow>
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Overflow(String);
crate::add_impls!(Overflow);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/overline-position>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct OverlinePosition(String);
crate::add_impls!(OverlinePosition);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/overline-thickness>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", f32)]
pub struct OverlineThickness(String);
crate::add_impls!(OverlineThickness);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/paint-order>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct PaintOrder(String);
crate::add_impls!(PaintOrder);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/panose-1>
#[derive(Debug, Attribute)]
#[attribute("panose-1", NumberOrString)]
pub struct Panose1(String);
crate::add_impls!(Panose1);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/path>
#[derive(Debug, Attribute)]
#[attribute("lowercase", String)]
pub struct Path(String);
crate::add_impls!(Path);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/pathLength>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct PathLength(String);
crate::add_impls!(PathLength);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/patternContentUnits>
#[derive(Debug, Attribute)]
#[attribute("camelCase", String)]
pub struct PatternContentUnits(String);
crate::add_impls!(PatternContentUnits);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/patternTransform>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct PatternTransform(String);
crate::add_impls!(PatternTransform);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/patternUnits>
#[derive(Debug, Attribute)]
#[attribute("camelCase", String)]
pub struct PatternUnits(String);
crate::add_impls!(PatternUnits);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/pointer-events>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct PointerEvents(String);
crate::add_impls!(PointerEvents);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/points>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", String)]
pub struct Points(String);
crate::add_impls!(Points);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/pointsAtX>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct PointsAtX(String);
crate::add_impls!(PointsAtX);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/pointsAtX>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct PointsAtY(String);
crate::add_impls!(PointsAtY);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/pointsAtZ>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct PointsAtZ(String);
crate::add_impls!(PointsAtZ);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/preserveAlpha>
#[derive(Debug, Attribute)]
#[attribute("camelCase", bool)]
pub struct PreserveAlpha(String);
crate::add_impls!(PreserveAlpha);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/preserveAspectRatio>
#[derive(Debug, Attribute)]
#[attribute("camelCase", String)]
pub struct PreserveAspectRatio(String);
crate::add_impls!(PreserveAspectRatio);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/primitiveUnits>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct PrimitiveUnits(String);
crate::add_impls!(PrimitiveUnits);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/r>
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct R(String);
crate::add_impls!(R);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/radius>
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Radius(String);
crate::add_impls!(Radius);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/refX>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct RefX(String);
crate::add_impls!(RefX);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/refY>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct RefY(String);
crate::add_impls!(RefY);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/rendering-intent>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct RenderingIntent(String);
crate::add_impls!(RenderingIntent);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/repeatCount>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct RepeatCount(String);
crate::add_impls!(RepeatCount);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/repeatDur>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct RepeatDur(String);
crate::add_impls!(RepeatDur);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/requiredExtensions>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct RequiredExtensions(String);
crate::add_impls!(RequiredExtensions);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/requiredFeatures>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct RequiredFeatures(String);
crate::add_impls!(RequiredFeatures);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/restart>
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Restart(String);
crate::add_impls!(Restart);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/result>
#[derive(Debug, Attribute)]
#[attribute("lowercase", String)]
pub struct Result(String);
crate::add_impls!(Result);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/rotate>
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Rotate(String);
crate::add_impls!(Rotate);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/rx>
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Rx(String);
crate::add_impls!(Rx);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/ry>
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Ry(String);
crate::add_impls!(Ry);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/scale>
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Scale(String);
crate::add_impls!(Scale);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/seed>
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct Seed(String);
crate::add_impls!(Seed);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/shape-rendering>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct ShapeRendering(String);
crate::add_impls!(ShapeRendering);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/slope>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct Slope(String);
crate::add_impls!(Slope);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/spacing>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct Spacing(String);
crate::add_impls!(Spacing);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/specularConstant>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct SpecularConstant(String);
crate::add_impls!(SpecularConstant);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/specularExponent>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct SpecularExponent(String);
crate::add_impls!(SpecularExponent);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/speed>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct Speed(String);
crate::add_impls!(Speed);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/spreadMethod>
#[derive(Debug, Attribute)]
#[attribute("camelCase", String)]
pub struct SpreadMethod(String);
crate::add_impls!(SpreadMethod);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/startOffset>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct StartOffset(String);
crate::add_impls!(StartOffset);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stdDeviation>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct StdDeviation(String);
crate::add_impls!(StdDeviation);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stemh>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct Stemh(String);
crate::add_impls!(Stemh);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stemv>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct Stemv(String);
crate::add_impls!(Stemv);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stitchTiles>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct StitchTiles(String);
crate::add_impls!(StitchTiles);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stop-color>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", String)]
pub struct StopColor(String);
crate::add_impls!(StopColor);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stop-opacity>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct StopOpacity(String);
crate::add_impls!(StopOpacity);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/strikethrough-position>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct StrikethroughPosition(String);
crate::add_impls!(StrikethroughPosition);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stroke>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", String)]
pub struct Stroke(String);
crate::add_impls!(Stroke);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stroke-dasharray>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", String)]
pub struct StrokeDasharray(String);
crate::add_impls!(StrokeDasharray);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stroke-dashoffset>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", String)]
pub struct StrokeDashoffset(String);
crate::add_impls!(StrokeDashoffset);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stroke-linecap>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", StrokeLinecapOption)]
pub struct StrokeLinecap(StrokeLinecapOption);
crate::add_impls!(StrokeLinecap);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stroke-linejoin>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", StrokeLinejoinOption)]
pub struct StrokeLinejoin(StrokeLinejoinOption);
crate::add_impls!(StrokeLinejoin);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stroke-miterlimit>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct StrokeMiterlimit(String);
crate::add_impls!(StrokeMiterlimit);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stroke-opacity>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct StrokeOpacity(String);
crate::add_impls!(StrokeOpacity);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stroke-width>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct StrokeWidth(String);
crate::add_impls!(StrokeWidth);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/surfaceScale>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct SurfaceScale(String);
crate::add_impls!(SurfaceScale);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/systemLanguage>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct SystemLanguage(String);
crate::add_impls!(SystemLanguage);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/tableValues>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct TableValues(String);
crate::add_impls!(TableValues);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/targetX>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct TargetX(String);
crate::add_impls!(TargetX);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/targetY>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct TargetY(String);
crate::add_impls!(TargetY);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/text-anchor>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct TextAnchor(String);
crate::add_impls!(TextAnchor);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/text-decoration>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct TextDecoration(String);
crate::add_impls!(TextDecoration);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/textLength>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct TextLength(String);
crate::add_impls!(TextLength);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/text-rendering>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct TextRendering(String);
crate::add_impls!(TextRendering);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/to>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct To(String);
crate::add_impls!(To);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/transform>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct Transform(String);
crate::add_impls!(Transform);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/transform-origin>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct TransformOrigin(String);
crate::add_impls!(TransformOrigin);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/u1>
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct U1(String);
crate::add_impls!(U1);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/u2>
#[derive(Debug, Attribute)]
#[attribute("lowercase", NumberOrString)]
pub struct U2(String);
crate::add_impls!(U2);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/underline-position>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct UnderlinePosition(String);
crate::add_impls!(UnderlinePosition);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/underline-thickness>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct UnderlineThickness(String);
crate::add_impls!(UnderlineThickness);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/unicode>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct Unicode(String);
crate::add_impls!(Unicode);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/unicode-bidi>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct UnicodeBidi(String);
crate::add_impls!(UnicodeBidi);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/unicode-range>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct UnicodeRange(String);
crate::add_impls!(UnicodeRange);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/units-per-em>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", NumberOrString)]
pub struct UnitsPerEm(String);
crate::add_impls!(UnitsPerEm);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/values>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", String)]
pub struct Values(String);
crate::add_impls!(Values);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/vector-effect>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", String)]
pub struct VectorEffect(String);
crate::add_impls!(VectorEffect);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/viewBox>
#[derive(Debug, Attribute)]
#[attribute("camelCase", String)]
pub struct ViewBox(String);
crate::add_impls!(ViewBox);

/// <https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/visibility>
#[derive(Debug, Attribute)]
#[attribute("camelCase", NumberOrString)]
pub struct Visibility(String);
crate::add_impls!(Visibility);

/// An enum defining the different attribute keys for SVG elements. Each variant takes a tuple
/// that represents the valid values for the attributes.
#[derive(Debug, Display)]
#[strum(serialize_all = "kebab-case")]
pub enum SVGAttributes<'a> {
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
pub enum StrokeLinecapOption {
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
pub enum StrokeLinejoinOption {
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
