use crate::utils::lines;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn start_idx(packet_marker_filename: &str) -> usize {
    let mut start_idx = 0;
    if let Ok(mut lines) = lines(packet_marker_filename) {
        if let Some(Ok(packet)) = lines.next() {
            let mut packet_chars_queue: VecDeque<char> = VecDeque::new();
            for packet_char in packet.chars() {
                packet_chars_queue.push_front(packet_char);
                start_idx += 1;
                if packet_chars_queue.len() < 4 {
                    continue;
                }
                let packet_chars_set: HashSet<char> = HashSet::from_iter(packet_chars_queue.clone());
                if packet_chars_set.len() == 4 {
                    break;
                }
                packet_chars_queue.pop_back();
            }
        }
    }
    start_idx
}
