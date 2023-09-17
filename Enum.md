## Enumeration

- Enum is a piece of data that can be one of multiple possiblities.
  - Each possiblity is called a "Variant"
- Enumeration provides information about your program to the compiler.

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
