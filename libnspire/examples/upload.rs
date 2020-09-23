use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;

fn main() {
    let dev = rusb::open_device_with_vid_pid(0x0451, 0xe012).unwrap();
    let handle = libnspire::Handle::new(dev).unwrap();
    let mut buf = vec![];
    File::open(std::env::current_exe().unwrap())
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();
    handle.write_file("test.tns", &buf, &mut |prog| println!("{}", prog));
}
