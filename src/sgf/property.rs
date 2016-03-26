use std::str::FromStr;
use std::collections::HashMap;
use std::str;
use std::result;

use super::{Result, Error};

use ::stone::Stone;

#[derive(Debug, Clone, Hash)]
pub struct Property {
    id: String,
    raw: Vec<Vec<u8>>,
}

impl Property {
    pub fn new(id: String, v: Vec<Vec<u8>>) -> Property {
        Property {
            id: id,
            raw: v,
        }
    }

    pub fn id(&self) -> &str { &self.id }

    pub fn description(&self) -> Option<&'static str> {
        DETAILS.get(&self.id[..]).map(|d| d.1)
    }

    pub fn ptype(&self) -> Option<Type> {
        DETAILS.get(&self.id[..]).map(|d| d.2)
    }

    pub fn inherit(&self) -> Option<bool> {
        DETAILS.get(&self.id[..]).map(|d| d.3)
    }

    pub fn values(&self) -> Result<Vec<Value>> {
        if let Some(parse) = DETAILS.get(&self.id[..]).map(|d| d.4) {
            fold_res(self.raw.iter().map(|r| parse(r)))
        } else {
            Ok(self.raw.iter().map(|r| Value::from(r.clone())).collect())
        }
    }

    pub fn value(&self) -> Result<Value> {
        let v = try!(self.values());
        match v.into_iter().next() {
            None => Err(Error::ValueError),
            Some(v) => Ok(v),
        }
    }

    pub fn len(&self) -> usize { self.raw.len() }
}

