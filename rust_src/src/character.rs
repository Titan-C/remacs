use std::ptr;

use lisp::{self, LispObject};
use remacs_sys::{EmacsInt, CHARACTERBITS};

/// Maximum character code
pub const MAX_CHAR: EmacsInt = (1 << CHARACTERBITS as EmacsInt) - 1;

fn max_char() -> LispObject {
    lisp::make_number(MAX_CHAR)
}

defun!("max-char",
       Fmax_char(),
       Smax_char,
       max_char,
       0,
       0,
       ptr::null(),
       "Return the character of the maximum code.");

// Nonzero iff X is a character.
pub fn CHARACTERP(x: LispObject) -> bool {
    lisp::NATNUMP(x) && lisp::XFASTINT(x) <= MAX_CHAR
}

// Check if Lisp object X is a character or not.
pub fn CHECK_CHARACTER(x: LispObject) -> LispObject {
    lisp::CHECK_TYPE(CHARACTERP(x), Qcharacterp, x)
}

// If C is not ASCII, make it multibyte.  Assumes C < 256.
pub fn MAKE_CHAR_MULTIBYTE(c: LispObject) -> LispObject {
    (debug_assert!((c) >= 0 && (c) < 256), (c) = UNIBYTE_TO_CHAR(c))
}

fn characterp(object: LispObject, _ignore: LispObject) -> LispObject {
    if CHARACTERP(object) {
        LispObject::constant_t()
    } else {
        LispObject::constant_nil()
    }
}

defun!("characterp",
       Fcharacterp(x, y),
       Scharacterp,
       characterp,
       1,
       2,
       ptr::null(),
       "Return non-nil if OBJECT is a character.
In Emacs Lisp, characters are represented by character codes, which
are non-negative integers.  The function `max-char' returns the
maximum character code.
usage: (characterp OBJECT)");

fn unibyte_char_to_multibyte(ch: LispObject) -> LispObject {

    CHECK_CHARACTER(ch);
    let c = XFASTINT(ch);
    if (c >= 0x100) {
        error("Not a unibyte character: %d", c);
    }
    MAKE_CHAR_MULTIBYTE(c);
    make_number(c)
}

defun!("unibyte-char-to-multibyte",
       Funibyte_char_to_multibyte(x),
       Sunibyte_char_to_multibyte,
       1,
       1,
       ptr::null(),
       "Convert the byte CH to multibyte character.");



fn multibyte_char_to_unibyte(ch: LispObject) -> LispObject {

    CHECK_CHARACTER(ch);
    let cm = XFASTINT(ch);
    if (cm < 256) {
        /* Can't distinguish a byte read from a unibyte buffer from
        a latin1 char, so let's let it slide.  */
        ch
    } else {
        let cu = CHAR_TO_BYTE_SAFE(cm);
        make_number(cu)
    }
}

defu!("multibyte-char-to-unibyte",
      Fmultibyte_char_to_unibyte(x),
      Smultibyte_char_to_unibyte,
      1,
      1,
      ptr::null(),
      "Convert the multibyte character CH to a byte.
If the multibyte character does not \
       represent a byte, return -1.");
