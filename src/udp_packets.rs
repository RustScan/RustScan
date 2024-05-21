pub fn craft_snmptrap_packet() -> Vec<u8> {
    let version = vec![0x30, 0x82]; // SNMP version
    let community_string = vec![0x04, 0x06, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63]; // Community string "public"
    let pdu_type = vec![0xa4, 0x82]; // PDU type for SNMP Trap
    let enterprise_oid = vec![0x06, 0x09, 0x2b, 0x06, 0x01, 0x02, 0x01, 0x01, 0x01, 0x00]; // Enterprise OID

    // Combine the components into a single SNMP Trap packet
    [version, community_string, pdu_type, enterprise_oid].concat()
}

pub fn craft_snmp_getrequest_packet() -> Vec<u8> {
    let version = vec![0x30, 0x29, 0x02, 0x01, 0x00]; // SNMP version (v1)
    let community_string = vec![0x04, 0x06, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63]; // Community string "public"
    let pdu_type = vec![0xa0, 0x1c]; // PDU type for SNMP GetRequest
    let request_id = vec![0x02, 0x04, 0x7a, 0x69, 0x67, 0x71]; // Request ID
    let error_status = vec![0x02, 0x01, 0x00]; // Error status
    let error_index = vec![0x02, 0x01, 0x00]; // Error index
    let variable_bindings = vec![
        0x30, 0x0e, 0x30, 0x0c, 0x06, 0x08, 0x2b, 0x06, 0x01, 0x02, 0x01, 0x01, 0x01, 0x00, 0x05,
        0x00,
    ]; // Variable bindings

    // Combine the components into a single SNMP GetRequest packet
    [
        version,
        community_string,
        pdu_type,
        request_id,
        error_status,
        error_index,
        variable_bindings,
    ]
    .concat()
}

pub fn craft_snmp_packet() -> Vec<u8> {
    vec![
        0x30, 0x26, 0x02, 0x01, 0x01, 0x04, 0x06, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0xa0, 0x19,
        0x02, 0x04, 0x71, 0x64, 0xfe, 0xf1, 0x02, 0x01, 0x00, 0x02, 0x01, 0x00, 0x30, 0x0b, 0x30,
        0x09, 0x06, 0x05, 0x2b, 0x06, 0x01, 0x02, 0x01, 0x05, 0x00,
    ]
}

pub fn craft_snmptrap_packet_retry() -> Vec<u8> {
    vec![
        0x30, 0x26, 0x02, 0x01, 0x01, 0x04, 0x06, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0xa0, 0x19,
        0x02, 0x04, 0x71, 0x64, 0xfe, 0xf1, 0x02, 0x01, 0x00, 0x02, 0x01, 0x00, 0x30, 0x0b, 0x30,
        0x09, 0x06, 0x05, 0x2b, 0x06, 0x01, 0x02, 0x01, 0x05, 0x00,
    ]
}

