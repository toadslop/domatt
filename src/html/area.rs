use crate::html::anchor::Target;
use crate::Attribute;
use strum::AsRefStr;

pub trait AreaAttribute: Attribute {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area#attr-alt>
#[derive(Debug, Clone, PartialEq)]
pub struct Alt(String);

impl Attribute for Alt {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn get_key(&self) -> &str {
        "alt"
    }
}

impl AreaAttribute for Alt {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area#attr-alt>
#[derive(Debug, Clone, PartialEq)]
pub struct Coords(AreaTagShape);

impl Attribute for Coords {
    fn get_val(&self) -> Option<&str> {
        match &self.0 {
            AreaTagShape::Rect(rect) => Some(rect.as_str()),
            AreaTagShape::Circle(circle) => Some(circle.as_str()),
            AreaTagShape::Poly(poly) => Some(poly.as_str()),
        }
    }

    fn get_key(&self) -> &str {
        "coords"
    }
}

impl AreaAttribute for Coords {}

pub use crate::html::anchor::Download;
/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area#attr-download>
impl AreaAttribute for Download {}

pub use crate::html::anchor::Href;
impl AreaAttribute for Href {}

pub use crate::html::anchor::HrefLang;
impl AreaAttribute for HrefLang {}

pub use crate::html::anchor::ReferrerPolicy;

impl AreaAttribute for ReferrerPolicy {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area#attr-rel>
#[derive(Debug, Clone, PartialEq)]
pub struct Rel {
    value: String,
}

impl Rel {
    pub fn new(rels: Vec<AreaTagRel>) -> Self {
        Self {
            value: rels
                .iter()
                .map(AreaTagRel::as_ref)
                .collect::<Vec<&str>>()
                .join(" "),
        }
    }
}

impl Attribute for Rel {
    fn get_val(&self) -> Option<&str> {
        Some(self.value.as_str())
    }

    fn get_key(&self) -> &str {
        "rel"
    }
}

impl AreaAttribute for Rel {}

/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area#attr-rel>
#[derive(Debug, Clone, PartialEq)]
pub struct Shape(AreaTagShape);

impl Attribute for Shape {
    fn get_val(&self) -> Option<&str> {
        Some(self.0.as_ref())
    }

    fn get_key(&self) -> &str {
        "shape"
    }
}

impl AreaAttribute for Shape {}
impl AreaAttribute for Target {}

/// An enum representing the different options for the both the shape attribute of
/// an area tag as well as the coordinates.
///
/// Shape tag: <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area#attr-shape>
///
/// Coord tag: <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area#attr-coords>
#[derive(Debug, AsRefStr, Clone, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum AreaTagShape {
    Rect(Rect),
    Circle(Circle),
    Poly(Poly),
}

/// Represents a rectangle shape for an html area tag.
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
