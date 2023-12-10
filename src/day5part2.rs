use crate::day5part1::{
    get_almanac_entries_from_chunk, get_mapped_destination_from_source, get_seeds_from_line, AlmanacSectionEntry,
};
use crate::read_file_into_chunks;

pub fn solve() {
    let chunks = read_file_into_chunks("input5.txt");
    println!("{}", process_file(chunks));
}

fn process_file(chunks: Vec<String>) -> u64 {
    let seed_ranges: Vec<u64> = get_seeds_from_line(chunks[0].to_string());
    let seed_to_soil: Vec<AlmanacSectionEntry> = get_almanac_entries_from_chunk(chunks[1].to_string());
    let soil_to_fertilizer: Vec<AlmanacSectionEntry> = get_almanac_entries_from_chunk(chunks[2].to_string());
    let fertilizer_to_water: Vec<AlmanacSectionEntry> = get_almanac_entries_from_chunk(chunks[3].to_string());
    let water_to_light: Vec<AlmanacSectionEntry> = get_almanac_entries_from_chunk(chunks[4].to_string());
    let light_to_temperature: Vec<AlmanacSectionEntry> = get_almanac_entries_from_chunk(chunks[5].to_string());
    let temperature_to_humidity: Vec<AlmanacSectionEntry> = get_almanac_entries_from_chunk(chunks[6].to_string());
    let humidity_to_location: Vec<AlmanacSectionEntry> = get_almanac_entries_from_chunk(chunks[7].to_string());

    let mut lowest_location = u64::MAX;
    let mut i = 0;

    while i < seed_ranges.len() {
        for j in seed_ranges[i]..seed_ranges[i] + seed_ranges[i + 1] + 1 {
            let soil = get_mapped_destination_from_source(&seed_to_soil, j);
            let fertilizer = get_mapped_destination_from_source(&soil_to_fertilizer, soil);
            let water = get_mapped_destination_from_source(&fertilizer_to_water, fertilizer);
            let light = get_mapped_destination_from_source(&water_to_light, water);
            let temperature = get_mapped_destination_from_source(&light_to_temperature, light);
            let humudity = get_mapped_destination_from_source(&temperature_to_humidity, temperature);
            let location = get_mapped_destination_from_source(&humidity_to_location, humudity);

            if location < lowest_location {
                lowest_location = location;
            }
        }
        i += 2;
    }

    return lowest_location;
}
