use alloc::format;
use alloc::string::{String, ToString};
#[cfg(not(feature = "with_serde"))]
use alloc::vec::Vec;
#[cfg(not(feature = "with_serde"))]
use binary_sv2::binary_codec_sv2::{self, free_vec, free_vec_2, CVec, CVec2};
#[cfg(not(feature = "with_serde"))]
use binary_sv2::Error;
use binary_sv2::{Deserialize, Seq0255, Serialize, B0255, B064K, U256};
#[cfg(not(feature = "with_serde"))]
use core::convert::TryInto;

/// ## NewTemplate (Server -> Client)
/// The primary template-providing function. Note that the coinbase_tx_outputs bytes will appear
/// as is at the end of the coinbase transaction.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct NewTemplate<'decoder> {
    /// Server’s identification of the template. Strictly increasing, the
    /// current UNIX time may be used in place of an ID.
    pub template_id: u64,
    /// True if the template is intended for future [`crate::SetNewPrevHash`]
    /// message sent on the channel. If False, the job relates to the last
    /// sent [`crate::SetNewPrevHash`] message on the channel and the miner
    /// should start to work on the job immediately.
    pub future_template: bool,
    /// Valid header version field that reflects the current network
    /// consensus. The general purpose bits (as specified in [BIP320]) can
    /// be freely manipulated by the downstream node. The downstream
    /// node MUST NOT rely on the upstream node to set the BIP320 bits
    /// to any particular value.
    pub version: u32,
    /// The coinbase transaction nVersion field.
    pub coinbase_tx_version: u32,
    /// Up to 8 bytes (not including the length byte) which are to be placed
    /// at the beginning of the coinbase field in the coinbase transaction.
    #[cfg_attr(feature = "with_serde", serde(borrow))]
    pub coinbase_prefix: B0255<'decoder>,
    ///bug
    /// The coinbase transaction input’s nSequence field.
    pub coinbase_tx_input_sequence: u32,
    /// The value, in satoshis, available for spending in coinbase outputs
    /// added by the client. Includes both transaction fees and block
    /// subsidy.
    pub coinbase_tx_value_remaining: u64,
    /// The number of transaction outputs included in coinbase_tx_outputs.
    pub coinbase_tx_outputs_count: u32,
    /// Bitcoin transaction outputs to be included as the last outputs in the
    /// coinbase transaction.
    #[cfg_attr(feature = "with_serde", serde(borrow))]
    pub coinbase_tx_outputs: B064K<'decoder>,
    /// The locktime field in the coinbase transaction.
    pub coinbase_tx_locktime: u32,
    /// Merkle path hashes ordered from deepest.
    #[cfg_attr(feature = "with_serde", serde(borrow))]
    pub merkle_path: Seq0255<'decoder, U256<'decoder>>,
}

#[repr(C)]
#[cfg(not(feature = "with_serde"))]
pub struct CNewTemplate {
    template_id: u64,
    future_template: bool,
    version: u32,
    coinbase_tx_version: u32,
    coinbase_prefix: CVec,
    coinbase_tx_input_sequence: u32,
    coinbase_tx_value_remaining: u64,
    coinbase_tx_outputs_count: u32,
    coinbase_tx_outputs: CVec,
    coinbase_tx_locktime: u32,
    merkle_path: CVec2,
}

#[no_mangle]
#[cfg(not(feature = "with_serde"))]
pub extern "C" fn free_new_template(s: CNewTemplate) {
    drop(s)
}

#[cfg(not(feature = "with_serde"))]
impl Drop for CNewTemplate {
    fn drop(&mut self) {
        free_vec(&mut self.coinbase_prefix);
        free_vec(&mut self.coinbase_tx_outputs);
        free_vec_2(&mut self.merkle_path);
    }
}

#[cfg(not(feature = "with_serde"))]
impl<'a> From<NewTemplate<'a>> for CNewTemplate {
    fn from(v: NewTemplate<'a>) -> Self {
        Self {
            template_id: v.template_id,
            future_template: v.future_template,
            version: v.version,
            coinbase_tx_version: v.coinbase_tx_version,
            coinbase_prefix: v.coinbase_prefix.into(),
            coinbase_tx_input_sequence: v.coinbase_tx_input_sequence,
            coinbase_tx_value_remaining: v.coinbase_tx_value_remaining,
            coinbase_tx_outputs_count: v.coinbase_tx_outputs_count,
            coinbase_tx_outputs: v.coinbase_tx_outputs.into(),
            coinbase_tx_locktime: v.coinbase_tx_locktime,
            merkle_path: v.merkle_path.into(),
        }
    }
}

