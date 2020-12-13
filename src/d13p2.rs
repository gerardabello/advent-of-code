use crate::d13p1::{next_bus_wait, parse_input};


fn is_next_bus_at_offset(timestamp: u64, bus_id: u64, offset: u64) -> bool {
    let wait = next_bus_wait(timestamp, bus_id);

    // There could be an earlier bus, but any (timestamp + wait + n * busID) is also valid.
    offset % bus_id == wait
}

// Checks if the timestamp is valid, if not, it returns the last bus that matched the offset.
fn check_timestamp_for_bus(ids_offsets: &[(u64, u64)], bus_index: usize, timestamp: u64) -> Result<(), u64> {
    if bus_index >= ids_offsets.len() - 1 {
        return Ok(());
    }

    let (next_bus, next_offset) = ids_offsets[bus_index + 1];
    if is_next_bus_at_offset(timestamp, next_bus, next_offset) {
        return check_timestamp_for_bus(ids_offsets, bus_index + 1,timestamp);
    }

    Err(bus_index as u64)
}

fn find_timestamp_matching_offsets(ids_offsets: &[(u64, u64)]) -> u64 {

    let mut base = ids_offsets[0].0;
    let mut base_i = 0;

    let mut timestamp = base;

    loop {
        let ret = check_timestamp_for_bus(ids_offsets, 0, timestamp);

        match ret {
            Ok(_) => return timestamp,
            Err(i) => {
                if i > base_i {
                    // if the timestamp has a new bus that matches, multiply it's ID, as we know
                    // that the same patters will only repeat in (bus1ID * bus2ID * ...)
                    base_i = i;
                    base *= ids_offsets[i as usize].0;
                }
            }
        }

        timestamp += base;
    }
}

pub fn solve(input: &str) -> String {
    let (_, (_, bus_ids)) = parse_input(input).unwrap();

    // Vec of id,offset
    let ids_offsets: Vec<(u64, u64)> = bus_ids
        .iter()
        .enumerate()
        .filter(|en| en.1.is_some())
        .map(|en| (en.0 as u64, en.1.unwrap()))
        .map(|(o, i)| (i, o))
        .collect();

    find_timestamp_matching_offsets(&ids_offsets).to_string()
}
