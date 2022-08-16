use crate::{
    anchor_html_attributes::AnchorAttribute, area_html_attributes::AreaAttribute, Attribute,
    AudioAttribute, BaseAttribute, BlockQuoteAttribute, ButtonAttribute, CanvasAttribute,
    ColAttribute, ColGroupAttribute, DataAttribute, DetailsAttribute,
};
use strum::AsRefStr;

pub trait AriaAttribute: Attribute {}

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-activedescendant>
#[derive(Debug)]
pub struct AriaActivedescendant(String);

impl Attribute for AriaActivedescendant {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-activedescendant"
    }
}
crate::add_impls!(AriaActivedescendant);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-atomic>
#[derive(Debug)]
pub struct AriaAtomic {
    val: String,
}

impl AriaAtomic {
    pub fn new(val: bool) -> Self {
        Self {
            val: val.to_string(),
        }
    }
}

impl Attribute for AriaAtomic {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-atomic"
    }
}
crate::add_impls!(AriaAtomic);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-autocomplete>
#[derive(Debug)]
pub struct AriaAutocomplete(AriaAutocompleteOption);

impl Attribute for AriaAutocomplete {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-autocomplete"
    }
}
crate::add_impls!(AriaAutocomplete);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-busy>
#[derive(Debug)]
pub struct AriaBusy {
    val: String,
}

impl AriaBusy {
    pub fn new(val: bool) -> Self {
        Self {
            val: val.to_string(),
        }
    }
}

impl Attribute for AriaBusy {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-busy"
    }
}
crate::add_impls!(AriaBusy);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-checked>
#[derive(Debug)]
pub struct AriaChecked(AriaCheckedOption);

impl Attribute for AriaChecked {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-checked"
    }
}
crate::add_impls!(AriaChecked);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-colcount>
#[derive(Debug)]
pub struct AriaColCount {
    val: String,
}

impl AriaColCount {
    pub fn new(val: u8) -> Self {
        Self {
            val: val.to_string(),
        }
    }
}

// Note: unsigned because we can't have a negative colcount and 8-bit because you
// could never render more than 255 columns on a screen, so we do this for optimization
impl Attribute for AriaColCount {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-colcount"
    }
}

// TODO: impl TableAttribute, GridAttribute, TreeGridAttribute

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-colindex>
#[derive(Debug)]
pub struct AriaColIndex {
    val: String,
}

impl AriaColIndex {
    pub fn new(val: u8) -> Self {
        Self {
            val: val.to_string(),
        }
    }
}

impl Attribute for AriaColIndex {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-colindex"
    }
}

// TODO: impl TableAttribute, GridAttribute, TreeGridAttribute

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-colspan>
#[derive(Debug)]
pub struct AriaColSpan {
    val: String,
}

impl AriaColSpan {
    pub fn new(val: u8) -> Self {
        Self {
            val: val.to_string(),
        }
    }
}

impl Attribute for AriaColSpan {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-colspan"
    }
}

// TODO: impl TableAttribute, GridAttribute, TreeGridAttribute

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-controls>
#[derive(Debug)]
pub struct AriaControls {
    val: String,
}

impl AriaControls {
    pub fn new(val: Vec<String>) -> Self {
        Self { val: val.join(" ") }
    }
}

impl Attribute for AriaControls {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-controls"
    }
}
crate::add_impls!(AriaControls);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-current>
#[derive(Debug)]
pub struct AriaCurrent(AriaCurrentOption);

impl Attribute for AriaCurrent {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-current"
    }
}
crate::add_impls!(AriaCurrent);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-describedby>
#[derive(Debug)]
pub struct AriaDescribedby {
    val: String,
}

impl AriaDescribedby {
    pub fn new(val: Vec<String>) -> Self {
        Self { val: val.join(" ") }
    }
}

impl Attribute for AriaDescribedby {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-describedby"
    }
}
crate::add_impls!(AriaDescribedby);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-details>
#[derive(Debug)]
pub struct AriaDetails {
    val: String,
}

impl AriaDetails {
    pub fn new(val: Vec<String>) -> Self {
        Self { val: val.join(" ") }
    }
}

impl Attribute for AriaDetails {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-details"
    }
}
crate::add_impls!(AriaDetails);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-disabled>
#[derive(Debug)]
pub struct AriaDisabled {
    val: String,
}

impl AriaDisabled {
    pub fn new(val: bool) -> Self {
        Self {
            val: val.to_string(),
        }
    }
}

