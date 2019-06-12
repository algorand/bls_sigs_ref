/*!
Serialization / deserialization
*/

mod g1;
#[cfg(test)]
mod tests;

use ff::{Field, PrimeFieldDecodingError};
use std::io::{Error, ErrorKind, Read, Result, Write};
use std::result::Result as RResult;

/// Serialization support for G1 and G2 points
pub trait SerDes: Sized {
    /// Serialize a point to a writer, compressed or uncompressed
    fn serialize<W: Write>(&self, writer: &mut W, compressed: bool) -> Result<()>;

    /// Deserialize a point
    fn deserialize<R: Read>(reader: &mut R) -> Result<Self>;
}

/// Convert PrimeFieldDecodingError to io::Error
fn to_ioresult<F: Field>(res: RResult<F, PrimeFieldDecodingError>, e_str: &str) -> Result<F> {
    match res {
        Err(PrimeFieldDecodingError::NotInField(s)) => Err(Error::new(
            ErrorKind::InvalidData,
            format!("{} coord is invalid: {}", e_str, s),
        )),
        Ok(v) => Ok(v),
    }
}
