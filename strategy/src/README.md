# The strategy pattern

One algorithm, one decision inside it that could be made several ways. The
strategy pattern pulls that decision out, puts it behind a trait, and lets you
swap it without touching the algorithm.

This crate uses retrying as the example: the algorithm is "try again until it
works", and the decision is how long to wait between tries.

## The problem it solves

Retrying has a part that never changes and a part that does.

The part that never changes is the loop: call the operation, return if it
worked, count the failure, give up after `max_attempts`, otherwise sleep and try
again.

The part that changes is one line of it — the sleep time. Sometimes you want a
fixed wait. Sometimes you want to double it each time. Sometimes you want to add
randomness on top.

If you write that choice directly into the loop, it becomes branches:

```rust
// what we are avoiding
let wait = if self.use_exponential {
    self.base * 2u32.pow(attempt)
} else if self.use_jitter {
    ...
} else {
    self.base
};
```

Three problems with this. Every new way of waiting means editing a loop that
already works and is already tested. The flags can contradict each other — what
does `use_exponential` *and* `use_jitter` mean? And you cannot test the waiting
maths on its own, because it only exists inside the loop.

## The pattern

Name the decision, and give it a trait:

```rust
pub trait DelayStrategy {
    fn delay(&self, attempt: u32) -> std::time::Duration;
}
```

There are three roles.

- **The trait** is the decision, written down: [`DelayStrategy`](strategies/mod.rs).
- **The strategies** are the different answers: [`FixedDelay`](strategies/fixed_delay.rs),
  [`ExponentialDelay`](strategies/expo.rs),
  [`ExponentialDelayWithJitter`](strategies/expo_with_jitter.rs). Each is a small
  type that implements the trait and nothing else.
- **The user of the strategy** is the algorithm that holds one and asks it:
  [`Retrier`](retrier.rs). It calls `self.strategy.delay(attempt)` and never
  finds out which one it got.

The branches are gone. `Retrier` shrinks to the part that never changes, and
each way of waiting sits in its own file, on its own.

## What you get

**Adding a way to wait does not mean changing the loop.** New file, implement
the trait, done. Code that already works stays untouched, so it cannot break.

**The choice can be made late.** `Retrier` holds a `Box<dyn DelayStrategy>`, so
which strategy to use can come from a config file or a command-line flag while
the program runs — not from a rebuild.

**Each strategy is easy to test.** They are plain functions from a number to a
duration. No loop, no real clock, no waiting around in a test suite. That is
often the biggest win, and it is easy to miss.

## Two ways to do it in Rust

Most languages give you one way. Rust gives you two, and they trade off against
each other.

- `Box<dyn DelayStrategy>` — **dynamic**. The exact type is not known until the
  program runs, so the choice can be made at runtime, and you can keep a mix of
  different strategies in one `Vec`. It costs one indirect call each time you
  ask. [`Retrier`](retrier.rs) stores its strategy this way, and an indirect
  call is nothing next to a sleep.
- `impl DelayStrategy` or `<S: DelayStrategy>` — **static**. The compiler knows
  the exact type and can inline the call away, so it is faster, but the choice
  is fixed when you build.

You can have both. [`Retrier::new`](retrier.rs) takes `S: DelayStrategy` and
boxes it inside, so callers write the plain thing and still get a runtime
choice.

One rule comes with the dynamic form: a trait only works in `Box<dyn ...>` if it
is *object safe*, which mostly means its methods take `self` by reference and
are not generic. `DelayStrategy` takes `&self`, so it qualifies. The cost is
that a strategy cannot change anything it owns — no counter of its own, no
random number generator of its own. See
[`expo_with_jitter.rs`](strategies/expo_with_jitter.rs), which has to build a
new generator on every call because of this.

## Traits compared to C++

If you have written this pattern in C++, you made `DelayStrategy` an abstract
base class with a pure virtual method, and every strategy inherited from it.
Rust traits do the same job, and are better at it in a few specific ways.

**You can implement a trait for a type you did not write.** In C++ a class has
to name its base class when it is written, so making an existing type fit an
interface means wrapping it in an adapter. In Rust the trait and the type are
separate, so you can just say:

```rust
impl DelayStrategy for std::time::Duration {
    fn delay(&self, _attempt: u32) -> std::time::Duration { *self }
}
```

