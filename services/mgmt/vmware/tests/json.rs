#[cfg(feature = "package-2023-03-01")]
#[test]
fn test_list_dhcp_configurations_deserialization() -> anyhow::Result<()> {
    use anyhow::{bail, ensure};
    use azure_mgmt_vmware::package_2023_03_01::models::*;
    // copied from specification\vmware\resource-manager\Microsoft.AVS\stable\2023-03-01\examples\WorkloadNetworks_ListDhcpConfigurations.json
    let json = br#"
    {
        "value": [
          {
            "id": "/subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.AVS/privateClouds/cloud1/workloadNetworks/default/dhcpConfigurations/dhcpConfigurations1",
            "name": "dhcp1",
            "properties": {
              "displayName": "dhcpConfigurations1",
              "dhcpType": "SERVER",
              "segments": [
                "segment1",
                "segment2"
              ],
              "serverAddress": "40.1.5.1/24",
              "leaseTime": 86400,
              "revision": 1
            },
            "type": "Microsoft.AVS/privateClouds/workloadNetworks/dhcpConfigurations"
          }
        ]
      }
      "#;

    let dhcp_list: WorkloadNetworkDhcpList = serde_json::from_slice(json)?;
    ensure!(dhcp_list.value.len() == 1);
    let dhcp = &dhcp_list.value[0];
    ensure!(dhcp.proxy_resource.resource.name == Some("dhcp1".to_string()));
    match &dhcp.properties {
        Some(WorkloadNetworkDhcpEntityUnion::Server(server)) => {
            ensure!(server.workload_network_dhcp_entity.display_name.as_deref() == Some("dhcpConfigurations1"));
            ensure!(server.workload_network_dhcp_entity.segments.len() == 2);
            ensure!(server.server_address.as_deref() == Some("40.1.5.1/24"));
            ensure!(server.lease_time == Some(86400));
            ensure!(server.workload_network_dhcp_entity.revision == Some(1));
        }
        _ => bail!("expected Server"),
    }
    Ok(())
}
