use std::{
    fmt,
    sync::{MutexGuard, PoisonError},
};

use crate::{proxy, proxy::next_mining_notify::NextMiningNotify};
use v1::server_to_client::Notify;

pub type ProxyResult<'a, T> = core::result::Result<T, Error<'a>>;

#[derive(Debug)]
pub enum LockError<'a> {
    Bridge(PoisonError<MutexGuard<'a, proxy::Bridge>>),
    NextMiningNotify(PoisonError<MutexGuard<'a, NextMiningNotify>>),
}

#[derive(Debug)]
pub enum ChannelSendError<'a> {
    SubmitSharesExtended(
        async_channel::SendError<roles_logic_sv2::mining_sv2::SubmitSharesExtended<'a>>,
    ),
    SetNewPrevHash(async_channel::SendError<roles_logic_sv2::mining_sv2::SetNewPrevHash<'a>>),
    Notify(async_channel::SendError<Notify<'a>>),
}

#[derive(Debug)]
pub enum Error<'a> {
    /// Errors on bad CLI argument input.
    BadCliArgs,
    /// Errors on bad `serde_json` serialize/deserialize.
    BadSerdeJson(serde_json::Error),
    /// Errors on bad `toml` deserialize.
    BadTomlDeserialize(toml::de::Error),
    /// Errors from `binary_sv2` crate.
    BinarySv2(binary_sv2::Error),
    /// Errors on bad noise handshake.
    CodecNoise(codec_sv2::noise_sv2::Error),
    /// Errors from `framing_sv2` crate.
    FramingSv2(framing_sv2::Error),
    /// Errors on bad `TcpStream` connection.
    Io(std::io::Error),
    /// Errors if SV1 downstream returns a `mining.submit` with no version bits.
    NoSv1VersionBits,
    /// Errors on bad `String` to `int` conversion.
    ParseInt(std::num::ParseIntError),
    /// Errors from `roles_logic_sv2` crate.
    RolesSv2Logic(roles_logic_sv2::errors::Error),
    UpstreamIncoming(roles_logic_sv2::errors::Error),
    /// SV1 protocol library error
    V1Protocol(v1::error::Error<'a>),
    SubprotocolMining(String),
    // Locking Errors
    // PoisonLock(LockError<'a>),
    // Channel Receiver Error
    ChannelErrorReceiver(async_channel::RecvError),
    // Channel Sender Errors
    ChannelErrorSender(ChannelSendError<'a>),
}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            BadCliArgs => write!(f, "Bad CLI arg input"),
            BadSerdeJson(ref e) => write!(f, "Bad serde json: `{:?}`", e),
            BadTomlDeserialize(ref e) => write!(f, "Bad `toml` deserialize: `{:?}`", e),
            BinarySv2(ref e) => write!(f, "Binary SV2 error: `{:?}`", e),
            CodecNoise(ref e) => write!(f, "Noise error: `{:?}", e),
            FramingSv2(ref e) => write!(f, "Framing SV2 error: `{:?}`", e),
            Io(ref e) => write!(f, "I/O error: `{:?}", e),
            NoSv1VersionBits => write!(
                f,
                "`mining.submit` received from SV1 downstream does not contain `version_bits`"
            ),
            ParseInt(ref e) => write!(f, "Bad convert from `String` to `int`: `{:?}`", e),
            RolesSv2Logic(ref e) => write!(f, "Roles SV2 Logic Error: `{:?}`", e),
            V1Protocol(ref e) => write!(f, "V1 Protocol Error: `{:?}`", e),
            SubprotocolMining(ref e) => write!(f, "Subprotocol Mining Error: `{:?}`", e),
            UpstreamIncoming(ref e) => write!(f, "Upstream parse incoming error: `{:?}`", e),
            // PoisonLock(ref e) => write!(f, "Poison Lock error: `{:?}`", e),
            ChannelErrorReceiver(ref e) => write!(f, "Channel receive error: `{:?}`", e),
            ChannelErrorSender(ref e) => write!(f, "Channel send error: `{:?}`", e),
        }
    }
}

impl<'a> From<binary_sv2::Error> for Error<'a> {
    fn from(e: binary_sv2::Error) -> Self {
        Error::BinarySv2(e)
    }
}

impl<'a> From<codec_sv2::noise_sv2::Error> for Error<'a> {
    fn from(e: codec_sv2::noise_sv2::Error) -> Self {
        Error::CodecNoise(e)
    }
}

impl<'a> From<framing_sv2::Error> for Error<'a> {
    fn from(e: framing_sv2::Error) -> Self {
        Error::FramingSv2(e)
    }
}

impl<'a> From<std::io::Error> for Error<'a> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl<'a> From<std::num::ParseIntError> for Error<'a> {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::ParseInt(e)
    }
}

impl<'a> From<roles_logic_sv2::errors::Error> for Error<'a> {
    fn from(e: roles_logic_sv2::errors::Error) -> Self {
        Error::RolesSv2Logic(e)
    }
}

impl<'a> From<serde_json::Error> for Error<'a> {
    fn from(e: serde_json::Error) -> Self {
        Error::BadSerdeJson(e)
    }
}

impl<'a> From<toml::de::Error> for Error<'a> {
    fn from(e: toml::de::Error) -> Self {
        Error::BadTomlDeserialize(e)
    }
}

impl<'a> From<v1::error::Error<'a>> for Error<'a> {
    fn from(e: v1::error::Error<'a>) -> Self {
        Error::V1Protocol(e)
    }
}

impl<'a> From<async_channel::RecvError> for Error<'a> {
    fn from(e: async_channel::RecvError) -> Self {
        Error::ChannelErrorReceiver(e)
    }
}

// *** LOCK ERRORS ***
// impl<'a> From<PoisonError<MutexGuard<'a, proxy::Bridge>>> for Error<'a> {
//     fn from(e: PoisonError<MutexGuard<'a, proxy::Bridge>>) -> Self {
//         Error::PoisonLock(
//             LockError::Bridge(e)
//         )
//     }
// }

// impl<'a> From<PoisonError<MutexGuard<'a, NextMiningNotify>>> for Error<'a> {
//     fn from(e: PoisonError<MutexGuard<'a, NextMiningNotify>>) -> Self {
//         Error::PoisonLock(
//             LockError::NextMiningNotify(e)
//         )
//     }
// }

// *** CHANNEL SENDER ERRORS ***
impl<'a> From<async_channel::SendError<roles_logic_sv2::mining_sv2::SubmitSharesExtended<'a>>>
    for Error<'a>
{
    fn from(
        e: async_channel::SendError<roles_logic_sv2::mining_sv2::SubmitSharesExtended<'a>>,
    ) -> Self {
        Error::ChannelErrorSender(ChannelSendError::SubmitSharesExtended(e))
    }
}

impl<'a> From<async_channel::SendError<roles_logic_sv2::mining_sv2::SetNewPrevHash<'a>>>
    for Error<'a>
{
    fn from(e: async_channel::SendError<roles_logic_sv2::mining_sv2::SetNewPrevHash<'a>>) -> Self {
        Error::ChannelErrorSender(ChannelSendError::SetNewPrevHash(e))
    }
}

impl<'a> From<async_channel::SendError<Notify<'a>>> for Error<'a> {
    fn from(e: async_channel::SendError<Notify<'a>>) -> Self {
        Error::ChannelErrorSender(ChannelSendError::Notify(e))
    }
}
