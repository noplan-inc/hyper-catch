use crate::contract::Contract;
use anyhow::{Context, Result};

trait OutputFormatter {
    fn header(&self, _: &Contract) -> Result<String>;
    fn format(&self, _: &Contract) -> Result<String>;
}

struct Csv {}
impl Csv {
    pub fn new() -> Self {
        Self {}
    }
}

struct Json {}

impl Json {
    pub fn new() -> Self {
        Self {}
    }
}

impl OutputFormatter for Csv {
    fn header(&self, _: &Contract) -> Result<String> {
        Ok("block_number, address, interface_ids".to_string())
    }

    fn format(&self, contract: &Contract) -> Result<String> {
        Ok(format!(
            "{}, {}, {}",
            contract.block_height, contract.address, contract.interface_ids
        ))
    }
}

impl OutputFormatter for Json {
    fn header(&self, _: &Contract) -> Result<String> {
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
            block_height: 199,
            interface_ids: InterfaceIds::new(vec![]),
            address: "0x0000000000000000000000000000000000000000".to_string(),
        };
        assert_eq!(
            csv.header(&c).unwrap(),
            "block_number, address, interface_ids".to_string()
        );
    }

    #[test]
    fn test_csv_format() {
        let csv = Csv::new();
        let c = Contract {
            block_height: 199,
            interface_ids: InterfaceIds::new(vec![
                "0xffffffff".to_string(),
                "0x12345678".to_string(),
            ]),
            address: "0x0000000000000000000000000000000000000000".to_string(),
        };
        assert_eq!(
            csv.format(&c).unwrap(),
            r#"199, 0x0000000000000000000000000000000000000000, ["0xffffffff", "0x12345678"]"#
                .to_string()
        );
    }

    #[test]
    fn test_json_header() {
        let json = Json::new();
        let c = Contract {
            block_height: 199,
            interface_ids: InterfaceIds::new(vec![
                "0xffffffff".to_string(),
                "0x12345678".to_string(),
            ]),
            address: "0x0000000000000000000000000000000000000000".to_string(),
        };

        assert_eq!(json.header(&c).unwrap(), "".to_string());
    }

    #[test]
    fn test_json_format() {
        let json = Json::new();
        let c = Contract {
            block_height: 199,
            interface_ids: InterfaceIds::new(vec![
                "0xffffffff".to_string(),
                "0x12345678".to_string(),
            ]),
            address: "0x0000000000000000000000000000000000000000".to_string(),
        };

        assert_eq!(json.format(&c).unwrap(), r#"{"block_height":199,"address":"0x0000000000000000000000000000000000000000","interface_ids":["0xffffffff","0x12345678"]}"#.to_string());
    }
}