fn fold_res<R, T, E>(vr: R) -> result::Result<Vec<T>, E>
    where R: IntoIterator<Item=result::Result<T,E>>
{
    vr.into_iter().fold(Ok(vec![]), |s, r| {
        match (s, r) {
            (Ok(mut v), Ok(r)) => { v.push(r); Ok(v) },
            (Ok(_), Err(e)) | (Err(e), _) => Err(e),
        }
    })
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Type {
    Move,
    Setup,
    Root,
    GameInfo,
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(Number),
    Real(Real),
    Double(Double),
    Color(Color),
    SimpleText(SimpleText),
    Text(Text),

    GoMove(go::Move),

    Compose(Box<Value>, Box<Value>),
    Raw(Vec<u8>),
}

impl Value {
    pub fn number(&self) -> Option<&Number> {
        if let &Value::Number(ref n) = self { Some(n) } else { None }
    }

    pub fn real(&self) -> Option<&Real> {
        if let &Value::Real(ref n) = self { Some(n) } else { None }
    }

    pub fn double(&self) -> Option<&Double> {
        if let &Value::Double(ref n) = self { Some(n) } else { None }
    }

    pub fn color(&self) -> Option<&Color> {
        if let &Value::Color(ref n) = self { Some(n) } else { None }
    }

    pub fn simpletext(&self) -> Option<&SimpleText> {
        if let &Value::SimpleText(ref n) = self { Some(n) } else { None }
    }

    pub fn text(&self) -> Option<&Text> {
        if let &Value::Text(ref n) = self { Some(n) } else { None }
    }

    pub fn gomove(&self) -> Option<&go::Move> {
        if let &Value::GoMove(ref n) = self { Some(n) } else { None }
    }
}

impl From<Number> for Value {
    fn from(v: Number) -> Value { Value::Number(v) }
}

impl From<Real> for Value {
    fn from(v: Real) -> Self { Value::Real(v) }
}

impl From<Double> for Value {
    fn from(v: Double) -> Self { Value::Double(v) }
}

impl From<Color> for Value {
    fn from(v: Color) -> Self { Value::Color(v) }
}

impl From<SimpleText> for Value {
    fn from(v: SimpleText) -> Self { Value::SimpleText(v) }
}

impl From<Text> for Value {
    fn from(v: Text) -> Self { Value::Text(v) }
}

impl From<go::Move> for Value {
    fn from(v: go::Move) -> Self { Value::GoMove(v) }
}

impl<L, R> From<Compose<L, R>> for Value
    where L: Into<Value>, R: Into<Value>
{
    fn from(Compose(l, r): Compose<L, R>) -> Self {
        Value::Compose(Box::new((*l).into()), Box::new((*r).into()))
    }
}

impl From<Vec<u8>> for Value {
    fn from(v: Vec<u8>) -> Self { Value::Raw(v) }
}

pub trait ValueParse: Sized {
    fn parse(raw: &[u8]) -> Result<Value>;
}

struct Nil;
impl ValueParse for Nil {
    fn parse(_: &[u8]) -> Result<Value> { Err(Error::ValueError) }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Number(u32);

impl ValueParse for Number {
    fn parse(raw: &[u8]) -> Result<Value> {
        match u32::from_str(str::from_utf8(raw).unwrap()) {
            Ok(v) => Ok(Value::from(Number(v))),
            Err(_) => Err(Error::ValueError),
        }
    }
}

impl<'a> Into<u32> for &'a Number {
    fn into(self) -> u32 { self.0 }
}

impl<'a> Into<usize> for &'a Number {
    fn into(self) -> usize { self.0 as usize }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Real(f32);

impl ValueParse for Real {
    fn parse(raw: &[u8]) -> Result<Value> {
        match f32::from_str(str::from_utf8(raw).unwrap()) {
            Ok(v) => Ok(Value::from(Real(v))),
            Err(_) => Err(Error::ValueError),
        }
    }
}

impl<'a> Into<f32> for &'a Real {
    fn into(self) -> f32 { self.0 }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Double {
    One,
    Two,
}

impl ValueParse for Double {
    fn parse(raw: &[u8]) -> Result<Value> {
        match raw {
            b"1" => Ok(Value::from(Double::One)),
            b"2" => Ok(Value::from(Double::Two)),
            _ => Err(Error::ValueError),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Color {
    Black,
    White,
}

impl ValueParse for Color {
    fn parse(raw: &[u8]) -> Result<Value> {
        match raw {
            b"w" | b"W" => Ok(Value::from(Color::White)),
            b"b" | b"B" => Ok(Value::from(Color::Black)),
            _ => Err(Error::ValueError),
        }
    }
}

impl<'a> Into<Stone> for &'a Color {
    fn into(self) -> Stone {
        match self {
            &Color::Black => Stone::Black,
            &Color::White => Stone::White,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SimpleText(String);

impl ValueParse for SimpleText {
    fn parse(raw: &[u8]) -> Result<Value> {
        let mut s = String::new();
        for c in raw {
            match *c as char {
                '\\' => (),
                '\n' | '\t' | '\r' => s.push(' '),
                c => s.push(c),
            }
        }

        Ok(Value::from(SimpleText(s)))
    }
}

impl<'a> Into<&'a str> for &'a SimpleText {
    fn into(self) -> &'a str { self.0.as_ref() }
}

impl<'a> Into<String> for &'a SimpleText {
    fn into(self) -> String { self.0.clone() }
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Text(String);

impl ValueParse for Text {
    fn parse(raw: &[u8]) -> Result<Value> {
        let mut s = String::new();
        let mut quote = false;
        for c in raw {
            // "Following chars have to be escaped, when used in Text: "]", "\" and ":" (only if used in compose data type)."
            // How do we know if we're in a compose?
            match *c as char {
                '\\' if !quote => quote = true,
                '\n' if quote => { quote = false; s.push(' ') },
                '\t' | '\r' => s.push(' '),
                c => { quote = false; s.push(c) },
            }
        }

        Ok(Value::from(Text(s)))
    }
}

impl<'a> Into<&'a str> for &'a Text {
    fn into(self) -> &'a str { self.0.as_ref() }
}

impl<'a> Into<String> for &'a Text {
    fn into(self) -> String { self.0.clone() }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Compose<L, R>(Box<L>, Box<R>);

impl<L, R> ValueParse for Compose<L, R>
    where L: ValueParse, R: ValueParse
{
    fn parse(raw: &[u8]) -> Result<Value> {
        // In theory we should special-case : in Text/SimpleText, but that's hard
        if let Some(colon) = str::from_utf8(raw).ok().and_then(|s| s.find(':')) {
            let l = L::parse(&raw[..colon]);
            let r = R::parse(&raw[colon+1..]);
            match (l, r) {
                (Ok(l), Ok(r)) => Ok(Value::from(Compose(Box::new(l), Box::new(r)))),
                _ => Err(Error::ValueError),
            }
        } else {
            Err(Error::ValueError)
        }
    }
}

pub mod go {
    use ::location::Location;

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct Point(usize, usize);
    pub type Move = Point;
    pub type Stone = Point;

    use super::{Value, ValueParse};
    use sgf::{Error, Result};

    impl ValueParse for Point {
        fn parse(raw: &[u8]) -> Result<Value> {
            if raw.len() == 2 {
                Ok(From::from(Point((raw[0] - ('a' as u8)) as usize,
                                    (raw[1] - ('a' as u8)) as usize)))
            } else {
                Err(Error::ValueError)
            }
        }
    }

    impl<'a> Into<Location> for &'a Point {
        fn into(self) -> Location { Location::new(self.0, self.1) }
    }
}

// id, description, type, inherit, value parser
struct Detail(&'static str, &'static str, Type, bool, fn (&[u8]) -> Result<Value>);

lazy_static!{
    static ref DETAILS: HashMap<&'static str, Detail> = {
        use self::Type::*;
        vec![
            Detail("AB", "Add Black", Setup, false, go::Stone::parse /* list of stone */),
            Detail("AE", "Add Empty", Setup, false, go::Point::parse /* list of point */),
            Detail("AN", "Annotation", GameInfo, false, SimpleText::parse /* simpletext */),
            Detail("AP", "Application", Root, false, Compose::<SimpleText, SimpleText>::parse /* composed simpletext ':' simpletext */),
            Detail("AR", "Arrow", None, false, Compose::<go::Point, go::Point>::parse /* list of composed point ':' point */),
            Detail("AS", "Who adds stones", None, false, SimpleText::parse /* simpletext */),
            Detail("AW", "Add White", Setup, false, go::Stone::parse /* list of stone */),
            Detail("B",  "Black", Move, false, go::Move::parse /* move */),
            Detail("BL", "Black time left", Move, false, Real::parse /* real */),
            Detail("BM", "Bad move", Move, false, Double::parse /* double */),
            Detail("BR", "Black rank", GameInfo, false, SimpleText::parse /* simpletext */),
            Detail("BT", "Black team", GameInfo, false, SimpleText::parse /* simpletext */),
            Detail("C",  "Comment", None, false, Text::parse /* text */),
            Detail("CA", "Charset", Root, false, SimpleText::parse /* simpletext */),
            Detail("CP", "Copyright", GameInfo, false, SimpleText::parse /* simpletext */),
            Detail("CR", "Circle", None, false, go::Point::parse /* list of point */),
            Detail("DD", "Dim points", None, true, go::Point::parse /* elist of point */),
            Detail("DM", "Even position", None, false, Double::parse /* double */),
            Detail("DO", "Doubtful", Move, false, Nil::parse /* none */),
            Detail("DT", "Date", GameInfo, false, SimpleText::parse /* simpletext */),
            Detail("EV", "Event", GameInfo, false, SimpleText::parse /* simpletext */),
            Detail("FF", "Fileformat", Root, false, Number::parse /* number (range: 1-4) */),
            Detail("FG", "Figure", None, false, Compose::<Number, SimpleText>::parse /* none | composed number ":" simpletext */),
            Detail("GB", "Good for Black", None, false, Double::parse /* double */),
            Detail("GC", "Game comment", GameInfo, false, Text::parse /* text */),
            Detail("GM", "Game", Root, false, Number::parse /* number (range: 1-5,7-16) */),
            Detail("GN", "Game name", GameInfo, false, SimpleText::parse /* simpletext */),
            Detail("GW", "Good for White", None, false, Double::parse /* double */),
            Detail("HA", "Handicap", GameInfo, false, Number::parse /* number */),
            Detail("HO", "Hotspot", None, false, Double::parse /* double */),
            Detail("IP", "Initial pos.", GameInfo, false, SimpleText::parse /* simpletext */),
            Detail("IT", "Interesting", Move, false, Nil::parse /* none */),
            Detail("IY", "Invert Y-axis", GameInfo, false, SimpleText::parse /* simpletext */),
            Detail("KM", "Komi", GameInfo, false, Real::parse /* real */),
            Detail("KO", "Ko", Move, false, Nil::parse /* none */),
            Detail("LB", "Label", None, false, Compose::<go::Point, SimpleText>::parse /* list of composed point ':' simpletext */),
            Detail("LN", "Line", None, false, Compose::<go::Point, go::Point>::parse /* list of composed point ':' point */),
            Detail("MA", "Mark", None, false, go::Point::parse /* list of point */),
            Detail("MN", "Set move number", Move, false, Number::parse /* number */),
            Detail("N",  "Nodename", None, false, SimpleText::parse /* simpletext */),
            Detail("OB", "OtStones Black", Move, false, Number::parse /* number */),
            Detail("ON", "Opening", GameInfo, false, SimpleText::parse /* simpletext */),
            Detail("OT", "Overtime", GameInfo, false, SimpleText::parse /* simpletext */),
            Detail("OW", "OtStones White", Move, false, Number::parse /* number */),
            Detail("PB", "Player Black", GameInfo, false, SimpleText::parse /* simpletext */),
            Detail("PC", "Place", GameInfo, false, SimpleText::parse /* simpletext */),
            Detail("PL", "Player to play", Setup, false, Color::parse /* color */),
            Detail("PM", "Print move mode", None, true, Number::parse /* number */),
            Detail("PW", "Player White", GameInfo, false, SimpleText::parse /* simpletext */),
            Detail("RE", "Result", GameInfo, false, SimpleText::parse /* simpletext */),
            Detail("RO", "Round", GameInfo, false, SimpleText::parse /* simpletext */),
            Detail("RU", "Rules", GameInfo, false, SimpleText::parse /* simpletext */),
            Detail("SE", "Markup", None, false, go::Point::parse /* point */),
            Detail("SL", "Selected", None, false, go::Point::parse /* list of point */),
            Detail("SO", "Source", GameInfo, false, SimpleText::parse /* simpletext */),
            Detail("SQ", "Square", None, false, go::Point::parse /* list of point */),
            Detail("ST", "Style", Root, false, Number::parse /* number (range: 0-3) */),
            Detail("SU", "Setup type", GameInfo, false, SimpleText::parse /* simpletext */),
            Detail("SZ", "Size", Root, false, Number::parse /* (number | composed number ':' number) */),
            Detail("TB", "Territory Black", None, false, go::Point::parse /* elist of point */),
            Detail("TE", "Tesuji", Move, false, Double::parse /* double */),
            Detail("TM", "Timelimit", GameInfo, false, Real::parse /* real */),
            Detail("TR", "Triangle", None, false, go::Point::parse /* list of point */),
            Detail("TW", "Territory White", None, false, go::Point::parse /* elist of point */),
            Detail("UC", "Unclear pos", None, false, Double::parse /* double */),
            Detail("US", "User", GameInfo, false, SimpleText::parse /* simpletext */),
            Detail("V",  "Value", None, false, Real::parse /* real */),
            Detail("VW", "View", None, true, go::Point::parse /* elist of point */),
            Detail("W",  "White", Move, false, go::Move::parse /* move */),
            Detail("WL", "White time left", Move, false, Real::parse /* real */),
            Detail("WR", "White rank", GameInfo, false, SimpleText::parse /* simpletext */),
            Detail("WT", "White team", GameInfo, false, SimpleText::parse /* simpletext */),
        ].into_iter().fold(HashMap::new(), |mut s, d| { s.insert(d.0, d); s })
    };
}
