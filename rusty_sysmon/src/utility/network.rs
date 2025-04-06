use local_ip_address::{self, linux::{local_ip, local_ipv6}};

pub struct NetworkUtill{}

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
}