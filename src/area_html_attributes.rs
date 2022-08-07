use strum::Display;
use url::Url;

use crate::{Attribute, HtmlAttributeAnchorTarget, HtmlAttributeReferrerPolicy};

/// An enum defining the different area-element-specific attribute keys. Each variant takes either tuple
/// that represents the valid values for the attributes or nothing to represent a boolean
/// attribute.
#[derive(Debug, Display)]
#[strum(serialize_all = "lowercase")]
pub enum AreaHtmlAttributes<'a> {
    Alt(&'a str),
    /// Coords takes a shape, just like [AreaHtmlAttributes::Shape]. To ensure that your coords and your
    /// shape are consistent, consider passing the same variable when setting these attributes.
    ///
    // TODO: add an example of this.
    Coords(&'a Shape),
    Download(Option<&'a str>),
    Href(Url),
    HrefLang(&'a str), // TODO: make an enum defining all the values of HrefLang
    Media(&'a str),
    ReferrerPolicy(&'a HtmlAttributeReferrerPolicy),
    Rel(&'a Vec<AreaTagRel>),
    /// Shape takes a [Shape], just like [AreaHtmlAttributes::Coords]. To ensure that your coords and your
    /// shape are consistent, consider passing the same variable when setting these attributes.
    Shape(&'a Shape),
    Target(&'a HtmlAttributeAnchorTarget),
}

impl<'a> Attribute for AreaHtmlAttributes<'a> {
    fn get_key(&self) -> String {
        self.to_string()
    }

    fn get_val(&self) -> Option<String> {
        match self {
            AreaHtmlAttributes::Alt(val) => Some(val.to_string()),
            AreaHtmlAttributes::Coords(val) => match val {
                Shape::Rect(rect) => Some(rect.to_string()),
                Shape::Circle(circle) => Some(circle.to_string()),
                Shape::Poly(poly) => Some(poly.to_string()),
            },
            AreaHtmlAttributes::Download(val) => val.map(|val| val.to_string()),
            AreaHtmlAttributes::Href(val) => Some(val.to_string()),
            AreaHtmlAttributes::HrefLang(val) => Some(val.to_string()),
            AreaHtmlAttributes::Media(val) => Some(val.to_string()),
            AreaHtmlAttributes::ReferrerPolicy(val) => Some(val.to_string()),
            AreaHtmlAttributes::Rel(val) => Some(
                val.iter()
                    .map(|url| url.to_string())
                    .collect::<Vec<String>>()
                    .join(" "),
            ),
            AreaHtmlAttributes::Shape(val) => Some(val.to_string()),
            AreaHtmlAttributes::Target(val) => Some(val.to_string()),
        }
    }
}

/// An enum representing the different options for the both the shape attribute of
/// an area tag as well as the coordinates.
///
/// Shape tag: <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area#attr-shape>
///
/// Coord tag: <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area#attr-coords>
#[derive(Debug, Display)]
#[strum(serialize_all = "lowercase")]
pub enum Shape {
    Rect(Rect),
    Circle(Circle),
    Poly(Poly),
}

/// Represents a rectangle shape for an html area tag.
#[derive(Debug)]
pub struct Rect {
    top_right: Coord,
    bottom_left: Coord,
}

impl Rect {
    pub fn new(top_right: Coord, bottom_left: Coord) -> Self {
        Self {
            top_right,
            bottom_left,
        }
    }
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{}",
            self.top_right.to_string(),
            self.bottom_left.to_string()
        )
    }
}

/// Represents a circle for an html area tag.
#[derive(Debug)]
pub struct Circle {
    center: Coord,
    radius: u16,
}

impl Circle {
    pub fn new(center: Coord, radius: u16) -> Self {
        Self { center, radius }
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.center.to_string(), self.radius)
    }
}

/// Represents a polygon for an html area tag.
#[derive(Debug)]
pub struct Poly(Vec<Coord>);

impl Poly {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add_cord(&mut self, coord: Coord) {
        self.0.push(coord);
    }
}

impl From<Vec<Coord>> for Poly {
    fn from(vec: Vec<Coord>) -> Self {
        Self(vec)
    }
}

impl std::fmt::Display for Poly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|coord| coord.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
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
#[derive(Debug)]
pub struct Coord {
    x: u16,
    y: u16,
}

impl Coord {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

/// An enum defining the options for the rel attribute of an area tag.
///
/// <https://developer.mozilla.org/en-US/docs/Web/HTML/Link_types>
#[derive(Debug, Display)]
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
