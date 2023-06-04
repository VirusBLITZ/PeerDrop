use std::env;

use ifcfg::IfCfg;

pub fn get_own_ip() -> String {
    let mut ip = "".to_string();
    let binding = ifcfg::IfCfg::get().unwrap();
    let guessed_iface = binding
        .iter()
        .filter(|iface| {
            iface.addresses.iter().any(|addr| {
                iface.name != "lo"
                    && addr.hop.is_some()
                    && addr.address.unwrap().is_ipv4()
                    && !addr.address.unwrap().ip().is_loopback()
            })
        })
        .next();

    match guessed_iface {
        Some(iface) => {
            println!("Found interface {}", iface.name);
            println!("Found interface {:#?}", iface.addresses);
            for addr in &iface.addresses {
                if addr.hop.is_some() && addr.address.unwrap().is_ipv4() {
                    ip = addr.address.unwrap().ip().to_string();
                    break;
                }
            }
            ip
        }
        None => panic!("No IP found for interface {}", guessed_iface.unwrap().name),
    }
}
