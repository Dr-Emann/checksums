use std::str::FromStr;


/// A hashing algorithm.
///
/// # Examples
///
/// ```
/// # use std::str::FromStr;
/// assert_eq!(checksums::Algorithm::from_str("SHA1"), Ok(checksums::Algorithm::SHA1));
/// assert_eq!(checksums::Algorithm::from_str("SHA-1"), Ok(checksums::Algorithm::SHA1));
///
/// assert_eq!(checksums::Algorithm::from_str("SHA2"), Ok(checksums::Algorithm::SHA2512));
/// assert_eq!(checksums::Algorithm::from_str("SHA-2"), Ok(checksums::Algorithm::SHA2512));
///
/// assert_eq!(checksums::Algorithm::from_str("BLAKE"), Ok(checksums::Algorithm::BLAKE));
/// assert_eq!(checksums::Algorithm::from_str("BLAKE2"), Ok(checksums::Algorithm::BLAKE2));
///
/// assert_eq!(checksums::Algorithm::from_str("MD5"), Ok(checksums::Algorithm::MD5));
/// ```
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Algorithm {
    SHA1,
    /// SHA2-256
    SHA2256,
    /// SHA2-512
    SHA2512,
    /// SHA3-256
    SHA3256,
    /// SHA3-512
    SHA3512,
    BLAKE,
    BLAKE2,
    CRC64,
    CRC32,
    CRC16,
    CRC8,
    MD5,
    /// MD6-128
    MD6128,
    /// MD6-256
    MD6256,
    /// MD6-512
    MD6512,
    XOR8,
}

impl Algorithm {
    /// Length, in bytes, of the algorithm's output hex string
    pub fn hexlen(&self) -> usize {
        match *self {
            Algorithm::XOR8 | Algorithm::CRC8 => 2,
            Algorithm::CRC16 => 4,
            Algorithm::CRC32 => 8,
            Algorithm::CRC64 => 16,
            Algorithm::MD5 |
            Algorithm::MD6128 => 32,
            Algorithm::SHA1 => 40,
            Algorithm::SHA2256 |
            Algorithm::SHA3256 |
            Algorithm::MD6256 => 64,
            Algorithm::SHA2512 |
            Algorithm::SHA3512 |
            Algorithm::BLAKE |
            Algorithm::BLAKE2 |
            Algorithm::MD6512 => 128,
        }
    }
}

impl FromStr for Algorithm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SHA1" | "SHA-1" | "sha1" => Ok(Algorithm::SHA1),
            "SHA2256" | "SHA2-256" | "SHA-2-256" | "SHA2_256" | "SHA-2_256" | "SHA_2-256" | "SHA_2_256" | "sha2256" | "sha2-256" | "sha2_256" => {
                Ok(Algorithm::SHA2256)
            }
            "SHA2" | "SHA-2" | "sha2" | "SHA2512" | "SHA2-512" | "SHA-2-512" | "SHA2_512" | "SHA-2_512" | "SHA_2-512" | "SHA_2_512" | "sha2512" |
            "sha2-512" | "sha2_512" => Ok(Algorithm::SHA2512),
            "SHA3256" | "SHA3-256" | "SHA-3-256" | "SHA3_256" | "SHA-3_256" | "SHA_3-256" | "SHA_3_256" | "sha3256" | "sha3-256" | "sha3_256" => {
                Ok(Algorithm::SHA3256)
            }
            "SHA3" | "SHA-3" | "sha3" | "SHA3512" | "SHA3-512" | "SHA-3-512" | "SHA3_512" | "SHA-3_512" | "SHA_3-512" | "SHA_3_512" | "sha3512" |
            "sha3-512" | "sha3_512" => Ok(Algorithm::SHA3512),
            "BLAKE" | "blake" => Ok(Algorithm::BLAKE),
            "BLAKE2" | "blake2" => Ok(Algorithm::BLAKE2),
            "CRC64" | "crc64" => Ok(Algorithm::CRC64),
            "CRC32" | "crc32" => Ok(Algorithm::CRC32),
            "CRC16" | "crc16" => Ok(Algorithm::CRC16),
            "CRC8" | "crc8" => Ok(Algorithm::CRC8),
            "MD5" | "md5" => Ok(Algorithm::MD5),
            "MD6128" | "MD6-128" | "MD6_128" | "md6128" | "md6-128" | "md6_128" => Ok(Algorithm::MD6128),
            "MD6256" | "MD6-256" | "MD6_256" | "md6256" | "md6-256" | "md6_256" => Ok(Algorithm::MD6256),
            "MD6512" | "MD6-512" | "MD6_512" | "md6512" | "md6-512" | "md6_512" => Ok(Algorithm::MD6512),
            "XOR8" | "xor8" => Ok(Algorithm::XOR8),
            _ => Err(format!("\"{}\" is not a recognised compression algorithm", s)),
        }
    }
}
