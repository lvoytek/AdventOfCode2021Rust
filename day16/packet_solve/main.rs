use std::fs;

#[derive(Clone, Copy)]
struct PacketHeader {
    #[allow(dead_code)]
    version: u8,
    type_id: u8,
}

#[derive(Clone)]
struct Packet {
    header: PacketHeader,
    is_operator: bool,
    value: u64,
    sub_packets: Vec<Packet>
}

// Convert hex to byte with value represented in the 4 lsbs
fn hex_to_nibble(hex_char: char) -> u8 {
    if hex_char >= '0' && hex_char <= '9' {
        const RADIX: u32 = 10;
        return hex_char.to_digit(RADIX).unwrap() as u8;
    }

    match hex_char {
        'a' | 'A' => return 10,
        'b' | 'B' => return 11,
        'c' | 'C' => return 12,
        'd' | 'D' => return 13,
        'e' | 'E' => return 14,
        'f' | 'F' => return 15,
        _ => {
            println!("Error, bad hex character: {}", hex_char);
            return 0;
        },
    }
}

fn extract_header(index: usize, bits: &Vec<u8>) -> PacketHeader {
    let packet_version = bits[index] << 2 | bits[index + 1] << 1 | bits[index + 2];
    let packet_type = bits[index + 3] << 2 | bits[index + 4] << 1 | bits[index + 5];

    return PacketHeader {
        version: packet_version,
        type_id: packet_type,
    };
}

// Extract value from literal packet then return it and the index of the end of the packet
fn extract_literal_value(index: usize, bits: &Vec<u8>) -> (u64, usize) {
    let mut new_index = index;
    let mut val: u64 = 0;

    while bits[new_index] == 1 {
        new_index += 1;

        for _ in 0..4 {
            val = val << 1;
            val |= bits[new_index] as u64;
            new_index += 1;
        }

    }

    new_index += 1;

    for _ in 0..4 {
        val = val << 1;
        val |= bits[new_index] as u64;
        new_index += 1;
    }

    return (val, new_index);
}

fn extract_packet(index: usize, bits: &Vec<u8>) -> (Packet, usize) {
    let header = extract_header(index, bits);

    let mut packet = Packet {
        header: header,
        is_operator: true,
        value: 0,
        sub_packets: Vec::<Packet>::new(),
    };

    // This is a literal value packet
    if header.type_id == 4 {
        packet.is_operator = false;
        let (literal_val, end_index) = extract_literal_value(index + 6, bits);
        packet.value = literal_val;

        return (packet, end_index);
    }
    // This is an operator packet
    else {
        let mut new_index = index + 6;
        let length_type_id = bits[new_index];
        new_index += 1;

        // Next 15 bits determine length
        if length_type_id == 0 {
            let mut num_bits_in_packet = 0;

            for _ in 0..15 {
                num_bits_in_packet = num_bits_in_packet << 1;
                num_bits_in_packet |= bits[new_index] as u16;
                new_index += 1;
            }

            let stop_index = new_index + num_bits_in_packet as usize;

            // Add all packets in this counted section as sub-packets
            while new_index < stop_index {
                let (internal_packet, updated_index) = extract_packet(new_index, bits);
                packet.sub_packets.push(internal_packet);
                new_index = updated_index;
            }

            // Notify user if a bad value was given
            if new_index != stop_index {
                println!("Error, bad number of bits given: {}", num_bits_in_packet);
            }
        }
        // Next 11 bits determine number of sub-packets
        else {
            let mut num_packets_in_packet = 0;

            for _ in 0..11 {
                num_packets_in_packet = num_packets_in_packet << 1;
                num_packets_in_packet |= bits[new_index] as u16;
                new_index += 1;
            }

            for _ in 0..num_packets_in_packet {
                let (internal_packet, updated_index) = extract_packet(new_index, bits);
                packet.sub_packets.push(internal_packet);
                new_index = updated_index;
            }
        }

        return (packet, new_index);
    }
}

fn extract_all_packets(bits: &Vec<u8>) -> Packet {
    let (full_packet, _) = extract_packet(0, bits);
    return full_packet;
}

fn get_packet_sum(packet: &Packet) -> u64 {
    let mut sum = 0;

    for sub_packet in &packet.sub_packets {
        sum += solve_packet(sub_packet);
    }

    return sum;
}

fn get_packet_product(packet: &Packet) -> u64 {
    let mut product = 1;

    for sub_packet in &packet.sub_packets {
        product *= solve_packet(sub_packet);
    }

    return product;
}

fn get_packet_min(packet: &Packet) -> u64 {
    let mut min = u64::MAX;

    for sub_packet in &packet.sub_packets {
        let new_val = solve_packet(sub_packet);

        if new_val < min {
            min = new_val;
        }
    }

    return min;
}

fn get_packet_max(packet: &Packet) -> u64 {
    let mut max = 0;

    for sub_packet in &packet.sub_packets {
        let new_val = solve_packet(sub_packet);

        if new_val > max {
            max = new_val;
        }
    }

    return max;
}

fn get_packet_gt(packet: &Packet) -> u64 {
    return if solve_packet(&packet.sub_packets[0]) > solve_packet(&packet.sub_packets[1]) {1} else {0};
}

fn get_packet_lt(packet: &Packet) -> u64 {
    return if solve_packet(&packet.sub_packets[0]) < solve_packet(&packet.sub_packets[1]) {1} else {0};
}

fn get_packet_et(packet: &Packet) -> u64 {
    return if solve_packet(&packet.sub_packets[0]) == solve_packet(&packet.sub_packets[1]) {1} else {0};
}

fn solve_packet(packet: &Packet) -> u64 {
    // Return value if packet is a literal
    if !packet.is_operator {
        return packet.value;
    }

    match packet.header.type_id {
        0 => return get_packet_sum(packet),
        1 => return get_packet_product(packet),
        2 => return get_packet_min(packet),
        3 => return get_packet_max(packet),
        5 => return get_packet_gt(packet),
        6 => return get_packet_lt(packet),
        7 => return get_packet_et(packet),
        _ => {
            println!("Error, bad packet type ID");
            return 0;
        }
    }
}

fn main() {
    let input_contents = fs::read_to_string("files/hex_transmission_input")
        .expect("Unable to read from input");

    let lines = input_contents.lines().collect::<Vec<&str>>();
    let hex_input = lines[0];

    let mut nibbles = Vec::<u8>::new();

    for hex_char in hex_input.chars() {
        nibbles.push(hex_to_nibble(hex_char));
    }

    // Split bits into their own values for convenience
    let mut bits = Vec::<u8>::new();

    for i in 0..nibbles.len() {
        bits.push((nibbles[i] >> 3) & 1);
        bits.push((nibbles[i] >> 2) & 1);
        bits.push((nibbles[i] >> 1) & 1);
        bits.push((nibbles[i] >> 0) & 1);
    }

    let all_packets = extract_all_packets(&bits);
    println!("Packet Result: {}", solve_packet(&all_packets));
}