impl Attribute for AriaDisabled {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-disabled"
    }
}
crate::add_impls!(AriaDisabled);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-dropeffect>
#[derive(Debug)]
pub struct AriaDropEffect(AriaDropEffectOption);

impl Attribute for AriaDropEffect {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-dropeffect"
    }
}
crate::add_impls!(AriaDropEffect);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-errormessage>
#[derive(Debug)]
pub struct AriaErrormessage(String);

impl Attribute for AriaErrormessage {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-errormessage"
    }
}
// TODO: impl this for input elements only

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-expanded>
#[derive(Debug)]
pub struct AriaExpanded {
    val: String,
}

impl AriaExpanded {
    pub fn new(val: bool) -> Self {
        Self {
            val: val.to_string(),
        }
    }
}

impl Attribute for AriaExpanded {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-expanded"
    }
}
crate::add_impls!(AriaExpanded);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-flowto>
#[derive(Debug)]
pub struct AriaFlowTo {
    val: String,
}

impl AriaFlowTo {
    pub fn new(val: Vec<String>) -> Self {
        Self { val: val.join(" ") }
    }
}

impl Attribute for AriaFlowTo {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-flowto"
    }
}
crate::add_impls!(AriaFlowTo);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-grabbed>
#[derive(Debug)]
pub struct AriaGrabbed {
    val: String,
}

impl AriaGrabbed {
    pub fn new(val: bool) -> Self {
        Self {
            val: val.to_string(),
        }
    }
}

impl Attribute for AriaGrabbed {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-grabbed"
    }
}
crate::add_impls!(AriaGrabbed);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-haspopup>
#[derive(Debug)]
pub struct AriaHasPopup(AriaHasPopupOption);

impl Attribute for AriaHasPopup {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-haspopup"
    }
}
crate::add_impls!(AriaHasPopup);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-hidden>
#[derive(Debug)]
pub struct AriaHidden {
    val: String,
}

impl AriaHidden {
    pub fn new(val: bool) -> Self {
        Self {
            val: val.to_string(),
        }
    }
}

impl Attribute for AriaHidden {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-hidden"
    }
}
crate::add_impls!(AriaHidden);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-invalid>
#[derive(Debug)]
pub struct AriaInvalid(AriaHasPopupOption);

impl Attribute for AriaInvalid {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-invalid"
    }
}
// TODO: impl this for input elements

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-keyshortcuts>
#[derive(Debug)]
pub struct AriaKeyshortcuts(String);

impl Attribute for AriaKeyshortcuts {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-keyshortcuts"
    }
}
crate::add_impls!(AriaKeyshortcuts);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-label>
#[derive(Debug)]
pub struct AriaLabel(String);

impl Attribute for AriaLabel {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-label"
    }
}
crate::add_impls!(AriaLabel);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-labelledby>
#[derive(Debug)]
pub struct AriaLabelledBy {
    val: String,
}

impl AriaLabelledBy {
    pub fn new(val: Vec<String>) -> Self {
        Self { val: val.join(" ") }
    }
}

impl Attribute for AriaLabelledBy {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-labelledby"
    }
}
crate::add_impls!(AriaLabelledBy);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-level>
#[derive(Debug)]
pub struct AriaLevel {
    val: String,
}

impl AriaLevel {
    pub fn new(val: u8) -> Self {
        Self {
            val: val.to_string(),
        }
    }
}

impl Attribute for AriaLevel {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-level"
    }
}
crate::add_impls!(AriaLevel);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-live>
#[derive(Debug)]
pub struct AriaLive(AriaLiveOption);

impl Attribute for AriaLive {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-live"
    }
}
crate::add_impls!(AriaLive);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-modal>
#[derive(Debug)]
pub struct AriaModal {
    val: String,
}

impl AriaModal {
    pub fn new(val: bool) -> Self {
        Self {
            val: val.to_string(),
        }
    }
}

impl Attribute for AriaModal {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-modal"
    }
}
crate::add_impls!(AriaModal);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-multiline>
#[derive(Debug)]
pub struct AriaMultiline {
    val: String,
}

impl AriaMultiline {
    pub fn new(val: bool) -> Self {
        Self {
            val: val.to_string(),
        }
    }
}

impl Attribute for AriaMultiline {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-multiline"
    }
}
crate::add_impls!(AriaMultiline);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-multiselectable>
#[derive(Debug)]
pub struct AriaMultiselectable {
    val: String,
}

impl AriaMultiselectable {
    pub fn new(val: bool) -> Self {
        Self {
            val: val.to_string(),
        }
    }
}

