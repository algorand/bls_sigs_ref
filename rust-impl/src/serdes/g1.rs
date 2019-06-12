/*!
SerDes for G1
*/

use super::{check_point, to_ioresult, SerDes};
use chain::chain_pm3div4;
use ff::{Field, PrimeField, PrimeFieldRepr};
use pairing::bls12_381::transmute::{fq, g1_affine};
use pairing::bls12_381::{Fq, FqRepr, G1Affine, G1};
use pairing::{CurveAffine, CurveProjective};
use signum::{Sgn0Result, Signum0};
use std::io::{Cursor, Error, ErrorKind, Read, Result, Write};

fn gx1(x: &Fq) -> Fq {
    const E1_B: Fq = unsafe {
        fq(FqRepr([
            0xaa270000000cfff3u64,
            0x53cc0032fc34000au64,
            0x478fe97a6b0a807fu64,
            0xb1d37ebee6ba24d7u64,
            0x8ec9733bbf78ab2fu64,
            0x09d645513d83de7eu64,
        ]))
    };
    let mut ret = *x;
    ret.square();
    ret.mul_assign(x);
    ret.add_assign(&E1_B);
    ret
}

fn deser_fq(cur: Cursor<&[u8]>) -> Result<Fq> {
    let mut repr = FqRepr([0; 6]);
    repr.read_be(cur)?;
    to_ioresult(Fq::from_repr(repr))
}

impl SerDes for G1Affine {
    fn serialize<W: Write>(&self, writer: &mut W, compressed: bool) -> Result<()> {
        let mut to_write = [0u8; 96];

        // point at infinity
        if self.is_zero() {
            let (len, tag) = if compressed {
                (48, 0xc0u8)
            } else {
                (96, 0x40u8)
            };
            to_write[0] = tag;
            writer.write(&to_write[..len])?;
            return Ok(());
        }

        // not the point at infinity
        let (x, y) = self.as_tuple();
        if !check_point(x, y, gx1) {
            return Err(Error::new(ErrorKind::InvalidData, "point is not on curve"));
        }
        // start of to_write borrow for serializing x and possibly y
        {
            let mut cur = Cursor::new(&mut to_write[..]);
            x.into_repr().write_be(&mut cur)?;
            if !compressed {
                // uncompressed point: no tag, just add bytes of y
                y.into_repr().write_be(&mut cur)?;
            }
        } // borrow of to_write ends
        if !compressed {
            writer.write(&to_write[..96])?;
            return Ok(());
        }

        // compressed. Tag with sign of y and we're done
        to_write[0] |= if y.sgn0() == Sgn0Result::Negative {
            0xa0u8
        } else {
            0x80u8
        };
        writer.write(&to_write[..48])?;
        Ok(())
    }

    fn deserialize<R: Read>(reader: &mut R) -> Result<G1Affine> {
        // read the first 48 bytes
        let mut x_in = [0u8; 48];
        reader.read_exact(&mut x_in)?;

        // take tag out of the top 3 bits
        let tag = x_in[0] >> 5;
        x_in[0] &= 0x1f;

        match tag {
            0 => {
                // this is an uncompressed point
                let x = deser_fq(Cursor::new(&x_in[..]))?;
                let y = {
                    let mut y_in = [0u8; 48];
                    reader.read_exact(&mut y_in)?;
                    deser_fq(Cursor::new(&y_in[..]))?
                };

                if !check_point(&x, &y, gx1) {
                    return Err(Error::new(ErrorKind::InvalidData, "point is not on curve"));
                }
                // XXX: check if point is in subgroup?

                Ok(unsafe { g1_affine(x, y, false) })
            }
            2 | 6 => {
                if x_in.iter().any(|&x| x != 0)
                    || tag == 2 && {
                        let mut y_in = [0u8; 48];
                        reader.read_exact(&mut y_in)?;
                        y_in.iter().any(|&y| y != 0)
                    }
                {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "invalid point at infinity: must be all 0s other than tag",
                    ));
                }

                Ok(G1Affine::zero())
            }
            4 | 5 => {
                let x = deser_fq(Cursor::new(&x_in[..]))?;
                let gx = gx1(&x);
                let y = {
                    let mut tmp = Fq::zero();
                    chain_pm3div4(&mut tmp, &gx); // g(x) ^ ((p - 3) // 4)
                    tmp.mul_assign(&gx); // g(x) ^ ((p - 1) // 4)
                    let y_neg = tmp.sgn0()
                        ^ if tag == 5 {
                            Sgn0Result::Negative
                        } else {
                            Sgn0Result::NonNegative
                        };
                    tmp.negate_if(y_neg);
                    tmp
                };
                if {
                    let mut ysq = y;
                    ysq.square();
                    ysq
                } != gx
                {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "invalid compressed point: not on curve",
                    ));
                }

                Ok(unsafe { g1_affine(x, y, false) })
            }
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("invalid tag {} for G1 point", tag),
                ))
            }
        }
    }
}

// The rustc conflicting implementation check appears to preclude making the below generic.
// Specifically, you could imagine implementing for T: CurveProjective where T::Affine has
// a SerDes impl. But then rustc tells you that an upstream crate might impl CurveProjective
// for T::Affine (which happens to be false here, but rustc doesn't know that).
impl SerDes for G1 {
    fn serialize<W: Write>(&self, writer: &mut W, compressed: bool) -> Result<()> {
        self.into_affine().serialize(writer, compressed)
    }

    fn deserialize<R: Read>(reader: &mut R) -> Result<G1> {
        let res_affine = G1Affine::deserialize(reader)?;
        Ok(res_affine.into_projective())
    }
}
