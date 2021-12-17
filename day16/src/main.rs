use std::fs;

#[derive(Debug, Default)]
struct Packet {
    version: usize,
    id: usize,
    value: usize,
    sub_packets: Vec<Packet>,
}

fn hex_to_binary(input: &str) -> String {
    let mut output = String::new();
    for c in input.chars() {
        let parsed;
        match c {
            '0' => parsed = "0000",
            '1' => parsed = "0001",
            '2' => parsed = "0010",
            '3' => parsed = "0011",
            '4' => parsed = "0100",
            '5' => parsed = "0101",
            '6' => parsed = "0110",
            '7' => parsed = "0111",
            '8' => parsed = "1000",
            '9' => parsed = "1001",
            'A' => parsed = "1010",
            'B' => parsed = "1011",
            'C' => parsed = "1100",
            'D' => parsed = "1101",
            'E' => parsed = "1110",
            'F' => parsed = "1111",
            _ => unreachable!(),
        }
        output.push_str(parsed);
    }
    output
}

fn read_chunk(input: &str, start: usize, len: usize) -> &str {
    &input[start..start + len]
}

fn chunk_to_int(chunk: &str) -> usize {
    match usize::from_str_radix(chunk, 2) {
        Ok(val) => val,
        Err(_) => 0,
    }
}

fn parse_packet(input: &str) -> (Packet, &str) {
    let mut packet = Packet::default();
    packet.version = chunk_to_int(read_chunk(input, 0, 3));
    packet.id = chunk_to_int(read_chunk(input, 3, 3));
    let mut local_data = &input[6..];
    match packet.id {
        4 => {
            let mut value = String::new();
            while local_data.starts_with("1") {
                value.push_str(read_chunk(local_data, 1, 4));
                local_data = &local_data[5..];
            }
            value.push_str(read_chunk(local_data, 1, 4));
            local_data = &local_data[5..];
            packet.value = chunk_to_int(&value);
        }
        _ => {
            let length_type_id = chunk_to_int(read_chunk(local_data, 0, 1));
            local_data = &local_data[1..];
            if length_type_id == 0 {
                let total_len_sub_packets = chunk_to_int(read_chunk(local_data, 0, 15));
                local_data = &local_data[15..];
                let mut sub_packet_data = &local_data[..total_len_sub_packets];
                let mut sub_packets = Vec::new();
                while sub_packet_data.len() > 0 {
                    let result = parse_packet(sub_packet_data);
                    sub_packets.push(result.0);
                    sub_packet_data = result.1;
                }
                packet.sub_packets = sub_packets;
                local_data = &local_data[total_len_sub_packets..];
            } else {
                let num_sub_packets = chunk_to_int(read_chunk(local_data, 0, 11));
                local_data = &local_data[11..];
                let mut sub_packets = Vec::new();
                for _ in 0..num_sub_packets {
                    let result = parse_packet(local_data);
                    sub_packets.push(result.0);
                    local_data = result.1;
                }
                packet.sub_packets = sub_packets;
            }
        }
    }

    (packet, local_data)
}

fn sum_packet_versions(packet: &Packet) -> usize {
    return packet.version + packet.sub_packets.iter().map(|p| sum_packet_versions(p)).sum::<usize>();
}

fn invoke(packet: &Packet) -> usize {
    let retval;
    match packet.id {
        0 => {
            retval = packet.sub_packets.iter().map(|p| invoke(p)).sum::<usize>();
        },
        1 => {
            retval = packet.sub_packets.iter().map(|p| invoke(p)).product::<usize>();
        },
        2 => {
            retval = packet.sub_packets.iter().map(|p| invoke(p)).min().unwrap();
        },
        3 => {
            retval = packet.sub_packets.iter().map(|p| invoke(p)).max().unwrap();
        },
        4 => {
            retval = packet.value
        },
        5 => {
            if invoke(&packet.sub_packets[0]) > invoke(&packet.sub_packets[1]) {
                retval = 1
            } else {
                retval = 0
            }
        },
        6 => {
            if invoke(&packet.sub_packets[0]) < invoke(&packet.sub_packets[1]) {
                retval = 1
            } else {
                retval = 0
            }
        },
        7 => {
            if invoke(&packet.sub_packets[0]) == invoke(&packet.sub_packets[1]) {
                retval = 1
            } else {
                retval = 0
            }
        },
        _ => unreachable!(),
    }
    retval
}

fn part1(input: &str) {
    let binary = hex_to_binary(input);
    let (packet, _) = parse_packet(&binary);
    println!("part1: {}", sum_packet_versions(&packet));
}

fn part2(input: &str) {
    let binary = hex_to_binary(input);
    let (packet, _) = parse_packet(&binary);
    println!("part2: {}", invoke(&packet));
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read input");
    part1(&input);
    part2(&input);
}
