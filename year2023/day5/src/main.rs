use std::{collections::HashMap, str::Lines, slice::Iter};

fn main() {
    let input = input::get_input_lines();

    let mut lines = input.lines();

    let seeds_line = lines.next().unwrap();
    let seeds_numbers = seeds_line.split("seeds: ");

    let coll = parse_numbers(&seeds_numbers.last().unwrap().to_string());
    let mut seed_input_ranges = coll.iter();

    let mut seed_ranges = vec![];
    loop {
        let first = seed_input_ranges.next();
        let second = seed_input_ranges.next();

        if first.is_none() || second.is_none() {
            break;
        }

        let seed = *first.unwrap();
        let range = *second.unwrap();

        seed_ranges.push(MappingRange::new(seed, 0, range));
    }
    println!("Analyzing {} seed ranges", seed_ranges.len());

    lines.next(); // skip empty line
    
    let segments = parse_all_segments(lines);

    let mut segments_by_from = HashMap::new();
    for ((from, _), segment) in segments {
        segments_by_from.insert(from, segment);
    }

    let mut closest_location = None;
    for seed in seed_ranges {
        todo!();

        // Questions to answer:
        // 1. How to get from Seed -> Location for a range
        // 2. Can we analyze the mapping to create a list of SeedRange -> LocationRange?
        //  1. [[0, Inf] -> [0, Inf]]
        //  2. [[0,10] -> [0,10]]
        //  3. [[11,15] -> [13-17]]
        //
        //  From example
        //  Seed->Soil
        //  0-48 -> 0-49
        //  49-97 -> 52-100
        //  98-99 -> 50-51
        //  Soil->Fert
        //  0-14 -> 39->54
        //  15-51 -> 0-37
        //  52-53 -> 37-38
        //  54-Inf -> 54->Inf
        //
        //  Then eventually
        //  Seed [0-49] -> Loc [?? - ??]

        let mut from_category = Category::Seed;
        let mut from = seed;
        let looking_for = Category::Location;
        loop {
            // dbg!(&segments_by_from, from_category);
            let segment = segments_by_from.get(&from_category).unwrap();
            let to = segment.mapping.get_destination_by_range(from);
            if segment.to == looking_for {
                if closest_location.is_none() || to < closest_location.unwrap() {
                    closest_location = Some(to);
                }
                break;
            }

            from = to;
            from_category = segment.to;
        }
    }
    dbg!(closest_location.unwrap());
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl Category {
    fn to_string(&self) -> String {
        match self {
            Category::Seed => "seed",
            Category::Soil => "soil",
            Category::Fertilizer => "fertilizer",
            Category::Water => "water",
            Category::Light => "light",
            Category::Temperature => "temperature",
            Category::Humidity => "humidity",
            Category::Location => "location",
        }.to_owned()
    }

    fn from_string(raw: &str) -> Category {
        match raw {
            "seed" => Category::Seed,
            "soil" => Category::Soil,
            "fertilizer" => Category::Fertilizer,
            "water" => Category::Water,
            "light" => Category::Light,
            "temperature" => Category::Temperature,
            "humidity" => Category::Humidity,
            "location" => Category::Location,
            _ => panic!("unknown category '{}'", raw)
        }
    }
}

#[derive(Debug)]
struct Segment {
    from: Category,
    to: Category,
    mapping: Mapping,
}

#[derive(Debug)]
struct Mapping {
    ranges: Vec<MappingRange>,
}

impl Mapping {
    fn new() -> Self {
        Self {ranges: vec![]}
    }

    fn get_destination(&self, from: usize) -> usize {
        let mut sorted_ranges = self.ranges.clone();
        sorted_ranges.sort_by(|a, b| a.start.cmp(&b.start));

        for range in sorted_ranges {
            // found a range that's out of range, we won't find anything so return its own value
            if range.start > from {
                return from
            }

            if range.start == from {
                return range.destination;
            }

            if range.start + range.range >= from {
                return range.destination + (from - range.start);
            }
        }

        0
    }

    fn get_destination_by_range(&self, from: MappingRange) -> MappingRange {
        todo!();

    }
}

#[derive(Debug, Copy, Clone)]
struct MappingRange {
    start: usize,
    destination: usize,
    range: usize,

}

impl MappingRange {
    fn new(start: usize, destination: usize, range: usize) -> Self { Self { start, destination, range } }
}

fn parse_all_segments(lines: Lines<>) -> HashMap<(Category, Category), Segment> {
    let mut segments = HashMap::new();
    let mut buffer: Vec<String> = vec![];
    for line in lines {
        if line.trim() == "" {
            let segment = create_segment(buffer);
            segments.insert((segment.from, segment.to), segment);
            buffer = vec![];
            continue;
        }

        buffer.push(line.to_string());
    }

    if buffer.len() > 0 {
        let segment = create_segment(buffer);
        segments.insert((segment.from, segment.to), segment);
    }

    segments
}

fn create_segment(raw: Vec<String>) -> Segment {
    let mut lines = raw.iter();

    let (from, to) = parse_header(lines.next().unwrap());
    Segment {
        from,
        to,
        mapping: parse_mapping(lines),
    }
}

fn parse_header(header: &str) -> (Category, Category) {
    let mut parts = header.split(" map:");
    let mut categories = parts.next().unwrap().split("-to-");

    (
        Category::from_string(categories.next().unwrap()),
        Category::from_string(categories.next().unwrap()),
    )
}

fn parse_mapping(lines: Iter<String>) -> Mapping {
    let mut mapping = Mapping::new(); 
    for line in lines {
        let coll = parse_numbers(line);
        let mut numbers = coll.iter();
        let (dest_start, source_start, range) = (numbers.next().unwrap(), numbers.next().unwrap(), numbers.next().unwrap());

        mapping.ranges.push(MappingRange::new(*source_start, *dest_start, *range));
    }
    mapping
}

fn parse_numbers(line: &String) -> Vec<usize> {
    line.split(" ").into_iter().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>()
}
