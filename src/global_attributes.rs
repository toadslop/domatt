use crate::{
    anchor_html_attributes::AnchorAttribute, area_html_attributes::AreaAttribute, Attribute,
    AudioAttribute, BaseAttribute, BlockQuoteAttribute, ButtonAttribute, CanvasAttribute,
    ColAttribute, ColGroupAttribute, DataAttribute, DetailsAttribute,
};
use std::fmt::Display;
use strum::AsRefStr;
use url::Url;

pub trait GlobalAttribute: Attribute {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/accesskey>
#[derive(Debug)]
pub struct AccessKey {
    val: String,
}

impl AccessKey {
    pub fn new(val: Vec<char>) -> Self {
        Self {
            val: val
                .iter()
                .map(char::to_string)
                .collect::<Vec<String>>()
                .join(" "),
        }
    }
}

impl Attribute for AccessKey {
    fn get_val(&self) -> Option<&str> {
        Some(&self.val)
    }

    fn get_key(&self) -> &str {
        "accesskey"
    }
}

impl GlobalAttribute for AccessKey {}
crate::add_impls!(AccessKey);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/autocapitalize>
#[derive(Debug)]
pub struct AutoCapitalize(AutoCapitalizeOptions);

impl Attribute for AutoCapitalize {
    fn get_val(&self) -> Option<&str> {
        Some(&self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "autocapitalize"
    }
}

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

impl GlobalAttribute for AutoCapitalize {}
crate::add_impls!(AutoCapitalize);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/autofocus>
#[derive(Debug)]
pub struct Autofocus;

impl Attribute for Autofocus {
    fn get_val(&self) -> Option<&str> {
        None
    }

    fn get_key(&self) -> &str {
        "autofocus"
    }
}

impl GlobalAttribute for Autofocus {}
crate::add_impls!(Autofocus);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/contenteditable>
#[derive(Debug)]
pub struct ContentEditable(ContentEditableOptions);

impl Attribute for ContentEditable {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "contenteditable"
    }
}

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
#[derive(Debug)]
pub struct Dir(DirOptions);

#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum DirOptions {
    Ltr,
    Rtl,
    Auto,
}

impl Attribute for Dir {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "dir"
    }
}
impl GlobalAttribute for Dir {}
crate::add_impls!(Dir);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/draggable>
#[derive(Debug)]
pub struct Draggable {
    val: String,
}

impl Draggable {
    pub fn new(val: bool) -> Self {
        Self {
            val: val.to_string(),
        }
    }
}

impl Attribute for Draggable {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_str())
    }

    fn get_key(&self) -> &str {
        "draggable"
    }
}

impl GlobalAttribute for Draggable {}
crate::add_impls!(Draggable);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/enterkeyhint>
#[derive(Debug)]
pub struct EnterKeyHint(EnterKeyHintOption);

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

impl Attribute for EnterKeyHint {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "enterkeyhint"
    }
}

impl GlobalAttribute for EnterKeyHint {}
crate::add_impls!(EnterKeyHint);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/hidden>
#[derive(Debug)]
pub struct Hidden;

impl Attribute for Hidden {
    fn get_val(&self) -> Option<&str> {
        None
    }

    fn get_key(&self) -> &str {
        "hidden"
    }
}

impl GlobalAttribute for Hidden {}
crate::add_impls!(Hidden);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/id>
#[derive(Debug)]
pub struct Id(String);

impl Attribute for Id {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "id"
    }
}

impl GlobalAttribute for Id {}
crate::add_impls!(Id);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/inputmode>
#[derive(Debug)]
pub struct InputMode(EnterKeyHintOption);

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

impl Attribute for InputMode {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "inputmode"
    }
}

impl GlobalAttribute for InputMode {}
crate::add_impls!(InputMode);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/is>
#[derive(Debug)]
pub struct Is(String);

impl Attribute for Is {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "is"
    }
}

impl GlobalAttribute for Is {}
crate::add_impls!(Is);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/itemid>
#[derive(Debug)]
pub struct ItemId(String);

impl Attribute for ItemId {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "itemid"
    }
}

