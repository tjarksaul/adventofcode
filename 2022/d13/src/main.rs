use std::cmp::Ordering;

fn main() {
    let input = read::read_input();
    let mut packet_list = read::read_packet_list();

    let sum = sum_equal_indexes(&input);
    let product = find_decoder_key(&mut packet_list);

    dbg!(sum);
    dbg!(product);
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

fn find_decoder_key(packets: &mut Vec<Packet>) -> usize {
    let decoder_packet_1 = Packet(vec![Item::L(vec![Item::I(2)])]);
    let decoder_packet_2 = Packet(vec![Item::L(vec![Item::I(6)])]);
    packets.push(decoder_packet_1.clone());
    packets.push(decoder_packet_2.clone());

    packets.sort();

    let mut product = 1;
    for i in 0..packets.len() {
        if packets[i] == decoder_packet_1 || packets[i] == decoder_packet_2 {
            product *= i + 1;
        }
    }

    return product;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Packet(Vec<Item>);

#[derive(Debug, Clone, Eq, Ord)]
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

    #[test]
    fn test_sorts_correctly() {
        let mut packets: Vec<Packet> = get_input()
            .into_iter()
            .flat_map(|p| {
                let Packets(packet1, packet2) = p;
                vec![packet1, packet2]
            })
            .collect();

        let decoder_key = find_decoder_key(&mut packets);

        assert_eq!(decoder_key, 140);
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

    pub fn read_packet_list() -> Vec<Packet> {
        let input_static = include_str!("../input.txt");
        let input = Span::new(input_static);

        let packets_res: Result<_, ErrorTree<Span>> =
            final_parser(parse_packet_list::<ErrorTree<Span>>)(input);

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

    fn parse_packet_list<'a, E: ParseError<Span<'a>>>(
        i: Span<'a>,
    ) -> IResult<Span<'a>, Vec<Packet>, E> {
        separated_list1(cc::multispace1, parse_packet)(i)
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

        let (i, packet1) = parse_packet(i)?;
        let (i, _) = tag("\n")(i)?;
        let (i, packet2) = parse_packet(i)?;
        let (i, _) = tag("\n")(i)?;

        Ok((i, Packets(packet1, packet2)))
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

    fn parse_packet<'a, E: ParseError<Span<'a>>>(i: Span<'a>) -> IResult<Span<'a>, Packet, E> {
        let (i, (_, items, _)) =
            tuple((tag("["), separated_list1(tag(","), parse_item), tag("]")))(i)?;
        Ok((i, Packet(items)))
    }
}
