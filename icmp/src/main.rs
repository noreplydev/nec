use pnet;

fn main() {
    let interfaces = pnet::datalink::interfaces()
        .into_iter()
        .filter(|interface| interface.name == "eth0" || interface.name == "en0");
}
