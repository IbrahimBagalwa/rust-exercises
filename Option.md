## Option Type

Option type can be one or two things:

1. One data of a specified type
2. Nothing
   Is used in scenarios where data may not be required or is unavailable:

- Unable to find something
- Ran out of items in a list
- Form field not filled out

### Definiton

```Rust
    enum Option<T>{
        Some(T),
        None,
    }
```

### Examples

```Rust
   struct Customer {
       age: Option<i32>,
       email: String,
   }

   let mark = Customer {
       age: Some(24),
       email: String::from("foo@example.com"),
   };

   let becky = Customer {
       age: None,
       email: String::from("foo1@example.com"),
   };

   match becky.age {
       Some(age) => {
           println!("customer is: {:?} years old", age);
       },
       None =>{
           println!("Customer age not provided");
       }
   }
```

```Rust
    struct GroceryItem {
        name: String,
        quantity: i32,
    }

    fn find_quantity(name: &str) -> Option<i32> {
        let groceries: Vec<GroceryItem> = vec![
            GroceryItem {name: String::from("Bananas"), quantity: 23},
            GroceryItem {name: String::from("eggs"), quantity: 4},
            GroceryItem {name: String::from("bread"), quantity: 3}
        ];
        for grocery in groceries {
            if grocery.name == name {
                return Some(grocery.quantity);
            }
        }
        None
    }
```
