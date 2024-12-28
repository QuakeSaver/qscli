use cidr::Ipv4Cidr;
use futures::future::join_all;
use local_ip_address::{list_afinet_netifas, local_ip};
use log::{debug, info};
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;
use tokio::time::timeout;

pub async fn scan(interface: Option<String>) -> Vec<(Ipv4Addr, Option<String>)> {
    let local_ip = get_local_ip(interface);

    info!("local IP address: {:?}", local_ip);
    let ip_address = to_ipv4_address(local_ip);
    let ipv4 = to_base_address(ip_address);

    let ip_range = Ipv4Cidr::new(ipv4, 24).unwrap();
    let results = probe_ip_range(ip_range).await;
    debug!("results: {:?}", results);
    results
}

async fn probe_host(address: Ipv4Addr, port: u16) -> Option<(Ipv4Addr, Option<String>)> {
    let response = reqwest::get(format!("http://{}:{}", address, port)).await;
    let response_text = match response {
        Ok(response) => response.text().await.ok(),
        Err(e) => {
            debug!("error: {}", e);
            None
        }
    };
    Some((address, response_text))
}

async fn probe_ip_range<'a>(ip_range: Ipv4Cidr) -> Vec<(Ipv4Addr, Option<String>)> {
    let mut futures = Vec::new();
    for ip in ip_range {
        debug!("probing {:?}", ip);
        futures.push(timeout(
            Duration::from_millis(1000),
            probe_host(ip.address(), 27740),
        ));
    }
    let results = join_all(futures).await;
    let results = results
        .into_iter()
        .filter_map(|result| result.ok())
        .collect::<Vec<_>>();

    results.into_iter().map(|e| e.unwrap()).collect::<Vec<_>>()
}

fn to_base_address(ip_address: Ipv4Addr) -> Ipv4Addr {
    let mut octets = ip_address.octets();
    octets[3] = 0;
    Ipv4Addr::from(octets)
}

fn to_ipv4_address(local_ip: IpAddr) -> Ipv4Addr {
    match local_ip {
        IpAddr::V4(ip4) => ip4,
        IpAddr::V6(_) => panic!("IPv6 addresses are not supported"),
    }
}

fn get_local_ip(interface: Option<String>) -> IpAddr {
    let local_ip = match interface {
        None => local_ip().unwrap(),
        Some(interface) => {
            let network_interfaces = list_afinet_netifas().unwrap();
            let (_, ip) = network_interfaces
                .iter()
                .find(|(name, _)| *name == interface)
                .expect("failed");
            *ip
        }
    };
    local_ip
}
