use serde::{Deserialize, Serialize};

// type InterfaceIds = Vec<String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct InterfaceIds(pub Vec<String>);

impl std::fmt::Display for InterfaceIds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        let ids = self
            .0
            .iter()
            .map(|id| format!("\"{}\"", id))
            .collect::<Vec<String>>()
            .join(", ");

        write!(f, "{}", ids)?;
        write!(f, "]")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contract {
    pub block_number: u64,
    pub address: String,
    pub interface_ids: InterfaceIds,
}

#[cfg(test)]
mod test {

    use crate::contract::InterfaceIds;

    use super::Contract;

    #[test]
    fn to_json() {
        let c = Contract {
            block_number: 1,
            // address: types::H160::from_low_u64_be(0x0000000000000000000000000000000000000000),
            address: String::from("0x0000000000000000000000000000000000000000"),
            interface_ids: InterfaceIds(vec!["0xffffffff".to_string(), "0x12345678".to_string()]),
        };

        let str = serde_json::to_string(&c).unwrap();

        assert_eq!(str, "{\"block_height\":1,\"address\":\"0x0000000000000000000000000000000000000000\",\"interface_ids\":[\"0xffffffff\",\"0x12345678\"]}".to_string());
    }

    #[test]
    fn test_interface_ids_fmt() {
        let ids = InterfaceIds(vec!["0xffffffff".to_string(), "0x12345678".to_string()]);
        assert_eq!(format!("{}", ids), "[\"0xffffffff\", \"0x12345678\"]");
    }
}
