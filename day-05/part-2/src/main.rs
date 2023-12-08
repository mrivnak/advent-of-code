use std::ops::RangeInclusive;
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

#[derive(Debug)]
struct Map {
    from: Item,
    to: Item,
    ranges: Vec<MapRange>,
}

impl Map {
    fn is_mapped(&self, seed: u64) -> bool {
        self.ranges.iter().any(|r| r.src.contains(&seed))
    }

    fn map(&self, seed: u64) -> u64 {
        if !self.is_mapped(seed) {
            return seed;
        }
        // TODO: could use binary search here
        let range = self.ranges.iter().find(|r| r.src.contains(&seed)).unwrap();
        let index = seed - range.src.start();
        range.dest.start() + index
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
    let min = almanac.seeds.iter().flat_map(|r| r.clone()).map(|seed| {
        let seed_map = almanac.get_map(&Item::Seed).expect("No seed map found");
        map_item(seed, seed_map, &almanac)
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
    let seed_ranges = seeds.chunks_exact(2).map(|c| c[0]..=c[0]+c[1]-1).collect::<Vec<_>>();

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
            let map = map.lines().map(|l| {
                let mut parts = l.split_whitespace();
                let dest_start = parts.next().expect("No map dest start found").parse::<u64>().expect("Invalid map dest start");
                let src_start = parts.next().expect("No map src start found").parse::<u64>().expect("Invalid map src start");
                let length = parts.next().expect("No map length found").parse::<u64>().expect("Invalid map length");
                MapRange::new(dest_start, src_start, length)
            });
            Map { from, to, ranges: map.collect::<Vec<_>>() }
        })
        .collect::<Vec<_>>();

    Almanac { seeds: seed_ranges, maps }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_is_mapped() {
        let map = Map {
            from: Item::Seed,
            to: Item::Soil,
            ranges: vec![MapRange::new(100, 0, 10)],
        };
        assert!(map.is_mapped(0));
        assert!(map.is_mapped(9));
        assert!(!map.is_mapped(10));
    }

    #[test]
    fn test_map_map() {
        let map = Map {
            from: Item::Seed,
            to: Item::Soil,
            ranges: vec![MapRange::new(100, 0, 10)],
        };
        assert_eq!(map.map(0), 100);
        assert_eq!(map.map(9), 109);
        assert_eq!(map.map(20), 20);
    }

    #[test]
    fn test_almanac_get_map() {
        let map0 = Map {
            from: Item::Seed,
            to: Item::Soil,
            ranges: vec![MapRange::new(100, 0, 10)],
        };
        let map1 = Map {
            from: Item::Humidity,
            to: Item::Location,
            ranges: vec![MapRange::new(100, 0, 10)],
        };

        let almanac = Almanac {
            seeds: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            maps: vec![map0, map1],
        };
        let map = almanac.get_map(&Item::Humidity).expect("No map found");
        assert!(matches!(map.from, Item::Humidity));
    }
}