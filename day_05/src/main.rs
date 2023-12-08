use std::cmp;
use std::env;
use std::io::{self, Read};

fn main() {
    if env::args().any(|x| x == "--p1") {
        println!("Part 1:");
        p1();
    } else {
        println!("Part 2:");
        p2();
    }
}

#[derive(Clone, Copy, Debug)]
struct Range {
    start: u64,
    end: u64,
}

#[derive(Clone, Copy, Debug)]
struct RangeMapEntry {
    dest_start: u64,
    src_start: u64,
    length: u64,
}

#[derive(Debug)]
struct RangeMap {
    entries: Vec<RangeMapEntry>,
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: RangeMap,
    soil_to_fert: RangeMap,
    fert_to_water: RangeMap,
    water_to_light: RangeMap,
    light_to_temp: RangeMap,
    temp_to_humidity: RangeMap,
    humidity_to_loc: RangeMap,
}

fn parse_range_map(s: &str) -> RangeMap {
    let entries = s.split("\n").skip(1).map(|l| {
        let range_parts: Vec<u64> = l.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let dest_start = range_parts[0];
        let src_start = range_parts[1];
        let length = range_parts[2];
        RangeMapEntry { dest_start, src_start, length }
    }).collect();
    RangeMap { entries }
}

fn read_almanac_from_stdin() -> Almanac {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    let parts: Vec<&str> = buf.split("\n\n").collect();
    let seeds: Vec<u64> = parts[0][6..].trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let seed_to_soil = parse_range_map(parts[1]);
    let soil_to_fert = parse_range_map(parts[2]);
    let fert_to_water = parse_range_map(parts[3]);
    let water_to_light = parse_range_map(parts[4]);
    let light_to_temp = parse_range_map(parts[5]);
    let temp_to_humidity = parse_range_map(parts[6]);
    let humidity_to_loc = parse_range_map(parts[7]);
    Almanac {
        seeds,
        seed_to_soil,
        soil_to_fert,
        fert_to_water,
        water_to_light,
        light_to_temp,
        temp_to_humidity,
        humidity_to_loc,
    }
}

fn map_value(map: &RangeMap, v: u64) -> u64 {
    for entry in map.entries.iter() {
        if v >= entry.src_start && (v - entry.src_start) < entry.length {
            let delta = v - entry.src_start;
            return delta + entry.dest_start;
        }
    }
    v
}

fn p1() {
    let alm = read_almanac_from_stdin();
    let result = alm.seeds.iter().copied().map(|seed| {
        let soil = map_value(&alm.seed_to_soil, seed);
        let fert = map_value(&alm.soil_to_fert, soil);
        let water = map_value(&alm.fert_to_water, fert);
        let light = map_value(&alm.water_to_light, water);
        let temp = map_value(&alm.light_to_temp, light);
        let humid = map_value(&alm.temp_to_humidity, temp);
        let loc = map_value(&alm.humidity_to_loc, humid);
        loc
    }).min().unwrap();
    println!("{}", result);
}

fn intersection(r_0: Range, r_1: Range) -> Option<Range> {
    if r_0.end >= r_1.start && r_1.end >= r_0.start {
        let start = cmp::max(r_0.start, r_1.start);
        let end = cmp::min(r_0.end, r_1.end);
        if end - start > 0 {
            return Some(Range { start, end });
        }
    }
    None
}

fn map_range_over_single_entry(entry: RangeMapEntry, r: Range) -> (Option<Range>, Vec<Range>) {
    let ent_r = Range { start: entry.src_start, end: entry.src_start + entry.length };
    let mut mapped_r = None;
    let mut leftover_rs = vec![];
    if let Some(int_r) = intersection(r, ent_r) {
        let start = int_r.start - ent_r.start + entry.dest_start;
        let end = int_r.end - int_r.start + start;
        mapped_r = Some(Range { start, end });
        if r.start < ent_r.start {
            leftover_rs.push(Range { start: r.start, end: ent_r.start });
        }
        if r.end > ent_r.end {
            leftover_rs.push(Range { start: ent_r.end, end: r.end });
        } 
    } else {
        leftover_rs.push(r);
    }
    (mapped_r, leftover_rs)
}

fn map_ranges(range_map: &RangeMap, rs: &[Range]) -> Vec<Range> {
    let mut leftover_rs: Vec<Range> = rs.iter().cloned().collect();
    let mut mapped_rs: Vec<Range> = vec![];
    for entry in range_map.entries.iter() {
        let map_results: Vec<_> = leftover_rs.iter()
            .map(|&r| map_range_over_single_entry(*entry, r))
            .collect();
        mapped_rs.extend(map_results.iter().map(|r| r.0).filter_map(|m| m));
        leftover_rs = map_results.into_iter().map(|r| r.1).flatten().collect();
    }
    mapped_rs.extend(leftover_rs.iter());
    simplify_ranges(&mapped_rs)
}

fn simplify_ranges(rs: &[Range]) -> Vec<Range> {
    let mut rs: Vec<Range> = rs.iter().cloned().collect();
    rs.sort_by_key(|r| r.start);

    if rs.len() <= 1 {
        return rs;
    }

    let mut result = vec![];
    let mut it = 0;
    while it < rs.len() {
        let start = rs[it].start;
        let mut end = rs[it].end;
        while it < rs.len() && rs[it].start <= end {
            end = cmp::max(end, rs[it].end);
            it += 1;
        }
        result.push(Range { start, end });
    }
    result
}

fn p2() {
    let alm = read_almanac_from_stdin();
    let seeds: Vec<Range> = alm.seeds.chunks(2).map(|s| {
        let start = s[0];
        let end = s[0] + s[1];
        Range { start, end }
    }).collect();
    let soils = map_ranges(&alm.seed_to_soil, &seeds);
    let ferts = map_ranges(&alm.soil_to_fert, &soils);
    let waters = map_ranges(&alm.fert_to_water, &ferts);
    let lights = map_ranges(&alm.water_to_light, &waters);
    let temps = map_ranges(&alm.light_to_temp, &lights);
    let humids = map_ranges(&alm.temp_to_humidity, &temps);
    let locs = map_ranges(&alm.humidity_to_loc, &humids);
    println!("{}", locs.iter().map(|r| r.start).min().unwrap());
}