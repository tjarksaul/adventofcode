use std::cmp;

fn main() {
    let input = read::read_all_lines(include_str!("../input.txt"));
    
    let smallest_seed = find_smallest_seed(input);

    dbg!(smallest_seed);
}

fn find_smallest_seed(mapping: SeedMapping) -> usize {
    let mut min = usize::MAX;
    for seed in mapping.seeds.iter() {
        let location = map_seed_to_location(seed, &mapping);
        min = cmp::min(min, location);
    }
    min
}

fn map_seed_to_location(seed: &usize, mapping: &SeedMapping) -> usize {
    let soil = mapping.seed_to_soil.get(seed).unwrap_or(*seed);
    let fertilizer = mapping.soil_to_fertilizer.get(&soil).unwrap_or(soil);
    let water = mapping.fertilizer_to_water.get(&fertilizer).unwrap_or(fertilizer);
    let light = mapping.water_to_light.get(&water).unwrap_or(water);
    let temperature = mapping.light_to_temperature.get(&light).unwrap_or(light);
    let humidity = mapping.temperature_to_humidity.get(&temperature).unwrap_or(temperature);
    let location = mapping.humidity_to_location.get(&humidity).unwrap_or(humidity);
    location.clone()
}

pub struct SeedMapping {
    seeds: Vec<usize>,

    seed_to_soil: Mapper,
    soil_to_fertilizer: Mapper,
    fertilizer_to_water: Mapper,
    water_to_light: Mapper,
    light_to_temperature: Mapper,
    temperature_to_humidity: Mapper,
    humidity_to_location: Mapper,
}

pub struct Mapper {
    maps: Vec<Mapping>
}

impl Mapper {
    fn get(&self, input: &usize) -> Option<usize> {
        for map in &self.maps {
            let out = map.has(&input);
            if out.is_some() {
                return out
            }
        }
        None
    }
}

pub struct Mapping {
    start: usize,
    end: usize,
    destination_start: usize,
}

impl Mapping {
    fn has(&self, input: &usize) -> Option<usize> {
        if input >= &self.start && input <= &self.end {
            return Some(self.destination_start + (input - self.start));
        }
        None
    }
}

mod read {
    // Sample input:
    // seeds: 79 14 55 13

    // seed-to-soil map:
    // 50 98 2
    use nom::{
        bytes::complete::tag, character::complete as cc, combinator::all_consuming, combinator::map,
        multi::separated_list1, sequence::delimited, Finish, IResult,
    };
    use super::{SeedMapping, Mapper, Mapping};

    pub fn read_all_lines(i: &'static str) -> SeedMapping {
        all_consuming(parse_seed_mapping)(i).finish().unwrap().1
    }

    fn parse_seed_mapping(i: &str) -> IResult<&str, SeedMapping> {
        let (i, seeds) = delimited(tag("seeds: "), separated_list1(cc::multispace1, parse_usize), cc::multispace0)(i)?;
        let (i, seed_to_soil_mappings) = delimited(tag("seed-to-soil map:\n"), separated_list1(tag("\n"), separated_list1(tag(" "), parse_usize)), cc::multispace0)(i)?;
        let (i, soil_to_fertilizer_mappings) = delimited(tag("soil-to-fertilizer map:\n"), separated_list1(tag("\n"), separated_list1(tag(" "), parse_usize)), cc::multispace0)(i)?;
        let (i, fertilizer_to_water_mappings) = delimited(tag("fertilizer-to-water map:\n"), separated_list1(tag("\n"), separated_list1(tag(" "), parse_usize)), cc::multispace0)(i)?;
        let (i, water_to_light_mappings) = delimited(tag("water-to-light map:\n"), separated_list1(tag("\n"), separated_list1(tag(" "), parse_usize)), cc::multispace0)(i)?;
        let (i, light_to_temperature_mappings) = delimited(tag("light-to-temperature map:\n"), separated_list1(tag("\n"), separated_list1(tag(" "), parse_usize)), cc::multispace0)(i)?;
        let (i, temperature_to_humidity_mappings) = delimited(tag("temperature-to-humidity map:\n"), separated_list1(tag("\n"), separated_list1(tag(" "), parse_usize)), cc::multispace0)(i)?;
        let (i, humidity_to_location_mappings) = delimited(tag("humidity-to-location map:\n"), separated_list1(tag("\n"), separated_list1(tag(" "), parse_usize)), cc::multispace0)(i)?;

        let seed_to_soil = create_mapping(&seed_to_soil_mappings);
        let soil_to_fertilizer = create_mapping(&soil_to_fertilizer_mappings);
        let fertilizer_to_water = create_mapping(&fertilizer_to_water_mappings);
        let water_to_light = create_mapping(&water_to_light_mappings);
        let light_to_temperature = create_mapping(&light_to_temperature_mappings);
        let temperature_to_humidity = create_mapping(&temperature_to_humidity_mappings);
        let humidity_to_location = create_mapping(&humidity_to_location_mappings);

        Ok((i, SeedMapping { 
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location ,
        }))
    }

    fn create_mapping(mapping: &Vec<Vec<usize>>) -> Mapper {
        let mut maps = vec![];

        for i in 0..mapping.iter().len() {
            if mapping[i].len() != 3 {
                panic!("Found invalid mapping entry {:#?}", mapping[i]);
            }
                
            maps.push(Mapping { start: mapping[i][1], end: mapping[i][1] + mapping[i][2], destination_start: mapping[i][0] });
        }

        Mapper { maps }
    }

    fn parse_usize(i: &str) -> IResult<&str, usize> {
        map(cc::u32, |num: u32| num as usize)(i)
    }

    // // 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    // fn parse_line(i: &str) -> IResult<&str, Card> {
    //     let (i, (_, _, id, _)) = tuple((tag("Card"), cc::multispace1, parse_usize, tag(":")))(i)?;
    //     let (i, (winning, _, numbers)) = delimited(
    //         cc::multispace0,
    //         tuple((
    //             delimited(cc::multispace0, separated_list1(cc::multispace1, parse_usize), cc::multispace0), 
    //             tag("|"), 
    //             delimited(cc::multispace0, separated_list1(cc::multispace1, parse_usize), cc::multispace0),
    //         )),
    //         cc::multispace0
    //       )(i)?;
    //     Ok((i, Card { id, winning, numbers }))
    // }
}
