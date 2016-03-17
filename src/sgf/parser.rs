// Parser for sgf files
// SGF format is defined at http://www.red-bean.com/sgf/
//
// Basic grammar is simple (rewritten to regexp form):
//
//  Collection = GameTree+
//  GameTree   = "(" Sequence GameTree* ")"
//  Sequence   = Node+
//  Node       = ";" Property*
//  Property   = PropIdent PropValue+
//  PropIdent  = UcLetter+
//  PropValue  = "[" CValueType "]"
//  CValueType = (ValueType | Compose)
//  ValueType  = (None | Number | Real | Double | Color | SimpleText |
//		Text | Point  | Move | Stone)
//
// White space (space, tab, carriage return, line feed, vertical tab and so on)
// may appear anywhere between PropValues, Properties, Nodes, Sequences and GameTrees.
use std::str;

use nom::{Err, IResult, Needed, ErrorKind, is_space, AsBytes};
use nom::IResult::*;

use super::{Node, Property};

named!(pub collection<Vec<Node> >, many1!(gametree));

named!(gametree<Node>,
    chain!(lparen ~ mut node:sequence ~ variants:many0!(gametree) ~ rparen, || {
        for v in variants {
            node.addchild(v)
        };
        node
    }));

named!(sequence<Node>, map!(many1!(node), |nodes: Vec<Node>| {
        let mut it = nodes.into_iter().rev();
        let last = it.next().unwrap();
        it.fold(last, |s, mut n| { n.addchild(s); n })
    }));

named!(node<Node>, chain!(semicolon ~ props:many0!(property), || {
        let mut n = Node::new();
        for p in props { n.addprop(p) };
        n
    }));
named!(property<Property>, chain!(id:ucident ~ v:many1!(propvalue), || { Property::new(id, v) }));
named!(propvalue<Vec<u8> >, chain!(lbracket ~ v:cvaluetype ~ rbracket, || { v }));
// In theory "rawvalue" is "alt!(valuetype | compose)" and compose is "chain!(valuetype ~ colon ~ valuetype)"
// but the syntax is ambiguous unless you know exactly what type each ident takes.
named!(cvaluetype<Vec<u8> >, map!(rawvalue, Vec::from));

#[inline]
fn ignore<T>(_: T) -> () { () }

named!(eol<()>, map!(alt!(apply!(ctag, "\n") |
                          apply!(ctag, "\r\n") |
                          apply!(ctag, "\u{2028}") |
                          apply!(ctag, "\u{2029}")),
                     ignore)
       );

named!(whitespace<()>,
       map!(take_while1!(is_space), ignore));

// `spaces` consumes spans of space and tab characters interpolated
// with comments, c-preproc and passthrough lines.
named!(spaces<()>,
       map!(many0!(alt!(eol | whitespace)),
            ignore));

fn ws(input: &[u8]) -> &[u8] {
    match spaces(input) {
        Done(rest, _) => rest,
        _ => input,
    }
}

named!(lparen, preceded!(spaces, apply!(ctag, "(")));
named!(rparen, preceded!(spaces, apply!(ctag, ")")));
named!(lbracket, preceded!(spaces, apply!(ctag, "[")));
named!(rbracket, preceded!(spaces, apply!(ctag, "]")));
named!(semicolon, preceded!(spaces, apply!(ctag, ";")));

// Complete tag
fn ctag<T: AsBytes>(input: &[u8], tag: T) -> IResult<&[u8], &[u8]> {
    complete!(input, tag!(tag))
}

fn ucident(input: &[u8]) -> IResult<&[u8], String> {
    let input = ws(input);

    for (idx, item) in input.iter().enumerate() {
        match *item as char {
            'A'...'Z' => continue,
            _ => if idx > 0 {
                return Done(&input[idx..], String::from(str::from_utf8(&input[0..idx]).unwrap()))
            } else {
                return Error(Err::Position(ErrorKind::AlphaNumeric, input))
            },
        }
    }
    Incomplete(Needed::Unknown)
}

// Read in a raw value up to ']', taking \ quotes into account, and preserving them
fn rawvalue(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let mut quote = false;
    for (idx, item) in input.iter().enumerate() {
        match *item as char {
            '\\' if !quote => quote = true,
            ']' if !quote => return Done(&input[idx..], &input[0..idx]),
            _ => quote = false,
        }
    }
    Incomplete(Needed::Unknown)
}

#[cfg(test)]
mod test {
    use super::{lparen, rparen, lbracket, rbracket, semicolon,
                rawvalue, propvalue, property, node, sequence, gametree};
    use nom::IResult::{Done};

