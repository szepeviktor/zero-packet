use core::fmt::{self};

/// Common EtherTypes.
///
/// See: <https://www.iana.org/assignments/ieee-802-numbers/ieee-802-numbers.xhtml.>.
#[repr(u16)]
#[derive(Debug, PartialEq)]
pub enum EtherType {
    ARP = 0x0806,
    IPv4 = 0x0800,
    IPv6 = 0x86dd,
    Unknown(u16),
}

impl From<u16> for EtherType {
    fn from(value: u16) -> Self {
        match value {
            0x0800 => EtherType::IPv4,
            0x0806 => EtherType::ARP,
            0x86DD => EtherType::IPv6,
            other => EtherType::Unknown(other),
        }
    }
}

/// Common IP protocols.
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum IpProtocol {
    ICMP = 1,
    TCP = 6,
    UDP = 17,
    Unknown(u8),
}

impl From<u8> for IpProtocol {
    fn from(value: u8) -> Self {
        match value {
            1 => IpProtocol::ICMP,
            6 => IpProtocol::TCP,
            17 => IpProtocol::UDP,
            other => IpProtocol::Unknown(other),
        }
    }
}

/// Common ICMP types.
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum IcmpType {
    EchoReply = 0,
    DestUnreachable = 3,
    SourceQuench = 4,
    Redirect = 5,
    EchoRequest = 8,
    RouterAdvertisement = 9,
    RouterSelection = 10,
    TimeExceeded = 11,
    ParameterProblem = 12,
    Timestamp = 13,
    TimestampReply = 14,
    InformationRequest = 15,
    InformationReply = 16,
    AddressMaskRequest = 17,
    AddressMaskReply = 18,
    Traceroute = 30,
    Photuris = 40,
    ExtendedEchoRequest = 42,
    ExtendedEchoReply = 43,
    Experimental1 = 253,
    Experimental2 = 254,
    Unknown,
}

impl From<u8> for IcmpType {
    fn from(value: u8) -> Self {
        match value {
            0 => IcmpType::EchoReply,
            3 => IcmpType::DestUnreachable,
            4 => IcmpType::SourceQuench,
            5 => IcmpType::Redirect,
            8 => IcmpType::EchoRequest,
            9 => IcmpType::RouterAdvertisement,
            10 => IcmpType::RouterSelection,
            11 => IcmpType::TimeExceeded,
            12 => IcmpType::ParameterProblem,
            13 => IcmpType::Timestamp,
            14 => IcmpType::TimestampReply,
            15 => IcmpType::InformationRequest,
            16 => IcmpType::InformationReply,
            17 => IcmpType::AddressMaskRequest,
            18 => IcmpType::AddressMaskReply,
            30 => IcmpType::Traceroute,
            40 => IcmpType::Photuris,
            42 => IcmpType::ExtendedEchoRequest,
            43 => IcmpType::ExtendedEchoReply,
            253 => IcmpType::Experimental1,
            254 => IcmpType::Experimental2,
            _ => IcmpType::Unknown,
        }
    }
}

/// Workaround for the lack of heap support in `no_std`.
pub fn to_hex_string(bytes: &[u8], buffer: &mut [u8]) -> usize {
    const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";

    let mut buffer_index = 0;
    for (i, &byte) in bytes.iter().enumerate() {
        if i != 0 {
            buffer[buffer_index] = b':';
            buffer_index += 1;
        }

        buffer[buffer_index] = HEX_CHARS[(byte >> 4) as usize];
        buffer[buffer_index + 1] = HEX_CHARS[(byte & 0xf) as usize];
        buffer_index += 2;
    }

    buffer_index
}

/// Wrapper to format an IP address.
pub struct IpFormatter<'a>(pub &'a [u8]);

impl fmt::Debug for IpFormatter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ip = self.0;
        write!(f, "{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3])
    }
}
