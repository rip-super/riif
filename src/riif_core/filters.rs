use image::{Rgba, RgbaImage};

fn get_byte(row: Option<&[u8]>, i: usize) -> u8 {
    if let Some(r) = row {
        r.get(i).copied().unwrap_or(0)
    } else {
        0
    }
}

// --- Add Filters to Image ---

// Filter Number: 0
fn none_filter(curr: &[u8], _prev: Option<&[u8]>) -> Vec<u8> {
    curr.to_vec()
}

// Filter Number: 1
fn sub_filter(curr: &[u8], _prev: Option<&[u8]>) -> Vec<u8> {
    let mut out = Vec::with_capacity(curr.len());
    for i in 0..curr.len() {
        let a = if i >= 4 { curr[i - 4] } else { 0 };
        out.push(curr[i].wrapping_sub(a));
    }
    out
}

// Filter Number: 2
fn up_filter(curr: &[u8], prev: Option<&[u8]>) -> Vec<u8> {
    let mut out = Vec::with_capacity(curr.len());
    for (i, &val) in curr.iter().enumerate() {
        let b = get_byte(prev, i);
        out.push(val.wrapping_sub(b));
    }
    out
}

// Filter Number: 3
fn average_filter(curr: &[u8], prev: Option<&[u8]>) -> Vec<u8> {
    let mut out = Vec::with_capacity(curr.len());
    for i in 0..curr.len() {
        let a = if i >= 4 { curr[i - 4] } else { 0 };
        let b = get_byte(prev, i);
        let avg = ((a as u16 + b as u16) / 2) as u8;
        out.push(curr[i].wrapping_sub(avg));
    }
    out
}

// Filter Number: 4
fn paeth_filter(curr: &[u8], prev: Option<&[u8]>) -> Vec<u8> {
    fn paeth_predictor(a: u8, b: u8, c: u8) -> u8 {
        let p = a as i32 + b as i32 - c as i32;
        let pa = (p - a as i32).abs();
        let pb = (p - b as i32).abs();
        let pc = (p - c as i32).abs();

        if pa <= pb && pa <= pc {
            a
        } else if pb <= pc {
            b
        } else {
            c
        }
    }

    let mut out = Vec::with_capacity(curr.len());
    for i in 0..curr.len() {
        let a = if i >= 4 { curr[i - 4] } else { 0 };
        let b = get_byte(prev, i);
        let c = if i >= 4 { get_byte(prev, i - 4) } else { 0 };
        out.push(curr[i].wrapping_sub(paeth_predictor(a, b, c)));
    }
    out
}

// Median Sum of Absolute Differences Heuristic
fn best_filter(curr: &[u8], prev: Option<&[u8]>) -> (u8, Vec<u8>) {
    let filters = [
        none_filter,
        sub_filter,
        up_filter,
        average_filter,
        paeth_filter,
    ];

    let (best_idx, best_row) = filters
        .iter()
        .enumerate()
        .map(|(i, f)| (i as u8, f(curr, prev)))
        .min_by_key(|(_, row)| row.iter().map(|&b| b as u32).sum::<u32>())
        .unwrap();

    (best_idx, best_row)
}

pub fn apply_filters(img: &RgbaImage) -> (Vec<u8>, Vec<u8>) {
    let (width, height) = img.dimensions();
    let row_len = (width * 4) as usize;

    let mut output = Vec::with_capacity((width * height * 4) as usize);
    let mut filters_used = Vec::with_capacity(height as usize);

    let mut prev_row = None;

    for y in 0..height {
        let mut row = Vec::with_capacity(row_len);
        for x in 0..width {
            let px = img.get_pixel(x, y);
            row.extend(px.0);
        }

        let (filter_type, filtered_row) = best_filter(&row, prev_row.as_deref());
        filters_used.push(filter_type);
        output.extend(filtered_row);

        prev_row = Some(row)
    }

    (filters_used, output)
}

// --- Remove Filters From Image ---

fn unfilter_none(curr: &[u8]) -> Vec<u8> {
    curr.to_vec()
}

fn unfilter_sub(curr: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(curr.len());
    for i in 0..curr.len() {
        let left = if i >= 4 { out[i - 4] } else { 0 };
        out.push(curr[i].wrapping_add(left));
    }
    out
}

fn unfilter_up(curr: &[u8], prev: Option<&[u8]>) -> Vec<u8> {
    let mut out = Vec::with_capacity(curr.len());
    for i in 0..curr.len() {
        let up = prev.map_or(0, |p| p[i]);
        out.push(curr[i].wrapping_add(up));
    }
    out
}

fn unfilter_average(curr: &[u8], prev: Option<&[u8]>) -> Vec<u8> {
    let mut out = Vec::with_capacity(curr.len());
    for i in 0..curr.len() {
        let left = if i >= 4 { out[i - 4] } else { 0 };
        let up = prev.map_or(0, |p| p[i]);
        let avg = ((left as u16 + up as u16) / 2) as u8;
        out.push(curr[i].wrapping_add(avg));
    }
    out
}

fn unfilter_paeth(curr: &[u8], prev: Option<&[u8]>) -> Vec<u8> {
    fn paeth_predictor(a: u8, b: u8, c: u8) -> u8 {
        let p = a as i32 + b as i32 - c as i32;
        let pa = (p - a as i32).abs();
        let pb = (p - b as i32).abs();
        let pc = (p - c as i32).abs();

        if pa <= pb && pa <= pc {
            a
        } else if pb <= pc {
            b
        } else {
            c
        }
    }

    let mut out = Vec::with_capacity(curr.len());
    for i in 0..curr.len() {
        let left = if i >= 4 { out[i - 4] } else { 0 };
        let up = prev.map_or(0, |p| p[i]);
        let up_left = if i >= 4 {
            prev.map_or(0, |p| p[i - 4])
        } else {
            0
        };
        let paeth = paeth_predictor(left, up, up_left);
        out.push(curr[i].wrapping_add(paeth));
    }
    out
}

pub fn remove_filters(width: u32, height: u32, filters: &[u8], filtered_bytes: &[u8]) -> RgbaImage {
    let row_len = (width * 4) as usize;
    let mut img = RgbaImage::new(width, height);
    let mut prev_row: Option<Vec<u8>> = None;

    for (y, (&filter, raw_row)) in filters
        .iter()
        .zip(filtered_bytes.chunks(row_len))
        .enumerate()
    {
        let row = match filter {
            0 => unfilter_none(raw_row),
            1 => unfilter_sub(raw_row),
            2 => unfilter_up(raw_row, prev_row.as_deref()),
            3 => unfilter_average(raw_row, prev_row.as_deref()),
            4 => unfilter_paeth(raw_row, prev_row.as_deref()),
            _ => panic!("Unknown filter type: {}", filter),
        };

        for x in 0..width as usize {
            let px_start = x * 4;
            let px = &row[px_start..px_start + 4];
            img.put_pixel(x as u32, y as u32, Rgba([px[0], px[1], px[2], px[3]]));
        }

        prev_row = Some(row);
    }

    img
}