#[cfg(not(feature = "with_serde"))]
impl<'a> CNewTemplate {
    #[cfg(not(feature = "with_serde"))]
    #[allow(clippy::wrong_self_convention)]
    pub fn to_rust_rep_mut(&'a mut self) -> Result<NewTemplate<'a>, Error> {
        let coinbase_prefix: B0255 = self.coinbase_prefix.as_mut_slice().try_into()?;

        let merkle_path_ = self.merkle_path.as_mut_slice();
        let mut merkle_path: Vec<U256> = Vec::new();
        for cvec in merkle_path_ {
            merkle_path.push(cvec.as_mut_slice().try_into()?);
        }
        let merkle_path = Seq0255::new(merkle_path)?;

        let coinbase_tx_outputs = self.coinbase_tx_outputs.as_mut_slice().try_into()?;

        Ok(NewTemplate {
            template_id: self.template_id,
            future_template: self.future_template,
            version: self.version,
            coinbase_tx_version: self.coinbase_tx_version,
            coinbase_prefix,
            coinbase_tx_input_sequence: self.coinbase_tx_input_sequence,
            coinbase_tx_value_remaining: self.coinbase_tx_value_remaining,
            coinbase_tx_outputs_count: self.coinbase_tx_outputs_count,
            coinbase_tx_outputs,
            coinbase_tx_locktime: self.coinbase_tx_locktime,
            merkle_path,
        })
    }
}
#[cfg(feature = "with_serde")]
use binary_sv2::GetSize;
#[cfg(feature = "with_serde")]
impl<'d> GetSize for NewTemplate<'d> {
    fn get_size(&self) -> usize {
        self.template_id.get_size()
            + self.future_template.get_size()
            + self.version.get_size()
            + self.coinbase_tx_version.get_size()
            + self.coinbase_prefix.get_size()
            + self.coinbase_tx_input_sequence.get_size()
            + self.coinbase_tx_value_remaining.get_size()
            + self.coinbase_tx_outputs_count.get_size()
            + self.coinbase_tx_outputs.get_size()
            + self.coinbase_tx_locktime.get_size()
            + self.merkle_path.get_size()
    }
}

impl<'a> NewTemplate<'a> {

    /// Returns the height of the block based on the coinbase_prefix
    /// The coinbase_prefix is a vector of bytes that contains the height of the block
    /// The height is stored in little endian
    /// This reverses the order of the int strings, then
    /// converts each to hex and then converts the vector to a string
    /// The string is then converted to a u32 and returned
    pub fn get_height(&self) -> u32 {
        let height = self.coinbase_prefix.to_vec();

        //Convert each element to hex
        let mut height_hex: Vec<String> = height
            .iter()
            //.map(|x| format!("{:x?}", x))
            .map(|x| format!("{:02x?}", x))
            .rev().collect();

        // if the last element of height_hex starts with a 0 remove it
        // because 0x89cc0b != 0x89ccb
        let mut last_element :String = height_hex.pop().unwrap();
        if last_element.starts_with("0") {
            last_element = last_element[1..].to_string();
        }
        height_hex.push(last_element);

        //Convert the vector to a string
        let height_hex_st: String = height_hex.join("");

        //Convert hex string to u32
        u32::from_str_radix(&height_hex_st, 16).unwrap()
    }

}

#[cfg(feature = "with_serde")]
impl<'a> NewTemplate<'a> {
    pub fn into_static(self) -> NewTemplate<'static> {
        panic!("This function shouldn't be called by the Messaege Generator");
    }
    pub fn as_static(&self) -> NewTemplate<'static> {
        panic!("This function shouldn't be called by the Messaege Generator");
    }
}

#[cfg(test)]
mod test {
    use alloc::vec::Vec;
    use core::convert::TryInto;
    use super::*;
    use binary_sv2::{Seq0255, Seq064K, B064K, U256};

