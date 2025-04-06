use std::{collections::HashMap, default, thread, time::Duration};


use local_ip_address::{self, linux::{local_ip, local_ipv6}};
use sysinfo::{Networks, System};

pub struct NetworkUtill{}

#[derive(Debug)]
pub struct NetworkTrafficDetail{
    pub interface_name: String,
    pub upload_per_sec: u64,
    pub download_per_sec: u64
}


#[derive(Debug)]
pub struct ResNetworkTrafficDetail{
    pub length: u32,
    pub data : Vec<NetworkTrafficDetail>,
}

impl Default for ResNetworkTrafficDetail {
    fn default() -> Self {
        Self {
            length: 0,
            data: vec![],
        }
    }
}

impl ResNetworkTrafficDetail {
    pub fn sort_by_download_desc(&mut self) {
        self.data.sort_by(|a, b| b.download_per_sec.cmp(&a.download_per_sec));
    }
}
impl NetworkUtill {
    pub fn get_host_ipv4() -> String {
        match local_ip() {
            Ok(data) => data.to_string(),
            Err(_) => {
                String::from("can't retrive ipv6 from this machine")
            }
        }
    }

    pub fn get_host_ipv6() -> String {
        match local_ipv6() {
            Ok(data) => data.to_string(),
            Err(_) => {
                String::from("can't retrive ipv6 from this machine")
            }
        }
    }

    pub fn get_network_interface_detail() {
        unimplemented!()
    }

    pub fn get_network_traffic_per_second() -> ResNetworkTrafficDetail  {
        
        // get network information
        let mut networks = Networks::new_with_refreshed_list();
        
        // create the initial value
        let mut old_data = HashMap::new();
        
        // insert old data to compare later
        for (name, data) in &networks {
            old_data.insert(name.clone(), (data.received(), data.transmitted()));
        }
        
        thread::sleep(Duration::from_secs(1));

        // refresh network then try to compare value
        networks.refresh(true);

        // define variable to return res resulf of this funtion
        let mut result = ResNetworkTrafficDetail::default();

        // start copare network
        for (name, data) in &networks {
            if let Some((old_received, old_transmitted)) = old_data.get(name) {
                
                let download = data.received().saturating_sub(*old_received);
                let upload = data.transmitted().saturating_sub(*old_transmitted);
                
                let temp = NetworkTrafficDetail {
                    interface_name: name.to_owned(),
                    upload_per_sec: upload,
                    download_per_sec: download

                };
                
                // println!("{}: ↓ {} B/s | ↑ {} B/s", temp.interface_name ,temp.download_per_sec, temp.upload_per_sec);
                result.data.push(temp);
            }
        }
        result.sort_by_download_desc();
        result

    }
}