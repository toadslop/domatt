use crate::Attribute;
use crate::NumberOrString;
use strum::AsRefStr;
use strum::Display;

macro_rules! number_or_string {
    ($name:ident,$serial:expr,$doc:expr) => {
        #[doc = $doc]
        #[derive(Debug)]
        pub struct $name {
            val: String,
        }

        impl $name {
            pub fn new(val: NumberOrString) -> Self {
                Self {
                    val: val.to_string(),
                }
            }
        }

        impl Attribute for $name {
            fn get_val(&self) -> Option<&str> {
                Some(self.val.as_str())
            }

            fn get_key(&self) -> &str {
                $serial
            }
        }
        impl SvgAttribute for $name {}
    };
}

macro_rules! string_type {
    ($name:ident,$serial:expr,$doc:expr) => {
        #[doc = $doc]
        #[derive(Debug)]
        pub struct $name(String);

        impl Attribute for $name {
            fn get_val(&self) -> Option<&str> {
                Some(self.0.as_str())
            }

            fn get_key(&self) -> &str {
                $serial
            }
        }
        impl SvgAttribute for $name {}
    };
}

macro_rules! custom_type {
    ($name:ident,$serial:expr,$input_type:ident,$doc:expr) => {
        #[doc = $doc]
        #[derive(Debug)]
        pub struct $name {
            val: String,
        }

        impl $name {
            pub fn new(val: $input_type) -> Self {
                Self { val: val.value }
            }
        }

        impl Attribute for $name {
            fn get_val(&self) -> Option<&str> {
                Some(self.val.as_str())
            }

            fn get_key(&self) -> &str {
                $serial
            }
        }
        impl SvgAttribute for $name {}
    };
}

macro_rules! enum_type {
    ($name:ident,$enum_name:ident,$serial:expr,$doc:expr) => {
        #[doc = $doc]
        #[derive(Debug)]
        pub struct $name($enum_name);

        impl Attribute for $name {
            fn get_val(&self) -> Option<&str> {
                Some(self.0.as_ref())
            }

            fn get_key(&self) -> &str {
                $serial
            }
        }
        impl SvgAttribute for $name {}
    };
}

pub trait SvgAttribute: Attribute {}

string_type!(
    Color,
    "color",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/color>"
);

custom_type!(
    Height,
    "height",
    SvgLength,
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/height>"
);

string_type!(
    Id,
    "id",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/id>"
);

string_type!(
    Lang,
    "lang",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/lang>"
);

number_or_string!(
    Max,
    "max",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/max>"
);

string_type!(
    Media,
    "media",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/media>"
);

string_type!(
    Method,
    "method",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/method>"
);

string_type!(
    Min,
    "min",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/min>"
);

string_type!(
    Name,
    "name",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/name>"
);

string_type!(
    Target,
    "target",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/target>"
);

string_type!(
    Type,
    "type",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/type>"
);

custom_type!(
    Width,
    "width",
    SvgLength,
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/width>"
);

enum_type!(
    Accumulate,
    AccumulateOption,
    "accumulate",
    "https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/accumulate"
);

enum_type!(
    Additive,
    AdditiveOption,
    "additive",
    "https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/additive"
);

enum_type!(
    AlignmentBaseline,
    AlignmentBaselineOption,
    "alignment-baseline",
    "https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/alignment-baseline"
);

number_or_string!(
    Amplitude,
    "amplitude",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/amplitude>"
);

string_type!(
    AttributeName,
    "attributeName",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/attributeName>"
);

/// No mdn documentation
#[derive(Debug)]
pub struct AutoReverse {
    val: String,
}

impl AutoReverse {
    pub fn new(val: bool) -> Self {
        Self {
            val: val.to_string(),
        }
    }
}

impl Attribute for AutoReverse {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_str())
    }

    fn get_key(&self) -> &str {
        "auto-reverse"
    }
}
impl SvgAttribute for AutoReverse {}

number_or_string!(
    Azimuth,
    "azimuth",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/azimuth>"
);

number_or_string!(
    BaseFrequency,
    "baseFrequency",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/baseFrequency>"
);

number_or_string!(
    BaselineShift,
    "baseline-shift",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/baseline-shift>"
);

number_or_string!(
    Begin,
    "begin",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/begin>"
);

number_or_string!(
    Bias,
    "bias",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/bias>"
);

number_or_string!(
    By,
    "by",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/by>"
);