    #[test]
    fn test_get_height() {
        let mut coinbase_prefix = Vec::new();

        // https://blockstream.info/tx/eede27543f086abd612b87096a7216229d4c736d39bbdfd4fefc1455f427997f
        coinbase_prefix.push(18);
        coinbase_prefix.push(158);
        coinbase_prefix.push(11);

        let temp = create_new_template(coinbase_prefix.clone().try_into().unwrap());

        // 761362 -> 0xB9E12
        // 18,     158,   11
        // 0x12,   0x9E,  0xB
        assert_eq!(temp.get_height(), 761362);

        //  -> 773256 -> 0xBCC88
        coinbase_prefix.clear();

        // https://blockstream.info/block/00000000000000000005d02cefd2336c605b03736db8c0f3cbd6881c723f0a2f        coinbase_prefix.push(8); // 0x08
        coinbase_prefix.push(8); // 0x08
        coinbase_prefix.push(200); // 0xc8
        coinbase_prefix.push(188); // 0xbc

        let temp = create_new_template(coinbase_prefix.clone().try_into().unwrap());

        assert_eq!(temp.get_height(), 773256);


        // 773257 -> 89cc0b
        coinbase_prefix.clear();
        coinbase_prefix.push(137); // 0x89
        coinbase_prefix.push(204); // 0xcc
        coinbase_prefix.push(11); // 0x0b

        let temp = create_new_template(coinbase_prefix.clone().try_into().unwrap());

        assert_eq!(temp.get_height(), 773257);

        // 626507 -> 0x98f4b
        coinbase_prefix.clear();

        //https://blockstream.info/block/00000000000000000004bc9fc532c790f32a5170fa9461110319d1c1b1c2e2d3?expand        coinbase_prefix.push(11); // 0x0b
        coinbase_prefix.push(11); // 0x0b
        coinbase_prefix.push(244); // 0xf4
        coinbase_prefix.push(152); // 0x98

        let temp = create_new_template(coinbase_prefix.clone().try_into().unwrap());

        assert_eq!(temp.get_height(), 626507);
    }

    fn create_new_template(coinbase_prefix: B0255) -> NewTemplate {
        NewTemplate {
            template_id: 1,
            future_template: true,
            version: 1,
            coinbase_tx_version: 1,
            coinbase_prefix,
            coinbase_tx_input_sequence: 1,
            coinbase_tx_value_remaining: 1,
            coinbase_tx_outputs_count: 0,
            coinbase_tx_outputs: Vec::new().try_into().unwrap(),
            coinbase_tx_locktime: 1,
            merkle_path: Vec::new().try_into().unwrap(),
        }
    }
}
#[cfg(feature = "prop_test")]
use quickcheck::{Arbitrary, Gen};

#[cfg(feature = "prop_test")]
use alloc::vec;

#[cfg(feature = "prop_test")]
use core::cmp;

#[cfg(feature = "prop_test")]
impl Arbitrary for NewTemplate<'static> {
    fn arbitrary(g: &mut Gen) -> NewTemplate<'static> {
        let coinbase_tx_version = (u32::arbitrary(g) % 2) + 1;
        let mut coinbase_prefix = vec::Vec::new();
        let coinbase_prefix_len = match coinbase_tx_version {
            1 => u8::arbitrary(g) as usize,
            2 => u8::arbitrary(g).checked_add(4).unwrap_or(4) as usize,
            _ => panic!(),
        };
        for _ in 0..coinbase_prefix_len {
            coinbase_prefix.push(u8::arbitrary(g))
        }
        let coinbase_prefix: binary_sv2::B0255 = coinbase_prefix.try_into().unwrap();

        // TODO uncomment when node provided outputs are supported
        //let mut coinbase_tx_outputs = vec::Vec::new();
        //let coinbase_tx_outputs_len = u16::arbitrary(g) as usize;
        //for _ in 0..coinbase_tx_outputs_len {
        //    coinbase_tx_outputs.push(u8::arbitrary(g))
        //};
        //coinbase_tx_outputs.resize(coinbase_tx_outputs.len() - coinbase_tx_outputs.len() % 36,0);
        //let coinbase_tx_outputs: binary_sv2::B064K = coinbase_tx_outputs.try_into().unwrap();

        let mut merkle_path = vec::Vec::new();
        let merkle_path_len = u8::arbitrary(g);
        for _ in 0..merkle_path_len {
            let mut path = Vec::new();
            for _ in 0..32 {
                path.push(u8::arbitrary(g));
            }
            let path: binary_sv2::U256 = path.try_into().unwrap();
            merkle_path.push(path);
        }
        let merkle_path: binary_sv2::Seq0255<binary_sv2::U256> = merkle_path.into();

        NewTemplate {
            template_id: u64::arbitrary(g) % u64::MAX,
            future_template: bool::arbitrary(g),
            version: u32::arbitrary(g),
            coinbase_tx_version,
            coinbase_prefix,
            coinbase_tx_input_sequence: u32::arbitrary(g),
            coinbase_tx_value_remaining: u64::arbitrary(g),
            // the belows should be used when node provided outputs are enabled
            //coinbase_tx_outputs_count: coinbase_tx_outputs.len().checked_div(36).unwrap_or(0) as u32,
            //coinbase_tx_outputs,
            coinbase_tx_outputs_count: 0,
            coinbase_tx_outputs: Vec::new().try_into().unwrap(),
            coinbase_tx_locktime: u32::arbitrary(g),
            merkle_path,
        }
    }
}
