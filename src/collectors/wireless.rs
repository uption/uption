//! Wireless interface collector gathers information about wireless interfaces
//! from the operating system.
use netlink_wi::interface::{ChannelWidth, WirelessInterface};
use netlink_wi::station::WirelessStation;
use netlink_wi::NlSocket;

use super::Collector;
use crate::error::{Error, Result, ResultError};
use crate::message::Message;

pub struct Wireless {}

impl Wireless {
    pub fn new() -> Self {
        Self {}
    }

    fn get_interfaces(&self) -> Result<Vec<WirelessInterface>> {
        let mut socket = NlSocket::connect()?;
        let interfaces = socket.list_interfaces()?;
        if interfaces.is_empty() {
            return Err(Error::new("No wireless interfaces found"));
        }
        Ok(interfaces)
    }

    fn get_stations(&self, if_index: u32) -> Result<Vec<WirelessStation>> {
        let mut socket = NlSocket::connect()?;
        let stations = socket.list_stations(if_index)?;
        Ok(stations)
    }
}

impl Collector for Wireless {
    fn collect(&self) -> Result<Vec<Message>> {
        let interfaces = self.get_interfaces().set_source("wireless_collector")?;
        let mut messages = Vec::new();
        for interface in interfaces {
            log::debug!("Found interface: {:?}", interface);
            let mut message = Message::new("wireless_interface");
            if interface.name.is_empty() {
                log::debug!("Skipping wireless interface with empty name");
                continue;
            }
            message.insert_tag("name", &interface.name);
            message.insert_tag("interface_mac", &interface.mac.to_string());

            if let Some(ssid) = interface.ssid {
                if !ssid.is_empty() {
                    message.insert_tag("ssid", &ssid);
                }
            }
            if let Some(frequency) = interface.frequency {
                message.insert_metric("frequency", frequency);
            }
            if let Some(channel_width) = channel_width_to_number(&interface.channel_width) {
                message.insert_metric("channel_width", channel_width);
            }
            if let Some(tx_power) = interface.tx_power {
                message.insert_metric("tx_power", tx_power);
            }

            if message.metrics().is_empty() {
                log::debug!("No metrics found for interface");
                continue;
            }
            messages.push(message);

            let stations = self.get_stations(interface.interface_index)?;
            for station in stations {
                log::debug!("Found station: {:?}", station);
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
                if message.metrics().is_empty() {
                    log::debug!("No metrics found for station");
                    continue;
                }
                messages.push(message);
            }
        }
        Ok(messages)
    }
}

fn channel_width_to_number(channel_width: &ChannelWidth) -> Option<u32> {
    let ch = match channel_width {
        ChannelWidth::Width20NoHT => 20,
        ChannelWidth::Width20 => 20,
        ChannelWidth::Width40 => 40,
        ChannelWidth::Width80 => 80,
        ChannelWidth::Width80P80 => 160,
        ChannelWidth::Width160 => 160,
        ChannelWidth::Width5 => 5,
        ChannelWidth::Width10 => 10,
        ChannelWidth::Width1 => 1,
        ChannelWidth::Width2 => 2,
        ChannelWidth::Width4 => 4,
        ChannelWidth::Width8 => 8,
        ChannelWidth::Width16 => 16,
        ChannelWidth::Width320 => 320,
        ChannelWidth::Unknown => return None,
    };
    Some(ch)
}
