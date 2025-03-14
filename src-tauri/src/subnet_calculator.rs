use serde::Serialize;
use std::net::Ipv4Addr;

#[derive(Serialize)]
pub struct SubnetInfo {
    netmask: String,
    wildcard: String,
    network_address: String,
    broadcast: String,
    first_host: String,
    last_host: String,
    hosts: u32,
}

impl SubnetInfo {
    fn new(ip: Ipv4Addr, prefix: u8) -> Self {
        // 计算子网掩码
        let netmask_u32 = u32::MAX.checked_shl(32 - prefix as u32).unwrap_or(0);
        let netmask = Ipv4Addr::from(netmask_u32);
        
        // 计算反掩码
        let wildcard_u32 = !netmask_u32;
        let wildcard = Ipv4Addr::from(wildcard_u32);

        // 计算网络地址
        let network_u32 = u32::from(ip) & netmask_u32;
        let network = Ipv4Addr::from(network_u32);

        // 计算广播地址
        let broadcast_u32 = network_u32 | wildcard_u32;
        let broadcast = Ipv4Addr::from(broadcast_u32);

        // 计算可用主机范围
        let (first_host, last_host, hosts) = match prefix {
            32 => (network, network, 1),
            31 => (network, broadcast, 2),
            _ => (
                Ipv4Addr::from(network_u32 + 1),
                Ipv4Addr::from(broadcast_u32 - 1),
                2u32.pow(32 - prefix as u32) - 2
            ),
        };

        Self {
            netmask: netmask.to_string(),
            wildcard: wildcard.to_string(),
            network_address: network.to_string(),
            broadcast: broadcast.to_string(),
            first_host: first_host.to_string(),
            last_host: last_host.to_string(),
            hosts,
        }
    }
}

#[tauri::command]
pub fn calculate_subnet_info(ip: String, mask: u8) -> Result<SubnetInfo, String> {
    // 转换 IP 地址
    let ip_addr = ip.parse::<Ipv4Addr>()
        .map_err(|_| "无效的 IP 地址格式")?;

    // 验证掩码位数
    if mask > 32 {
        return Err("子网掩码位数必须在 0-32 之间".into());
    }

    // 计算子网信息
    Ok(SubnetInfo::new(ip_addr, mask))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_subnet_info() {
        // 测试普通情况
        let result = calculate_subnet_info("192.168.1.0".to_string(), 24)
            .expect("应该成功计算子网信息");
        assert_eq!(result.netmask, "255.255.255.0");
        assert_eq!(result.network_address, "192.168.1.0");
        assert_eq!(result.broadcast, "192.168.1.255");
        assert_eq!(result.hosts, 254);

        // 测试 /32 网络
        let result = calculate_subnet_info("192.168.1.1".to_string(), 32)
            .expect("应该成功计算 /32 网络");
        assert_eq!(result.hosts, 1);

        // 测试 /31 网络
        let result = calculate_subnet_info("192.168.1.0".to_string(), 31)
            .expect("应该成功计算 /31 网络");
        assert_eq!(result.hosts, 2);
    }

    #[test]
    fn test_invalid_inputs() {
        // 测试无效的 IP 地址
        assert!(calculate_subnet_info("256.256.256.256".to_string(), 24).is_err());
        
        // 测试无效的掩码位数
        assert!(calculate_subnet_info("192.168.1.0".to_string(), 33).is_err());
    }
}