number_or_string!(
    CalcMode,
    "calcMode",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/calcMode>"
);

string_type!(
    ClipPath,
    "clip-path",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/clip-path>"
);

enum_type!(
    ClipPathUnits,
    ClipPathUnitsOption,
    "clipPathUnits",
    "https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/clipPathUnits"
);

enum_type!(
    ClipRule,
    ClipRuleOption,
    "clip-rule",
    "https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/clip-rule"
);

number_or_string!(
    ColorInterpolation,
    "color-interpolation",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/color-interpolation>"
);

enum_type!(
    ColorInterpolationFilters,
    ColorInterpolationFiltersOption,
    "color-interpolation-filters",
    "https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/color-interpolation-filters"
);

number_or_string!(
    ColorRendering,
    "color-rendering",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/color-rendering>"
);

number_or_string!(
    Cursor,
    "cursor",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/cursor>"
);

number_or_string!(
    Cx,
    "cx",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/cx>"
);

number_or_string!(
    Cy,
    "cy",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/cy>"
);

string_type!(
    D,
    "d",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/d>"
);

number_or_string!(
    Decelerate,
    "decelerate",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/decelerate>"
);

number_or_string!(
    DiffuseConstant,
    "diffuseConstant",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/diffuseConstant>"
);

number_or_string!(
    Direction,
    "direction",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/direction>"
);

number_or_string!(
    Display,
    "display",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/display>"
);

number_or_string!(
    Divisor,
    "divisor",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/divisor>"
);

number_or_string!(
    DominantBaseline,
    "dominant-baseline",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/dominant-baseline>"
);

number_or_string!(
    Dur,
    "dur",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/dur>"
);

number_or_string!(
    Dx,
    "dx",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/dx>"
);

number_or_string!(
    Dy,
    "dy",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/dy>"
);

number_or_string!(
    EdgeMode,
    "edgeMode",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/edgeMode>"
);

number_or_string!(
    Elevation,
    "elevation",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/elevation>"
);

number_or_string!(
    End,
    "end",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/end>"
);

number_or_string!(
    Exponent,
    "exponent",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/exponent>"
);

string_type!(
    Fill,
    "fill",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fill>"
);

number_or_string!(
    FillOpacity,
    "fill-opacity",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fill-opacity>"
);

enum_type!(
    FillRule,
    FillRuleOption,
    "fill-rule",
    "https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fill-rule"
);

string_type!(
    Filter,
    "filter",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/filter>"
);

number_or_string!(
    FilterUnits,
    "filterUnits",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/filterUnits>"
);

number_or_string!(
    FloodColor,
    "flood-color",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/flood-color>"
);

number_or_string!(
    FloodOpacity,
    "flood-opacity",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/flood-opacity>"
);

string_type!(
    FontFamily,
    "font-family",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/font-family>"
);

number_or_string!(
    FontSize,
    "font-size",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/font-size>"
);

number_or_string!(
    FontSizeAdjust,
    "font-size-adjust",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/font-size-adjust>"
);

number_or_string!(
    FontStretch,
    "font-stretch",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/font-stretch>"
);

number_or_string!(
    FontStyle,
    "font-style",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/font-style>"
);

number_or_string!(
    FontVariant,
    "font-variant",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/font-variant>"
);

number_or_string!(
    FontWeight,
    "font-weight",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/font-weight>"
);

number_or_string!(
    Fr,
    "fr",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fr>"
);

number_or_string!(
    From,
    "from",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/from>"
);

number_or_string!(
    Fx,
    "fx",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fx>"
);

number_or_string!(
    Fy,
    "fy",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fy>"
);

string_type!(
    GradientTransform,
    "gradientTransform",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/gradientTransform>"
);

string_type!(
    GradientUnits,
    "gradientUnits",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/gradientUnits>"
);

pub use crate::anchor::Href;
impl SvgAttribute for Href {}

number_or_string!(
    ImageRendering,
    "image-rendering",
    "<https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/image-rendering>"
);

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
pub struct SvgLength {
    value: String,
}

impl SvgLength {
    pub fn new(value: f64, unit: SvgLengthUnit) -> Self {
        let value = value.to_string();
        let unit = unit.as_ref();
        let mut as_string = String::with_capacity(value.len() + unit.len());
        as_string.push_str(value.as_str());
        as_string.push_str(unit);
        Self { value: as_string }
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
