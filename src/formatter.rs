use crate::contract::Contract;
use anyhow::{Context, Result};

pub trait OutputFormatter {
    fn header(&self) -> Result<String>;
    fn format(&self, _: &Contract) -> Result<String>;
}

pub struct Csv {}
impl Csv {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct Json {}

impl Json {
    pub fn new() -> Self {
        Self {}
    }
}

impl OutputFormatter for Csv {
    fn header(&self) -> Result<String> {
        Ok("block_number, address, interface_ids".to_string())
    }

    fn format(&self, contract: &Contract) -> Result<String> {
        Ok(format!(
            "{}, {:?}, {}",
            contract.block_number, contract.address, contract.interface_ids
        ))
    }
}

impl OutputFormatter for Json {
    fn header(&self) -> Result<String> {
        Ok(String::new())
    }

    fn format(&self, contract: &Contract) -> Result<String> {
        serde_json::to_string(contract).context("failed to to_json")
    }
}

#[cfg(test)]
mod tests {

    use crate::contract::{Contract, InterfaceIds};

    use super::{Csv, Json, OutputFormatter};

    #[test]
    fn test_csv_header() {
        let csv = Csv::new();
        let c = Contract {
            block_number: 199,
            interface_ids: InterfaceIds(vec![]),
            address: String::from("0x0000000000000000000000000000000000000000"),
        };
        assert_eq!(
            csv.header().unwrap(),
            "block_number, address, interface_ids".to_string()
        );
    }

    #[test]
    fn test_csv_format() {
        let csv = Csv::new();
        let c = Contract {
            block_number: 199,
            interface_ids: InterfaceIds(vec!["0xffffffff".to_string(), "0x12345678".to_string()]),
            address: String::from("0x0000000000000000000000000000000000000000"),
        };
        assert_eq!(
            csv.format(&c).unwrap(),
            r#"199, "0x0000000000000000000000000000000000000000", ["0xffffffff", "0x12345678"]"#
                .to_string()
        );
    }

    #[test]
    fn test_json_header() {
        let json = Json::new();
        let c = Contract {
            block_number: 199,
            interface_ids: InterfaceIds(vec!["0xffffffff".to_string(), "0x12345678".to_string()]),
            address: String::from("0x0000000000000000000000000000000000000000"),
        };

        assert_eq!(json.header().unwrap(), "".to_string());
    }

    #[test]
    fn test_json_format() {
        let json = Json::new();
        let c = Contract {
            block_number: 199,
            interface_ids: InterfaceIds(vec!["0xffffffff".to_string(), "0x12345678".to_string()]),
            address: String::from("0x0000000000000000000000000000000000000000"),
        };

        assert_eq!(json.format(&c).unwrap(), r#"{"block_number":199,"address":"0x0000000000000000000000000000000000000000","interface_ids":["0xffffffff","0x12345678"]}"#.to_string());
    }
}
