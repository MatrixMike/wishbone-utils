use clap::ArgMatches;
use super::bridge::BridgeKind;
use super::server::ServerKind;

#[derive(Debug)]
pub enum ConfigError {
    /// Couldn't parse string as number
    NumberParseError(String, std::num::ParseIntError),

    /// Specified a bridge kind that we didn't recognize
    UnknownServerKind(String),

    /// No operation was specified
    NoOperationSpecified,
}

pub fn get_base(value: &str) -> (&str, u32) {
    if value.starts_with("0x") {
        (value.trim_start_matches("0x"), 16)
    } else if value.starts_with("0X") {
        (value.trim_start_matches("0X"), 16)
    } else if value.starts_with("0b") {
        (value.trim_start_matches("0b"), 2)
    } else if value.starts_with("0B") {
        (value.trim_start_matches("0B"), 2)
    } else if value.starts_with("0") && value != "0" {
        (value.trim_start_matches("0"), 8)
    } else {
        (value, 10)
    }
}

pub fn parse_u16(value: &str) -> Result<u16, ConfigError> {
    let (value, base) = get_base(value);
    match u16::from_str_radix(value, base) {
        Ok(o) => Ok(o),
        Err(e) => Err(ConfigError::NumberParseError(value.to_owned(), e))
    }
}

pub fn parse_u32(value: &str) -> Result<u32, ConfigError> {
    let (value, base) = get_base(value);
    match u32::from_str_radix(value, base) {
        Ok(o) => Ok(o),
        Err(e) => Err(ConfigError::NumberParseError(value.to_owned(), e))
    }
}

pub struct Config {
    pub usb_pid: Option<u16>,
    pub usb_vid: Option<u16>,
    pub memory_address: Option<u32>,
    pub memory_value: Option<u32>,
    pub server_kind: ServerKind,
    pub bridge_kind: BridgeKind,
    pub serial_port: Option<String>,
    pub serial_baud: Option<usize>,
    pub bind_addr: String,
    pub bind_port: u32,
    pub random_loops: Option<u32>,
    pub random_address: Option<u32>,
}

impl Config {
    pub fn parse(matches: ArgMatches) -> Result<Self, ConfigError> {
        let mut bridge_kind = BridgeKind::UsbBridge;

        let usb_vid = if let Some(vid) = matches.value_of("vid") {
            Some(parse_u16(vid)?)
        } else {
            None
        };

        let usb_pid = if let Some(pid) = matches.value_of("pid") {
            Some(parse_u16(pid)?)
        } else {
            None
        };

        let serial_port = if let Some(port) = matches.value_of("serial") {
            bridge_kind = BridgeKind::UartBridge;
            Some(port.to_owned())
        } else {
            None
        };

        let serial_baud = if let Some(baud) = matches.value_of("baud") {
            Some(parse_u32(baud)? as usize)
        } else {
            None
        };

        let memory_address = if let Some(addr) = matches.value_of("address") {
            Some(parse_u32(addr)?)
        } else {
            None
        };

        let memory_value = if let Some(v) = matches.value_of("value") {
            Some(parse_u32(v)?)
        } else {
            None
        };

        let bind_port = if let Some(port) = matches.value_of("port") {
            parse_u32(port)?
        } else {
            3333
        };

        let bind_addr = if let Some(addr) = matches.value_of("bind-addr") {
            addr.to_owned()
        } else {
            "127.0.0.1".to_owned()
        };

        let server_kind = ServerKind::from_string(&matches.value_of("server-kind"))?;

        let random_loops = if let Some(random_loops) = matches.value_of("random-loops") {
            Some(parse_u32(random_loops)?)
        } else {
            None
        };

        let random_address = if let Some(random_address) = matches.value_of("random-address") {
            Some(parse_u32(random_address)?)
        } else {
            None
        };

        if memory_address.is_none() && server_kind == ServerKind::None {
            Err(ConfigError::NoOperationSpecified)
        }
        else {
            Ok(Config {
                usb_pid,
                usb_vid,
                serial_port,
                serial_baud,
                memory_address,
                memory_value,
                server_kind,
                bridge_kind,
                bind_port,
                bind_addr,
                random_loops,
                random_address,
            })
        }
    }
}