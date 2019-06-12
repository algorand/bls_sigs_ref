/*!
SerDes for G2
*/

use super::{check_point, to_ioresult, SerDes};
use chain::chain_p2m9div16;
use ff::{Field, PrimeField, PrimeFieldRepr};
use pairing::bls12_381::transmute::{fq, g2_affine};
use pairing::bls12_381::{Fq, Fq2, FqRepr, G2Affine, G2};
use pairing::{CurveAffine, CurveProjective};
use signum::{Sgn0Result, Signum0};
use std::io::{Cursor, Error, ErrorKind, Read, Result, Write};

fn gx2(x: &Fq2) -> Fq2 {
    const E2_B: Fq2 = unsafe {
        Fq2 {
            c0: fq(FqRepr([
                0xaa270000000cfff3u64,
                0x53cc0032fc34000au64,
                0x478fe97a6b0a807fu64,
                0xb1d37ebee6ba24d7u64,
                0x8ec9733bbf78ab2fu64,
                0x09d645513d83de7eu64,
            ])),
            c1: fq(FqRepr([
                0xaa270000000cfff3u64,
                0x53cc0032fc34000au64,
                0x478fe97a6b0a807fu64,
                0xb1d37ebee6ba24d7u64,
                0x8ec9733bbf78ab2fu64,
                0x09d645513d83de7eu64,
            ])),
        }
    };
    let mut ret = *x;
    ret.square();
    ret.mul_assign(x);
    ret.add_assign(&E2_B);
    ret
}

fn deser_fq2(mut cur: Cursor<&[u8]>) -> Result<Fq2> {
    let mut c1_repr = FqRepr([0; 6]);
    c1_repr.read_be(&mut cur)?;
    let mut c0_repr = FqRepr([0; 6]);
    c0_repr.read_be(&mut cur)?;
    let c0 = to_ioresult(Fq::from_repr(c0_repr))?;
    let c1 = to_ioresult(Fq::from_repr(c1_repr))?;
    Ok(Fq2 { c0, c1 })
}

fn sqrt_fq2(gx: &Fq2) -> Option<Fq2> {
    use osswu_map::g2::ROOTS_OF_UNITY;
    let sqrt_candidate = {
        let mut tmp = *gx;
        chain_p2m9div16(&mut tmp, gx); // gx ^ ((p^2 - 9) // 16)
        tmp.mul_assign(gx); // gx ^ ((p^2 + 7) // 16)
        tmp
    };
    for root in &ROOTS_OF_UNITY[..] {
        let mut y = *root;
        y.mul_assign(&sqrt_candidate);

        let mut tmp = y;
        tmp.square();
        if tmp == *gx {
            return Some(y);
        }
    }

    None
}

impl SerDes for G2Affine {
    fn serialize<W: Write>(&self, writer: &mut W, compressed: bool) -> Result<()> {
        let mut to_write = [0u8; 192];
        let tag1 = if compressed { 0xe0u8 } else { 0x60u8 };

        // point at infinity
        if self.is_zero() {
            let len = if compressed { 96 } else { 192 };
            to_write[0] = tag1;
            to_write[48] = 0xc0u8; // g2 tag for point at infinity
            writer.write(&to_write[..len])?;
            return Ok(());
        }

        // not point at infinity
        let (x, y) = self.as_tuple();
        if !check_point(x, y, gx2) {
            return Err(Error::new(ErrorKind::InvalidData, "point is not on curve"));
        }
        // to_write borrow for serializing x and maybe y
        let (tag2, len) = {
            let mut cur = Cursor::new(&mut to_write[..]);
            x.c1.into_repr().write_be(&mut cur)?;
            x.c0.into_repr().write_be(&mut cur)?;
            if !compressed {
                // uncompressed point needs bytes of y
                y.c1.into_repr().write_be(&mut cur)?;
                y.c0.into_repr().write_be(&mut cur)?;
                (0x80u8, 192)
            } else if y.sgn0() == Sgn0Result::Negative {
                (0xa0u8, 96)
            } else {
                (0x80u8, 96)
            }
        }; // borrow of to_write ends
        to_write[0] |= tag1;
        to_write[48] |= tag2;
        writer.write(&to_write[..len])?;
        Ok(())
    }

    fn deserialize<R: Read>(reader: &mut R) -> Result<G2Affine> {
        // read the first 96 bytes
        let mut x_in = [0u8; 96];
        reader.read_exact(&mut x_in)?;

        // take out the tags
        let tag1 = x_in[0] >> 5;
        let tag2 = x_in[48] >> 5;
        x_in[0] &= 0x1f;
        x_in[48] &= 0x1f;

        // point at infinity
        if tag2 == 6 {
            if tag1 != 3 && tag1 != 7 {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("invalid tag1 {} for G2 point", tag1),
                ));
            }
        }

        // tag1 can be 3 or 7
        // tag2 can be 4, 5, or 6
        // (7, 5) is invalid
        match (tag1, tag2) {
            (3, 6) | (7, 6) => {
                // point at infinity
                if x_in.iter().any(|&x| x != 0)
                    || tag1 == 3 && {
                        let mut y_in = [0u8; 96];
                        reader.read_exact(&mut y_in)?;
                        y_in.iter().any(|&y| y != 0)
                    }
                {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "invalid point at infinity: must be all 0s other than tag",
                    ));
                }

                Ok(G2Affine::zero())
            }
            (3, 4) => {
                // uncompressed point
                let x = deser_fq2(Cursor::new(&x_in[..]))?;
                let y = {
                    let mut y_in = [0u8; 96];
                    reader.read_exact(&mut y_in)?;
                    deser_fq2(Cursor::new(&y_in[..]))?
                };

                if !check_point(&x, &y, gx2) {
                    return Err(Error::new(ErrorKind::InvalidData, "point is not on curve"));
                }
                // XXX: check if point is in subgroup?

                Ok(unsafe { g2_affine(x, y, false) })
            }
            (7, 4) | (7, 5) => {
                // compressed point
                let x = deser_fq2(Cursor::new(&x_in[..]))?;
                let gx = gx2(&x);
                let y = match sqrt_fq2(&gx) {
                    Some(mut tmp) => {
                        let y_neg = tmp.sgn0()
                            ^ if tag2 == 5 {
                                Sgn0Result::Negative
                            } else {
                                Sgn0Result::NonNegative
                            };
                        tmp.negate_if(y_neg);
                        tmp
                    }
                    None => {
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            "invalid compressed point: not on curve",
                        ));
                    }
                };

                Ok(unsafe { g2_affine(x, y, false) })
            }
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("invalid tag {}:{} for G2 point", tag1, tag2),
                ));
            }
        }
    }
}

impl SerDes for G2 {
    fn serialize<W: Write>(&self, writer: &mut W, compressed: bool) -> Result<()> {
        self.into_affine().serialize(writer, compressed)
    }

    fn deserialize<R: Read>(reader: &mut R) -> Result<G2> {
        let res_affine = G2Affine::deserialize(reader)?;
        Ok(res_affine.into_projective())
    }
}
