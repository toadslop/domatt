use crate::{
    anchor::AnchorAttribute, area::AreaAttribute, audio::AudioAttribute, base::BaseAttribute,
    blockquote::BlockQuoteAttribute, button::ButtonAttribute, canvas::CanvasAttribute,
    col::ColAttribute, colgroup::ColGroupAttribute, data::DataAttribute, details::DetailsAttribute,
    svg::SvgAttribute, Attribute,
};
use strum::AsRefStr;

pub trait AriaAttribute: Attribute {}

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-activedescendant>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", String)]
pub struct AriaActivedescendant(String);
crate::add_impls!(AriaActivedescendant);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-atomic>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", bool)]
pub struct AriaAtomic(String);
crate::add_impls!(AriaAtomic);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-autocomplete>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", AriaAutocompleteOption)]
pub struct AriaAutocomplete(AriaAutocompleteOption);
crate::add_impls!(AriaAutocomplete);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-busy>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", bool)]
pub struct AriaBusy(String);
crate::add_impls!(AriaBusy);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-checked>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", AriaCheckedOption)]
pub struct AriaChecked(AriaCheckedOption);
crate::add_impls!(AriaChecked);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-colcount>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", u8)]
pub struct AriaColcount(String);
// TODO: impl TableAttribute, GridAttribute, TreeGridAttribute

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-colindex>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", u8)]
pub struct AriaColindex(String);
// TODO: impl TableAttribute, GridAttribute, TreeGridAttribute

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-colspan>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", u8)]
pub struct AriaColspan(String);
// TODO: impl TableAttribute, GridAttribute, TreeGridAttribute

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-controls>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", Vec<String>)]
pub struct AriaControls(String);
crate::add_impls!(AriaControls);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-current>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", AriaCurrentOption)]
pub struct AriaCurrent(AriaCurrentOption);
crate::add_impls!(AriaCurrent);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-describedby>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", Vec<String>)]
pub struct AriaDescribedby(String);
crate::add_impls!(AriaDescribedby);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-details>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", Vec<String>)]
pub struct AriaDetails(String);
crate::add_impls!(AriaDetails);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-disabled>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", bool)]
pub struct AriaDisabled(String);
crate::add_impls!(AriaDisabled);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-dropeffect>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", AriaDropEffectOption)]
pub struct AriaDropeffect(AriaDropEffectOption);
crate::add_impls!(AriaDropeffect);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-errormessage>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", String)]
pub struct AriaErrormessage(String);
// TODO: impl this for input elements only

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-expanded>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", bool)]
pub struct AriaExpanded(String);
crate::add_impls!(AriaExpanded);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-flowto>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", Vec<String>)]
pub struct AriaFlowto(String);
crate::add_impls!(AriaFlowto);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-grabbed>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", bool)]
pub struct AriaGrabbed(String);
crate::add_impls!(AriaGrabbed);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-haspopup>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", AriaHasPopupOption)]
pub struct AriaHaspopup(AriaHasPopupOption);
crate::add_impls!(AriaHaspopup);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-hidden>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", bool)]
pub struct AriaHidden(String);
crate::add_impls!(AriaHidden);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-invalid>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", AriaHasPopupOption)]
pub struct AriaInvalid(AriaHasPopupOption);
// TODO: impl this for input elements

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-keyshortcuts>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", String)]
pub struct AriaKeyshortcuts(String);
crate::add_impls!(AriaKeyshortcuts);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-label>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", String)]
pub struct AriaLabel(String);
crate::add_impls!(AriaLabel);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-labelledby>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", Vec<String>)]
pub struct AriaLabelledBy(String);
crate::add_impls!(AriaLabelledBy);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-level>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", u8)]
pub struct AriaLevel(String);
crate::add_impls!(AriaLevel);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-live>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", AriaLiveOption)]
pub struct AriaLive(AriaLiveOption);
crate::add_impls!(AriaLive);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-modal>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", bool)]
pub struct AriaModal(String);
crate::add_impls!(AriaModal);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-multiline>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", bool)]
pub struct AriaMultiline(String);
crate::add_impls!(AriaMultiline);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-multiselectable>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", bool)]
pub struct AriaMultiselectable(String);
crate::add_impls!(AriaMultiselectable);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-orientation>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", AriaOrientationOption)]
pub struct AriaOrientation(AriaOrientationOption);
crate::add_impls!(AriaOrientation);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-owns>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", Vec<String>)]
pub struct AriaOwns(String);
crate::add_impls!(AriaOwns);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-placeholder>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", String)]
pub struct AriaPlaceholder(String);
crate::add_impls!(AriaPlaceholder);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-posinset>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", u16)]
pub struct AriaPosinset(String);
crate::add_impls!(AriaPosinset);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-pressed>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", AriaPressedOption)]
pub struct AriaPressed(AriaPressedOption);
impl ButtonAttribute for AriaPressed {}

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-readonly>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", bool)]
pub struct AriaReadonly(String);
crate::add_impls!(AriaReadonly);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-relevant>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", AriaRelevantOption)]
pub struct AriaRelevant(AriaRelevantOption);
crate::add_impls!(AriaRelevant);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-required>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", bool)]
pub struct AriaRequired(String);
// TODO: just implement for input element

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-roledescription>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", String)]
pub struct AriaRoledescription(String);
crate::add_impls!(AriaRoledescription);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-rowcount>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", u8)]
pub struct AriaRowcount(String);
// TODO: impl TableAttribute, GridAttribute, TreeGridAttribute

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-rowindex>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", u8)]
pub struct AriaRowindex(String);
// TODO: impl TableAttribute, GridAttribute, TreeGridAttribute

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-rowspan>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", u8)]
pub struct AriaRowspan(String);
// TODO: impl TableAttribute, GridAttribute, TreeGridAttribute

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-selected>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", bool)]
pub struct AriaSelected(String);
crate::add_impls!(AriaSelected);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-setsize>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", i16)]
pub struct AriaSetsize(String);
crate::add_impls!(AriaSetsize);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-sort>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", AriaSortOption)]
pub struct AriaSort(AriaSortOption);
crate::add_impls!(AriaSort);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-valuemax>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", i16)]
pub struct AriaValuemax(String);
crate::add_impls!(AriaValuemax);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-valuemin>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", i16)]
pub struct AriaValuemin(String);
crate::add_impls!(AriaValuemin);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-valuenow>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", i16)]
pub struct AriaValuenow(String);
crate::add_impls!(AriaValuenow);

/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-valuetext>
#[derive(Debug, Attribute)]
#[attribute("kebab-case", String)]
pub struct AriaValuetext(String);
crate::add_impls!(AriaValuetext);

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