impl Attribute for AriaMultiselectable {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-multiselectable"
    }
}
crate::add_impls!(AriaMultiselectable);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-orientation>
#[derive(Debug)]
pub struct AriaOrientation(AriaOrientationOption);

impl Attribute for AriaOrientation {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-orientation"
    }
}
crate::add_impls!(AriaOrientation);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-owns>
#[derive(Debug)]
pub struct AriaOwns {
    val: String,
}

impl AriaOwns {
    pub fn new(val: Vec<String>) -> Self {
        Self { val: val.join(" ") }
    }
}

impl Attribute for AriaOwns {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-owns"
    }
}
crate::add_impls!(AriaOwns);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-placeholder>
#[derive(Debug)]
pub struct AriaPlaceholder(String);

impl Attribute for AriaPlaceholder {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-placeholder"
    }
}
crate::add_impls!(AriaPlaceholder);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-posinset>
#[derive(Debug)]
pub struct AriaPosInset {
    val: String,
}

impl AriaPosInset {
    pub fn new(val: u16) -> Self {
        Self {
            val: val.to_string(),
        }
    }
}

impl Attribute for AriaPosInset {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-posinset"
    }
}
crate::add_impls!(AriaPosInset);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-pressed>
#[derive(Debug)]
pub struct AriaPressed(AriaOrientationOption);

impl Attribute for AriaPressed {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-pressed"
    }
}
impl ButtonAttribute for AriaPressed {}

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-readonly>
#[derive(Debug)]
pub struct AriaReadonly {
    val: String,
}

impl AriaReadonly {
    pub fn new(val: bool) -> Self {
        Self {
            val: val.to_string(),
        }
    }
}

impl Attribute for AriaReadonly {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-readonly"
    }
}
crate::add_impls!(AriaReadonly);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-relevant>
#[derive(Debug)]
pub struct AriaRelevant(AriaRelevantOption);

impl Attribute for AriaRelevant {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-relevant"
    }
}
crate::add_impls!(AriaRelevant);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-required>
#[derive(Debug)]
pub struct AriaRequired {
    val: String,
}

impl AriaRequired {
    pub fn new(val: bool) -> Self {
        Self {
            val: val.to_string(),
        }
    }
}

impl Attribute for AriaRequired {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-required"
    }
}
// TODO: just implement for input element

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-roledescription>
#[derive(Debug)]
pub struct AriaRoleDescription(String);

impl Attribute for AriaRoleDescription {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-roledescription"
    }
}
crate::add_impls!(AriaRoleDescription);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-rowcount>
#[derive(Debug)]
pub struct AriaRowCount {
    val: String,
}

impl AriaRowCount {
    pub fn new(val: u8) -> Self {
        Self {
            val: val.to_string(),
        }
    }
}

impl Attribute for AriaRowCount {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-rowcount"
    }
}
// TODO: impl TableAttribute, GridAttribute, TreeGridAttribute

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-rowindex>
#[derive(Debug)]
pub struct AriaRowIndex {
    val: String,
}

impl AriaRowIndex {
    pub fn new(val: u8) -> Self {
        Self {
            val: val.to_string(),
        }
    }
}

impl Attribute for AriaRowIndex {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-rowindex"
    }
}
// TODO: impl TableAttribute, GridAttribute, TreeGridAttribute

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-rowspan>
#[derive(Debug)]
pub struct AriaRowSpan {
    val: String,
}

impl AriaRowSpan {
    pub fn new(val: u8) -> Self {
        Self {
            val: val.to_string(),
        }
    }
}

impl Attribute for AriaRowSpan {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-rowspan"
    }
}
// TODO: impl TableAttribute, GridAttribute, TreeGridAttribute

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-selected>
#[derive(Debug)]
pub struct AriaSelected {
    val: String,
}

impl AriaSelected {
    pub fn new(val: bool) -> Self {
        Self {
            val: val.to_string(),
        }
    }
}

impl Attribute for AriaSelected {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-selected"
    }
}
crate::add_impls!(AriaSelected);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-setsize>
#[derive(Debug)]
pub struct AriaSetSize {
    val: String,
}

impl AriaSetSize {
    pub fn new(val: i16) -> Self {
        Self {
            val: val.to_string(),
        }
    }
}

impl Attribute for AriaSetSize {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-setsize"
    }
}
crate::add_impls!(AriaSetSize);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-sort>
#[derive(Debug)]
pub struct AriaSort(AriaSortOption);

