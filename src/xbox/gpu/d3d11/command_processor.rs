//! Xbox command processor layer.
//!
//! Planned home for PFIFO/pushbuffer decode, D3D method routing, and draw
//! command production. Kept separate from the D3D11 graphics system so command
//! semantics can move toward Cxbx-R/xemu-style structure without dragging
//! render-target ownership along with it.

#[cfg(windows)]
pub(super) fn parse_draw_ranges_env(name: &str) -> Vec<(u32, u32)> {
    let Ok(raw) = std::env::var(name) else {
        return Vec::new();
    };
    let nums: Vec<u32> = raw
        .split(|c: char| c == ',' || c == ';' || c == ':' || c == '-' || c.is_whitespace())
        .filter_map(|part| part.trim().parse::<u32>().ok())
        .collect();
    let mut ranges = Vec::new();
    for pair in nums.chunks(2) {
        match pair {
            [single] => ranges.push((*single, *single)),
            [a, b] => ranges.push(((*a).min(*b), (*a).max(*b))),
            _ => {}
        }
    }
    ranges
}

#[cfg(windows)]
pub(super) fn parse_u32_list_env(name: &str) -> Vec<u32> {
    let Ok(raw) = std::env::var(name) else {
        return Vec::new();
    };
    raw.split(|c: char| c == ',' || c == ';' || c == ':' || c.is_whitespace())
        .filter_map(|part| {
            let part = part.trim();
            if part.is_empty() {
                return None;
            }
            let part = part
                .strip_prefix("0x")
                .or_else(|| part.strip_prefix("0X"))
                .unwrap_or(part);
            u32::from_str_radix(part, 16)
                .ok()
                .or_else(|| part.parse::<u32>().ok())
        })
        .collect()
}

#[cfg(windows)]
pub(super) fn draw_index_in_ranges(draw_index: u32, ranges: &[(u32, u32)]) -> bool {
    ranges
        .iter()
        .any(|(start, end)| draw_index >= *start && draw_index <= *end)
}
