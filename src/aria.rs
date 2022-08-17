use crate::{
    anchor::AnchorAttribute, area::AreaAttribute, audio::AudioAttribute, base::BaseAttribute,
    blockquote::BlockQuoteAttribute, button::ButtonAttribute, canvas::CanvasAttribute,
    col::ColAttribute, colgroup::ColGroupAttribute, data::DataAttribute, details::DetailsAttribute,
    svg::SvgAttribute, Attribute,
};
use strum::AsRefStr;

pub trait AriaAttribute: Attribute {}

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-activedescendant>
#[derive(Debug)]
pub struct AriaActiveDescendant(String);

impl Attribute for AriaActiveDescendant {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-activedescendant"
    }

    type InputType = String;

    fn new(value: Self::InputType) -> Self {
        Self(value)
    }
}
crate::add_impls!(AriaActiveDescendant);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-atomic>
#[derive(Debug)]
pub struct AriaAtomic(String);

impl Attribute for AriaAtomic {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-atomic"
    }

    type InputType = bool;

    fn new(value: Self::InputType) -> Self {
        Self(value.to_string())
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

    type InputType = AriaAutocompleteOption;

    fn new(value: Self::InputType) -> Self {
        Self(value)
    }
}
crate::add_impls!(AriaAutocomplete);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-busy>
#[derive(Debug)]
pub struct AriaBusy(String);

impl Attribute for AriaBusy {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-busy"
    }

    type InputType = bool;

    fn new(value: Self::InputType) -> Self {
        Self(value.to_string())
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

    type InputType = AriaCheckedOption;

    fn new(value: Self::InputType) -> Self {
        Self(value)
    }
}
crate::add_impls!(AriaChecked);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-colcount>
#[derive(Debug)]
pub struct AriaColCount(String);

// Note: unsigned because we can't have a negative colcount and 8-bit because you
// could never render more than 255 columns on a screen, so we do this for optimization
impl Attribute for AriaColCount {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-colcount"
    }

    type InputType = u8;

    fn new(value: Self::InputType) -> Self {
        Self(value.to_string())
    }
}

// TODO: impl TableAttribute, GridAttribute, TreeGridAttribute

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-colindex>
#[derive(Debug)]
pub struct AriaColIndex(String);

impl Attribute for AriaColIndex {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-colindex"
    }

    type InputType = u8;

    fn new(value: Self::InputType) -> Self {
        Self(value.to_string())
    }
}

// TODO: impl TableAttribute, GridAttribute, TreeGridAttribute

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-colspan>
#[derive(Debug)]
pub struct AriaColSpan(String);

impl Attribute for AriaColSpan {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-colspan"
    }

    type InputType = u8;

    fn new(value: Self::InputType) -> Self {
        Self(value.to_string())
    }
}

// TODO: impl TableAttribute, GridAttribute, TreeGridAttribute

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-controls>
#[derive(Debug)]
pub struct AriaControls(String);

impl Attribute for AriaControls {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-controls"
    }

    type InputType = Vec<String>;

    fn new(value: Self::InputType) -> Self {
        Self(value.join(" "))
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

    type InputType = AriaCurrentOption;

    fn new(value: Self::InputType) -> Self {
        Self(value)
    }
}
crate::add_impls!(AriaCurrent);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-describedby>
#[derive(Debug)]
pub struct AriaDescribedby(String);

impl Attribute for AriaDescribedby {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-describedby"
    }

    type InputType = Vec<String>;

    fn new(value: Self::InputType) -> Self {
        Self(value.join(" "))
    }
}
crate::add_impls!(AriaDescribedby);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-details>
#[derive(Debug)]
pub struct AriaDetails(String);

impl Attribute for AriaDetails {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-details"
    }

    type InputType = Vec<String>;

    fn new(value: Self::InputType) -> Self {
        Self(value.join(" "))
    }
}
crate::add_impls!(AriaDetails);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-disabled>
#[derive(Debug)]
pub struct AriaDisabled(String);

