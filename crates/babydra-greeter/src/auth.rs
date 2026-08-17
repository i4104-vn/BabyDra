use greetd_ipc::{Request, Response};
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

/// Write req.
fn write_req(stream: &mut UnixStream, req: &Request) -> Result<(), String> {
    let body = serde_json::to_vec(req).map_err(|e| e.to_string())?;
    let len = (body.len() as u32).to_ne_bytes();
    stream.write_all(&len).map_err(|e| e.to_string())?;
    stream.write_all(&body).map_err(|e| e.to_string())?;
    Ok(())
}

/// Read res.
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

/// Do login.
pub fn do_login(user: String, pass: String) -> Result<(), String> {
    tracing::info!(target: "babydra-greeter", "Initiating Greetd authentication session for user: {:?}", user);

    let socket_path = match std::env::var("GREETD_SOCK") {
        Ok(path) => {
            tracing::info!(target: "babydra-greeter", "Connecting to greetd Unix socket at: {:?}", path);
            path
        }
        Err(_) => {
            let err = "GREETD_SOCK environment variable not set. Are you running under greetd?"
                .to_string();
            tracing::error!(target: "babydra-greeter", "{}", err);
            return Err(err);
        }
    };

    let mut stream = UnixStream::connect(&socket_path).map_err(|e| {
        let err_msg = format!(
            "Failed to connect to greetd socket at {:?}: {}",
            socket_path, e
        );
        tracing::error!(target: "babydra-greeter", "{}", err_msg);
        err_msg
    })?;

    tracing::info!(target: "babydra-greeter", "Greetd socket connected successfully. Sending CreateSession request...");
    let req = Request::CreateSession {
        username: user.clone(),
    };
    write_req(&mut stream, &req)?;

    let res = read_res(&mut stream)?;
    tracing::info!(target: "babydra-greeter", "Received response from greetd after CreateSession");

    match res {
        Response::AuthMessage {
            auth_message_type,
            auth_message,
        } => {
            tracing::info!(
                target: "babydra-greeter",
                "Greetd requested auth response (type={:?}, message={:?}). Sending password...",
                auth_message_type, auth_message
            );
            let req = Request::PostAuthMessageResponse {
                response: Some(pass),
            };
            write_req(&mut stream, &req)?;

            let res = read_res(&mut stream)?;
            match res {
                Response::Success => {
                    tracing::info!(target: "babydra-greeter", "Password accepted by greetd. Sending StartSession (cmd: ['labwc'])...");
                    let req = Request::StartSession {
                        cmd: vec!["labwc".to_string()],
                        env: vec![],
                    };
                    write_req(&mut stream, &req)?;
                    let res = read_res(&mut stream)?;
                    match res {
                        Response::Success => {
                            tracing::info!(target: "babydra-greeter", "StartSession successful! Handing session execution over to labwc.");
                            Ok(())
                        }
                        Response::Error {
                            error_type,
                            description,
                        } => {
                            let err = format!("{:?}: {}", error_type, description);
                            tracing::error!(target: "babydra-greeter", "StartSession failed: {}", err);
                            Err(err)
                        }
                        _ => {
                            let err =
                                "Unexpected response from greetd after StartSession".to_string();
                            tracing::error!(target: "babydra-greeter", "{}", err);
                            Err(err)
                        }
                    }
                }
                Response::Error {
                    error_type,
                    description,
                } => {
                    let err = format!("{:?}: {}", error_type, description);
                    tracing::error!(target: "babydra-greeter", "Authentication failed: {}", err);
                    Err(err)
                }
                _ => {
                    let err = "Unexpected response from greetd after password submit".to_string();
                    tracing::error!(target: "babydra-greeter", "{}", err);
                    Err(err)
                }
            }
        }
        Response::Error {
            error_type,
            description,
        } => {
            let err = format!("{:?}: {}", error_type, description);
            tracing::error!(target: "babydra-greeter", "CreateSession failed: {}", err);
            Err(err)
        }
        _ => {
            let err = "Unexpected response from greetd after CreateSession".to_string();
            tracing::error!(target: "babydra-greeter", "{}", err);
            Err(err)
        }
    }
}
