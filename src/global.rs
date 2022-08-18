use crate::{
    anchor::AnchorAttribute, area::AreaAttribute, audio::AudioAttribute, base::BaseAttribute,
    blockquote::BlockQuoteAttribute, button::ButtonAttribute, canvas::CanvasAttribute,
    col::ColAttribute, colgroup::ColGroupAttribute, data::DataAttribute, details::DetailsAttribute,
    svg::SvgAttribute, Attribute,
};
use std::fmt::Display;
use strum::AsRefStr;
use url::Url;

pub trait GlobalAttribute: Attribute {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/accesskey>
#[derive(Debug, Attribute)]
#[attribute("lowercase", Vec<char>)]
pub struct AccessKey(String);
impl GlobalAttribute for AccessKey {}
crate::add_impls!(AccessKey);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/autocapitalize>
#[derive(Debug, Attribute)]
#[attribute("lowercase", AutoCapitalizeOptions)]
pub struct AutoCapitalize(AutoCapitalizeOptions);
impl GlobalAttribute for AutoCapitalize {}
crate::add_impls!(AutoCapitalize);

#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum AutoCapitalizeOptions {
    Off,
    None,
    On,
    Sentences,
    Words,
    Characters,
}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/autofocus>
#[derive(Debug, Attribute)]
#[attribute("lowercase")]
pub struct Autofocus;
impl GlobalAttribute for Autofocus {}
crate::add_impls!(Autofocus);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/contenteditable>
#[derive(Debug, Attribute)]
#[attribute("lowercase", ContentEditableOptions)]
pub struct ContentEditable(ContentEditableOptions);
impl GlobalAttribute for ContentEditable {}
crate::add_impls!(ContentEditable);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/data-*>
#[derive(Debug)]
pub struct Data {
    key: String,
    value: String,
}

impl Data {
    const KEY_BASE: &'static str = "data-";
    pub fn new<T: Display>(suffix: String, value: T) -> Self {
        let mut key = String::with_capacity(suffix.len() + Self::KEY_BASE.len());
        key.push_str(Self::KEY_BASE);
        key.push_str(&suffix);
        Self {
            key,
            value: value.to_string(),
        }
    }
}

impl Attribute for Data {
    fn get_key(&self) -> &str {
        &self.key
    }

    fn get_val(&self) -> Option<&str> {
        Some(self.value.as_str())
    }
}

impl GlobalAttribute for Data {}
crate::add_impls!(Data);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/dir>
#[derive(Debug, Attribute)]
#[attribute("lowercase", DirOptions)]
pub struct Dir(DirOptions);
impl GlobalAttribute for Dir {}
crate::add_impls!(Dir);

#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum DirOptions {
    Ltr,
    Rtl,
    Auto,
}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/draggable>
#[derive(Debug, Attribute)]
#[attribute("lowercase", bool)]
pub struct Draggable(String);
impl GlobalAttribute for Draggable {}
crate::add_impls!(Draggable);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/enterkeyhint>
#[derive(Debug, Attribute)]
#[attribute("lowercase", EnterKeyHintOption)]
pub struct EnterKeyHint(EnterKeyHintOption);
impl GlobalAttribute for EnterKeyHint {}
crate::add_impls!(EnterKeyHint);

#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum EnterKeyHintOption {
    Enter,
    Done,
    Go,
    Next,
    Previous,
    Search,
    Send,
}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/hidden>
#[derive(Debug, Attribute)]
#[attribute("lowercase")]
pub struct Hidden;
impl GlobalAttribute for Hidden {}
crate::add_impls!(Hidden);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/id>
#[derive(Debug, Attribute)]
#[attribute("lowercase", String)]
pub struct Id(String);
impl GlobalAttribute for Id {}
crate::add_impls!(Id);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/inputmode>
#[derive(Debug, Attribute)]
#[attribute("lowercase", InputModeOption)]
pub struct InputMode(InputModeOption);
impl GlobalAttribute for InputMode {}
crate::add_impls!(InputMode);

