
use std::net::IpAddr;

pub struct ClusterDriver{
    node_id: String,
    instance_id: String,
    start_stamp: u64,
    host_private_ip:  IpAddr,
    private_Ip: IpAddr
}

pub struct Cluster{
    pub id: String,
}
