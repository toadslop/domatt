use crate::Attribute;
use strum::AsRefStr;
use url::Url;

pub trait AudioAttribute: Attribute {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/audio#attr-autoplay>
#[derive(Debug, Attribute)]
#[attribute("lowercase")]
pub struct Autoplay;
impl AudioAttribute for Autoplay {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/audio#attr-controls>
#[derive(Debug, Attribute)]
#[attribute("lowercase")]
pub struct Controls;
impl AudioAttribute for Controls {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/audio#attr-crossorigin>
#[derive(Debug, Attribute)]
#[attribute("lowercase", AudioCrossOriginOptions)]
pub struct CrossOrigin(AudioCrossOriginOptions);
impl AudioAttribute for CrossOrigin {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/audio#attr-disableremoteplayback>
#[derive(Debug, Attribute)]
#[attribute("lowercase")]
pub struct DisableRemotePlayback;
impl AudioAttribute for DisableRemotePlayback {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/audio#attr-loop>
#[derive(Debug, Attribute)]
#[attribute("lowercase")]
pub struct Loop;
impl AudioAttribute for Loop {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/audio#attr-muted>
#[derive(Debug, Attribute)]
#[attribute("lowercase")]
pub struct Muted;
impl AudioAttribute for Muted {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/audio#attr-preload>
#[derive(Debug, Attribute)]
#[attribute("lowercase", AudioPreloadOptions)]
pub struct Preload(AudioPreloadOptions);
impl AudioAttribute for Preload {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/a#attr-src>
#[derive(Debug, Attribute)]
#[attribute("lowercase", Url)]
pub struct Src(String);

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
