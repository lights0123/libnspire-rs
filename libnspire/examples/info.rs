use std::convert::TryFrom;

fn main() {
    let dev = rusb::open_device_with_vid_pid(0x0451, 0xe012).unwrap();
    let handle = libnspire::Handle::new(dev).unwrap();
    dbg!(handle.info());
    dbg!(handle.list_dir("/"));
    dbg!(image::DynamicImage::try_from(handle.screenshot().unwrap())
        .unwrap()
        .save("test.png"));
}
