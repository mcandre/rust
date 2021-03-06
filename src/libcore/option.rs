/*
Module: option

Represents the presence or absence of a value.

Every option<T> value can either be some(T) or none. Where in other languages
you might use a nullable type, in Rust you would use an option type.
*/

/*
Tag: t

The option type
*/
enum t<T> {
    /* Variant: none */
    none,
    /* Variant: some */
    some(T),
}

/* Section: Operations */

/*
Function: get

Gets the value out of an option

Failure:

Fails if the value equals `none`.
*/
pure fn get<T: copy>(opt: t<T>) -> T {
    alt opt { some(x) { ret x; } none { fail "option none"; } }
}

/*
*/
fn map<T, U: copy>(opt: t<T>, f: fn(T) -> U) -> t<U> {
    alt opt { some(x) { some(f(x)) } none { none } }
}

/*
Function: is_none

Returns true if the option equals none
*/
pure fn is_none<T>(opt: t<T>) -> bool {
    alt opt { none { true } some(_) { false } }
}

/*
Function: is_some

Returns true if the option contains some value
*/
pure fn is_some<T>(opt: t<T>) -> bool { !is_none(opt) }

/*
Function: from_maybe

Returns the contained value or a default
*/
pure fn from_maybe<T: copy>(def: T, opt: t<T>) -> T {
    alt opt { some(x) { x } none { def } }
}

/*
Function: maybe

Applies a function to the contained value or returns a default
*/
fn maybe<T, U: copy>(def: U, opt: t<T>, f: fn(T) -> U) -> U {
    alt opt { none { def } some(t) { f(t) } }
}

// FIXME: Can be defined in terms of the above when/if we have const bind.
/*
Function: may

Performs an operation on the contained value or does nothing
*/
fn may<T>(opt: t<T>, f: fn(T)) {
    alt opt { none {/* nothing */ } some(t) { f(t); } }
}

#[test]
fn test() { let _x = some::<int>(10); }

// Local Variables:
// mode: rust;
// fill-column: 78;
// indent-tabs-mode: nil
// c-basic-offset: 4
// buffer-file-coding-system: utf-8-unix
// End:
