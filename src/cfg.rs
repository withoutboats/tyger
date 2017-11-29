use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Duration;

use log::LevelFilter;

#[derive(Configure, Deserialize)]
#[configure(generate_docs)]
#[serde(default)]
pub struct Config {
    #[configure(docs = "The address that your server will listen on. localhost:7878 by default.")]
    pub addr: SocketAddr,
    #[configure(docs = "Logging level for tyger (INFO by default).")]
    pub log_level: LevelFilter,
    #[configure(docs = "The maximum timeout that will be applied to every request (no timeout by \
                        default).")]
    pub timeout: Option<Duration>,
    #[configure(docs = "The address that the command/control server will listen on. If not set, \
                        the command server will not be run.")]
    pub cmd_addr: Option<SocketAddr>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            addr: ("localhost", 7878).to_socket_addrs().unwrap().next().unwrap(),
            log_level: LevelFilter::Info,
            timeout: None,
            cmd_addr: None,
        }
    }
}
