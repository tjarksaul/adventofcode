use std::cmp::Ordering;

fn main() {
    let input = read::read_input();

    let sum = sum_equal_indexes(&input);

    dbg!(sum);
}

fn sum_equal_indexes(packets: &Vec<Packets>) -> usize {
    let mut sum = 0;

    for i in 0..packets.len() {
        let Packets(packet1, packet2) = &packets[i];
        if packet2 > packet1 {
            sum += i + 1;
        }
    }

    return sum;
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Packet(Vec<Item>);

#[derive(Debug)]
pub enum Item {
    I(i32),
    L(Vec<Item>),
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        return match self {
            Self::I(wrapped) => match other {
                Self::I(other_wrapped) => wrapped == other_wrapped,
                Self::L(items) => vec![Self::I(*wrapped)] == *items,
            },
            Self::L(items) => match other {
                Self::I(other_wrapped) => *items == vec![Self::I(*other_wrapped)],
                Self::L(other_items) => items == other_items,
            },
        };
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return match self {
            Self::I(wrapped) => match other {
                Self::I(other_wrapped) => wrapped.partial_cmp(other_wrapped),
                Self::L(items) => vec![Self::I(*wrapped)].partial_cmp(items),
            },
            Self::L(items) => match other {
                Self::I(other_wrapped) => items.partial_cmp(&vec![Self::I(*other_wrapped)]),
                Self::L(other_items) => items.partial_cmp(other_items),
            },
        };
    }
}

pub struct Packets(Packet, Packet);

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<Packets> {
        return vec![
            Packets(
                Packet(vec![
                    Item::I(1),
                    Item::I(1),
                    Item::I(3),
                    Item::I(1),
                    Item::I(1),
                ]),
                Packet(vec![
                    Item::I(1),
                    Item::I(1),
                    Item::I(5),
                    Item::I(1),
                    Item::I(1),
                ]),
            ),
            Packets(
                Packet(vec![
                    Item::L(vec![Item::I(1)]),
                    Item::L(vec![Item::I(2), Item::I(3), Item::I(4)]),
                ]),
                Packet(vec![Item::L(vec![Item::I(1)]), Item::I(4)]),
            ),
            Packets(
                Packet(vec![Item::I(9)]),
                Packet(vec![Item::L(vec![Item::I(8), Item::I(7), Item::I(6)])]),
            ),
            Packets(
                Packet(vec![
                    Item::L(vec![Item::I(4), Item::I(4)]),
                    Item::I(4),
                    Item::I(4),
                ]),
                Packet(vec![
                    Item::L(vec![Item::I(4), Item::I(4)]),
                    Item::I(4),
                    Item::I(4),
                    Item::I(4),
                ]),
            ),
            Packets(
                Packet(vec![Item::I(7), Item::I(7), Item::I(7), Item::I(7)]),
                Packet(vec![Item::I(7), Item::I(7), Item::I(7)]),
            ),
            Packets(Packet(vec![]), Packet(vec![Item::I(3)])),
            Packets(
                Packet(vec![Item::L(vec![Item::L(vec![])])]),
                Packet(vec![Item::L(vec![])]),
            ),
            Packets(
                Packet(vec![
                    Item::I(1),
                    Item::L(vec![
                        Item::I(2),
                        Item::L(vec![
                            Item::I(3),
                            Item::L(vec![
                                Item::I(4),
                                Item::L(vec![Item::I(5), Item::I(6), Item::I(7)]),
                            ]),
                        ]),
                    ]),
                    Item::I(8),
                    Item::I(9),
                ]),
                Packet(vec![
                    Item::I(1),
                    Item::L(vec![
                        Item::I(2),
                        Item::L(vec![
                            Item::I(3),
                            Item::L(vec![
                                Item::I(4),
                                Item::L(vec![Item::I(5), Item::I(6), Item::I(0)]),
                            ]),
                        ]),
                    ]),
                    Item::I(8),
                    Item::I(9),
                ]),
            ),
        ];
    }

    #[test]
    fn test_sums_correctly() {
        let packets = get_input();

        let sum_indexes = sum_equal_indexes(&packets);

        assert_eq!(sum_indexes, 13);
    }
}

mod read {
    use miette::GraphicalReportHandler;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete as cc,
        combinator::map,
        error::ParseError,
        multi::separated_list1,
        sequence::{delimited, tuple},
        IResult,
    };
    use nom_locate::LocatedSpan;
    use nom_supreme::{
        error::{BaseErrorKind, ErrorTree, GenericErrorTree},
        final_parser::final_parser,
    };

    use super::{Item, Packet, Packets};

    pub type Span<'a> = LocatedSpan<&'a str>;

    #[derive(thiserror::Error, Debug, miette::Diagnostic)]
    #[error("bad input")]
    struct BadInput {
        #[source_code]
        src: &'static str,

        #[label("{kind}")]
        bad_bit: miette::SourceSpan,

        kind: BaseErrorKind<&'static str, Box<dyn std::error::Error + Send + Sync>>,
    }

    pub fn read_input() -> Vec<Packets> {
        let input_static = concat!(include_str!("../input.txt"), "\n");
        let input = Span::new(input_static);

        let packets_res: Result<_, ErrorTree<Span>> =
            final_parser(parse_all_packets::<ErrorTree<Span>>)(input);

        return match packets_res {
            Ok(packets) => packets,
            Err(e) => {
                match e {
                    GenericErrorTree::Base { location, kind } => {
                        let offset = location.location_offset().into();
                        let err = BadInput {
                            src: input_static,
                            bad_bit: miette::SourceSpan::new(offset, 0.into()),
                            kind: kind,
                        };
                        let mut s = String::new();
                        GraphicalReportHandler::new()
                            .render_report(&mut s, &err)
                            .unwrap();
                        println!("{s}");
                    }
                    GenericErrorTree::Stack { .. } => todo!("stack"),
                    GenericErrorTree::Alt(_) => todo!("alt"),
                }
                return vec![];
            }
        };
    }

    fn parse_all_packets<'a, E: ParseError<Span<'a>>>(
        i: Span<'a>,
    ) -> IResult<Span<'a>, Vec<Packets>, E> {
        separated_list1(tag("\n"), parse_packets)(i)
    }

    fn parse_packets<'a, E: ParseError<Span<'a>>>(i: Span<'a>) -> IResult<Span<'a>, Packets, E> {
        // Sample input:
        // [[[1,6,[1,9,0,9],6]]]
        // [[],[[[5,6,3],6,[6,5,3,3]],8,3],[],[4]]

        let (i, (_, packet1, _, _)) = parse_packet(i)?;
        let (i, (_, packet2, _, _)) = parse_packet(i)?;

        Ok((i, Packets(Packet(packet1), Packet(packet2))))
    }

    fn parse_item<'a, E: ParseError<Span<'a>>>(i: Span<'a>) -> IResult<Span<'a>, Item, E> {
        let result: IResult<Span<'a>, Item, E> = alt((
            map(cc::i32, Item::I),
            delimited(
                cc::char('['),
                map(separated_list1(tag(","), parse_item), Item::L),
                cc::char(']'),
            ),
        ))(i);
        return match result {
            Ok(it) => Ok(it),
            Err(_) => Ok((i, Item::L(vec![]))),
        };
    }

    fn parse_packet<'a, E: ParseError<Span<'a>>>(
        i: Span<'a>,
    ) -> IResult<Span<'a>, (Span<'a>, Vec<Item>, Span<'a>, Span<'a>), E> {
        tuple((
            tag("["),
            separated_list1(tag(","), parse_item),
            tag("]"),
            tag("\n"),
        ))(i)
    }
}