impl Attribute for AriaDisabled {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-disabled"
    }

    type InputType = bool;

    fn new(value: Self::InputType) -> Self {
        Self(value.to_string())
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

    type InputType = AriaDropEffectOption;

    fn new(value: Self::InputType) -> Self {
        Self(value)
    }
}
crate::add_impls!(AriaDropEffect);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-errormessage>
#[derive(Debug)]
pub struct AriaErrorMessage(String);

impl Attribute for AriaErrorMessage {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-errormessage"
    }

    type InputType = String;

    fn new(value: Self::InputType) -> Self {
        Self(value)
    }
}
// TODO: impl this for input elements only

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-expanded>
#[derive(Debug)]
pub struct AriaExpanded(String);

impl Attribute for AriaExpanded {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-expanded"
    }

    type InputType = bool;

    fn new(value: Self::InputType) -> Self {
        Self(value.to_string())
    }
}
crate::add_impls!(AriaExpanded);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-flowto>
#[derive(Debug)]
pub struct AriaFlowTo(String);

impl Attribute for AriaFlowTo {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-flowto"
    }

    type InputType = Vec<String>;

    fn new(value: Self::InputType) -> Self {
        Self(value.join(" "))
    }
}
crate::add_impls!(AriaFlowTo);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-grabbed>
#[derive(Debug)]
pub struct AriaGrabbed(String);

impl Attribute for AriaGrabbed {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-grabbed"
    }

    type InputType = bool;

    fn new(value: Self::InputType) -> Self {
        Self(value.to_string())
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

    type InputType = AriaHasPopupOption;

    fn new(value: Self::InputType) -> Self {
        Self(value)
    }
}
crate::add_impls!(AriaHasPopup);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-hidden>
#[derive(Debug)]
pub struct AriaHidden(String);

impl Attribute for AriaHidden {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-hidden"
    }

    type InputType = bool;

    fn new(value: Self::InputType) -> Self {
        Self(value.to_string())
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

    type InputType = AriaHasPopupOption;

    fn new(value: Self::InputType) -> Self {
        Self(value)
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

    type InputType = String;

    fn new(value: Self::InputType) -> Self {
        Self(value)
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

    type InputType = String;

    fn new(value: Self::InputType) -> Self {
        Self(value)
    }
}
crate::add_impls!(AriaLabel);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-labelledby>
#[derive(Debug)]
pub struct AriaLabelledBy(String);

impl Attribute for AriaLabelledBy {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-labelledby"
    }

    type InputType = Vec<String>;

    fn new(value: Self::InputType) -> Self {
        Self(value.join(" "))
    }
}
crate::add_impls!(AriaLabelledBy);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-level>
#[derive(Debug)]
pub struct AriaLevel(String);

impl Attribute for AriaLevel {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-level"
    }

    type InputType = u8;

    fn new(value: Self::InputType) -> Self {
        Self(value.to_string())
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

    type InputType = AriaLiveOption;

    fn new(value: Self::InputType) -> Self {
        Self(value)
    }
}
crate::add_impls!(AriaLive);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-modal>
#[derive(Debug)]
pub struct AriaModal(String);

impl Attribute for AriaModal {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-modal"
    }

    type InputType = bool;

    fn new(value: Self::InputType) -> Self {
        Self(value.to_string())
    }
}
crate::add_impls!(AriaModal);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-multiline>
#[derive(Debug)]
pub struct AriaMultiline(String);

impl Attribute for AriaMultiline {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-multiline"
    }

    type InputType = bool;

    fn new(value: Self::InputType) -> Self {
        Self(value.to_string())
    }
}
crate::add_impls!(AriaMultiline);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-multiselectable>
#[derive(Debug)]
pub struct AriaMultiselectable(String);

impl Attribute for AriaMultiselectable {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-multiselectable"
    }

    type InputType = bool;

    fn new(value: Self::InputType) -> Self {
        Self(value.to_string())
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

    type InputType = AriaOrientationOption;

    fn new(value: Self::InputType) -> Self {
        Self(value)
    }
}
crate::add_impls!(AriaOrientation);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-owns>
#[derive(Debug)]
pub struct AriaOwns(String);

