# Stapifaction

Stapifaction is a Rust library allowing to easily generate [static APIs],
by just decorating your existing structs with `#[derive]` attributes.
It works as a [Serde] superset to define how your structs are persisted
on disk.

[static APIs]: https://www.seancdavis.com/posts/lets-talk-about-static-apis/
[Serde]: https://serde.rs/

## Features

## Usage example

```rust
use serde::Serialize;
use stapifaction::{Persist, ToJson};

#[derive(Serialize, Persist)]
#[persist(path = "products")]
struct Product {
    #[persist(id)]
    pub id: u64,
    pub label: String,
    pub price: u64,
    #[persist(expand = "all")]
    #[serde(skip)]
    pub orders: Vec<Order>,
}

#[derive(Serialize, Persist)]
struct Order {
    #[persist(id)]
    pub id: String,
    pub quantity: u64,
}

fn main() {
    let product = Product {
        id: 1,
        label: String::from("Phone"),
        price: 600,
        orders: vec![
            Order {
                id: String::from("ZGFS"),
                quantity: 5,
            },
            Order {
                id: String::from("OJGD"),
                quantity: 10,
            },
        ],
    };

    product.to_json("./");
}
```

The code above will produce the following files:

- `./products/1/data.json`
- `./products/1/orders/ZGFS.json`
- `./products/1/orders/OJGD.json`

As you can see `#[derive]` attributes are used to control how the files are
created:

- #[persist(id)] is used to specify the file name,
- #[persist(expand)] allows to extract the corresponding field to another
  file, or even multiple files when `#[persist(expand = "all")]` is used on a `Vec`.