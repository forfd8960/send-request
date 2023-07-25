## Life time subtyping

![Lifetime SubTyping](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch19-02-advanced-lifetimes.html)

Context has a lifetime parameter `'s`, and `UseContext` contains a reference to `Context`.

And if the `UseContext` want to extract some of data from `Context str` or return some data from `Context str`, then the data's lifetime parameter need to be same
with `Context str`'s lifetime parameter.

eg, `data_from_context: &'s str`

`fn parse(&self) -> Result<(), 's str> {`

```rust

struct Context<'s><&'s str>

struct UseContext<'c, 's> {
    data_from_context: &'s str,
    context: &'c Context<'s>,
}

impl UseContext<'c, 's> {
    fn new(context: &'c Context<'s>) -> Self {
        Self {
            data_from_context: "",
            context: context,
        }
    }
    fn parse(&self) -> Result<(), 's str> {
        ...
    }
}
```

```
the Rust feature lifetime subtyping is a way to specify that one lifetime parameter lives at least as long as another one. In the angle brackets where we declare lifetime parameters, we can declare a lifetime 'a as usual, and declare a lifetime 'b that lives at least as long as 'a by declaring 'b with the syntax 'b: 'a
```
