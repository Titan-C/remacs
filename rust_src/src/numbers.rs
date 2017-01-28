extern crate libc;

use std::os::raw::c_char;
use std::ptr;

use lisp::{LispObject, LispSubr, INTEGERP, FLOATP, MARKERP, NATNUMP, NUMBERP};

fn Ffloatp(object: LispObject) -> LispObject {
    LispObject::from_bool(FLOATP(object))
}

defun!("floatp",
       Ffloatp,
       Sfloatp,
       1,
       1,
       ptr::null(),
       "Return t if OBJECT is a floating point number.

(fn OBJECT)");

fn Fintegerp(object: LispObject) -> LispObject {
    LispObject::from_bool(INTEGERP(object))
}

defun!("integerp",
       Fintegerp,
       Sintegerp,
       1,
       1,
       ptr::null(),
       "Return t if OBJECT is an integer.

(fn OBJECT)");

fn Finteger_or_marker_p(object: LispObject) -> LispObject {
    LispObject::from_bool(MARKERP(object) || INTEGERP(object))
}

defun!("integer-or-marker-p",
       Finteger_or_marker_p,
       Sinteger_or_marker_p,
       1,
       1,
       ptr::null(),
       "Return t if OBJECT is an integer or a marker (editor pointer).

(fn OBJECT)");

fn Fnatnump(object: LispObject) -> LispObject {
    LispObject::from_bool(NATNUMP(object))
}

defun!("natnump",
       Fnatnump,
       Snatnump,
       1,
       1,
       ptr::null(),
       "Return t if OBJECT is a non-negative integer.

(fn OBJECT)");

fn Fnumberp(object: LispObject) -> LispObject {
    LispObject::from_bool(NUMBERP(object))
}

defun!("numberp",
       Fnumberp,
       Snumberp,
       1,
       1,
       ptr::null(),
       "Return t if OBJECT is a number (floating point or integer).

(fn OBJECT)");

fn Fnumber_or_marker_p(object: LispObject) -> LispObject {
    LispObject::from_bool(NUMBERP(object) || MARKERP(object))
}

defun!("number-or-marker-p",
       Fnumber_or_marker_p,
       Snumber_or_marker_p,
       1,
       1,
       ptr::null(),
       "Return t if OBJECT is a number or a marker (editor pointer).

(fn OBJECT)");