    #[test] fn punctuation() {
        assert_eq!(lparen(&b"(thing"[..]), Done(&b"thing"[..], &b"("[..]));
        assert_eq!(rparen(&b")thing"[..]), Done(&b"thing"[..], &b")"[..]));
        assert_eq!(lbracket(&b"[thing"[..]), Done(&b"thing"[..], &b"["[..]));
        assert_eq!(rbracket(&b"]thing"[..]), Done(&b"thing"[..], &b"]"[..]));
        assert_eq!(semicolon(&b";thing"[..]), Done(&b"thing"[..], &b";"[..]));
    }

    #[test] fn wspunctuation() {
        assert_eq!(lparen(&b"  (thing"[..]), Done(&b"thing"[..], &b"("[..]));
        assert_eq!(rparen(&b"  )thing"[..]), Done(&b"thing"[..], &b")"[..]));
        assert_eq!(lbracket(&b"  [thing"[..]), Done(&b"thing"[..], &b"["[..]));
        assert_eq!(rbracket(&b"   ]thing"[..]), Done(&b"thing"[..], &b"]"[..]));
        assert_eq!(semicolon(&b" ;thing"[..]), Done(&b"thing"[..], &b";"[..]));

        assert_eq!(lparen(&b"  \n(thing"[..]), Done(&b"thing"[..], &b"("[..]));
        assert_eq!(rparen(&b"  )thing"[..]), Done(&b"thing"[..], &b")"[..]));
        assert_eq!(lbracket(&b"\n  [thing"[..]), Done(&b"thing"[..], &b"["[..]));
        assert_eq!(rbracket(&b"   ]thing"[..]), Done(&b"thing"[..], &b"]"[..]));
        assert_eq!(semicolon(&b"\n;thing"[..]), Done(&b"thing"[..], &b";"[..]));
    }

    #[test] fn t_rawvalue() {
        assert_eq!(rawvalue(&b"a basic string]"[..]), Done(&b"]"[..], &b"a basic string"[..]));
        assert_eq!(rawvalue(&b"a \\] string ]"[..]), Done(&b"]"[..], &b"a \\] string "[..]));
        assert_eq!(rawvalue(&b"a \\\n string\n]"[..]), Done(&b"]"[..], &b"a \\\n string\n"[..]));
    }

    #[test] fn t_propvalue() {
        assert_eq!(propvalue(&b"  [ a basic string]xx"[..]), Done(&b"xx"[..], Vec::from(&b" a basic string"[..])));
        assert_eq!(propvalue(&b"[ a \\] string ]xx"[..]), Done(&b"xx"[..], Vec::from(&b" a \\] string "[..])));
        assert_eq!(propvalue(&b" [a \\\n string\n] x"[..]), Done(&b" x"[..], Vec::from(&b"a \\\n string\n"[..])));
    }

    #[test] fn t_property() {
        use sgf::property::go::Point;
        use sgf::property::ValueParse;

        match property(b"  W [aa][bb] [cc][de] xxx") {
            Done(b" xxx", prop) => {
                assert_eq!(prop.id(), "W");
                assert_eq!(prop.values().expect("values"), vec![Point::parse(b"aa").unwrap(), Point::parse(b"bb").unwrap(), Point::parse(b"cc").unwrap(), Point::parse(b"de").unwrap()]);
            },
            other => panic!("other result {:?}", other),
        }
    }

    #[test] fn t_node() {
        use sgf::property::{ValueParse, Text, SimpleText};

        match node(b" ; C[This is a comment] DT [some wednesday] AN[goo] x") {
            Done(b" x", node) => {
                match node.prop("C") {
                    Some(prop) => assert_eq!(prop.values().expect("values"), vec![Text::parse(b"This is a comment").unwrap()]),
                    other => panic!("other {:?}", other),
                }
                match node.prop("DT") {
                    Some(prop) => assert_eq!(prop.values().expect("values"), vec![SimpleText::parse(b"some wednesday").unwrap()]),
                    other => panic!("other {:?}", other),
                }
                match node.prop("AN") {
                    Some(prop) => assert_eq!(prop.values().expect("values"), vec![SimpleText::parse(b"goo").unwrap()]),
                    other => panic!("other {:?}", other),
                }
            },
            other => panic!("other {:?}", other),
        }
    }

    #[test] fn t_sequence() {
        match sequence(b";W[nf] ;B[qf] ;W[lc] ;B[od] ;W[oe] ;B[md] ;W[ld] ;B[ne] ;W[me] x") {
            Done(b" x", nodes) => println!("nodes: {:?}", nodes),
            other => panic!("other: {:?}", other),
        }
    }

    #[test] fn t_gametree() {
        match gametree(b"(;W[nf] ;B[qf] ;W[lc] ;B[od] ;W[oe] ;B[md] ;W[ld] ;B[ne] ;W[me] )") {
            Done(b"", nodes) => println!("nodes: {:?}", nodes),
            other => panic!("other: {:?}", other),
        }
        match gametree(b"(;W[nf] ;B[qf] ;W[lc] ;B[od] (;W[oe] ;B[md]) (;W[ld] ;B[ne] ;W[me]) )") {
            Done(b"", nodes) => println!("nodes: {:?}", nodes),
            other => panic!("other: {:?}", other),
        }
    }
}
