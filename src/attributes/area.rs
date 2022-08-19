use strum::AsRefStr;

pub trait AreaAttribute: Attribute {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area#attr-alt>
#[derive(Debug, Attribute)]
#[attribute("lowercase", String)]
pub struct Alt(String);
impl AreaAttribute for Alt {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area#attr-alt>
#[derive(Debug, Attribute)]
#[attribute("lowercase", AreaTagShape)]
pub struct Coords(AreaTagShape);
impl AreaAttribute for Coords {}

pub use super::anchor::Download;
/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area#attr-download>
impl AreaAttribute for Download {}

pub use super::anchor::Href;
impl AreaAttribute for Href {}

pub use super::anchor::HrefLang;
impl AreaAttribute for HrefLang {}

pub use super::anchor::ReferrerPolicy;

use super::Attribute;
impl AreaAttribute for ReferrerPolicy {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area#attr-rel>
#[derive(Debug, Attribute)]
#[attribute("lowercase", Vec<AreaTagRel>)]
pub struct Rel(String);
impl AreaAttribute for Rel {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area#attr-rel>
#[derive(Debug, Attribute)]
#[attribute("lowercase", AreaTagShape)]
pub struct Shape(AreaTagShape);
impl AreaAttribute for Shape {}

pub use super::anchor::Target;
impl AreaAttribute for Target {}

/// An enum representing the different options for the both the shape attribute of
/// an area tag as well as the coordinates.
///
/// Shape tag: <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area#attr-shape>
///
/// Coord tag: <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area#attr-coords>
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum AreaTagShape {
    Rect(Rect),
    Circle(Circle),
    Poly(Poly),
}

/// Represents a rectangle shape for an html area tag.
#[derive(Debug)]
pub struct Rect {
    rect: String,
}

impl Rect {
    pub fn new(top_right: Coord, bottom_left: Coord) -> Self {
        let top_right = top_right.as_str();
        let bottom_left = bottom_left.as_str();
        let mut rect = String::with_capacity(top_right.len() + bottom_left.len() + 1);
        rect.push_str(top_right);
        rect.push_str(",");
        rect.push_str(bottom_left);
        Self { rect }
    }

    pub fn as_str(&self) -> &str {
        self.rect.as_str()
    }
}

/// Represents a circle for an html area tag.
#[derive(Debug)]
pub struct Circle {
    circle: String,
}

impl Circle {
    pub fn new(center: Coord, radius: u16) -> Self {
        let center = center.as_str();
        let radius = radius.to_string();
        let mut circle = String::with_capacity(center.len() + radius.len() + 1);
        circle.push_str(center);
        circle.push_str(",");
        circle.push_str(&radius);
        Self { circle }
    }

    pub fn as_str(&self) -> &str {
        self.circle.as_str()
    }
}

/// Represents a polygon for an html area tag.
#[derive(Debug)]
pub struct Poly {
    poly: String,
}

impl Poly {
    pub fn new() -> Self {
        Self {
            poly: String::new(),
        }
    }

    pub fn add_cord(&mut self, coord: Coord) {
        self.poly.push_str(coord.as_str());
    }

    pub fn as_str(&self) -> &str {
        self.poly.as_str()
    }
}

impl From<Vec<Coord>> for Poly {
    fn from(vec: Vec<Coord>) -> Self {
        let poly = vec
            .iter()
            .map(|coord| coord.as_str())
            .collect::<Vec<&str>>()
            .join(",");
        Self { poly }
    }
}

/// Represents a single coordinate, used for modelling the coordinates of an html
/// area tag.
///
/// See the following:
///
/// * [Poly]
/// * [Rect]
/// * [Circle]
#[derive(Debug, Clone)]
pub struct Coord {
    coord: String,
}

impl Coord {
    pub fn new(x: u16, y: u16) -> Self {
        let x = x.to_string();
        let y = y.to_string();
        let mut coord = String::with_capacity(x.len() + y.len() + 1);
        coord.push_str(&x);
        coord.push_str(",");
        coord.push_str(&y);
        Self { coord }
    }

    pub fn as_str(&self) -> &str {
        &self.coord
    }
}

/// An enum defining the options for the rel attribute of an area tag.
///
/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Link_types>
#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum AreaTagRel {
    Alternate,
    Archives,
    Author,
    Bookmark,
    External,
    First,
    Help,
    Index,
    Last,
    License,
    Next,
    Nofollow,
    Noopener,
    Noreferrer,
    Opener,
    Prev,
    Search,
    Sidebar,
    Tag,
    Up,
}
