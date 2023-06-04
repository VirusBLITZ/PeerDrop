use std::env;

use ifcfg::IfCfg;

pub fn get_own_ip() -> String {
    let mut ip = "".to_string();
    let binding = ifcfg::IfCfg::get().unwrap();
    let mut guessed_iface = binding
        .iter()
        .filter(|iface| {
            iface
                .addresses
                .iter()
                .any(|addr| !iface.name.contains("lo") && addr.hop.is_some() && addr.address.unwrap().is_ipv4())
        })
        .next();

    match guessed_iface {
        Some(iface) => {
            println!("Found interface {}", iface.name);
            iface.addresses.iter().for_each(|addr| {
                if addr.address.unwrap().is_ipv4() {
                    ip = addr.address.unwrap().to_string();
                }
            });
            ip
        }
        None => panic!("No IP found for interface {}", guessed_iface.unwrap().name),
    }
}
