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
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyData {
    content1: String,
    content2: i16,
}

//Make this the way you want
//I use serde in this example beacuse is the easyest.
impl json_struct_db::JsonConverter for MyData {
    fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("Error converting to json")
    }
    fn from_json(json: String) -> Self {
        serde_json::from_str(&json).expect("Error converting to MyData")
    }
}

fn main() {
    let data = MyData {
        content1: String::from("First content to save"),
        content2: 1,
    };
    //This is the name of the file to save or read, you will have to use this identifier to get the
    //data back
    let identifier = "MyData";

    match json_struct_db::save(data, identifier) {
        Ok(path) => println!("The file was saved it the path: {}", path),
        Err(e) => eprintln!("Error: {}", e),
    }
}


```

This code exaple makes and output in a file with the name of the identifier, inside a directory with the name of the program.
{"content1":"First content to save","content2":1}

### Reading data

Using the same as before, implement the `JsonConverter` trait to your data structure and call the read method

```
use json_struct_db::JsonConverter;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyData {
    content1: String,
    content2: i16,
}

impl MyData {
    //Default data to inicialize the struct
    fn new() -> Self {
        MyData {
            content1: String::new(),
            content2: 0,
        }
    }
}

//Make this the way you want
//I use serde in this example beacuse is the easyest.
impl json_struct_db::JsonConverter for MyData {
    fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("Error converting to json")
    }
    fn from_json(json: String) -> Self {
        serde_json::from_str(&json).expect("Error converting to MyData")
    }
}

fn main() {
    let identifier = "MyData";
    let mut my_data: MyData = MyData::new();

    match json_struct_db::read(identifier) {
        Ok(contents) => my_data = MyData::from_json(contents),
        Err(e) => eprintln!("Error: {}", e),
    }

    println!(
        "Content1: {}, Content2: {}",
        my_data.content1, my_data.content2
    );
}

```

Now the JsonConverter also implements a `from_json` function, so you can use the contents and call the `from_json` function to complente your data

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

We welcome contributions and ideas of how to upgrade the proyect!
