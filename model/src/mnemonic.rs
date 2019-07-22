use crate::address::{Address, AddressError};
use crate::extended_private_key::{ExtendedPrivateKey, ExtendedPrivateKeyError};
use crate::extended_public_key::ExtendedPublicKey;
use crate::private_key::PrivateKey;
use crate::public_key::PublicKey;
use crate::wordlist::WordlistError;

use std::{fmt::{Debug, Display}, str::FromStr};

/// The interface for a generic mnemonic.
pub trait Mnemonic:
    Clone
    + Debug
    + Display
    + FromStr
    + Send
    + Sync
    + 'static
    + Eq
    + Sized
{
    type Address: Address;
    type ExtendedPrivateKey: ExtendedPrivateKey;
    type ExtendedPublicKey: ExtendedPublicKey;
    type Format;
    type PrivateKey: PrivateKey;
    type PublicKey: PublicKey;

    /// Returns a new mnemonic phrase given the word count and language.
    fn new(word_count: u8) -> Result<Self, MnemonicError>;

    /// Returns the extended private key of the corresponding mnemonic.
    fn to_extended_private_key(
        &self,
        password: Option<&str>
    ) -> Result<Self::ExtendedPrivateKey, MnemonicError>;

    /// Returns the extended public key of the corresponding mnemonic.
    fn to_extended_public_key(
        &self,
        password: Option<&str>
    ) -> Result<Self::ExtendedPublicKey, MnemonicError>;

    /// Returns the private key of the corresponding mnemonic.
    fn to_private_key(
        &self,
        password: Option<&str>
    ) -> Result<Self::PrivateKey, MnemonicError>;

    /// Returns the public key of the corresponding mnemonic.
    fn to_public_key(
        &self,
        password: Option<&str>
    ) -> Result<Self::PublicKey, MnemonicError>;

    /// Returns the address of the corresponding mnemonic.
    fn to_address(
        &self,
        password: Option<&str>,
        format: &Self::Format
    ) -> Result<Self::Address, MnemonicError>;
}

#[derive(Debug, Fail)]
pub enum MnemonicError {

    #[fail(display = "{}", _0)]
    AddressError(AddressError),

    #[fail(display = "{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[fail(display = "{}", _0)]
    ExtendedPrivateKeyError(ExtendedPrivateKeyError),

    #[fail(display = "Invalid mnemonic word count: {}", _0)]
    InvalidWordCount(u8),

    #[fail(display = "Invalid entropy length: {}", _0)]
    InvalidEntropyLength(usize),

    #[fail(display = "Invalid phrase: {}", _0)]
    InvalidPhrase(String),

    #[fail(display = "Invalid word not found in dictionary: {}", _0)]
    InvalidWord(String),

    #[fail(display = "{}", _0)]
    WordlistError(WordlistError),
}

impl From<AddressError> for MnemonicError {
    fn from(error: AddressError) -> Self {
        MnemonicError::AddressError(error)
    }
}

impl From<ExtendedPrivateKeyError> for MnemonicError {
    fn from(error: ExtendedPrivateKeyError) -> Self {
        MnemonicError::ExtendedPrivateKeyError(error)
    }
}

impl From<WordlistError> for MnemonicError {
    fn from(error: WordlistError) -> Self {
        MnemonicError::WordlistError(error)
    }
}

impl From<rand_core::Error> for MnemonicError {
    fn from(error: rand_core::Error) -> Self {
        MnemonicError::Crate("rand", format!("{:?}", error))
    }
}

impl From<std::io::Error> for MnemonicError {
    fn from(error: std::io::Error) -> Self {
        MnemonicError::Crate("std::io", format!("{:?}", error))
    }
}