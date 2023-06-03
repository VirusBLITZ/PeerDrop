// use std::env;

// use ifcfg::IfCfg;

// pub fn get_subnetmask(interface: &str) -> String {
//     // get os
//     let mut interfaces: Vec<&IfCfg> = vec![];
//     let binding = ifcfg::IfCfg::get().unwrap();
//     binding
//         .iter()
//         .filter(|iface| iface.addresses.iter().any(|addr| addr.hop.is_some()))
//         .for_each(|iface: &IfCfg| interfaces.push(iface));

//     for iface in interfaces {
//         println!("Interface: {}", iface.name);
//         println!("IP: {:#?}", iface.addresses);
//         println!();
//     }
//     "".to_string()
// }
