## STRING

In rust there is two types of strings

- String - owned and
- &str - borrowed string slice

- You must use an owned string to store in struct
- You must use &str when passing to a function

### Example - Passing Data to function

```Rust
   fn print_it(data: &str){
       println!("{:?}", data);
   }

   fn main(){
        print_it("a string slice");
        let owned_string = "Owned string slice".to_owned();
        let another_owned_string = String::from("Another string slice");
        println!(&owned_string);
        println!(&another_owned_string);
   }
```

- Strings are automatically borrowed
- Use .to_owned() or String::from() to create an owned string slice
- Use owned string when storing in a struct
