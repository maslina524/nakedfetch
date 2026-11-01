use core::fmt::Write;

use common::{formatf, io::Stdout};
use image::Image;

use crate::base64;

const CHUNK: usize = 4096;

pub fn print_image(
    image: &Image,
    cols: Option<usize>,
    rows: Option<usize>,
    image_id: u32,
) -> core::fmt::Result {
    let (w, h) = image.get_size();

    let raw = image.as_rgba_bytes();
    let b64 = base64::encode(&raw);

    let mut ctrl = formatf!("a=T,f=32,s={w},v={h},C=1");
    if image_id != 0 {
        ctrl.push_str(&formatf!(",i={image_id}"));
    }
    if let Some(c) = cols {
        ctrl.push_str(&formatf!(",c={c}"));
    }
    if let Some(r) = rows {
        ctrl.push_str(&formatf!(",r={r}"));
    }

    let bytes = b64.as_bytes();

    let mut first = true;
    let mut offset = 0usize;
    let mut stdout = Stdout::new();
    while offset < bytes.len() {
        let end = (offset + CHUNK).min(bytes.len());
        let more = end < bytes.len();
        let m = i32::from(more);

        if first {
            let ctrl_with_m = format_args!("{ctrl},m={m}");
            stdout.write_fmt(format_args!("\x1b_G{ctrl_with_m};"))?;
            first = false;
        } else {
            stdout.write_fmt(format_args!("\x1b_Gm={m};"))?;
        }

        stdout.write_bytes(&bytes[offset..end]);
        stdout.write_bytes(b"\x1b\\");

        offset = end;
    }

    Ok(())
}
