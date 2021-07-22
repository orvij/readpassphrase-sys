#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::{c_char, c_int};

/// Turn off echo (default).
pub const RPP_ECHO_OFF: c_int = 0x00;

/// Leave echo on.
pub const RPP_ECHO_ON: c_int = 0x01;

/// Fail if there is no tty.
pub const RPP_REQUIRE_TTY: c_int = 0x02;

/// Force input to lower case.
pub const RPP_FORCELOWER: c_int = 0x04;

/// Force input to upper case.
pub const RPP_FORCEUPPER: c_int = 0x08;

/// Strip the high bit from input.
pub const RPP_SEVENBIT: c_int = 0x10;

/// Read from stdin, no /dev/tty
pub const RPP_STDIN: c_int = 0x20;

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
extern "C" {
    pub fn readpassphrase(_prompt: *const c_char, _buf: *mut c_char, _bufsiz: usize, _flags: c_int) -> *mut c_char;
}
