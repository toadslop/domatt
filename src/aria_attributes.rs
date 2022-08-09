use crate::Attribute;
use strum::{AsRefStr, Display};

/// Models the possible values of the `aria-autocomplete` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-autocomplete>
#[derive(Debug, AsRefStr, Default)]
#[strum(serialize_all = "lowercase")]
pub enum AriaAutocomplete {
    #[default]
    None,
    Inline,
    List,
    Both,
}

/// Models the possible values of the `aria-checked` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-checked>
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum AriaChecked {
    False,
    Mixed,
    True,
}

/// Models the possible values of the `aria-current` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-current>
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum AriaCurrent {
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
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum AriaDropeffect {
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
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum AriaHasPopup {
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
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum AriaInvalid {
    False,
    True,
    Grammar,
    Spelling,
}

/// Models the possible values of the `aria-live` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-live>
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum AriaLive {
    Off,
    Assertive,
    Polite,
}

/// Models the possible values of the `aria-orientation` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-orientation>
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum AriaOrientation {
    Horizontal,
    Vertical,
}

/// Models the possible values of the `aria-pressed` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-pressed>
type AriaPressed = AriaChecked;

/// Models the possible values of the `aria-relevant` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-relevant>
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum AriaRelevant {
    Additions,
    #[strum(serialize = "additions removals")]
    AdditionsRemovals,
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
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum AriaSort {
    None,
    Ascending,
    Descending,
    Other,
}

/// An enum defining the different Aria attribute keys. Each variant takes a tuple
/// that represents the valid values for the attributes.
///
/// <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA>
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum AriaAttributes<'a> {
    /// Identifies the currently active element when DOM focus is on a composite widget, textbox, group, or application.
    AriaActivedescendant(&'a str),

    /// Indicates whether assistive technologies will present all, or only parts of, the changed region based on the
    /// change notifications defined by the aria-relevant attribute.
    AriaAtomic(bool),

    /// Indicates whether inputting text could trigger display of one or more predictions of the user's intended value
    /// for an input and specifies how predictions would be presented if they are made.
    AriaAutocomplete(&'a AriaAutocomplete),

    /// Indicates an element is being modified and that assistive technologies MAY want to wait until the modifications
    /// are complete before exposing them to the user.
    AriaBusy(bool),

    /// Indicates the current "checked" state of checkboxes, radio buttons, and other widgets.
    ///
    /// see [AriaPressed](`AriaAttributes::AriaPressed`) see [AriaSelected](`AriaAttributes::AriaSelected`).
    AriaChecked(&'a AriaChecked),

    /// Defines the total number of columns in a table, grid, or treegrid.
    ///
    /// see [AriaColindex](`AriaAttributes::AriaColindex`).
    AriaColcount(i64),

    /// Defines an element's column index or position with respect to the total number of columns within a table, grid, or treegrid.
    ///
    /// See [AriaColcount](`AriaAttributes::AriaColcount`).
    ///
    /// See [AriaColspan](`AriaAttributes::AriaColspan`).
    AriaColindex(i64),

    /// Defines the number of columns spanned by a cell or gridcell within a table, grid, or treegrid.
    ///
    /// See[AriaColindex](`AriaAttributes::AriaColindex`).
    ///
    /// See [AriaRowspan](`AriaAttributes::AriaRowspan`).
    AriaColspan(i64),

    /// Identifies the element (or elements) whose contents or presence are controlled by the current element.
    ///
    /// See [AriaOwns](`AriaAttributes::AriaOwns`).
    AriaControls(&'a str),

    /// Indicates the element that represents the current item within a container or set of related elements.
    AriaCurrent(&'a AriaCurrent),

    /// Identifies the element (or elements) that describes the object.
    ///
    /// See [AriaLabelledby](`AriaAttributes::AriaLabelledby`)
    AriaDescribedby(&'a str),

    /// Identifies the element that provides a detailed, extended description for the object.
    ///
    /// See [AriaDescribedby](`AriaAttributes::AriaDescribedby`).
    AriaDetails(&'a str),

    /// Indicates that the element is perceivable but disabled, so it is not editable or otherwise operable.
    ///
    /// See [AriaHidden](`AriaAttributes::AriaHidden`)
    ///
    /// See [AriaReadonly](`AriaAttributes::AriaReadonly`).
    AriaDisabled(bool),

    /// Indicates what functions can be performed when a dragged object is released on the drop target.
    AriaDropeffect(&'a AriaDropeffect),

    /// Identifies the element that provides an error message for the object.
    ///
    /// See [AriaInvalid](`AriaAttributes::AriaInvalid`).
    ///
    /// See [AriaDescribedby](`AriaAttributes::AriaDescribedby`)).
    AriaErrormessage(&'a str),

    /// Indicates whether the element, or another grouping element it controls, is currently expanded or collapsed.
    AriaExpanded(bool),

    /// Identifies the next element (or elements) in an alternate reading order of content which, at the user's discretion,
    /// allows assistive technology to override the general default of reading in document source order.
    AriaFlowto(&'a str),

    /// Indicates an element's "grabbed" state in a drag-and-drop operation.
    AriaGrabbed(bool),

    /// Indicates the availability and type of interactive popup element, such as menu or dialog, that can
    /// be triggered by an element.
    AriaHaspopup(&'a AriaHasPopup),

    /// Indicates whether the element is exposed to an accessibility API.
    ///
    /// see [AriaDisabled](`AriaAttributes::AriaDisabled`).
    AriaHidden(bool),

    /// Indicates the entered value does not conform to the format expected by the application.
    ///
    /// see [AriaErrormessage](`AriaAttributes::AriaErrormessage`).
    AriaInvalid(&'a AriaInvalid),

    /// Indicates keyboard shortcuts that an author has implemented to activate or give focus to an element.
    AriaKeyshortcuts(&'a str),

    /// Defines a string value that labels the current element.
    ///
    /// see [AriaLabelledby](`AriaAttributes::AriaLabelledby`)
    AriaLabel(&'a str),

    /// Identifies the element (or elements) that labels the current element.
    ///
    /// see [AriaDescribedby](`AriaAttributes::AriaDescribedby`).
    AriaLabelledby(&'a str),

    /// Defines the hierarchical level of an element within a structure.
    AriaLevel(i64),

    /// Indicates that an element will be updated, and describes the types of updates the user agents, assistive
    /// technologies, and user can expect from the live region.
    AriaLive(&'a AriaLive),

    /// Indicates whether an element is modal when displayed.
    AriaModal(bool),

    /// Indicates whether a text box accepts multiple lines of input or only a single line.
    AriaMultiline(bool),

    /// Indicates that the user may select more than one item from the current selectable descendants.
    AriaMultiselectable(bool),

    /// Indicates whether the element's orientation is horizontal, vertical, or unknown/ambiguous.
    AriaOrientation(&'a AriaOrientation),

    /// Identifies an element (or elements) in order to define a visual, functional, or contextual parent/child relationship
    /// between DOM elements where the DOM hierarchy cannot be used to represent the relationship.
    ///
    /// see [AriaControls](`AriaAttributes::AriaControls`).
    AriaOwns(&'a str),

    /// Defines a short hint (a word or short phrase) intended to aid the user with data entry when the control has no value.
    /// A hint could be a sample value or a brief description of the expected format.
    AriaPlaceholder(&'a str),

    /// Defines an element's number or position in the current set of listitems or treeitems. Not required if all elements
    /// in the set are present in the DOM.
    ///
    /// See [AriaSetsize](`AriaAttributes::AriaSetsize`)
    AriaPosinset(i64),

    /// Indicates the current "pressed" state of toggle buttons.
    ///
    /// See [AriaChecked](`AriaAttributes::AriaChecked`).
    ///
    /// See [AriaSelected](`AriaAttributes::AriaSelected`).
    AriaPressed(&'a AriaPressed),

    /// Indicates that the element is not editable, but is otherwise operable.
    /// see [AriaDisabled](`AriaAttributes::AriaDisabled`).
    AriaReadonly(bool),

    /// Indicates what notifications the user agent will trigger when the accessibility tree within a live region is modified.
    ///
    /// see [AriaAtomic](`AriaAttributes::AriaAtomic`).
    AriaRelevant(&'a AriaRelevant),

    /// Indicates that user input is required on the element before a form may be submitted.
    AriaRequired(bool),

    /// Defines a human-readable, author-localized description for the role of an element.
    AriaRoledescription(&'a str),

    /// Defines the total number of rows in a table, grid, or treegrid.
    ///
    /// see [AriaRowindex](`AriaAttributes::AriaRowindex`).
    AriaRowcount(i64),

    /// Defines an element's row index or position with respect to the total number of rows within a table, grid, or treegrid.
    ///
    /// see [AriaRowcount](`AriaAttributes::AriaRowcount`) see [AriaRowspan](`AriaAttributes::AriaRowspan`).
    AriaRowindex(i64),

    /// Defines the number of rows spanned by a cell or gridcell within a table, grid, or treegrid.
    ///
    /// see [AriaRowindex](`AriaAttributes::AriaRowindex`) see [AriaColspan](`AriaAttributes::AriaColspan`).
    AriaRowspan(i64),

    /// Indicates the current "selected" state of various widgets.
    ///
    /// see [AriaChecked](`AriaAttributes::AriaChecked`) see [AriaPressed](`AriaAttributes::AriaPressed`).
    AriaSelected(bool),

    /// Defines the number of items in the current set of listitems or treeitems. Not required if all
    /// elements in the set are present in the DOM.
    ///
    /// see [AriaPosinset](`AriaAttributes::AriaPosinset`).
    AriaSetsize(i64),

    /// Indicates if items in a table or grid are sorted in ascending or descending order.
    AriaSort(&'a AriaSort),

    /// Defines the maximum allowed value for a range widget.
    AriaValuemax(i64),

    /// Defines the minimum allowed value for a range widget.
    AriaValuemin(i64),

    /// Defines the current value for a range widget.
    ///
    /// see [AriaValuetext](`AriaAttributes::AriaValuetext`).
    AriaValuenow(i64),

    /// Defines the human readable text alternative of aria-valuenow for a range widget.
    AriaValuetext(&'a str),
}

impl<'a> Attribute<'a> for AriaAttributes<'a> {
    fn get_key(&self) -> &str {
        self.as_ref()
    }

    fn get_val(&self) -> Option<&str> {
        match self {
            AriaAttributes::AriaActivedescendant(val) => Some(val),
            AriaAttributes::AriaAtomic(val) => Some(val.to_string().as_ref()),
            AriaAttributes::AriaAutocomplete(val) => Some(val.as_ref()),
            AriaAttributes::AriaBusy(val) => Some(val.to_string().as_ref()),
            AriaAttributes::AriaChecked(val) => Some(val.as_ref()),
            AriaAttributes::AriaColcount(val) => Some(val.to_string().as_ref()),
            AriaAttributes::AriaColindex(val) => Some(val.to_string().as_ref()),
            AriaAttributes::AriaColspan(val) => Some(val.to_string().as_ref()),
            AriaAttributes::AriaControls(val) => Some(val),
            AriaAttributes::AriaCurrent(val) => Some(val.as_ref()),
            AriaAttributes::AriaDescribedby(val) => Some(val),
            AriaAttributes::AriaDetails(val) => Some(val),
            AriaAttributes::AriaDisabled(val) => Some(val.to_string().as_ref()),
            AriaAttributes::AriaDropeffect(val) => Some(val.as_ref()),
            AriaAttributes::AriaErrormessage(val) => Some(val),
            AriaAttributes::AriaExpanded(val) => Some(val.to_string().as_ref()),
            AriaAttributes::AriaFlowto(val) => Some(val),
            AriaAttributes::AriaGrabbed(val) => Some(val.to_string().as_ref()),
            AriaAttributes::AriaHaspopup(val) => Some(val.as_ref()),
            AriaAttributes::AriaHidden(val) => Some(val.to_string().as_ref()),
            AriaAttributes::AriaInvalid(val) => Some(val.as_ref()),
            AriaAttributes::AriaKeyshortcuts(val) => Some(val),
            AriaAttributes::AriaLabel(val) => Some(val),
            AriaAttributes::AriaLabelledby(val) => Some(val),
            AriaAttributes::AriaLevel(val) => Some(val.to_string().as_ref()),
            AriaAttributes::AriaLive(val) => Some(val.as_ref()),
            AriaAttributes::AriaModal(val) => Some(val.to_string().as_ref()),
            AriaAttributes::AriaMultiline(val) => Some(val.to_string().as_ref()),
            AriaAttributes::AriaMultiselectable(val) => Some(val.to_string().as_ref()),
            AriaAttributes::AriaOrientation(val) => Some(val.as_ref()),
            AriaAttributes::AriaOwns(val) => Some(val),
            AriaAttributes::AriaPlaceholder(val) => Some(val),
            AriaAttributes::AriaPosinset(val) => Some(val.to_string().as_ref()),
            AriaAttributes::AriaPressed(val) => Some(val.as_ref()),
            AriaAttributes::AriaReadonly(val) => Some(val.to_string().as_ref()),
            AriaAttributes::AriaRelevant(val) => Some(val.as_ref()),
            AriaAttributes::AriaRequired(val) => Some(val.to_string().as_ref()),
            AriaAttributes::AriaRoledescription(val) => Some(val),
            AriaAttributes::AriaRowcount(val) => Some(val.to_string().as_ref()),
            AriaAttributes::AriaRowindex(val) => Some(val.to_string().as_ref()),
            AriaAttributes::AriaRowspan(val) => Some(val.to_string().as_ref()),
            AriaAttributes::AriaSelected(val) => Some(val.to_string().as_ref()),
            AriaAttributes::AriaSetsize(val) => Some(val.to_string().as_ref()),
            AriaAttributes::AriaSort(val) => Some(val.as_ref()),
            AriaAttributes::AriaValuemax(val) => Some(val.to_string().as_ref()),
            AriaAttributes::AriaValuemin(val) => Some(val.to_string().as_ref()),
            AriaAttributes::AriaValuenow(val) => Some(val.to_string().as_ref()),
            AriaAttributes::AriaValuetext(val) => Some(val),
        }
    }
}
