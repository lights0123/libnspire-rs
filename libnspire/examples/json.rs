use std::convert::TryFrom;

fn main() {
    let dev = rusb::open_device_with_vid_pid(0x0451, 0xe012).unwrap();
    let handle = libnspire::Handle::new(dev).unwrap();
    println!(
        "{}",
        serde_json::to_string_pretty(&handle.info().unwrap()).unwrap()
    );
}
