use autocontrol::AutoControl;

fn main() {
    let auto = AutoControl::new();
    auto.set_enabled(false).unwrap();
    auto.wait(3000).unwrap();
    auto.set_enabled(true).unwrap();
}
