## Vectors Data Structure

Vector is a data structure that allows you to store multiple pieces of data,
the data must be the same type

They are used for the list of information.
Vectors allows data to be added and removed and you can also easily traverse the entries

The `vec!` macro can be used to make vectors
you can use `for ... in` to iterate though items of a vector

### Example

```rust
let my_number = vec![1, 2, 3];
let mut my_number = Vec::new();
    my_number.push(1);
    my_number.push(2);
    my_number.push(3);
    my_number.pop();
    my_number.len(); //this is 2

let two = my_number[1];

```

We can iterate over the vector

### Example

```rust
const my_numbers = vec![1, 2, 3];

for num in my_numbers {
    println!("{:?}", num);
}
```
