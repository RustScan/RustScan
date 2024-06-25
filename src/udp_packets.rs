pub mod udp_payload {
    // const MAX_PAYLOAD_IPV4: usize = 65507; // 65,535 - 20 (IP header) - 8 (UDP header)
    // const MAX_PAYLOAD_IPV6: usize = 65527; // 65,535 - 40 (IPv6 header) - 8 (UDP header)

    /// From [RFC 768](https://www.rfc-editor.org/rfc/rfc768.txt)
    /// We send blank UDP packets, unless the port is determined to be special.
    ///
    /// ```text
    /// HEADER Handled by the OS
    /// 0                   1                   2                   3
    /// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// |          Source Port          |       Destination Port        |
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// |            Length             |           Checksum            |
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// We append the data, based on the port the the header
    /// DATA
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// |                    Data (based on port)                       |
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// ```
    ///
    /// # Fields
    ///
    /// - **Source Port**: The source port number.
    /// - **Destination Port**: The destination port number.
    /// - **Length**: The length of the UDP header and data.
    /// - **Checksum**: The checksum for error-checking.
    /// - **Data**: The payload data, which can vary based on the port.
    #[allow(clippy::match_single_binding)]
    pub fn cust_payload(dst_prt: u16) -> Vec<u8> {
        match dst_prt {
            // 53 => craft_dns_query_packet(),
            // 67 => craft_dhcpc_packet(),
            // 123 => craft_ntp_packet(),
            // 161 => craft_snmp_packet(),
            _ => vec![],
        }
    }

    pub fn craft_snmptrap_packet() -> Vec<u16> {
        let version = vec![0x30, 0x82]; // SNMP version
        let community_string = vec![0x04, 0x06, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63]; // Community string "public"
        let pdu_type = vec![0xa4, 0x82]; // PDU type for SNMP Trap
        let enterprise_oid = vec![0x06, 0x09, 0x2b, 0x06, 0x01, 0x02, 0x01, 0x01, 0x01, 0x00]; // Enterprise OID

        // Combine the components into a single SNMP Trap packet
        [version, community_string, pdu_type, enterprise_oid].concat()
    }

    /// From [RFC 1592](https://www.rfc-editor.org/rfc/rfc1592.txt)
    /// Crafts an SNMP packet based on the structure from RFC 1592.
    ///
    /// RFC 1592 defines the structure of an SNMP packet as follows
    /// - **ASN.1 header**: 0x30
    /// - **PDU length**: 37 + length of community name
    /// - **SNMP version**: 0x02 0x01 0x00 (integer, length=1, value=0)
    /// - **Community name**: A string of varying length
    /// - **SNMP GET request**: 0xa0 0x1c (request type=0xa0, length=0x1c)
    /// - **SNMP request ID**: 0x02 0x01 0x01 (integer, length=1, ID=1)
    /// - **SNMP error status**: 0x02 0x01 0x00 (integer, length=1, error=0)
    /// - **SNMP index**: 0x02 0x01 0x00 (integer, length=1, index=0)
    /// - **VarBind list**: 0x30 0x11 (length=0x11)
    /// - **VarBind**: 0x30 0x0f (length=0x0f)
    /// - **Object ID**: 0x06 0x0b (length=0x0b)
    /// - **Object instance**: 1.3.6.1.4.1.2.2.1.1.1.0
    /// - **Null value**: 0x05 0x00 (length=0)
    ///
    /// The PDU length formula:
    /// ```text
    /// PDU_length = length of version field and string tag (4 bytes)
    ///            + length of community length field (1 byte)
    ///            + length of community name (depends...)
    ///            + length of SNMP GET request (32 bytes)
    /// ```
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` representing the crafted SNMP packet.
    pub fn craft_snmp_packet() -> Vec<u8> {
        // TODO all versions of snmp
        let v1 = vec![0x30, 0x29, 0x02, 0x01, 0x00];

        let community_string = vec![0x04, 0x06, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63];
        let pdu_type = vec![0xa0, 0x1c];

        let request_id = vec![0x02, 0x04, 0x7a, 0x69, 0x67, 0x71];
        let error_status = vec![0x02, 0x01, 0x00];
        let error_index = vec![0x02, 0x01, 0x00];
        let variable_bindings = vec![
            0x30, 0x0e, 0x30, 0x0c, 0x06, 0x08, 0x2b, 0x06, 0x01, 0x02, 0x01, 0x01, 0x01, 0x00,
            0x05, 0x00,
        ];

        let _basic_snmp = [
            0x30, 0x26, 0x02, 0x01, 0x01, 0x04, 0x06, 0x70, 0x75, 0x62, 0x6c, 0x69, 0x63, 0xa0,
            0x19, 0x02, 0x04, 0x71, 0x64, 0xfe, 0xf1, 0x02, 0x01, 0x00, 0x02, 0x01, 0x00, 0x30,
            0x0b, 0x30, 0x09, 0x06, 0x05, 0x2b, 0x06, 0x01, 0x02, 0x01, 0x05, 0x00,
        ];

        [
            v1,
            community_string,
            pdu_type,
            request_id,
            error_status,
            error_index,
            variable_bindings,
        ]
        .concat()
    }

    /// Constructs a DHCP packet based on the RFC 1541 Dynamic Host Configuration Protocol specification.
    ///
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` representing the DHCP packet.
    ///
    /// This function crafts a DHCP Discover packet with the following fields:
    /// - **op**: BOOTREQUEST (1)
    /// - **htype**: Ethernet (1)
    /// - **hlen**: 6
    /// - **hops**: 0
    /// - **xid**: Random transaction ID
    /// - **secs**: 0
    /// - **flags**: 0x8000 (Broadcast)
    /// - **ciaddr**: 0.0.0.0
    /// - **yiaddr**: 0.0.0.0
    /// - **siaddr**: 0.0.0.0
    /// - **giaddr**: 0.0.0.0
    /// - **chaddr**: 00:00:00:00:00:00 (Client MAC address)
    /// - **sname**: Not given
    /// - **file**: Not given
    /// - **options**: DHCP options
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

    /// Constructs a DNS query packet.
    ///
    /// The structure of the DNS packet is as follows:
    ///
    /// ```text
    /// +---------------------+
    /// |        Header       |
    /// +---------------------+
    /// |       Question      | the question for the name server
    /// +---------------------+
    /// |        Answer       | RRs answering the question
    /// +---------------------+
    /// |      Authority      | RRs pointing toward an authority
    /// +---------------------+
    /// |      Additional     | RRs holding additional information
    /// +---------------------+
    /// ```
    ///
    /// The function builds a DNS query packet with the following fields:
    /// - **Header**: Contains the transaction ID and flags.
    /// - **Question**: Specifies the query (e.g., the domain name to look up).
    /// - **Answer**: Resource records that answer the question (empty for a query).
    /// - **Authority**: Resource records that point toward an authority (empty for a query).
    /// - **Additional**: Resource records holding additional information (empty for a query).
    ///
    /// # Returns
    ///
    /// A `Vec<u8>` representing the DNS query packet.
    pub fn craft_dns_query_packet() -> Vec<u8> {
        let mut packet = Vec::new();

        // Transaction ID: Random
        packet.extend(&[0x12, 0x34]);

        // Flags: Standard query
        packet.extend(&[0x01, 0x00]);
        // Header ^

        // Questions: 1
        packet.extend(&[0x00, 0x01]);

        // Answer RRs: 0
        packet.extend(&[0x00, 0x00]);

        // Authority RRs: 0
        packet.extend(&[0x00, 0x00]);

        // Additional RRs: 0
        packet.extend(&[0x00, 0x00]);

        // Query: www.google.com
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

    /// Creates the simplest NTP packet possible
    /// 0x1b followed by 47 0s, (Version = 3, Mode = 3)
    pub fn craft_ntp_packet() -> Vec<u8> {
        let mut packet = vec![0u8; 48];
        packet[0] = 0x1b;
        packet
    }

    // TODO add more packets for the top 1000 ports
    // `sort -r -k3 /usr/share/nmap/nmap-services | grep udp | head -n 1000`
    // There has to be a better way to structure this, maybe internal crate for packet payload
    // creation
}