impl GlobalAttribute for ItemId {}
crate::add_impls!(ItemId);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/itemprop>
#[derive(Debug)]
pub struct ItemProp(String);

impl Attribute for ItemProp {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "itemprop"
    }
}

impl GlobalAttribute for ItemProp {}
crate::add_impls!(ItemProp);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/itemref>
#[derive(Debug)]
pub struct ItemRef {
    val: String,
}

impl ItemRef {
    pub fn new(val: Vec<char>) -> Self {
        Self {
            val: val
                .iter()
                .map(char::to_string)
                .collect::<Vec<String>>()
                .join(" "),
        }
    }
}

impl Attribute for ItemRef {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_str())
    }

    fn get_key(&self) -> &str {
        "itemref"
    }
}

impl GlobalAttribute for ItemRef {}
crate::add_impls!(ItemRef);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/itemscope>
#[derive(Debug)]
pub struct ItemScope;

impl Attribute for ItemScope {
    fn get_val(&self) -> Option<&str> {
        None
    }

    fn get_key(&self) -> &str {
        "itemscope"
    }
}

impl GlobalAttribute for ItemScope {}
crate::add_impls!(ItemScope);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/itemtype>
#[derive(Debug)]
pub struct ItemType(Url);

impl Attribute for ItemType {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "itemtype"
    }
}

impl GlobalAttribute for ItemType {}
crate::add_impls!(ItemType);

// TODO: make a struct to help with making language tags
/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/lang>
#[derive(Debug)]
pub struct Lang(String);

impl Attribute for Lang {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "lang"
    }
}

impl GlobalAttribute for Lang {}
crate::add_impls!(Lang);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/nonce>
#[derive(Debug)]
pub struct Nonce(String);

impl Attribute for Nonce {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "nonce"
    }
}

impl GlobalAttribute for Nonce {}
crate::add_impls!(Nonce);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/part>
#[derive(Debug)]
pub struct Part {
    val: String,
}

impl Part {
    pub fn new(val: Vec<char>) -> Self {
        Self {
            val: val
                .iter()
                .map(char::to_string)
                .collect::<Vec<String>>()
                .join(" "),
        }
    }
}

impl Attribute for Part {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_str())
    }

    fn get_key(&self) -> &str {
        "part"
    }
}

impl GlobalAttribute for Part {}
crate::add_impls!(Part);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/slot>
#[derive(Debug)]
pub struct Slot(String);

impl Attribute for Slot {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "slot"
    }
}

impl GlobalAttribute for Slot {}
crate::add_impls!(Slot);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/spellcheck>
#[derive(Debug)]
pub struct Spellcheck {
    val: String,
}

impl Spellcheck {
    pub fn new(val: bool) -> Self {
        Self {
            val: val.to_string(),
        }
    }
}

impl Attribute for Spellcheck {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_str())
    }

    fn get_key(&self) -> &str {
        "spellcheck"
    }
}

impl GlobalAttribute for Spellcheck {}
crate::add_impls!(Spellcheck);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/tabindex>
#[derive(Debug)]
pub struct TabIndex {
    val: String,
}

impl TabIndex {
    pub fn new(val: i16) -> Self {
        Self {
            val: val.to_string(),
        }
    }
}

impl Attribute for TabIndex {
    fn get_val(&self) -> Option<&str> {
        Some(self.val.as_str())
    }

    fn get_key(&self) -> &str {
        "tabindex"
    }
}

impl GlobalAttribute for TabIndex {}
crate::add_impls!(TabIndex);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/title>
#[derive(Debug)]
pub struct Title(String);

impl Attribute for Title {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "title"
    }
}

impl GlobalAttribute for Title {}
crate::add_impls!(Title);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/translate>
#[derive(Debug)]
pub struct Translate(TranslateOption);

impl Attribute for Translate {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "translate"
    }
}

impl GlobalAttribute for Translate {}
crate::add_impls!(Translate);

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/role>
#[derive(Debug)]
pub struct Role(AriaRole);

impl Attribute for Role {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "role"
    }
}

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
