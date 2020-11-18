//! Wireless interface collector gathers information about wireless interfaces
//! from the operating system.
use netlink_wi::{AttrParseError, NlSocket, WirelessInterface, WirelessStation};

use super::Collector;
use crate::error::{Error, Result, ResultError};
use crate::message::Message;

pub struct Wireless {}

impl Wireless {
    pub fn new() -> Self {
        Self {}
    }

    fn get_interfaces(&self) -> Result<Vec<WirelessInterface>> {
        let socket = NlSocket::connect()?;
        let interfaces = socket
            .list_interfaces()?
            .into_iter()
            .collect::<std::result::Result<Vec<_>, AttrParseError>>()?;
        if interfaces.is_empty() {
            return Err(Error::new("No wireless interfaces found"));
        }
        Ok(interfaces)
    }

    fn get_stations(&self, if_index: u32) -> Result<Vec<WirelessStation>> {
        let socket = NlSocket::connect()?;
        let stations = socket
            .list_stations(if_index)?
            .into_iter()
            .collect::<std::result::Result<Vec<_>, AttrParseError>>()?;
        Ok(stations)
    }
}

impl Collector for Wireless {
    fn collect(&self) -> Result<Vec<Message>> {
        let interfaces = self.get_interfaces().set_source("wireless_collector")?;
        let mut messages = Vec::new();
        for interface in interfaces {
            let mut message = Message::new("wireless_interface");
            message.insert_tag("name", &interface.name);
            message.insert_tag("interface_mac", &interface.mac.to_string());
            if let Some(ssid) = interface.ssid {
                message.insert_tag("ssid", &ssid);
            }
            if let Some(frequency) = interface.frequency {
                message.insert_metric("frequency", frequency);
            }
            if let Some(channel_width) = interface.channel_width {
                message.insert_metric("channel_width", Into::<u32>::into(channel_width));
            }
            if let Some(tx_power) = interface.tx_power {
                message.insert_metric("tx_power", tx_power);
            }
            messages.push(message);

            let stations = self.get_stations(interface.interface_index)?;
            for station in stations {
                let mut message = Message::new("wireless_station");
                message.insert_tag("interface_mac", &interface.mac.to_string());
                message.insert_tag("station_mac", &station.mac.to_string());
                if let Some(signal) = station.signal {
                    message.insert_metric("signal_strength", signal);
                }
                if let Some(rx_bytes) = station.rx_bytes64 {
                    message.insert_metric("rx_bytes", rx_bytes);
                }
                if let Some(tx_bytes) = station.tx_bytes64 {
                    message.insert_metric("tx_bytes", tx_bytes);
                }
                if let Some(rx_packets) = station.rx_packets {
                    message.insert_metric("rx_packets", rx_packets);
                }
                if let Some(tx_packets) = station.tx_packets {
                    message.insert_metric("tx_packets", tx_packets);
                }
                if let Some(tx_retries) = station.tx_retries {
                    message.insert_metric("tx_retries", tx_retries);
                }
                if let Some(tx_failed) = station.tx_failed {
                    message.insert_metric("tx_failed", tx_failed);
                }
                if let Some(rateinfo) = station.rx_bitrate {
                    message.insert_metric("rx_bitrate", rateinfo.bitrate);
                    message.insert_metric("rx_mcs", rateinfo.mcs);
                    message
                        .insert_metric("rx_connection_type", rateinfo.connection_type.to_string());
                }
                if let Some(rateinfo) = station.tx_bitrate {
                    message.insert_metric("tx_bitrate", rateinfo.bitrate);
                    message.insert_metric("tx_mcs", rateinfo.mcs);
                    message
                        .insert_metric("tx_connection_type", rateinfo.connection_type.to_string());
                }
                messages.push(message);
            }
        }
        Ok(messages)
    }
}
