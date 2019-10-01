extern crate pcap;

fn main() {
    for device in pcap::Device::list().unwrap() {
        println!("Found device! {:?}", device);

        let mut cap = device.open().unwrap();

        let packet = cap.next();

        println!("got a packet! {:?}", packet);
    }
}