## Result

Result is a data type that contains one or two types of data:

- "Successful" data
- "Error" data

It used in senarios where an action needs to be taken, but has the possibility of failure.

Some examples are:

- copyin a file
- connecting to a website

Here is the definition of the result

```Rust
    enum Result<T,E> {
        Ok(T),
        Err(E)
    }
```

### Example

```Rust
    fn get_sound(name: &str) -> Result<SoundData, String> {
        if name == "alert" {
            Ok(SoundData::new("alert")),
        }else {
            Err(String::from("Unable to find sound data"))
        }
    }

    let sound = get_sound("alert");
    match sound {
        Ok(_) => println!("Sound data located"),
        Err(e) => println!("error: {:?}", e)
    }

```
