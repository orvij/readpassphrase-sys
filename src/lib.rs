#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// Supply a prompt to and read password from `/dev/tty`
///
/// From `readpassphrase(3)`:
///
/// ```no_build
/// The readpassphrase() function displays a prompt to, and reads in a
/// passphrase from, /dev/tty. If this file is inaccessible and the
/// RPP_REQUIRE_TTY flag is not set, readpassphrase() displays the prompt on
/// the standard error output and reads from the standard input. In this
/// case it is generally not possible to turn off echo.
///
/// Up to bufsiz - 1 characters (one is for the NUL) are read into the
/// provided buffer buf. Any additional characters and the terminating
/// newline (or return) character are discarded.
///
/// The flags argument is the bitwise OR of zero or more of the following
/// values:
///
///        RPP_ECHO_OFF            turn off echo (default behavior)
///        RPP_ECHO_ON             leave echo on
///        RPP_REQUIRE_TTY         fail if there is no tty
///        RPP_FORCELOWER          force input to lower case
///        RPP_FORCEUPPER          force input to upper case
///        RPP_SEVENBIT            strip the high bit from input
///   	   RPP_STDIN               read passphrase from stdin; ignore prompt
///
/// The calling process should zero the passphrase as soon as possible to
/// avoid leaving the cleartext passphrase visible in the process's address
/// space.
///
///	RETURN VALUES
///
/// Upon successful completion, readpassphrase() returns a pointer to the
/// NUL-terminated passphrase.  If an error is encountered, the terminal
/// state is restored and a null pointer is returned.
/// ```
#[cfg(not(target_os = "openbsd"))]
pub extern "C" fn readpassphrase(_prompt: *const libc::c_char, _buf: *mut libc::c_char, _bufsiz: u64, _flags: i32);

#[cfg(test)]
mod tests {
}
