use super::set_attributes;
use crate::html::{
    anchor::AnchorAttribute, area::AreaAttribute, audio::AudioAttribute, base::BaseAttribute,
    blockquote::BlockQuoteAttribute, button::ButtonAttribute, canvas::CanvasAttribute,
    col::ColAttribute, colgroup::ColGroupAttribute, data::DataAttribute, details::DetailsAttribute,
};
use crate::Attribute;
use strum::AsRefStr;

pub trait AriaAttribute: Attribute {}

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-activedescendant>
#[derive(Debug, Clone, PartialEq)]
pub struct AriaActivedescendant(String);

impl Attribute for AriaActivedescendant {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-activedescendant"
    }
}

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-atomic>
#[derive(Debug, Clone, PartialEq)]
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
set_attributes!(AriaAtomic);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-autocomplete>
#[derive(Debug, Clone, PartialEq)]
pub struct AriaAutocomplete(AriaAutocompleteOption);

impl Attribute for AriaAutocomplete {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-autocomplete"
    }
}
set_attributes!(AriaAutocomplete);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-busy>
#[derive(Debug, Clone, PartialEq)]
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
set_attributes!(AriaBusy);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-checked>
#[derive(Debug, Clone, PartialEq)]
pub struct AriaChecked(AriaCheckedOption);

impl Attribute for AriaChecked {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-checked"
    }
}
set_attributes!(AriaChecked);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-colcount>
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
set_attributes!(AriaControls);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-current>
#[derive(Debug, Clone, PartialEq)]
pub struct AriaCurrent(AriaCurrentOption);

impl Attribute for AriaCurrent {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-current"
    }
}
set_attributes!(AriaCurrent);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-describedby>
#[derive(Debug, Clone, PartialEq)]
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
set_attributes!(AriaDescribedby);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-details>
#[derive(Debug, Clone, PartialEq)]
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
set_attributes!(AriaDetails);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-disabled>
#[derive(Debug, Clone, PartialEq)]
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
set_attributes!(AriaDisabled);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-dropeffect>
#[derive(Debug, Clone, PartialEq)]
pub struct AriaDropEffect(AriaDropEffectOption);

impl Attribute for AriaDropEffect {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-dropeffect"
    }
}
set_attributes!(AriaDropEffect);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-errormessage>
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
set_attributes!(AriaExpanded);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-flowto>
#[derive(Debug, Clone, PartialEq)]
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
set_attributes!(AriaFlowTo);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-grabbed>
#[derive(Debug, Clone, PartialEq)]
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
set_attributes!(AriaGrabbed);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-haspopup>
#[derive(Debug, Clone, PartialEq)]
pub struct AriaHasPopup(AriaHasPopupOption);

impl Attribute for AriaHasPopup {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-haspopup"
    }
}
set_attributes!(AriaHasPopup);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-hidden>
#[derive(Debug, Clone, PartialEq)]
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
set_attributes!(AriaHidden);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-invalid>
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
pub struct AriaKeyshortcuts(String);

impl Attribute for AriaKeyshortcuts {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-keyshortcuts"
    }
}
set_attributes!(AriaKeyshortcuts);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-label>
#[derive(Debug, Clone, PartialEq)]
pub struct AriaLabel(String);

impl Attribute for AriaLabel {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-label"
    }
}
set_attributes!(AriaLabel);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-labelledby>
#[derive(Debug, Clone, PartialEq)]
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
set_attributes!(AriaLabelledBy);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-level>
#[derive(Debug, Clone, PartialEq)]
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
set_attributes!(AriaLevel);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-live>
#[derive(Debug, Clone, PartialEq)]
pub struct AriaLive(AriaLiveOption);

impl Attribute for AriaLive {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-live"
    }
}
set_attributes!(AriaLive);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-modal>
#[derive(Debug, Clone, PartialEq)]
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
set_attributes!(AriaModal);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-multiline>
#[derive(Debug, Clone, PartialEq)]
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
set_attributes!(AriaMultiline);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-multiselectable>
#[derive(Debug, Clone, PartialEq)]
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
set_attributes!(AriaMultiselectable);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-orientation>
#[derive(Debug, Clone, PartialEq)]
pub struct AriaOrientation(AriaOrientationOption);

impl Attribute for AriaOrientation {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-orientation"
    }
}
set_attributes!(AriaOrientation);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-owns>
#[derive(Debug, Clone, PartialEq)]
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
set_attributes!(AriaOwns);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-placeholder>
#[derive(Debug, Clone, PartialEq)]
pub struct AriaPlaceholder(String);

impl Attribute for AriaPlaceholder {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-placeholder"
    }
}
set_attributes!(AriaPlaceholder);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-posinset>
#[derive(Debug, Clone, PartialEq)]
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
set_attributes!(AriaPosInset);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-pressed>
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
set_attributes!(AriaReadonly);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-relevant>
#[derive(Debug, Clone, PartialEq)]
pub struct AriaRelevant(AriaRelevantOption);

impl Attribute for AriaRelevant {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-relevant"
    }
}
set_attributes!(AriaRelevant);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-required>
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
pub struct AriaRoleDescription(String);

impl Attribute for AriaRoleDescription {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-roledescription"
    }
}
set_attributes!(AriaRoleDescription);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-rowcount>
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
set_attributes!(AriaSelected);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-setsize>
#[derive(Debug, Clone, PartialEq)]
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
set_attributes!(AriaSetSize);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-sort>
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
set_attributes!(AriaValueMax);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-valuemin>
#[derive(Debug, Clone, PartialEq)]
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
set_attributes!(AriaValueMin);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-valuenow>
#[derive(Debug, Clone, PartialEq)]
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
set_attributes!(AriaValueNow);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-valuetext>
#[derive(Debug, Clone, PartialEq)]
pub struct AriaValueText(String);

impl Attribute for AriaValueText {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-valuetext"
    }
}
set_attributes!(AriaValueText);

/// Models the possible values of the `aria-autocomplete` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-autocomplete>
#[derive(Debug, AsRefStr, Default, Clone, PartialEq)]
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
#[derive(Debug, AsRefStr, Default, Clone, PartialEq)]
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
#[derive(Debug, AsRefStr, Default, Clone, PartialEq)]
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
#[derive(Debug, AsRefStr, Default, Clone, PartialEq)]
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
#[derive(Debug, AsRefStr, Default, Clone, PartialEq)]
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
#[derive(Debug, AsRefStr, Default, Clone, PartialEq)]
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
#[derive(Debug, AsRefStr, Clone, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum AriaOrientationOption {
    Horizontal,
    Vertical,
}

// / Models the possible values of the `aria-pressed` attribute.
// /
// / <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-pressed>
// type AriaPressedOption = AriaCheckedOption;

/// Models the possible values of the `aria-relevant` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-relevant>
#[derive(Debug, AsRefStr, Default, Clone, PartialEq)]
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
#[derive(Debug, AsRefStr, Default, Clone, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum AriaSortOption {
    #[default]
    None,
    Ascending,
    Descending,
    Other,
}