pub fn craft_dhcpc_packet() -> Vec<u8> {
    let mut packet = Vec::with_capacity(240);

    // BOOTP message type: Boot Request (1)
    packet.push(0x01);

    // Hardware type: Ethernet (10Mb) (1)
    packet.push(0x01);

    // Hardware address length: 6
    packet.push(0x06);

    // Hops: 0
    packet.push(0x00);

    // Transaction ID: random (0x3903F326)
    packet.extend(&[0x39, 0x03, 0xF3, 0x26]);

    // Seconds elapsed: 0
    packet.extend(&[0x00, 0x00]);

    // Bootp flags: 0x8000 (Broadcast) (0x8000)
    packet.extend(&[0x80, 0x00]);

    // Client IP address: 0.0.0.0
    packet.extend(&[0x00, 0x00, 0x00, 0x00]);

    // Your (client) IP address: 0.0.0.0
    packet.extend(&[0x00, 0x00, 0x00, 0x00]);

    // Next server IP address: 0.0.0.0
    packet.extend(&[0x00, 0x00, 0x00, 0x00]);

    // Relay agent IP address: 0.0.0.0
    packet.extend(&[0x00, 0x00, 0x00, 0x00]);

    // Client MAC address: 00:00:00:00:00:00
    packet.extend(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

    // Client hardware address padding: 00000000000000000000
    packet.extend(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

    // Server host name not given
    packet.extend(&[0x00; 64]);

    // Boot file name not given
    packet.extend(&[0x00; 128]);

    // Magic cookie: DHCP
    packet.extend(&[0x63, 0x82, 0x53, 0x63]);

    // DHCP Message Type: DHCP Discover (1)
    packet.extend(&[0x35, 0x01, 0x01]);

    // DHCP Option: Parameter Request List
    packet.extend(&[0x37, 0x04, 0x01, 0x03, 0x06, 0x2a]);

    // End Option
    packet.push(0xff);

    packet
}

pub fn craft_ntp_packet() -> Vec<u8> {
    let mut packet = vec![0u8; 48];
    packet[0] = 0x1b;
    packet
}

pub fn craft_msrpc_packet() -> Vec<u8> {
    let mut packet = Vec::new();

    // MSRPC packet example
    // Version: 5.0
    packet.extend(&[0x05, 0x00]);

    // Packet Type: Bind (0x0B)
    packet.push(0x0B);

    // Packet Flags: First Fragment (0x03)
    packet.push(0x03);

    // Data Representation: Little Endian (0x10), ASCII (0x00), Floating Point IEEE (0x00), Reserved (0x00)
    packet.extend(&[0x10, 0x00, 0x00, 0x00]);

    // Call ID
    packet.extend(&[0x01, 0x00, 0x00, 0x00]);

    // Max Xmit Frag: 4280
    packet.extend(&[0x10, 0x80]);

    // Max Recv Frag: 4280
    packet.extend(&[0x10, 0x80]);

    // Assoc Group: 0
    packet.extend(&[0x00, 0x00, 0x00, 0x00]);

    // Number of Context Items
    packet.push(0x01);

    // Reserved
    packet.push(0x00);

    // Interface UUID: {12345678-1234-5678-1234-567812345678}
    packet.extend(&[
        0x12, 0x34, 0x56, 0x78, // TimeLow
        0x12, 0x34, // TimeMid
        0x56, 0x78, // TimeHiAndVersion
        0x12, 0x34, // ClockSeqHiAndReserved, ClockSeqLow
        0x56, 0x78, 0x12, 0x34, 0x56, 0x78, // Node
    ]);

    // Interface Version: 1.0
    packet.extend(&[0x01, 0x00]);

    // Transfer Syntax: {8A885D04-1CEB-11C9-9FE8-08002B104860}
    packet.extend(&[
        0x8A, 0x88, 0x5D, 0x04, 0x1C, 0xEB, 0x11, 0xC9, 0x9F, 0xE8, 0x08, 0x00, 0x2B, 0x10, 0x48,
        0x60,
    ]);

    // Transfer Syntax Version: 2.0
    packet.extend(&[0x02, 0x00]);

    packet
}

pub fn craft_profile_packet() -> Vec<u8> {
    vec![/* PROFILE PAYLOAD */]
}

pub fn craft_netbios_ns_packet() -> Vec<u8> {
    vec![/* NETBIOS-NS PAYLOAD */]
}

pub fn craft_netbios_dgm_packet() -> Vec<u8> {
    vec![/* NETBIOS-DGM PAYLOAD */]
}

pub fn craft_netbios_ssn_packet() -> Vec<u8> {
    vec![/* NETBIOS-SSN PAYLOAD */]
}

pub fn craft_microsoft_ds_packet() -> Vec<u8> {
    vec![/* MICROSOFT-DS PAYLOAD */]
}

pub fn craft_dns_query_packet() -> Vec<u8> {
    let mut packet = Vec::new();

    // Transaction ID: Random
    packet.extend(&[0x12, 0x34]);

    // Flags: Standard query
    packet.extend(&[0x01, 0x00]);

    // Questions: 1
    packet.extend(&[0x00, 0x01]);

    // Answer RRs: 0
    packet.extend(&[0x00, 0x00]);

    // Authority RRs: 0
    packet.extend(&[0x00, 0x00]);

    // Additional RRs: 0
    packet.extend(&[0x00, 0x00]);

    // Query: www.example.com
    let query_name = "www.google.com";
    for part in query_name.split('.') {
        packet.push(part.len() as u8);
        packet.extend(part.as_bytes());
    }
    packet.push(0); // End of query name

    // Query Type: A (host address)
    packet.extend(&[0x00, 0x01]);

    // Query Class: IN (internet)
    packet.extend(&[0x00, 0x01]);

    packet
}

pub fn craft_http_rpc_epmap_packet() -> Vec<u8> {
    let mut packet = Vec::new();

    // Example MS-RPC EPMAP packet structure
    // UUID: {12345678-1234-ABCD-EF00-0123456789AB}
    let uuid = vec![
        0x12, 0x34, 0x56, 0x78, // TimeLow
        0x12, 0x34, // TimeMid
        0xAB, 0xCD, // TimeHiAndVersion
        0xEF, 0x00, // ClockSeqHiAndReserved, ClockSeqLow
        0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, // Node
    ];

    // Other fields and options as needed
    // This is a simplified example and may need adjustments
    packet.extend(&uuid);

    packet
}

pub fn craft_isakmp_packet() -> Vec<u8> {
    let mut packet = Vec::new();

    // ISAKMP header
    packet.extend(&[0x00; 8]); // Initiator Cookie
    packet.extend(&[0x00; 8]); // Responder Cookie
    packet.push(0x01); // Next Payload: Security Association
    packet.push(0x10); // Version: 1.0
    packet.push(0x00); // Exchange Type: IKE_SA_INIT
    packet.push(0x00); // Flags
    packet.extend(&[0x00, 0x00, 0x00, 0x00]); // Message ID
    packet.extend(&[0x00, 0x00, 0x00, 0x28]); // Length

    // Security Association Payload
    packet.push(0x00); // Next Payload: None
    packet.push(0x00); // Reserved
    packet.extend(&[0x00, 0x24]); // Payload Length
    packet.extend(&[0x00, 0x00, 0x00, 0x01]); // DOI, Situation

    // Proposal Payload
    packet.push(0x00); // Next Payload: None
    packet.push(0x00); // Reserved
    packet.extend(&[0x00, 0x1c]); // Payload Length
    packet.push(0x02); // Proposal Number
    packet.push(0x01); // Protocol ID: IKE
    packet.push(0x00); // SPI Size
    packet.push(0x01); // Number of Transforms
    packet.extend(&[0x00, 0x00, 0x00, 0x01]); // Transform ID: Key Exchange

    // Transform Payload
    packet.push(0x00); // Next Payload: None
    packet.push(0x00); // Reserved
    packet.extend(&[0x00, 0x14]); // Payload Length
    packet.extend(&[0x00, 0x01, 0x00, 0x00]); // Transform Number, Transform ID: KEY_IKE

    packet
}
