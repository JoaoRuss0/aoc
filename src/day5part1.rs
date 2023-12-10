use crate::read_file_into_chunks;

pub struct AlmanacSectionEntry {
    destination: u64,
    source: u64,
    range: u64,
}

pub fn solve() {
    let chunks = read_file_into_chunks("input5.txt");
    println!("{}", process_file(chunks));
}

fn process_file(chunks: Vec<String>) -> u64 {
    let seeds: Vec<u64> = get_seeds_from_line(chunks[0].to_string());
    let seed_to_soil: Vec<AlmanacSectionEntry> = get_almanac_entries_from_chunk(chunks[1].to_string());
    let soil_to_fertilizer: Vec<AlmanacSectionEntry> = get_almanac_entries_from_chunk(chunks[2].to_string());
    let fertilizer_to_water: Vec<AlmanacSectionEntry> = get_almanac_entries_from_chunk(chunks[3].to_string());
    let water_to_light: Vec<AlmanacSectionEntry> = get_almanac_entries_from_chunk(chunks[4].to_string());
    let light_to_temperature: Vec<AlmanacSectionEntry> = get_almanac_entries_from_chunk(chunks[5].to_string());
    let temperature_to_humudity: Vec<AlmanacSectionEntry> = get_almanac_entries_from_chunk(chunks[6].to_string());
    let humidity_to_location: Vec<AlmanacSectionEntry> = get_almanac_entries_from_chunk(chunks[7].to_string());

    let mut lowest_location = u64::MAX;
    for seed in seeds {
        let soil = get_mapped_destination_from_source(&seed_to_soil, seed);
        let fertilizer = get_mapped_destination_from_source(&soil_to_fertilizer, soil);
        let water = get_mapped_destination_from_source(&fertilizer_to_water, fertilizer);
        let light = get_mapped_destination_from_source(&water_to_light, water);
        let temperature = get_mapped_destination_from_source(&light_to_temperature, light);
        let humudity = get_mapped_destination_from_source(&temperature_to_humudity, temperature);
        let location = get_mapped_destination_from_source(&humidity_to_location, humudity);

        if location < lowest_location {
            lowest_location = location;
        }
    }

    return lowest_location;
}

pub fn get_seeds_from_line(line: String) -> Vec<u64> {
    let seed_numbers_string: &str = line.split(':').collect::<Vec<&str>>().get(1).unwrap().trim();
    return seed_numbers_string
        .split(' ')
        .map(|number_string| number_string.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
}

pub fn get_almanac_entries_from_chunk(chunk: String) -> Vec<AlmanacSectionEntry> {
    let mut almanac_entries: Vec<AlmanacSectionEntry> = Vec::new();
    let lines = chunk.split('\n').collect::<Vec<&str>>();

    for i in 1..lines.len() {
        let numbers = lines[i]
            .split(' ')
            .map(|number_string| number_string.parse().unwrap())
            .collect::<Vec<u64>>();
        almanac_entries.push(AlmanacSectionEntry {
            destination: numbers[0],
            source: numbers[1],
            range: numbers[2],
        })
    }

    return almanac_entries;
}

pub fn get_mapped_destination_from_source(almanac_entries: &Vec<AlmanacSectionEntry>, source: u64) -> u64 {
    let mut mapper_entry_optional: Option<&AlmanacSectionEntry> = Option::None;

    for almanac_entry in almanac_entries {
        if (almanac_entry.source + almanac_entry.range) > source && almanac_entry.source <= source {
            mapper_entry_optional = Option::Some(almanac_entry);
            break;
        }
    }

    return match mapper_entry_optional {
        None => source,
        Some(mapper) => (*mapper).destination + (source - (*mapper).source),
    };
}
