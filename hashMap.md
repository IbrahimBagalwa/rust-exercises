## Hash map

Hash map are useful for storing retrieving information if you really know what you are looking for.

- HashMap is a collection that stores data as key value pairs
  - data is located using the key
  - the data it's self is the value
- It is similar to a definition in the dictionary
- HashMap are very fast to retrieve data using the key

### Example find data

```Rust
    let mut people = HashMap::new();
    people.insert("Susan", 22)
    people.insert("Will", 13)
    people.insert("Cathy", 22)
    people.insert("Susan", 22)
    people.insert("Will", 13)
    people.insert("Cathy", 22)

    people.remove("Susan")

    match people.get("Will"){
        Some(age) => println!("Age: {:?}", age),
        None => println!("Notfound"),
    }
```

### Example: Iterate

```Rust
    for( person, age) in people.iter(){
        println!("person: {:?} age: {:?}", person, age)
    }

    for person in people.keys(){
        println!("person: {:?}", person)
    }

    for age in people.values(){
        println!("age: {:?}", age)
    }
```

### Notice:

The data in a hashmap are store in a random order
