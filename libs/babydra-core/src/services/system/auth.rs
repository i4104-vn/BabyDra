//! PAM authentication helper for the BabyDra screen locker.
//! Integrates with the system's "login" PAM service to verify passwords.

use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_void};

/// PAM message styles indicating prompt echo off.
pub const PAM_PROMPT_ECHO_OFF: c_int = 1;
/// PAM message styles indicating prompt echo on.
pub const PAM_PROMPT_ECHO_ON: c_int = 2;
/// Status code for PAM success.
pub const PAM_SUCCESS: c_int = 0;

/// Represents a PAM message structure containing the prompt style and the text message.
#[repr(C)]
pub struct pam_message {
    pub msg_style: c_int,
    pub msg: *const c_char,
}

/// Represents a PAM response containing the response string and the return status code.
#[repr(C)]
pub struct pam_response {
    pub resp: *mut c_char,
    pub resp_retcode: c_int,
}

/// PAM conversation structure holding the callback handler and application data pointer.
#[repr(C)]
pub struct pam_conv {
    pub conv: Option<
        unsafe extern "C" fn(
            num_msg: c_int,
            msg: *mut *mut pam_message,
            resp: *mut *mut pam_response,
            appdata_ptr: *mut c_void,
        ) -> c_int,
    >,
    pub appdata_ptr: *mut c_void,
}

#[link(name = "pam")]
extern "C" {
    /// Starts a PAM session.
    pub fn pam_start(
        service_name: *const c_char,
        user: *const c_char,
        pam_conversation: *const pam_conv,
        pamh: *mut *mut c_void,
    ) -> c_int;

    /// Authenticates the user in the active PAM session.
    pub fn pam_authenticate(pamh: *mut c_void, flags: c_int) -> c_int;

    /// Terminates the PAM session.
    pub fn pam_end(pamh: *mut c_void, pam_status: c_int) -> c_int;
}

/// C-compatible callback conversation handler that supplies the user's password to PAM prompts.
pub unsafe extern "C" fn pam_conversation_fn(
    num_msg: c_int,
    msg: *mut *mut pam_message,
    resp: *mut *mut pam_response,
    appdata_ptr: *mut c_void,
) -> c_int {
    if num_msg <= 0 {
        return 0;
    }

    let resps =
        libc::malloc(num_msg as usize * std::mem::size_of::<pam_response>()) as *mut pam_response;
    if resps.is_null() {
        return 4; // PAM_BUF_ERR
    }
    std::ptr::write_bytes(resps, 0, num_msg as usize);

    let password_ptr = appdata_ptr as *const c_char;

    for i in 0..num_msg {
        let msg_ptr = *msg.add(i as usize);
        let msg_style = (*msg_ptr).msg_style;

        if msg_style == PAM_PROMPT_ECHO_OFF || msg_style == PAM_PROMPT_ECHO_ON {
            let resp_str = libc::strdup(password_ptr);
            (*resps.add(i as usize)).resp = resp_str;
            (*resps.add(i as usize)).resp_retcode = 0;
        }
    }

    *resp = resps;
    PAM_SUCCESS
}

/// Verifies user credentials using the system PAM "login" service.
///
/// Returns `true` if authentication succeeded, `false` otherwise.
pub fn verify_password(username: &str, password: &str) -> bool {
    // Guard against NUL bytes from user input before touching C APIs.
    let (Ok(username_c), Ok(password_c)) = (CString::new(username), CString::new(password)) else {
        return false;
    };

    let conv = pam_conv {
        conv: Some(pam_conversation_fn),
        appdata_ptr: password_c.as_ptr() as *mut c_void,
    };

    let mut pamh: *mut c_void = std::ptr::null_mut();

    unsafe {
        // "login" is a compile-time constant, safe to unwrap.
        let service_c = CString::new("login").unwrap();
        let start_status = pam_start(service_c.as_ptr(), username_c.as_ptr(), &conv, &mut pamh);

        if start_status != PAM_SUCCESS {
            return false;
        }

        let auth_status = pam_authenticate(pamh, 0);
        pam_end(pamh, auth_status);

        auth_status == PAM_SUCCESS
    }
}
