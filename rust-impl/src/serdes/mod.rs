/*!
Serialization / deserialization
*/

mod g1;
mod g2;
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
fn to_ioresult<F: Field>(res: RResult<F, PrimeFieldDecodingError>) -> Result<F> {
    match res {
        Err(PrimeFieldDecodingError::NotInField(s)) => Err(Error::new(
            ErrorKind::InvalidInput,
            format!("coord is invalid: {}", s),
        )),
        Ok(v) => Ok(v),
    }
}

/// Check whether point is on curve
fn check_point<F: Field, G: Fn(&F) -> F>(x: &F, y: &F, g: G) -> bool {
    let ysq = {
        let mut tmp = *y;
        tmp.square();
        tmp
    };

    ysq == g(x)
}
