## Struct

- struct allows you to create a data type which contains multiple pieces of data
  - struct can be all or nothing: cannot have somee pieces of data and not others
- Each piece of data is called a `field`
- Makes working with data easier

### Example

```Rust
    struct ShippingBox {
        depth: i32,
        width: i32,
        height: i32,
    }

    let my_box = ShippingBox {
        depth: 3,
        width: 2,
        height: 5,
    };
    let tall = my_box.height;
    println!("the box is {:?} units tall", tall);
```

## Recap

- Structs deal with multiple pieces of data.
- All fields must be present to create a struct
- Fields can be accessed using a dot notation (.)
