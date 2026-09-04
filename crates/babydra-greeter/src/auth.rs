use greetd_ipc::{Request, Response};
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

/// Write request to greetd socket.
fn write_req(stream: &mut UnixStream, req: &Request) -> Result<(), String> {
    let body = serde_json::to_vec(req).map_err(|e| e.to_string())?;
    let len = (body.len() as u32).to_ne_bytes();
    stream.write_all(&len).map_err(|e| e.to_string())?;
    stream.write_all(&body).map_err(|e| e.to_string())?;
    Ok(())
}

/// Read response from greetd socket.
fn read_res(stream: &mut UnixStream) -> Result<Response, String> {
    let mut len_bytes = [0; 4];
    stream
        .read_exact(&mut len_bytes)
        .map_err(|e| e.to_string())?;
    let len = u32::from_ne_bytes(len_bytes);
    let mut body = vec![0; len as usize];
    stream.read_exact(&mut body).map_err(|e| e.to_string())?;
    serde_json::from_slice(&body).map_err(|e| e.to_string())
}

fn format_greetd_err(stage: &str, res: Response) -> String {
    match res {
        Response::Error { error_type, description } => {
            let err = format!("{stage} failed: {:?}: {}", error_type, description);
            tracing::error!(target: "babydra-greeter", "{}", err);
            err
        }
        other => {
            let err = format!("Unexpected response from greetd after {stage}: {:?}", other);
            tracing::error!(target: "babydra-greeter", "{}", err);
            err
        }
    }
}

/// Perform authentication via greetd IPC.
pub fn do_login(user: String, pass: String) -> Result<(), String> {
    tracing::info!(target: "babydra-greeter", "Initiating Greetd authentication session for user: {:?}", user);

    let socket_path = std::env::var("GREETD_SOCK").map_err(|_| {
        let err = "GREETD_SOCK environment variable not set. Are you running under greetd?".to_string();
        tracing::error!(target: "babydra-greeter", "{}", err);
        err
    })?;

    let mut stream = UnixStream::connect(&socket_path).map_err(|e| {
        let err_msg = format!("Failed to connect to greetd socket at {:?}: {}", socket_path, e);
        tracing::error!(target: "babydra-greeter", "{}", err_msg);
        err_msg
    })?;

    tracing::info!(target: "babydra-greeter", "Greetd socket connected. Sending CreateSession...");
    write_req(&mut stream, &Request::CreateSession { username: user })?;

    // Step 1: Wait for AuthMessage
    let res = read_res(&mut stream)?;
    let (auth_msg_type, auth_msg) = match res {
        Response::AuthMessage { auth_message_type, auth_message } => (auth_message_type, auth_message),
        other => return Err(format_greetd_err("CreateSession", other)),
    };

    tracing::info!(
        target: "babydra-greeter",
        "Greetd requested auth response (type={:?}, message={:?}). Sending password...",
        auth_msg_type, auth_msg
    );
    write_req(&mut stream, &Request::PostAuthMessageResponse { response: Some(pass) })?;

    // Step 2: Verify password response
    let res = read_res(&mut stream)?;
    match res {
        Response::Success => {}
        other => return Err(format_greetd_err("Authentication", other)),
    }

    // Step 3: Start session
    tracing::info!(target: "babydra-greeter", "Password accepted. Sending StartSession (cmd: ['labwc'])...");
    write_req(&mut stream, &Request::StartSession {
        cmd: vec!["labwc".to_string()],
        env: vec![],
    })?;

    let res = read_res(&mut stream)?;
    match res {
        Response::Success => {
            tracing::info!(target: "babydra-greeter", "StartSession successful! Handing session execution over to labwc.");
            Ok(())
        }
        other => Err(format_greetd_err("StartSession", other)),
    }
}
