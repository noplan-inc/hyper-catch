use core::fmt;

#[derive(Debug)]
pub enum NftTypes {
    Erc721,
    Erc1155,
}

impl fmt::Display for NftTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NftTypes::Erc721 => write!(f, "erc721"),
            NftTypes::Erc1155 => write!(f, "erc1155"),
        }
    }
}

impl NftTypes {
    pub fn interface_id_bytes(&self) -> [u8; 4] {
        match *self {
            NftTypes::Erc721 => [128, 172, 88, 205],
            NftTypes::Erc1155 => [217, 182, 122, 38],
        }
    }
    pub fn interface_id_str(&self) -> String {
        match *self {
            NftTypes::Erc721 => String::from("0x80ac58cd"),
            NftTypes::Erc1155 => String::from("0xd9b67a26"),
        }
    }
}
