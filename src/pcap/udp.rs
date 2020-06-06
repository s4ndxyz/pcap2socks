pub use super::layer::{Layer, LayerType, LayerTypes};
use pnet::packet::ipv4::Ipv4;
use pnet::packet::udp::{self, MutableUdpPacket, UdpPacket};
use std::clone::Clone;
use std::fmt::{self, Display, Formatter};
use std::net::IpAddr;

/// Represents an UDP packet.
#[derive(Clone, Debug)]
pub struct Udp {
    pub layer: udp::Udp,
    pub src: IpAddr,
    pub dst: IpAddr,
}

impl Udp {
    /// Creates an `Udp`.
    pub fn new(udp: udp::Udp, src: IpAddr, dst: IpAddr) -> Udp {
        Udp {
            layer: udp,
            src,
            dst,
        }
    }

    /// Creates a `Udp` according to the given UDP packet, source and destination.
    pub fn parse(packet: &UdpPacket, src: IpAddr, dst: IpAddr) -> Udp {
        Udp {
            layer: udp::Udp {
                source: packet.get_source(),
                destination: packet.get_destination(),
                length: packet.get_length(),
                checksum: packet.get_checksum(),
                payload: vec![],
            },
            src,
            dst,
        }
    }
}

impl Display for Udp {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {} -> {}, Length = {}",
            LayerTypes::Udp,
            self.layer.source,
            self.layer.destination,
            self.layer.length
        )
    }
}

impl Layer for Udp {
    fn get_type(&self) -> LayerType {
        LayerTypes::Udp
    }

    fn get_size(&self) -> usize {
        UdpPacket::packet_size(&self.layer)
    }

    fn serialize(&self, buffer: &mut [u8]) -> Result<(), String> {
        let mut packet = match MutableUdpPacket::new(buffer) {
            Some(packet) => packet,
            None => return Err(format!("buffer is too small")),
        };

        packet.populate(&self.layer);

        // Checksum
        let checksum;
        match self.src {
            IpAddr::V4(src) => {
                if let IpAddr::V4(dst) = self.dst {
                    checksum = udp::ipv4_checksum(&packet.to_immutable(), &src, &dst);
                } else {
                    return Err(format!(
                        "source and destination's IP version is not matched"
                    ));
                }
            }
            IpAddr::V6(src) => {
                if let IpAddr::V6(dst) = self.dst {
                    checksum = udp::ipv6_checksum(&packet.to_immutable(), &src, &dst);
                } else {
                    return Err(format!(
                        "source and destination's IP version is not matched"
                    ));
                }
            }
        };
        packet.set_checksum(checksum);

        Ok(())
    }

    fn serialize_n(&self, n: usize, buffer: &mut [u8]) -> Result<usize, String> {
        let mut packet = match MutableUdpPacket::new(buffer) {
            Some(packet) => packet,
            None => return Err(format!("buffer is too small")),
        };

        packet.populate(&self.layer);

        // Recalculate size
        packet.set_length((self.get_size() + n) as u16);

        // Checksum
        let checksum;
        match self.src {
            IpAddr::V4(src) => {
                if let IpAddr::V4(dst) = self.dst {
                    checksum = udp::ipv4_checksum(&packet.to_immutable(), &src, &dst);
                } else {
                    return Err(format!(
                        "source and destination's IP version is not matched"
                    ));
                }
            }
            IpAddr::V6(src) => {
                if let IpAddr::V6(dst) = self.dst {
                    checksum = udp::ipv6_checksum(&packet.to_immutable(), &src, &dst);
                } else {
                    return Err(format!(
                        "source and destination's IP version is not matched"
                    ));
                }
            }
        };
        packet.set_checksum(checksum);

        Ok(self.get_size() + n)
    }
}