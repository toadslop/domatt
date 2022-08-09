use strum::{AsRefStr, Display};

use crate::Attribute;

// TODO: many of these attributes belong to somewhat obscure HTML elements.
// Should separate them out into their own enums rather than lumping them in
// here.

/// An enum defining the different generic html attribute keys. Each variant takes either tuple
/// that represents the valid values for the attributes or nothing to represent a boolean
/// attribute. Note that 'class' and 'style' are not available because these are expected to
/// be handled by whatever UI framework you're using.
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum HtmlAttributes<'a> {
    // Standard HTML Attributes
    AccessKey(&'a str),
    ContentEditable(&'a ContentEditable),
    ContextMenu(&'a str),
    Dir(&'a str),
    Draggable(bool),
    Hidden(bool),
    Id(&'a str),
    Lang(&'a str),
    Placeholder(&'a str),
    Slot(&'a str),
    SpellCheck(bool),
    // Style(CSSProperties), TODO: make a css properties handler
    TabIndex(u64),
    Title(&'a str),
    Translate(&'a Translate),

    // WAI-ARIA
    Role(&'a AriaRole),

    // RDFa Attributes
    About(&'a str),
    Datatype(&'a str),
    Inlist(&'a str),
    Prefix(&'a str),
    Property(&'a str),
    Resource(&'a str),
    Typeof(&'a str),
    Vocab(&'a str),

    // Non-standard Attributes
    AutoCapitalize(&'a str),
    AutoCorrect(&'a str),
    AutoSave(&'a str),
    Color(&'a str),
    ItemProp(&'a str),
    ItemScope(bool),
    ItemType(&'a str),
    ItemID(&'a str),
    ItemRef(&'a str),
    Results(u64),
    Security(&'a str),
    Unselectable(&'a Unselectable),

    // Living Standard
    /// Hints at the type of data that might be entered by the user while editing the element or its contents
    ///
    /// See <https://html.spec.whatwg.org/multipage/interaction.html#input-modalities:-the-inputmode-attribute>
    InputMode(&'a InputMode),

    /// Specify that a standard HTML element should behave like a defined custom built-in element
    ///
    /// See <https://html.spec.whatwg.org/multipage/custom-elements.html#attr-is>
    Is(&'a str),
}

/// An enum representing the different options for the `aria-role` attribute.
///
///  <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles>
#[derive(Debug, Display)]
#[strum(serialize_all = "lowercase")]
pub enum AriaRole {
    Alert,
    AlertDialog,
    Application,
    Article,
    Banner,
    Button,
    Cell,
    Checkbox,
    ColumnHeader,
    Combobox,
    Complementary,
    ContentInfo,
    Definition,
    Dialog,
    Directory,
    Document,
    Feed,
    Figure,
    Form,
    Grid,
    GridCell,
    Group,
    Heading,
    Img,
    Link,
    List,
    ListBox,
    ListItem,
    Log,
    Main,
    Marquee,
    Math,
    Menu,
    Menubar,
    MenuItem,
    MenuItemCheckbox,
    MenuItemRadio,
    Navigation,
    None,
    Note,
    Option,
    Presentation,
    ProgressBar,
    Radio,
    RadioGroup,
    Region,
    Row,
    RowGroup,
    RowHeader,
    ScrollBar,
    Search,
    SearchBox,
    Separator,
    Slider,
    SpinButton,
    Status,
    Switch,
    Tab,
    Table,
    TabLList,
    TabPanel,
    Term,
    TextNox,
    Timer,
    Toolbar,
    Tooltip,
    Tree,
    TreeGrid,
    TreeItem,
    Custom(String),
}

impl<'a> Attribute<'a> for HtmlAttributes<'a> {
    fn get_key(&self) -> &str {
        self.as_ref()
    }

    fn get_val(&self) -> Option<&str> {
        match self {
            HtmlAttributes::AccessKey(val) => Some(val),
            HtmlAttributes::ContentEditable(val) => Some(val.as_ref()),
            HtmlAttributes::ContextMenu(val) => Some(val),
            HtmlAttributes::Dir(val) => Some(val),
            HtmlAttributes::Draggable(val) => Some(val.to_string().as_ref()),
            HtmlAttributes::Hidden(val) => Some(val.to_string().as_ref()),
            HtmlAttributes::Id(val) => Some(val),
            HtmlAttributes::Lang(val) => Some(val),
            HtmlAttributes::Placeholder(val) => Some(val),
            HtmlAttributes::Slot(val) => Some(val),
            HtmlAttributes::SpellCheck(val) => Some(val.to_string().as_ref()),
            HtmlAttributes::TabIndex(val) => Some(val.to_string().as_ref()),
            HtmlAttributes::Title(val) => Some(val),
            HtmlAttributes::Translate(val) => Some(val.as_ref()),
            HtmlAttributes::About(val) => Some(val),
            HtmlAttributes::Datatype(val) => Some(val),
            HtmlAttributes::Inlist(val) => Some(val),
            HtmlAttributes::Prefix(val) => Some(val),
            HtmlAttributes::Property(val) => Some(val),
            HtmlAttributes::Resource(val) => Some(val),
            HtmlAttributes::Typeof(val) => Some(val),
            HtmlAttributes::Vocab(val) => Some(val),
            HtmlAttributes::AutoCapitalize(val) => Some(val),
            HtmlAttributes::AutoCorrect(val) => Some(val),
            HtmlAttributes::AutoSave(val) => Some(val),
            HtmlAttributes::Color(val) => Some(val),
            HtmlAttributes::ItemProp(val) => Some(val),
            HtmlAttributes::ItemScope(val) => Some(val.to_string().as_ref()),
            HtmlAttributes::ItemType(val) => Some(val),
            HtmlAttributes::ItemID(val) => Some(val),
            HtmlAttributes::ItemRef(val) => Some(val),
            HtmlAttributes::Results(val) => Some(val.to_string().as_ref()),
            HtmlAttributes::Security(val) => Some(val),
            HtmlAttributes::Unselectable(val) => Some(val.as_ref()),
            HtmlAttributes::InputMode(val) => Some(val.as_ref()),
            HtmlAttributes::Is(val) => Some(val),
            HtmlAttributes::Role(val) => match val {
                AriaRole::Custom(custom_val) => Some(custom_val),
                _ => Some(val.to_string().as_ref()),
            },
        }
    }
}

/// An enum representing the different options for the `aria-role` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/contenteditable>
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum ContentEditable {
    True,
    False,
    Inherit,
}

/// An enum representing the different options for the `translate` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/translate>
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum Translate {
    Yes,
    No,
}

/// An enum representing the different options for the `unselectable` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/unselectable>
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum Unselectable {
    On,
    Off,
}

/// An enum representing the different options for the `inputmode` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/inputmode>
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum InputMode {
    None,
    Text,
    Tel,
    Url,
    Email,
    Numeric,
    Decimal,
    Search,
}
