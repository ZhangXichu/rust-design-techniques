# The newtype pattern

A newtype is a one-field struct around a type you already have. Same data,
new type, so you can give it rules the inner type does not have.

This crate uses HTTP header names as the example. The data is still a
`String`. Equality and hashing ignore case, so `"Content-Type"` and
`"content-type"` are the same map key.

## The problem it solves

`String` compares byte for byte. Put two header names in a `HashMap` and they
are different keys if the capital letters differ, even when HTTP says they
are the same name.

You cannot fix that by writing `impl PartialEq for String`. Those impls
already belong to `String`, in the standard library. The
[orphan rule](https://doc.rust-lang.org/book/ch10-02-traits.html#implementing-a-trait-on-a-type)
stops your crate from adding a second one.

So you wrap it. The wrapper is yours, and you decide what equal means.

## The pattern

```rust
pub struct CaseInsensitiveString(String);
```

That is the whole shape: a tuple struct with one field. The inner string is
`self.0`. Construction looks like `Self(value)`.

It is a different type from `String`. A function that takes a
`CaseInsensitiveString` will not silently take an ordinary string. You can
implement traits on it that `String` already has, with different behaviour.

That last point is also why people wrap types they do not own. You cannot
impl a foreign trait on a foreign type, but you *can* impl it on a wrapper
that is yours. The book covers that as
[using newtypes to implement external traits](https://doc.rust-lang.org/book/ch20-02-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types).

## Most traits can be derived

A newtype does not inherit the inner type's traits. `Clone`, `Debug`,
`Display`, `From`, `Eq` each need a derive or a manual `impl`.

`Clone` and `Debug` keep the inner type's behaviour, so
`#[derive(Clone, Debug)]` is enough. `Clone` copies the inner string.
`Debug` prints `CaseInsensitiveString("Content-Type")`. There is nothing
to customise, so we do not write those by hand.

`PartialEq`, `Eq`, and `Hash` are the ones we implement ourselves. They
lower-case first, so `"Foo"` and `"foo"` compare equal. `Hash` has to use
that same mapping, or a `HashMap` will lose entries that compare equal —
if `a == b`, then `hash(a)` must equal `hash(b)`
([the contract on `Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html#hash-and-eq)).

That contrast is the lesson. Derive the traits that keep the inner type's
behaviour. Implement the ones you are changing.

## Not a type alias

```rust
type HeaderName = String; // still just String
```

An alias is a nickname. The compiler still sees `String`, so you can pass a
header name to anything that takes a string, and you cannot change `Eq`.

A newtype is a wall. `CaseInsensitiveString` and `String` do not mix until
you convert. That is the point. The book puts the two side by side under
[type safety and abstraction](https://doc.rust-lang.org/book/ch20-03-advanced-types.html#type-safety-and-abstraction-with-the-newtype-pattern).

## What you get

**The compiler keeps the two types apart.** Miles and kilometres, user ids
and passwords, header names and ordinary strings — if they are different
newtypes, mixing them is a type error, not a runtime surprise.

**You pick the behaviour.** This crate changes equality. Another newtype
might only expose a few methods, or hide a messy inner type behind a small
API.

**It is free at runtime.** The wrapper is the same size as the `String`
inside it. There is no extra allocation and no extra pointer.

**The field can stay private.** Callers use `as_str()`, `Display`, and
`From`. They cannot reach in and bypass the case-insensitive rules.

## When not to reach for it

- **You only want a shorter name?** Use a type alias. A newtype that does
  nothing except wrap `String` is ceremony.
- **You need almost every `String` method?** Each method you want on the
  wrapper needs a forwarding `impl`. That boilerplate is the usual drawback
  of newtypes. Do not implement `Deref` to reuse `String`'s methods — the
  API guidelines reserve `Deref` for
  [actual smart pointers](https://rust-lang.github.io/api-guidelines/predictability.html#only-smart-pointers-implement-deref-and-derefmut-c-deref),
  because it exposes the inner type and lets callers bypass the wrapper.

## The code

| File | Role |
| --- | --- |
| [`src/case_insensitive_str.rs`](src/case_insensitive_str.rs) | the newtype: derive `Clone`/`Debug`, write `Eq`/`Hash`/`Display`/`From` |
| [`src/main.rs`](src/main.rs) | uses it as a `HashMap` key for header names |

```
cargo run
```

`"Content-Type"`, `"content-type"`, and `"CONTENT-TYPE"` hit the same entry.
Inserting a fourth spelling replaces the value instead of growing the map.

One limit of this example: `to_lowercase()` follows Unicode simple
lowercasing, not full case folding. `"Ä"` and `"ä"` match; `"ß"` and `"SS"`
do not. For real caseless matching, look at
[`unicase`](https://docs.rs/unicase).

## Further reading

- [Type safety and abstraction with the newtype pattern](https://doc.rust-lang.org/book/ch20-03-advanced-types.html#type-safety-and-abstraction-with-the-newtype-pattern) — the book's introduction, including why a type alias is not the same thing.
- [Using the newtype pattern to implement external traits](https://doc.rust-lang.org/book/ch20-02-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types) — wrapping a foreign type so you can impl a foreign trait.
- [New Type Idiom](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) — a short miles/kilometres example.
- [Newtype](https://rust-unofficial.github.io/patterns/patterns/behavioural/newtype.html) — this pattern in the Rust Design Patterns book.
- [Hash and Eq](https://doc.rust-lang.org/std/hash/trait.Hash.html#hash-and-eq) — equal values must hash the same, which is why this crate writes both by hand.