Now a plain `Duration` is a strategy, and `Duration` is not even ours. The one
rule is the [orphan rule](https://doc.rust-lang.org/book/ch10-02-traits.html#implementing-a-trait-on-a-type):
either the trait or the type must be yours, so two crates cannot both add the
same impl and disagree.

**The object does not carry anything extra.** A C++ class with a virtual method
[stores a hidden pointer to its vtable in every object](https://en.cppreference.com/w/cpp/language/virtual),
and it does so forever, even where you use the type directly. In Rust the vtable
pointer lives in the `&dyn` or `Box<dyn>` reference, not in the value, so
`FixedDelay` is exactly as big as the `Duration` inside it. Implementing ten
traits costs the struct nothing. The reference is the thing that gets bigger —
it is two words instead of one, which is why `dyn Trait` has no size of its own
and always sits behind a pointer
([the Nomicon on unsized types](https://doc.rust-lang.org/nomicon/exotic-sizes.html)).

**You choose dynamic or static at the point of use.** C++ decides when the class
is declared: a method is `virtual` or it is not, and every caller lives with
that. In Rust the same trait, unchanged, is dynamic in `Box<dyn DelayStrategy>`
and static in `<S: DelayStrategy>`. `Retrier` uses both, and neither strategy
type had to know.

**No inheritance means a set of old problems simply do not exist.** There is no
diamond problem, because there is no base class to inherit twice. There is no
[object slicing](https://isocpp.org/wiki/faq/proper-inheritance#slicing), where
copying a derived object into a base-typed variable silently throws away the
derived part — Rust cannot even express it, because `dyn Trait` has no size and
must stay behind a pointer. And you cannot forget a
[virtual destructor](https://en.cppreference.com/w/cpp/language/destructor):
dropping a `Box<dyn DelayStrategy>` finds the right destructor through the
vtable, always.

**Generic code is checked when you write it, not when someone uses it.** A C++
template only really gets type-checked once it is instantiated, which is why a
small mistake used to produce pages of errors from inside the library. In Rust,
`<S: DelayStrategy>` is a promise checked up front: the body may only use what
the bound allows, and a caller passing the wrong type gets an error pointing at
the call. C++20
[concepts](https://en.cppreference.com/w/cpp/language/constraints) narrowed this
gap a lot, but they are opt-in, and bounds in Rust are the only way.

What you give up is real, and worth knowing:

- **No inherited implementation or shared state.** A trait can supply a default
  method body, but no fields. Sharing data between strategies means putting a
  struct inside them, not inheriting one.
- **Not every trait works as `dyn`.** Generic methods and methods taking `self`
  by value rule it out; the rules are
  [dyn compatibility](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility),
  once called object safety.
- **No asking "what is this really?"** C++ has `dynamic_cast`. Rust makes you go
  through [`Any`](https://doc.rust-lang.org/std/any/trait.Any.html), on purpose,
  and needing it is usually a sign the trait is missing a method.

## When not to reach for it

The pattern earns its keep when there are several strategies, they are picked at
runtime, or people will keep adding more. Otherwise:

- **One line of logic, chosen at the call site?** Take a closure —
  `F: Fn(u32) -> Duration`. A whole trait and three types is a lot of ceremony
  for that.
- **A short, fixed list that will not grow?** An `enum` with a `match` is
  simpler, keeps everything in one file, and lets the compiler tell you when you
  forget a case.

Traits are the right answer when other people, or other crates, need to add
strategies you have not thought of.

## The code

| File | Role |
| --- | --- |
| [`strategies/mod.rs`](strategies/mod.rs) | the trait |
| [`strategies/fixed_delay.rs`](strategies/fixed_delay.rs) | same wait every time |
| [`strategies/expo.rs`](strategies/expo.rs) | doubles the wait after each failure |
| [`strategies/expo_with_jitter.rs`](strategies/expo_with_jitter.rs) | doubles, caps, then randomises, so many clients do not all retry at once |
| [`retrier.rs`](retrier.rs) | the loop that uses a strategy |
| [`main.rs`](main.rs) | runs a failing command with each one |

```
cargo run
```

To add a strategy: new file in `strategies/`, `impl DelayStrategy for YourType`,
then `pub mod your_type;` in [`strategies/mod.rs`](strategies/mod.rs). Nothing
in `Retrier` changes. That is the point.

## Further reading

- [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html) — the book's introduction.
- [Trait objects](https://doc.rust-lang.org/book/ch18-02-trait-objects.html) — `Box<dyn Trait>` and why you would want it.
- [Characteristics of object-oriented languages](https://doc.rust-lang.org/book/ch18-01-what-is-oo.html) — where Rust agrees with OOP and where it does not.
- [Strategy pattern](https://rust-unofficial.github.io/patterns/patterns/behavioural/strategy.html) — this pattern in the Rust Design Patterns book, with a different example.
- [Object safety and trait objects](https://huonw.github.io/blog/2015/01/object-safety/) — old but still the clearest explanation of the rules and the reason for them.
- [Trait objects in the Reference](https://doc.rust-lang.org/reference/types/trait-object.html) — the exact wording, when you need it.
