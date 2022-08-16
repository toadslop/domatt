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
}

impl AudioAttribute for Preload {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#attr-src>
#[derive(Debug)]
pub struct Src(Url);

impl Attribute for Src {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "src"
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
