extern crate clap;
extern crate pnet;
extern crate reqwest;

use pnet::datalink;
use std::{thread, time};
use std::fs;
use std::fs::OpenOptions;
use clap::{Arg, App};

fn main() {
	// Arguments parser and help
	let matches = App::new("PIPS - Public IP Status").version("1.0")
						.arg(Arg::with_name("lan").short("l").long("lan").value_name("INTERFACE").help("LAN interface name to monitor").takes_value(true).required(true))
						.arg(Arg::with_name("vpn").short("v").long("vpn").value_name("INTERFACE").help("VPN interface name to monitor").takes_value(true).required(true))
						.arg(Arg::with_name("path").short("p").long("path").value_name("FILE_PATH").help("Set the path to the file to update on disk").takes_value(true).required(true))
						.arg(Arg::with_name("time").short("t").long("time").value_name("TIME").help("Set the time in seconds that will elapse before checking to see if a new interface state is true. - Default: 1 Second").takes_value(true))
						.get_matches();

	// Argument variables
	let lan: String = matches.value_of("lan").expect("ERROR READING LAN INTERFACE VALUE!").to_string();
	let vpn: String = matches.value_of("vpn").expect("ERROR READING VPN INTERFACE VALUE!").to_string();
	let path: String = matches.value_of("path").expect("ERROR READING FILE PATH VALUE!").to_string();
	let time: u64 = matches.value_of("time").unwrap_or("1").parse().expect("ERROR PARSING TIME VALUE AS UNSIGNED INTEGER!");

	// Internal tracking variables
	let mut vpn_up: bool = vpn_is_up(vpn.clone());
    let mut pub_ip: String = get_ip();

	// Main logic
    loop {
        if !lan_is_up(lan.clone()) {
            pub_ip = "LAN DOWN!".to_string();
        } else if vpn_up != vpn_is_up(vpn.clone()) {
			write_ip("CONFIRMING PUBLIC IP".to_string(), path.clone());
            pub_ip = get_ip();            
        }
        write_ip(pub_ip.clone(), path.clone());
		vpn_up = vpn_is_up(vpn.clone());
        thread::sleep(time::Duration::from_millis(time*1000));
    }
}

fn vpn_is_up(vpn: String) -> bool {
	datalink::interfaces().iter().any(|x| x.name.contains(&vpn.clone()))
}

fn lan_is_up(lan: String) -> bool {
	datalink::interfaces().iter().any(|x| x.name.contains(&lan.clone()) && x.is_up())
}

fn get_ip() -> String {
	loop {
    	if let Ok(mut request) = reqwest::get("http://ipecho.net/plain") {
        	if let Ok(body) = request.text() {
            	return body;
			}
        }
    }
}

fn write_ip(ip: String, path: String) {
    OpenOptions::new().write(true).create(true).open(path.clone()).expect("Couldn't open the supplied file path!");
    fs::write(path, ip).expect("Couldn't write to the supplied file path!");
}
