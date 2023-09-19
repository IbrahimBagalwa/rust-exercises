## Tuples

- tuples is a type of record
- tuples are essentially a way to access each piece of data within that line or within that record.
- tuples store data anonymously and there is no need to name the fields in your tuples unlike the structs or enum where each field or variant has to have its own unique name.
- they are usefull to return pairs of data from functions and
- they can also be destructured easily into a variables.

### Example

```rust
enum Access{
    Full,
}
fn one_two_three()->(i32, i32,i32){
    (1,2,3)
}
let numbers = one_two_three();
let (a,b,c)=one_two_three();
println!("{:?}, {:?} ",a, numbers.0);
println!("{:?}, {:?} ",b, numbers.1);
println!("{:?}, {:?} ",c, numbers.2);

let (employee,access) = ("Ibrahim", Acess::Full);
```
