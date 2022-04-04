use core::panic;
use std::fs;

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    body: Value
}

#[derive(Debug)]
enum Value {
    Literal(usize),
    Operation(Vec<Packet>)
}

impl Packet {
    fn read(input: &mut String) -> Option<Packet> {

        if input.len() < 11 { // minimum length of a packet (6+5 for a value packet)
            return None;
        }

        let version: String = input.drain(..3).collect();
        let version = u8::from_str_radix(&version, 2).unwrap();
        let type_id: String = input.drain(..3).collect();
        let type_id = u8::from_str_radix(&type_id, 2).unwrap();

        let body: Value = match type_id {
            4 => {
                let mut bytes = Vec::new();
                loop {
                    let group: String = input.drain(..5).collect();
                    let group_split = group.split_at(1);
                    bytes.push(group_split.1.to_string());
                    if group_split.0 == "0" {
                        break;
                    }
                }
                
                // // drain remaining zeros
                // let consumed_bits = 6 + (bytes.len() * 5); // version + id + value-parts x 5
                // let drain_counter = (consumed_bits + 7) / 8; // rounding up after division
                // input.drain(..drain_counter);

                // combine bytes to packet value
                let value = bytes.concat();
                let value = usize::from_str_radix(&value, 2).unwrap();
                Value::Literal(value)
            },
            _ => {
                let length_type = input.drain(..1).next().unwrap();
                match length_type {
                    '0' => {
                        let sub_packets_length: String = input.drain(..15).collect();
                        let sub_packets_length = usize::from_str_radix(&sub_packets_length, 2).unwrap();
                        let mut sub_packets_bits: String = input.drain(..sub_packets_length).collect();

                        // parse sub packets
                        let mut sub_packets: Vec<Packet> = Vec::new();
                        while let Some(packet) = Packet::read(&mut sub_packets_bits) {
                            sub_packets.push(packet);
                        }

                        // // drain remaining bits:
                        // let consumed_bits = 7 + 15 + sub_packets_length;
                        // let drain_counter = (consumed_bits / 7) / 8;
                        // input.drain(..drain_counter);

                        Value::Operation(sub_packets)
                    },
                    '1' => {
                        let sub_packets_count: String = input.drain(..11).collect();
                        let sub_packets_count = usize::from_str_radix(&sub_packets_count, 2).unwrap();

                        let mut sub_packets: Vec<Packet> = Vec::new();
                        for _ in 0..sub_packets_count {
                            sub_packets.push(Packet::read(input).unwrap());
                        }

                        Value::Operation(sub_packets)
                    }
                    _ => panic!()
                }
            }
        };

        Some(Packet { version, type_id, body })
    }
}

pub fn puzzle1() {
    let input = fs::read_to_string("input-day16.txt").expect("error reading input file");
    let bits_input = (0..input.len()).step_by(2).map(|i| format!("{:08b}", u8::from_str_radix(&input[i..=i+1], 16).unwrap()));
    let mut bits_input = String::from_iter(bits_input);
    //dbg!(&bits_input);

    let p = Packet::read(&mut bits_input).unwrap();
    
    let sum = count(&p);
    println!("result is {}", sum);
}

fn count(packet: &Packet) -> usize {
    packet.version as usize + match &packet.body {
        Value::Literal(_) => 0,
        Value::Operation(subpackets) => subpackets.iter().fold(0, |acc, p| acc + count(p))
    }
}

pub fn puzzle2() {
    let input = fs::read_to_string("input-day16.txt").expect("error reading input file");
    let bits_input = (0..input.len()).step_by(2).map(|i| format!("{:08b}", u8::from_str_radix(&input[i..=i+1], 16).unwrap()));
    let mut bits_input = String::from_iter(bits_input);

    let p = Packet::read(&mut bits_input).unwrap();
    
    let result = process(&p);
    println!("result is {}", result);
}

fn process(packet: &Packet) -> usize {
    match &packet.body {
        Value::Literal(value) => *value,
        Value::Operation(subpackets) => {
            match packet.type_id {
                0 => subpackets.iter().map(|p| process(p)).sum(),
                1 => subpackets.iter().fold(1, |acc, p| acc * process(p)),
                2 => subpackets.iter().map(|p| process(p)).min().unwrap(),
                3 => subpackets.iter().map(|p| process(p)).max().unwrap(),
                5 => if process(&subpackets[0]) > process(&subpackets[1]) { 1 } else { 0 },
                6 => if process(&subpackets[0]) < process(&subpackets[1]) { 1 } else { 0 },
                7 => if process(&subpackets[0]) == process(&subpackets[1]) { 1 } else { 0 },
                _ => panic!()
            }
        }
    }
}