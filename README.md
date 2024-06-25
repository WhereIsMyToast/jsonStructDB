# JsonStructDB

JsonStructDB is a lightweight library designed to simplify saving and retrieving data as JSON in files within the app data directory. This is especially useful for applications that require straightforward data persistence without the overhead of a full-fledged database.

## Features

- Easy Integration: Simple API to save and load JSON data to and from files.
- Automatic Directory Management: Automatically creates necessary directories in the app data path.
- Customizable: Easily extendable to support different data formats and structures.
- Error Handling: Provides detailed error messages to help diagnose issues.

## Usage

### Adding the dependency, for now the only way is importing it through git

In cargo.toml add

```
[dependencies]
json_struct_db = { git = "https://github.com/WhereIsMyToast/jsonStructDB.git"}
```

### Saving data

To save data, implement the `JsonConverter` trait for your data structure and call the save function.

```
    use json_struct_db::{save, JsonConverter, Result};

    struct MyData {
        content: String,
    }

    impl JsonConverter for MyData {
        fn to_json(&self) -> String {
            serde_json::to_string(self).expect("Failed to serialize MyData")
        }

        fn from_json(json: String) -> Self {
            serde_json::from_str(&json).expect("Failed to deserialize MyData")
        }
    }

    fn main() {
        let data = MyData { content: String::from("Hello, world!") };
        let identifier = "output";

        match save(data, identifier) {
            Ok(path) => println!("Data saved successfully at {}", path),
            Err(e) => eprintln!("Error: {}", e.message),
        }
    }

```

### Reading data

Using the same as before, implement the `JsonConverter` trait to your data structure and call the read method

```
    use json_struct_db::{read, Result};

    fn main() {
        let identifier = "output";

        match read(identifier) {
            Ok(contents) => println!("Read data: {}", contents),
            Err(e) => eprintln!("Error: {}", e.message),
        }
    }

```

Now the JsonConverter also implements a `from_json` function, so you can use the contents and call the `from_json` function to complente your data

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

We welcome contributions and ideas of how to upgrade the proyect!
