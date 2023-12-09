use std::cmp::Ordering;
use std::ops::{Range, RangeInclusive};
use rayon::prelude::*;
use regex::Regex;
use strum_macros::EnumString;

#[derive(Clone, Debug, EnumString, Eq, PartialEq)]
#[strum(serialize_all = "snake_case")]
enum Item {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Debug)]
struct MapRange {
    src: RangeInclusive<u64>,
    dest: RangeInclusive<u64>,
}

impl MapRange {
    fn new(dest_start: u64, src_start: u64, length: u64) -> Self {
        Self { src: src_start..=src_start + length - 1, dest: dest_start..=dest_start + length - 1 }
    }
}

impl Eq for MapRange {}

impl PartialEq<Self> for MapRange {
    fn eq(&self, other: &Self) -> bool {
        self.src.start() == other.src.start()
    }
}

impl PartialOrd<Self> for MapRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.src.start().partial_cmp(&other.src.start())
    }
}

impl Ord for MapRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.src.start().cmp(other.src.start())
    }
}

#[derive(Debug)]
struct Map {
    from: Item,
    to: Item,
    ranges: Vec<MapRange>,
}

impl Map {
    fn map(&self, seed: u64) -> u64 {
        let Some(range) = self.find_range(seed, 0..self.ranges.len()) else {
            return seed;
        };

        let index = seed - range.src.start();
        range.dest.start() + index
    }

    fn find_range(&self, seed: u64, idx_range: Range<usize>) -> Option<&MapRange> {
        if idx_range.is_empty() || (idx_range.len() == 1 && !self.ranges[idx_range.start].src.contains(&seed)) {
            return None;
        }

        let mid = idx_range.start + (idx_range.end - idx_range.start) / 2;
        let range = &self.ranges[mid];
        match range.src.contains(&seed) {
            true => Some(range),
            false => {
                match range.src.start().cmp(&seed) {
                    Ordering::Less => self.find_range(seed, mid + 1..idx_range.end),
                    Ordering::Greater => self.find_range(seed, idx_range.start..mid),
                    Ordering::Equal => unreachable!("Two ranges cannot have the same start"),
                }
            }
        }
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<RangeInclusive<u64>>,
    maps: Vec<Map>,
}

impl Almanac {
    fn get_map(&self, from: &Item) -> Option<&Map> {
        self.maps.iter().find(|m| m.from.clone() == *from)
    }
}

fn main() {
    let input = include_str!("../../input.txt");

    let almanac = build_almanac(input);
    let min = almanac.seeds
        .iter()
        .flat_map(|r| r.clone())
        .collect::<Vec<_>>()
        .par_iter()
        .map(|seed| {
            let seed_map = almanac.get_map(&Item::Seed).expect("No seed map found");
            map_item(*seed, seed_map, &almanac)
        }).min().expect("No min seed found");
    println!("{}", min);
}

fn map_item(item: u64, map: &Map, almanac: &Almanac) -> u64 {
    match map.to {
        Item::Location => map.map(item),
        _ => {
            let next_map = almanac.get_map(&map.to).expect("No map found");
            map_item(map.map(item), next_map, almanac)
        }
    }
}

fn build_almanac(input: &str) -> Almanac {
    let seeds_re = Regex::new(r"seeds:\s(\d+(?:\s\d+)*)").expect("Invalid seeds regex");
    let seeds = seeds_re
        .captures(input)
        .expect("No seeds found")
        .get(1)
        .expect("No seeds found")
        .as_str()
        .split_whitespace()
        .map(|s| s.parse::<u64>().expect("Invalid seed"))
        .collect::<Vec<u64>>();
    let seed_ranges = seeds.chunks_exact(2).map(|c| c[0]..=c[0] + c[1] - 1).collect::<Vec<_>>();

    let maps_re = Regex::new(r"(.*-to-.*) map:\n((?:\d+\s\d+\s\d+\n?)+)").expect("Invalid maps regex");
    let map_name_re = Regex::new(r"(.*)-to-(.*)").expect("Invalid map name regex");
    let maps = maps_re
        .captures_iter(input)
        .map(|cap| {
            let name = cap.get(1).expect("No map name found").as_str();
            let name_cap = map_name_re.captures(name).expect("Invalid map name");

            let from = name_cap.get(1).expect("No map from found").as_str().parse::<Item>().expect("Invalid map from");
            let to = name_cap.get(2).expect("No map to found").as_str().parse::<Item>().expect("Invalid map to");

            let map = cap.get(2).expect("No map found").as_str();
            let mut map = map.lines().map(|l| {
                let mut parts = l.split_whitespace();
                let dest_start = parts.next().expect("No map dest start found").parse::<u64>().expect("Invalid map dest start");
                let src_start = parts.next().expect("No map src start found").parse::<u64>().expect("Invalid map src start");
                let length = parts.next().expect("No map length found").parse::<u64>().expect("Invalid map length");
                MapRange::new(dest_start, src_start, length)
            }).collect::<Vec<_>>();
            map.par_sort();
            Map { from, to, ranges: map }
        })
        .collect::<Vec<_>>();

    Almanac { seeds: seed_ranges, maps }
}
