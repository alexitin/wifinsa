use std::{process, io};

use pcap::Linktype;

use crate::{frame::{WifiDevice, NetSignals}, device};

pub struct AirNoise {
    pub linktype: Option<Linktype>,
    pub radio_air: Vec<NetSignals>,
}

impl AirNoise {
    pub fn show(self, wifi_device: &WifiDevice) {
        let mut list_ssid: Vec<&String> = Vec::new();
        for air in &self.radio_air {
            let ssid_set: Vec<_> = air.ssid_rssi.keys().clone().collect();
            for ssid in ssid_set {
                if !list_ssid.contains(&ssid) {list_ssid.push(ssid)}
            }
        
        }
        let home_ssid = if !list_ssid.is_empty() {
            choice_home_ssid(list_ssid)
        } else {
            "".to_owned()
        };

        let mode = match wifi_device.mode {
            device::DeviceMode::Monitor => "monitor".to_owned(),
            device::DeviceMode::Promiscouos => "promiscous".to_owned(),
            device::DeviceMode::Normal => "normal".to_owned(),
            
        };
        let linktype = match self.linktype {
            Some(dlt) => dlt.get_name().unwrap(),
            None => "-".to_owned(),
        };
        
        println!("Device: {}, Mode: {}, Linktype: {}", wifi_device.name, mode, linktype);
        for mut air in self.radio_air { 
            air.ssid_rssi.remove_entry(&home_ssid);
            let number_ap = air.ssid_rssi.len();
            let mut signals: Vec<_> = air.ssid_rssi.values().cloned().collect();
            signals.sort();
            let signal_max = signals.last().unwrap_or(&0);
            let unit_measurement = if signal_max < &0 {
                "dBm".to_owned()
            } else {
                "".to_owned()
            };
            println!("Channel: {}, Number access point: {}, Max signal: {} {};", 
                air.channel, number_ap, signal_max, unit_measurement);
        }
    }
}

fn choice_home_ssid(list_ssid: Vec<&String>) -> String {
    let mut i = 0;
    for ssid in &list_ssid {
        i += 1;
        println!("{}. {}", &i, ssid);
    }
    println!("Choose your home ssid, or press 0 if home ssid does not exist:");

    let buf = loop {
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .unwrap_or_else(|err| {
                println!("Failed read yuor choice: {}", err);
                process::exit(1)
            });
        let buf = match buf.trim().parse::<usize>() {
            Ok(num) => num,
            Err(_) => {
                println!("Incorrect choice");
                continue;
            },
        };
        if buf > list_ssid.len() {
            println!("Incorrect choice: {}", &buf);
            continue;
        } else {
            break buf
        };
    };
    if buf > 0 {
        list_ssid[buf - 1].to_owned()
    } else {
        "0".to_owned()
    }
}