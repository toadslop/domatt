use strum::AsRefStr;
use url::Url;

use crate::{Attribute, HtmlAttributeAnchorTarget, HtmlAttributeReferrerPolicy};

/// An enum defining the different area-element-specific attribute keys. Each variant takes either tuple
/// that represents the valid values for the attributes or nothing to represent a boolean
/// attribute.
#[derive(Debug, AsRefStr)]
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

impl<'a> Attribute<'a> for AreaHtmlAttributes<'a> {
    fn get_key(&self) -> &str {
        self.as_ref()
    }

    fn get_val(&self) -> Option<&str> {
        match self {
            AreaHtmlAttributes::Alt(val) => Some(val),
            AreaHtmlAttributes::Coords(val) => match val {
                Shape::Rect(rect) => Some(rect.into()),
                Shape::Circle(circle) => Some(circle.into()),
                Shape::Poly(poly) => Some(poly.into()),
            },
            AreaHtmlAttributes::Download(val) => val.map(|val| val.as_ref()),
            AreaHtmlAttributes::Href(val) => Some(val.as_ref()),
            AreaHtmlAttributes::HrefLang(val) => Some(val),
            AreaHtmlAttributes::Media(val) => Some(val),
            AreaHtmlAttributes::ReferrerPolicy(val) => Some(val.as_ref()),
            AreaHtmlAttributes::Rel(val) => Some(
                val.iter()
                    .map(|url| url.as_ref())
                    .collect::<Vec<&str>>()
                    .join(" ")
                    .as_ref(),
            ),
            AreaHtmlAttributes::Shape(val) => Some(val.as_ref()),
            AreaHtmlAttributes::Target(val) => Some(val.as_ref()),
        }
    }
}

/// An enum representing the different options for the both the shape attribute of
/// an area tag as well as the coordinates.
///
/// Shape tag: <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area#attr-shape>
///
/// Coord tag: <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/area#attr-coords>
#[derive(Debug, AsRefStr)]
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

// impl From<&str> for Rect {
//     fn from(input: &str) -> Self {
//         use std::process;
//         let split: Vec<&str> = input.split(",").collect();
//         if split.len() != 4 {
//             process::abort()
//         }
//         Self {
//             top_right: Coord {
//                 x: split[0].parse().unwrap(),
//                 y: split[1].parse().unwrap(),
//             },
//             bottom_left: Coord {
//                 x: split[2].parse().unwrap(),
//                 y: split[3].parse().unwrap(),
//             },
//         }
//     }
// }

impl From<&Rect> for &str {
    fn from(input: &Rect) -> Self {
        let x1 = input.top_right.x.to_string();
        let y1 = input.top_right.y.to_string();
        let x2 = input.bottom_left.x.to_string();
        let y2 = input.bottom_left.y.to_string();
        format!("{},{},{},{}", x1, y1, x2, y2).as_str()
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

// impl From<&str> for Circle {
//     fn from(input: &str) -> Self {
//         use std::process;
//         let split: Vec<&str> = input.split(",").collect();
//         if split.len > 2 || split.len() < 2 {
//             process::abort()
//         }
//         Self {
//             center: split[0],
//             radius: split[1],
//         }
//     }
// }

impl From<&Circle> for &str {
    fn from(input: &Circle) -> Self {
        let x = input.center.x.to_string();
        let y = input.center.y.to_string();
        let radius = input.radius.to_string();
        format!("{},{},{}", x, y, radius).as_str()
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

impl From<&Poly> for &str {
    fn from(input: &Poly) -> Self {
        let string: String = "".into();

        for (index, coord) in input.0.iter().enumerate() {
            if index != 0 || index != input.0.len() {
                string.push_str(",")
            }
            string.push_str(coord.to_string().as_str())
        }
        string.as_str()
    }
}

// impl From<&str> for Poly {
//     fn from(input: &str) -> Self {
//         use std::process;
//         let split: Vec<&str> = input.split(",").collect();
//         if !split.len % 2 == 0 {
//             process::abort()
//         }
//         let mut poly = Self::new();

//         for x in 0..split.len() {
//             if x % 2 == 0 {
//                 continue;
//             }
//             let coord = Coord::new(split[x - 1], split[x]);
//             poly.add_cord(coord);
//         }

//         poly
//     }
// }

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
