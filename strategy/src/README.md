# The strategy pattern

One algorithm, one decision inside it that could be made several ways. The
strategy pattern pulls that decision out, puts it behind a trait, and lets you
swap it without touching the algorithm.

This crate uses retrying as the example: the algorithm is "try again until it
works", and the decision is how long to wait between tries.

## The problem it solves

A retry loop does two things. The loop itself is always the same: call the
operation, return if it worked, give up after `max_attempts`, otherwise sleep
and try again. The sleep time varies: fixed, doubling, or doubling with
randomness.

If you write that choice into the loop as flags, every new way of waiting
means editing code that already works, the flags can contradict each other,
and you cannot test the waiting maths on its own.

## The pattern

Name the decision, and give it a trait:

```rust
pub trait DelayStrategy {
    fn delay(&self, attempt: u32) -> std::time::Duration;
}
```

Three roles:

- **The trait** is the decision: [`DelayStrategy`](strategies/mod.rs).
- **The strategies** are the answers: [`FixedDelay`](strategies/fixed_delay.rs),
  [`ExponentialDelay`](strategies/expo.rs),
  [`ExponentialDelayWithJitter`](strategies/expo_with_jitter.rs).
- **The user** is the algorithm that holds one and asks it:
  [`Retrier`](retrier.rs). It calls `self.strategy.delay(attempt)` and never
  finds out which one it got.

`Retrier` shrinks to the part that never changes. Each way of waiting sits in
its own file.

## What you get

**Adding a way to wait does not mean changing the loop.** New file, implement
the trait, done.

**The choice can be made late.** `Retrier` holds a `Box<dyn DelayStrategy>`, so
which strategy to use can come from a config file or a flag at runtime.

**Each strategy is easy to test.** They are functions from a number to a
duration. No loop, no real clock.

## Two ways to do it in Rust

A trait can be used two ways:

- **Dynamic:** `Box<dyn DelayStrategy>`. The exact type is not known until
  the program runs. One extra indirect call per ask; nothing next to a sleep.
- **Static:** `<S: DelayStrategy>`. The compiler knows the type and can
  inline the call, but the choice is fixed at build time.

This crate uses both, at different points. [`Retrier`](retrier.rs) *stores*
`Box<dyn DelayStrategy>`, so which strategy you pass can change at runtime.
[`Retrier::new`](retrier.rs) *takes* `S: DelayStrategy` and boxes it, so
callers write `Retrier::new(FixedDelay::new(base), 4)` instead of wrapping
it themselves.

A trait only works in `Box<dyn ...>` if it is *object safe*: methods take
`self` by reference and are not generic. `DelayStrategy` takes `&self`, so it
qualifies. The cost is that a strategy cannot mutate what it owns — see
[`expo_with_jitter.rs`](strategies/expo_with_jitter.rs), which builds a new
RNG on every call.

## Traits compared to C++

In C++ this is an abstract base class with a pure virtual method. Rust traits
do the same job, with a few differences that matter.

**You can impl a trait for a type you did not write.** In C++ the class has to
name its base when it is written. In Rust the trait and the type are separate,
so a plain `Duration` can be a strategy. The
[orphan rule](https://doc.rust-lang.org/book/ch10-02-traits.html#implementing-a-trait-on-a-type)
still applies: either the trait or the type must be yours.

**The vtable pointer lives on the reference, not the value.** A C++ object with
a virtual method [carries a vptr forever](https://en.cppreference.com/w/cpp/language/virtual).
`FixedDelay` is exactly as big as the `Duration` inside it. `Box<dyn Trait>`
is two words: data pointer plus vtable.

**You choose dynamic or static at the point of use.** C++ decides when the
class is declared (`virtual` or not). The same Rust trait is dynamic in
`Box<dyn DelayStrategy>` and static in `<S: DelayStrategy>`.

**No inheritance means no diamond problem, no
[object slicing](https://isocpp.org/wiki/faq/proper-inheritance#slicing), and
no forgotten [virtual destructor](https://en.cppreference.com/w/cpp/language/destructor).**
`dyn Trait` has no size and must sit behind a pointer; dropping a
`Box<dyn DelayStrategy>` always finds the right destructor.

**Generic code is checked when you write it.** `<S: DelayStrategy>` is a
promise: the body may only use what the bound allows. C++ templates were
checked at instantiation (C++20
[concepts](https://en.cppreference.com/w/cpp/language/constraints) narrowed
this, but they are opt-in).

What you give up: no inherited fields, not every trait is `dyn`-compatible
([dyn compatibility](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility)),
and no `dynamic_cast` — needing [`Any`](https://doc.rust-lang.org/std/any/trait.Any.html)
usually means the trait is missing a method.

## When not to reach for it

The pattern earns its keep when there are several strategies, they are picked
at runtime, or people will keep adding more. Otherwise:

- **One line of logic, chosen at the call site?** Take a closure —
  `F: Fn(u32) -> Duration`.
- **A short, fixed list that will not grow?** An `enum` with a `match` is
  simpler and lets the compiler tell you when you forget a case.

## The code

| File | Role |
| --- | --- |
| [`strategies/mod.rs`](strategies/mod.rs) | the trait |
| [`strategies/fixed_delay.rs`](strategies/fixed_delay.rs) | same wait every time |
| [`strategies/expo.rs`](strategies/expo.rs) | doubles the wait after each failure |
| [`strategies/expo_with_jitter.rs`](strategies/expo_with_jitter.rs) | doubles, caps, then randomises |
| [`retrier.rs`](retrier.rs) | the loop that uses a strategy |
| [`main.rs`](main.rs) | runs a failing command with each one |

```
cargo run
```

To add a strategy: new file in `strategies/`, `impl DelayStrategy for YourType`,
then `pub mod your_type;` in [`strategies/mod.rs`](strategies/mod.rs). Nothing
in `Retrier` changes. That is the point.

## Further reading

- [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Trait objects](https://doc.rust-lang.org/book/ch18-02-trait-objects.html)
- [Strategy pattern](https://rust-unofficial.github.io/patterns/patterns/behavioural/strategy.html)
- [Object safety and trait objects](https://huonw.github.io/blog/2015/01/object-safety/)
