## Enumeration

- Enum is a piece of data that can be one of multiple possiblities.
  - Each possibility is called a "Variant"
- Enumeration provides information about your program to the compiler.
- Enum variants can optionally contain data and than data can be another enum
- More than one piece of data can be assiciated with a variant.

### Example

```rust
enum Direction {
    Up,
    Down,
    Right,
    Left
}

fn which_way(go: Direction){
    match go{
        Direction::Up=>"up",
        Direction::Down=>"down",
        Direction::Right=>"right",
        Direction::Left=>"left",
    }
}

```
