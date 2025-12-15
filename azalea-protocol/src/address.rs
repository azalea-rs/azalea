use std::{
    fmt::{self, Debug, Display},
    net::SocketAddr,
    str::FromStr,
};

use hickory_resolver::ResolveError;

use crate::resolve::resolve_address;

/// Something that might be able to be parsed and looked up as a server address.
///
/// This is typically used by Azalea as a generic argument, so the user can
/// choose to pass either a string or an already-resolved address.
pub trait ResolvableAddr: Debug + Clone {
    fn server_addr(self) -> Result<ServerAddr, ResolveError>;
    fn resolve(self) -> impl Future<Output = Result<ResolvedAddr, ResolveError>> + Send;
}
impl<T: TryInto<ServerAddr, Error = ServerAddrParseError> + Debug + Send + Clone> ResolvableAddr
    for T
{
    fn server_addr(self) -> Result<ServerAddr, ResolveError> {
        self.try_into()
            .map_err(|_| "failed to parse address".into())
    }

    async fn resolve(self) -> Result<ResolvedAddr, ResolveError> {
        ResolvedAddr::new(self.server_addr()?).await
    }
}

impl ResolvableAddr for &ResolvedAddr {
    fn server_addr(self) -> Result<ServerAddr, ResolveError> {
        Ok(self.server.clone())
    }

    async fn resolve(self) -> Result<ResolvedAddr, ResolveError> {
        Ok(self.clone())
    }
}

/// A host and port. It's possible that the port doesn't resolve to anything.
///
/// # Examples
///
/// `ServerAddr` implements TryFrom<&str>, so you can use it like this:
/// ```
/// use azalea_protocol::address::ServerAddr;
///
/// let addr = ServerAddr::try_from("localhost:25565").unwrap();
/// assert_eq!(addr.host, "localhost");
/// assert_eq!(addr.port, 25565);
/// ```
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ServerAddr {
    pub host: String,
    pub port: u16,
}

/// An empty error type that's used when we fail to convert a type to a
/// `ServerAddr`.
///
/// You usually want to use the [`ResolvableAddr`] type instead, which works
/// with [`ResolveError`]s.
#[derive(Debug)]
pub struct ServerAddrParseError;

impl TryFrom<&str> for ServerAddr {
    type Error = ServerAddrParseError;

    /// Convert a Minecraft server address (`host:port`, the port is optional)
    /// to a `ServerAddress`
    fn try_from(string: &str) -> Result<Self, Self::Error> {
        if string.is_empty() {
            return Err(ServerAddrParseError);
        }
        let mut parts = string.split(':');
        let host = parts.next().ok_or(ServerAddrParseError)?.to_owned();
        // default the port to 25565
        let port = parts.next().unwrap_or("25565");
        let port = u16::from_str(port).ok().ok_or(ServerAddrParseError)?;
        Ok(ServerAddr { host, port })
    }
}
impl TryFrom<String> for ServerAddr {
    type Error = ServerAddrParseError;

    fn try_from(string: String) -> Result<Self, Self::Error> {
        ServerAddr::try_from(string.as_str())
    }
}

impl From<SocketAddr> for ServerAddr {
    /// Convert an existing `SocketAddr` into a `ServerAddress`.
    ///
    /// This just converts the IP to a string and passes along the port. The
    /// resolver will realize it's already an IP address and not do any DNS
    /// requests.
    fn from(addr: SocketAddr) -> Self {
        ServerAddr {
            host: addr.ip().to_string(),
            port: addr.port(),
        }
    }
}

impl Display for ServerAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

/// Serde deserialization for ServerAddress.
///
/// This is useful if you're storing the server address in a config file.
impl<'de> serde::Deserialize<'de> for ServerAddr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        ServerAddr::try_from(string.as_str())
            .map_err(|_| serde::de::Error::custom("failed to parse address"))
    }
}

/// Serde serialization for ServerAddress.
///
/// This uses the Display impl, so it will serialize to a string.
impl serde::Serialize for ServerAddr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// An address that may be used to connect to a Minecraft server.
#[derive(Clone, Debug)]
pub struct ResolvedAddr {
    /// The initial address that we passed when trying to connect.
    ///
    /// This is necessary because clients send this to the server when they
    /// connect.
    pub server: ServerAddr,
    /// The IP and port that we will actually connect to.
    pub socket: SocketAddr,
}

impl ResolvedAddr {
    pub async fn new(server: impl Into<ServerAddr>) -> Result<Self, ResolveError> {
        let server = server.into();
        let socket = resolve_address(&server).await?;
        Ok(Self { server, socket })
    }
}
