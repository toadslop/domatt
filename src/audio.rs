use crate::Attribute;
use strum::AsRefStr;
use url::Url;

pub trait AudioAttribute: Attribute {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/audio#attr-autoplay>
#[derive(Debug)]
pub struct Autoplay;

impl Attribute for Autoplay {
    fn get_val(&self) -> Option<&str> {
        None
    }

    fn get_key(&self) -> &str {
        "autoplay"
    }

    type InputType = Self;

    fn new(value: Self::InputType) -> Self {
        Self
    }
}

impl AudioAttribute for Autoplay {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/audio#attr-controls>
#[derive(Debug)]
pub struct Controls;

impl Attribute for Controls {
    fn get_val(&self) -> Option<&str> {
        None
    }

    fn get_key(&self) -> &str {
        "controls"
    }

    type InputType = Self;

    fn new(value: Self::InputType) -> Self {
        Self
    }
}

impl AudioAttribute for Controls {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/audio#attr-crossorigin>
#[derive(Debug)]
pub struct CrossOrigin(AudioCrossOriginOptions);

impl Attribute for CrossOrigin {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "crossorigin"
    }

    type InputType = AudioCrossOriginOptions;

    fn new(value: Self::InputType) -> Self {
        Self(value)
    }
}

impl AudioAttribute for CrossOrigin {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/audio#attr-disableremoteplayback>
#[derive(Debug)]
pub struct DisableRemotePlayback;

impl Attribute for DisableRemotePlayback {
    fn get_val(&self) -> Option<&str> {
        None
    }

    fn get_key(&self) -> &str {
        "disableremoteplayback"
    }

    type InputType = Self;

    fn new(value: Self::InputType) -> Self {
        todo!()
    }
}

impl AudioAttribute for DisableRemotePlayback {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/audio#attr-loop>
#[derive(Debug)]
pub struct Loop;

impl Attribute for Loop {
    fn get_val(&self) -> Option<&str> {
        None
    }

    fn get_key(&self) -> &str {
        "loop"
    }

    type InputType = Self;

    fn new(value: Self::InputType) -> Self {
        Self
    }
}

impl AudioAttribute for Loop {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/audio#attr-muted>
#[derive(Debug)]
pub struct Muted;

impl Attribute for Muted {
    fn get_val(&self) -> Option<&str> {
        None
    }

    fn get_key(&self) -> &str {
        "muted"
    }

    type InputType = Self;

    fn new(value: Self::InputType) -> Self {
        Self
    }
}

impl AudioAttribute for Muted {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/audio#attr-preload>
#[derive(Debug)]
pub struct Preload(AudioPreloadOptions);

impl Attribute for Preload {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "preload"
    }

    type InputType = AudioPreloadOptions;

    fn new(value: Self::InputType) -> Self {
        Self(value)
    }
}

impl AudioAttribute for Preload {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#attr-src>
#[derive(Debug)]
pub struct Src(String);

impl Attribute for Src {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "src"
    }

    type InputType = Url;

    fn new(value: Self::InputType) -> Self {
        Self(value.to_string())
    }
}

#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum AudioCrossOriginOptions {
    Anonymous,
    UseCredentials,
}

#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum AudioPreloadOptions {
    None,
    Metadata,
    Auto,
}
