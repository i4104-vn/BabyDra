use std::io::Write;
use std::os::unix::net::UnixStream;

pub const WORKSPACE_SOCKET_PATH: &str = "/tmp/babydra-workspace.socket";

/// Tries to signal an existing running workspace daemon.
pub fn try_signal_daemon(msg: &[u8]) -> bool {
    if let Ok(mut stream) = UnixStream::connect(WORKSPACE_SOCKET_PATH) {
        let _ = stream.write_all(msg);
        return true;
    }
    false
}
