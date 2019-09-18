//! Placeholder
//! This module defines the APIs;
//! It is essentially a wrapper of BLSSignature trait with public key in G1
use crate::signature;
use pairing::bls12_381::*;
use pairing::serdes::SerDes;
/// ciphersuite identifier
type CSID = u8;
/// Secret key structure.
/// Note: our secret key is a bit different from the internet draft.
/// Here we store the actual exponent in byte, which is 32 bytes;
/// in the ID, it says to store the 32 byte seed, and derive the sk from the seed when needed.
#[derive(Debug, Clone)]
pub struct BLSSK(CSID, Fr);
/// Public key structure
#[derive(Debug, Clone)]
pub struct BLSPK(CSID, G1);
/// Signature structure
#[derive(Debug, Clone)]
pub struct BLSSIG(CSID, G2);
/// Pop is identical to BLSSIG
#[derive(Debug, Clone)]
pub struct BLSPOP(CSID, G2);

pub trait BLSAPI {
    /// * input: a seed (with appropriate length)
    /// * input: ciphersuite ID
    /// * output: a pair of keys
    fn keygen<B: AsRef<[u8]>>(sk: B, ciphersuite: CSID) -> (BLSSK, BLSPK) {
        if sk.as_ref().len() < 32 {
            panic!("seed is not long enough");
        }
        let (x_prime, pk) = <G2 as signature::BLSSigCore>::keygen(sk);
        (BLSSK(ciphersuite, x_prime), BLSPK(ciphersuite, pk))
    }

    /// * input: a secret key
    /// * input: a message blob
    /// * output: a signature
    fn sign<B: AsRef<[u8]>>(sk: &BLSSK, msg: B) -> BLSSIG {
        let sig = signature::BLSSignaturePop::sign(sk.1, msg.as_ref(), sk.0);
        BLSSIG(sk.0, sig)
    }

    /// * input: a public key
    /// * input: a message blob
    /// * input: a signature
    /// * output: if the signature is valid w.r.t. the key and message
    fn verify<B: AsRef<[u8]>>(pk: &BLSPK, msg: B, sig: &BLSSIG) -> bool {
        signature::BLSSignaturePop::verify(pk.1, sig.1, msg.as_ref(), pk.0) && (pk.0 == sig.0)
    }

    fn pop_gen(_sk: &BLSSK, _pk: &BLSPK) -> Result<BLSPOP, String> {
        // FIXME
        Err("TBD".to_owned())
    }

    /// * input: a public key
    /// * input: a proof of possesion
    /// * output: if the pop is valid w.r.t. the pk
    fn pop_verify(pk: &BLSPK, pop: &BLSPOP) -> bool {
        signature::BLSSignaturePop::pop_verify(pk.1, pop.1, pop.0) && (pk.0 == pop.0)
    }

    /// * input: a list of public keys
    /// * input: a list of signatures
    /// * output: the aggregated signature
    fn aggregate_without_verify(sig_list: &[BLSSIG]) -> Result<BLSSIG, String> {
        let mut sig_vec: Vec<G2> = vec![sig_list[0].1];
        for sig in sig_list.iter().skip(1) {
            if sig.0 != sig_list[0].0 {
                return Err("Ciphersuite IDs do not match".to_owned());
            }
            sig_vec.push(sig.1)
        }
        Ok(BLSSIG(
            sig_list[0].0,
            signature::BLSSigCore::aggregate(sig_vec.as_slice()),
        ))
    }

    /// * input: a list of public keys
    /// * input: a message blob
    /// * input: an aggregated signature
    /// * output: if the signature is valid w.r.t. the keys and message
    fn verify_aggregated<B: AsRef<[u8]>>(pk_list: &[BLSPK], msg: B, sig: &BLSSIG) -> bool {
        let mut pk_vec: Vec<G1> = vec![];
        let ciphersuite = sig.0;
        for pk in pk_list.iter() {
            if ciphersuite != pk.0 {
                return false;
            }
            pk_vec.push(pk.1)
        }
        signature::BLSSignaturePop::multisig_verify(
            pk_vec.as_slice(),
            sig.1,
            msg.as_ref(),
            ciphersuite,
        )
    }
}

pub struct BLSPKInG1;
impl BLSAPI for BLSPKInG1 {}

type Compressed = bool;
use std::io::{Read, Write};

impl SerDes for BLSSK {
    /// | ciphersuite | sk | -> bytes
    fn serialize<W: Write>(&self, writer: &mut W, compressed: Compressed) -> std::io::Result<()> {
        let mut buf: Vec<u8> = vec![self.0];
        self.1.serialize(&mut buf, compressed)?;
        // format the output
        writer.write_all(&buf)?;
        Ok(())
    }

    /// bytes -> | ciphersuite | sk |
    fn deserialize<R: Read>(reader: &mut R) -> std::io::Result<(Self, Compressed)> {
        let mut ciphersuite: [u8; 1] = [0u8; 1];
        reader.read_exact(&mut ciphersuite)?;

        let (sk, compressed) = Fr::deserialize(reader)?;
        Ok((BLSSK(ciphersuite[0], sk), compressed))
    }
}

impl SerDes for BLSPK {
    /// | ciphersuite | pk | -> bytes
    fn serialize<W: Write>(&self, writer: &mut W, compressed: Compressed) -> std::io::Result<()> {
        let mut buf: Vec<u8> = vec![self.0];
        self.1.serialize(&mut buf, compressed)?;
        // format the output
        writer.write_all(&buf)?;
        Ok(())
    }

    /// bytes -> | ciphersuite | pk |
    fn deserialize<R: Read>(reader: &mut R) -> std::io::Result<(Self, Compressed)> {
        let mut ciphersuite: [u8; 1] = [0u8; 1];
        reader.read_exact(&mut ciphersuite)?;

        let (pk, compressed) = G1::deserialize(reader)?;
        Ok((BLSPK(ciphersuite[0], pk), compressed))
    }
}

impl SerDes for BLSSIG {
    /// | ciphersuite | sig | -> bytes
    fn serialize<W: Write>(&self, writer: &mut W, compressed: Compressed) -> std::io::Result<()> {
        let mut buf: Vec<u8> = vec![self.0];
        self.1.serialize(&mut buf, compressed)?;
        // format the output
        writer.write_all(&buf)?;
        Ok(())
    }

    /// bytes -> | ciphersuite | sig |
    fn deserialize<R: Read>(reader: &mut R) -> std::io::Result<(Self, Compressed)> {
        let mut ciphersuite: [u8; 1] = [0u8; 1];
        reader.read_exact(&mut ciphersuite)?;

        let (sig, compressed) = G2::deserialize(reader)?;
        Ok((BLSSIG(ciphersuite[0], sig), compressed))
    }
}

impl SerDes for BLSPOP {
    /// | ciphersuite | sig | -> bytes
    fn serialize<W: Write>(&self, writer: &mut W, compressed: Compressed) -> std::io::Result<()> {
        let mut buf: Vec<u8> = vec![self.0];
        self.1.serialize(&mut buf, compressed)?;
        // format the output
        writer.write_all(&buf)?;
        Ok(())
    }

    /// bytes -> | ciphersuite | sig |
    fn deserialize<R: Read>(reader: &mut R) -> std::io::Result<(Self, Compressed)> {
        let mut ciphersuite: [u8; 1] = [0u8; 1];
        reader.read_exact(&mut ciphersuite)?;

        let (sig, compressed) = G2::deserialize(reader)?;
        Ok((BLSPOP(ciphersuite[0], sig), compressed))
    }
}