#[derive(Debug, AsRefStr, Default)]
#[strum(serialize_all = "lowercase")]
pub enum InputModeOption {
    None,
    #[default]
    Text,
    Decimal,
    Numeric,
    Tel,
    Search,
    Email,
    Url,
}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/is>
#[derive(Debug, Attribute)]
#[attribute("lowercase", String)]
pub struct Is(String);
impl GlobalAttribute for Is {}
crate::add_impls!(Is);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/itemid>
#[derive(Debug, Attribute)]
#[attribute("lowercase", String)]
pub struct ItemId(String);
impl GlobalAttribute for ItemId {}
crate::add_impls!(ItemId);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/itemprop>
#[derive(Debug, Attribute)]
#[attribute("lowercase", String)]
pub struct ItemProp(String);
impl GlobalAttribute for ItemProp {}
crate::add_impls!(ItemProp);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/itemref>
#[derive(Debug, Attribute)]
#[attribute("lowercase", Vec<String>)]
pub struct ItemRef(String);
impl GlobalAttribute for ItemRef {}
crate::add_impls!(ItemRef);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/itemscope>
#[derive(Debug, Attribute)]
#[attribute("lowercase")]
pub struct ItemScope;
impl GlobalAttribute for ItemScope {}
crate::add_impls!(ItemScope);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/itemtype>
#[derive(Debug, Attribute)]
#[attribute("lowercase", Url)]
pub struct ItemType(String);
impl GlobalAttribute for ItemType {}
crate::add_impls!(ItemType);

// TODO: make a struct to help with making language tags
/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/lang>
#[derive(Debug, Attribute)]
#[attribute("lowercase", String)]
pub struct Lang(String);
impl GlobalAttribute for Lang {}
crate::add_impls!(Lang);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/nonce>
#[derive(Debug, Attribute)]
#[attribute("lowercase", String)]
pub struct Nonce(String);
impl GlobalAttribute for Nonce {}
crate::add_impls!(Nonce);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/part>
#[derive(Debug, Attribute)]
#[attribute("lowercase", Vec<String>)]
pub struct Part(String);
impl GlobalAttribute for Part {}
crate::add_impls!(Part);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/slot>
#[derive(Debug, Attribute)]
#[attribute("lowercase", String)]
pub struct Slot(String);
impl GlobalAttribute for Slot {}
crate::add_impls!(Slot);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/spellcheck>
#[derive(Debug, Attribute)]
#[attribute("lowercase", bool)]
pub struct Spellcheck(String);

impl GlobalAttribute for Spellcheck {}
crate::add_impls!(Spellcheck);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/tabindex>
#[derive(Debug, Attribute)]
#[attribute("lowercase", i16)]
pub struct TabIndex(String);
impl GlobalAttribute for TabIndex {}
crate::add_impls!(TabIndex);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/title>
#[derive(Debug, Attribute)]
#[attribute("lowercase", String)]
pub struct Title(String);
impl GlobalAttribute for Title {}
crate::add_impls!(Title);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/translate>
#[derive(Debug, Attribute)]
#[attribute("lowercase", TranslateOption)]
pub struct Translate(TranslateOption);
impl GlobalAttribute for Translate {}
crate::add_impls!(Translate);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/role>
#[derive(Debug, Attribute)]
#[attribute("lowercase", AriaRole)]
pub struct Role(AriaRole);
impl GlobalAttribute for Role {}
crate::add_impls!(Role);

/// An enum representing the different options for the `aria-role` attribute.
///
///  <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles>
#[derive(Debug, AsRefStr)]
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

/// An enum representing the different options for the `aria-role` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/contenteditable>
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum ContentEditableOptions {
    True,
    False,
    Inherit,
}

/// An enum representing the different options for the `translate` attribute.
///
/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/translate>
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum TranslateOption {
    Yes,
    No,
}
