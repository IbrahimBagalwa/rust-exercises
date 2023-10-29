## Expression in Rust

- Rust is an expression-based language
  - Most things are evaluated and return some a value
- Expression values coalesce to a single point
  - As result expression can be used for nesting logic
- Example and understand what i mean about coalesce

```Rust
    let my_number = 3;
    //here we have some things called let binding
    let is_my_number_5 = if my_number > 5 {
       true
    } else {
       false
    };
    // a shorthand for this since it will be evaluated to true or false we can do this:

    let is_my_number_5 = my_number > 5;
```

We can also do this with `match` expressions:

```Rust
    let my_number = 3;
    let message = match my_number{
        1 => "Hello",
        _ => "Goodbye"
    };
```

It also posible to nest expressions:

```Rust
    enum Menu {
        Burger,
        Fruits,
        Drink
    }

    let paid = true;
    let item = Menu:Drink;
    let drink_type = "water";
    let order_place = match item {
        Menu::Drink => {
            if drink_type == "water" {
                true
            }else{
                false
            }
        }
        _=>true,
    };
```

### Recap

- Expressions allow nested logic
- `if` and `match` expressions can be nested but is not best to use more than two or three levels