impl Attribute for AriaSort {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-sort"
    }
}

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-valuemax>
#[derive(Debug)]
pub struct AriaValueMax {
    val: String,
}

impl AriaValueMax {
    pub fn new(val: i16) -> Self {
        Self {
            val: val.to_string(),
        }
    }
}

impl Attribute for AriaValueMax {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-valuemax"
    }
}
crate::add_impls!(AriaValueMax);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-valuemin>
#[derive(Debug)]
pub struct AriaValueMin {
    val: String,
}

impl AriaValueMin {
    pub fn new(val: i16) -> Self {
        Self {
            val: val.to_string(),
        }
    }
}

impl Attribute for AriaValueMin {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-valuemin"
    }
}
crate::add_impls!(AriaValueMin);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-valuenow>
#[derive(Debug)]
pub struct AriaValueNow {
    val: String,
}

impl AriaValueNow {
    pub fn new(val: i16) -> Self {
        Self {
            val: val.to_string(),
        }
    }
}

impl Attribute for AriaValueNow {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-valuenow"
    }
}
crate::add_impls!(AriaValueNow);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-valuetext>
#[derive(Debug)]
pub struct AriaValueText(String);

impl Attribute for AriaValueText {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-valuetext"
    }
}
crate::add_impls!(AriaValueText);

/// Models the possible values of the `aria-autocomplete` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-autocomplete>
#[derive(Debug, AsRefStr, Default)]
#[strum(serialize_all = "lowercase")]
pub enum AriaAutocompleteOption {
    #[default]
    None,
    Inline,
    List,
    Both,
}

/// Models the possible values of the `aria-checked` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-checked>
#[derive(Debug, AsRefStr, Default)]
#[strum(serialize_all = "lowercase")]
pub enum AriaCheckedOption {
    False,
    Mixed,
    True,
    #[default]
    Undefined,
}

/// Models the possible values of the `aria-current` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-current>
#[derive(Debug, AsRefStr, Default)]
#[strum(serialize_all = "lowercase")]
pub enum AriaCurrentOption {
    #[default]
    False,
    True,
    Page,
    Step,
    Location,
    Date,
    Time,
}

/// Models the possible values of the `aria-dropeffect` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-dropeffect>
#[derive(Debug, AsRefStr, Default)]
#[strum(serialize_all = "lowercase")]
pub enum AriaDropEffectOption {
    #[default]
    None,
    Copy,
    Execute,
    Link,
    Move,
    Popup,
}

/// Models the possible values of the `aria-haspopup` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-haspopup>
#[derive(Debug, AsRefStr, Default)]
#[strum(serialize_all = "lowercase")]
pub enum AriaHasPopupOption {
    #[default]
    False,
    True,
    Menu,
    Listbox,
    Tree,
    Grid,
    Dialog,
}

/// Models the possible values of the `aria-invalid` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-invalid>
#[derive(Debug, AsRefStr, Default)]
#[strum(serialize_all = "lowercase")]
pub enum AriaInvalidOption {
    #[default]
    False,
    True,
    Grammar,
    Spelling,
}

/// Models the possible values of the `aria-live` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-live>
#[derive(Debug, AsRefStr, Default)]
#[strum(serialize_all = "lowercase")]
pub enum AriaLiveOption {
    #[default]
    Off,
    Assertive,
    Polite,
}

/// Models the possible values of the `aria-orientation` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-orientation>
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum AriaOrientationOption {
    Horizontal,
    Vertical,
}

/// Models the possible values of the `aria-pressed` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-pressed>
type AriaPressedOption = AriaCheckedOption;

/// Models the possible values of the `aria-relevant` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-relevant>
#[derive(Debug, AsRefStr, Default)]
#[strum(serialize_all = "lowercase")]
pub enum AriaRelevantOption {
    Additions,
    #[strum(serialize = "additions removals")]
    AdditionsRemovals,
    #[default]
    #[strum(serialize = "additions text")]
    AdditionsText,
    All,
    Removals,
    #[strum(serialize = "removals additions")]
    RemovalsAdditions,
    #[strum(serialize = "removals text")]
    RemovalsText,
    Text,
    #[strum(serialize = "text additions")]
    TextAdditions,
    #[strum(serialize = "text removals")]
    TextRemovals,
}

/// Models the possible values of the `aria-sort` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-sort>
#[derive(Debug, AsRefStr, Default)]
#[strum(serialize_all = "lowercase")]
pub enum AriaSortOption {
    #[default]
    None,
    Ascending,
    Descending,
    Other,
}