impl Attribute for AriaOwns {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-owns"
    }

    type InputType = Vec<String>;

    fn new(value: Self::InputType) -> Self {
        Self(value.join(" "))
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

    type InputType = String;

    fn new(value: Self::InputType) -> Self {
        Self(value)
    }
}
crate::add_impls!(AriaPlaceholder);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-posinset>
#[derive(Debug)]
pub struct AriaPosInset(String);

impl Attribute for AriaPosInset {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-posinset"
    }

    type InputType = u16;

    fn new(value: Self::InputType) -> Self {
        Self(value.to_string())
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

    type InputType = AriaOrientationOption;

    fn new(value: Self::InputType) -> Self {
        Self(value)
    }
}
impl ButtonAttribute for AriaPressed {}

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-readonly>
#[derive(Debug)]
pub struct AriaReadonly(String);

impl Attribute for AriaReadonly {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-readonly"
    }

    type InputType = bool;

    fn new(value: Self::InputType) -> Self {
        Self(value.to_string())
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

    type InputType = AriaRelevantOption;

    fn new(value: Self::InputType) -> Self {
        Self(value)
    }
}
crate::add_impls!(AriaRelevant);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-required>
#[derive(Debug)]
pub struct AriaRequired(String);

impl Attribute for AriaRequired {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-required"
    }

    type InputType = bool;

    fn new(value: Self::InputType) -> Self {
        Self(value.to_string())
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

    type InputType = String;

    fn new(value: Self::InputType) -> Self {
        Self(value)
    }
}
crate::add_impls!(AriaRoleDescription);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-rowcount>
#[derive(Debug)]
pub struct AriaRowCount(String);

impl Attribute for AriaRowCount {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-rowcount"
    }

    type InputType = u8;

    fn new(value: Self::InputType) -> Self {
        Self(value.to_string())
    }
}
// TODO: impl TableAttribute, GridAttribute, TreeGridAttribute

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-rowindex>
#[derive(Debug)]
pub struct AriaRowIndex(String);

impl Attribute for AriaRowIndex {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-rowindex"
    }

    type InputType = u8;

    fn new(value: Self::InputType) -> Self {
        Self(value.to_string())
    }
}
// TODO: impl TableAttribute, GridAttribute, TreeGridAttribute

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-rowspan>
#[derive(Debug)]
pub struct AriaRowSpan(String);

impl Attribute for AriaRowSpan {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-rowspan"
    }

    type InputType = u8;

    fn new(value: Self::InputType) -> Self {
        Self(value.to_string())
    }
}
// TODO: impl TableAttribute, GridAttribute, TreeGridAttribute

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-selected>
#[derive(Debug)]
pub struct AriaSelected(String);

impl Attribute for AriaSelected {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "aria-selected"
    }

    type InputType = bool;

    fn new(value: Self::InputType) -> Self {
        Self(value.to_string())
    }
}
crate::add_impls!(AriaSelected);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-setsize>
#[derive(Debug)]
pub struct AriaSetSize(String);

impl Attribute for AriaSetSize {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-setsize"
    }

    type InputType = i16;

    fn new(value: Self::InputType) -> Self {
        Self(value.to_string())
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

    type InputType = AriaSortOption;

    fn new(value: Self::InputType) -> Self {
        Self(value)
    }
}

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-valuemax>
#[derive(Debug)]
pub struct AriaValueMax(String);

impl Attribute for AriaValueMax {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-valuemax"
    }

    type InputType = i16;

    fn new(value: Self::InputType) -> Self {
        Self(value.to_string())
    }
}
crate::add_impls!(AriaValueMax);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-valuemin>
#[derive(Debug)]
pub struct AriaValueMin(String);

impl Attribute for AriaValueMin {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-valuemin"
    }

    type InputType = i16;

    fn new(value: Self::InputType) -> Self {
        Self(value.to_string())
    }
}
crate::add_impls!(AriaValueMin);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-valuenow>
#[derive(Debug)]
pub struct AriaValueNow(String);

impl Attribute for AriaValueNow {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "aria-valuenow"
    }

    type InputType = i16;

    fn new(value: Self::InputType) -> Self {
        Self(value.to_string())
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

    type InputType = String;

    fn new(value: Self::InputType) -> Self {
        Self(value)
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
// type AriaPressedOption = AriaCheckedOption;

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
