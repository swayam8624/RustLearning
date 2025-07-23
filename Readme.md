# Rust

### Introduction

- Rust is a systems programming language that runs fast, prevents segfaults, and guarantees thread safety
- High level language with a focus on safety and performance. (no performance penalty for safety
  )
- Program behaviour can be enforced at compile time --> enhanced program reliability.
- Built-in dependency management and package management , similar to npm
- Quickly growing ecosystem of libraries and tools.
- Friendly compiler with helpful error messages and welcoming community.

### Terminologies

- **Crate**: A package of Rust code, which can be a library or an executable. Crates are the fundamental unit of code organization in Rust.
- **Cargo**: The Rust package manager and build system, which helps manage dependencies, build projects, and run tests. It simplifies the process of working with Rust code and its dependencies.
- **Module**: A way to organize code into separate namespaces, allowing for better code organization and encapsulation. Modules can contain functions, structs, enums, and other items, and they can be nested within each other.
- **Ownership**: A key feature of Rust that ensures memory safety by enforcing strict rules about how data is accessed and modified. Each piece of data has a single owner, and when the owner goes out of scope, the data is automatically cleaned up. This prevents issues like dangling pointers and memory leaks.
- **Borrowing**: A mechanism that allows references to data without taking ownership. Borrowing enables multiple parts of a program to access the same data safely, while still adhering to Rust's ownership rules. There are two types of borrowing: mutable and immutable.

```RUST
//Examples for crate :
// A simple crate with a library and an executable
// src/lib.rs
pub fn greet(name: &str) {
    println!("Hello, {}!", name);
}
```

### Features

- First-class support for concurrency and parallelism. --> multithreading and async programming , compiler error to improperly access shared data.
  Type System : - Can uncover bugs at compile time - Makes Refactoring easier - Reduces the number of tests needed
- Robust Module system makes it easy to organize code and manage dependencies.
- Adding a dependency is a one line in a config file
- Tooling :
  - Cargo : package manager and build system / Generate docs
  - Clippy : linter for Rust code
  - Rustfmt : code formatter
  - Rust Analyzer : IDE support for Rust

`Lets Enjot RUST!`

## Fundamentals

### Data Types

- Memory only stores in binary data --> Anything can be represented as a sequence of bits.
- Program determines what the bits mean.
- Basic types that are universally useful are provided by the language. ( we are not limited to these types, we can define our own types)
- Rust is a statically typed language, meaning that the type of a variable is known at compile time.
- Rust has a strong type system, which means that types are enforced at compile time, reducing runtime errors.

- Basic types in Rust:
  - Scalar Types : represent a single value
    - Integer Types : i8, i16, i32, i64, i128, isize (signed) and u8, u16, u32, u64, u128, usize (unsigned)
    - Double or Floating Point Types : f32 and f64
    - Boolean Type : bool (true or false)
    - Character Type : char (represents a single Unicode character) 'c'
    - Strings : a sequence of characters, represented as a collection of bytes "hello"
  - Compound Types : can group multiple values into one type
    - Tuple : fixed-size collection of values of different types
    - Array : fixed-size collection of values of the same type
    - Slice : dynamically sized view into a contiguous sequence of elements (not a fixed-size collection like an array)
    - String Slice : a view into a string, represented as a reference to a string (e.g., &str)
    - Struct : a custom data type that allows you to create complex types by combining multiple values
    - Enum : a type that can be one of several different variants, each with its own data
    - Option : a type that can be either Some(value) or None, used for representing optional values
    - Result : a type that can be either Ok(value) or Err(error), used for error handling
- Rust also has a powerful type inference system, which means that the compiler can often determine the type of a variable based on how it is used, allowing for more concise code without explicit type annotations.
- Rust's type system also includes features like lifetimes, which help manage memory safety by ensuring that references are valid for the duration of their use, preventing dangling pointers .
- Rust's ownership model, which is a key feature of the language, ensures memory safety without needing a garbage collector. It enforces rules about how data is accessed and modified to prevent the common pitfalls of memory management, such as data threading and memory leaks.
- Rust's type system is designed to be expressive and flexible, allowing developers to create complex data structures while maintaining safety and performance.
- Rust's type system also supports generics, which allow you to write functions and data structures that can operate on different types without sacrificing type safety. This enables code reuse and abstraction while still ensuring that types are checked at compile time.
- Rust's type system is designed to be ergonomic and user-friendly, with a focus on providing clear error messages and helpful suggestions when type-related issues arise. This makes it easier for developers to understand and fix type errors during development.

### Variables and Mutability

- Assign data to a temporary location in memory, which is called a variable --> which allows the programmer to easily work with memory .
- Can be set to any value & type
- Immutable by default, but can be mutable
  - Immutable : cannot be changed after it is set
  - Mutable : can be changed after it is set
- Variables are bound to a value, not a type.
- Variables can be shadowed, meaning that a new variable with the same name can be created
  - This allows for reusing variable names in different scopes or contexts, but it can also lead to confusion if not used carefully.
- Variables can be declared using the `let` keyword, and their type can be explicitly specified or inferred by the compiler.

```RUST
let x: i32 = 5; // Explicit type declaration
let y = 10; // Type inferred by the compiler
let mut z = 15; // Mutable variable // mut is the keyword to make a variable mutable
z = 20; // Changing the value of a mutable variable
// semicolons are used to terminate statements in Rust, but they are not required for expressions that return a value.
let a = 5 + 10; // This is an expression that evaluates to 15
let string = "Hello, Rust!"; // String literal, which is a sequence of characters
let copied_value = string; // Copying the value of x into a new variable
let quit = false;
let (a, b) = (1, 2); // Destructuring a tuple into two variables

```

- Rust's ownership model ensures that variables are properly managed in terms of memory safety, preventing issues like double frees or use-after-free errors.
- Variables can be used to store values of different types, and Rust's type system ensures that type mismatches are caught at compile time, reducing runtime errors.
- Rust's variable system also supports destructuring, which allows you to unpack values from tuples, arrays, and structs into individual variables for easier access and manipulation.

### Functions

- A way to encapsulate a block of code that performs a specific task, allowing for code reuse and modularity.
- Optionally accept data as input and return a value as output.
- Often used to break down complex problems into smaller, manageable pieces --> Utilized for code organization and readability.
- Functions can be defined using the `fn` keyword, followed by the function name, parameters, and return type.
- Parameters can have explicit types, and the return type is specified after an arrow (`->`).
- Functions can be called by their name, passing the required arguments in parentheses.
- Rust supports both named and anonymous functions (closures), allowing for flexible function definitions.

```RUST
fn add(x: i32, y: i32) -> i32 { // Function definition
    x + y // Implicit return, no semicolon needed
}
fn multiply(x: i32, y: i32) -> i32 {
    x * y // Implicit return
}
fn main() {
    let resultmul = multiply(5, 10); // Function call
    println!("The product is: {}", result); // Output the result
    let resultadd = add(resultmul, 10); // Function call
    println!("The sum is: {}", result); // Output the result
}
```

- There can be multiple functions in a Rust program, and they can call each other as needed , the main function is the entry point of the program and there is only one main function in a Rust program.
- Functions can also be defined within other functions, allowing for nested functions and closures.

```RUST
//Example for a anonymous function (closure)
fn main() {
    let add = |x: i32, y: i32| x + y;
    let result = add(5, 10); // Calling the closure
    println!("The sum is: {}", result); // Output the result
}
// Example for a nested function
fn main() {
    fn inner_function(x: i32, y: i32) -> i32 {
        x + y
    }
    let result = inner_function(5, 10); // Calling the inner function
    println!("The sum is: {}", result); // Output the result
}
```

- Functions can also have default parameters, allowing for optional arguments that can be omitted when calling the function.
- Rust's ownership model applies to function parameters and return values, ensuring that data is properly managed and preventing issues like dangling pointers or memory leaks.
- Functions can also be generic, allowing them to operate on different types without sacrificing type safety. This enables code reuse and abstraction while still ensuring that types are checked at compile time.
- Rust's type system ensures that function signatures are clear and explicit, making it easier for developers to understand how to use functions and what types of data they expect.

#### Println Macro

- A macro in Rust that allows you to print formatted output to the console.
- It is commonly used for debugging and displaying information during program execution.
- The `println!` macro takes a format string and optional arguments, similar to `printf` in C or `System.out.printf` in Java.
- The format string can include placeholders for variables, which are replaced with their values when the macro is executed.
- The `println!` macro automatically adds a newline character at the end of the output, so you don't need to include it explicitly.
- It is a convenient way to output information without needing to import additional libraries or modules.
- we use a `{:?}` to print the debug representation of a value, which is useful for debugging complex data structures.
- It is also possible to use `println!("{varname:?}";)` to print the value of a variable name in debug format.
- The `println!` macro is a powerful tool for quickly displaying information during development and debugging.

### Control Flow

- Control flow refers to the order in which statements are executed in a program.
- Code is executed sequentially, but control flow statements allow you to change the order of execution based on certain conditions or loops.
- Rust provides several control flow constructs, including:
  - Conditional statements: `if`, `else if`, and `else` for branching logic
  - Loops: `loop`, `while`, and `for` for repeating code blocks
  - Match statements: a powerful pattern matching construct for handling different cases based on the value of a variable
- Control flow statements can be nested, allowing for complex logic and decision-making in your code.
- Rust's control flow constructs are designed to be expressive and concise, making it easy to write clear and readable code.
- Rust's ownership model applies to control flow as well, ensuring that data is properly managed and preventing issues like dangling pointers or memory leaks.
- Control flow statements can also return values, allowing you to use them in expressions or assign them to variables.
- Rust's type system ensures that control flow statements are type-safe, meaning that the types of variables and expressions used in control flow constructs are checked at compile time, reducing runtime errors.

```Rust

fn main() {
    let x = 5;
    if x < 10 {
        println!("x is less than 10");
    } else if x == 10 {
        println!("x is equal to 10");
    } else {
        println!("x is greater than 10");
    }

    // Loop example
    let mut count = 0;
    loop {
        count += 1;
        if count == 5 {
            break; // Exit the loop when count reaches 5
        }
        println!("Count: {}", count);
    }

    // For loop example
    for i in 0..5 { // Range from 0 to 4 // .. operator creates a iterable
        // The range is inclusive of the start and exclusive of the end
        println!("i: {}", i);
    }

    // While loop example
    let mut j = 0;
    while j < 5 {
        println!("j: {}", j);
        j += 1; // Increment j
    }

    // Match statement example
    let value = 3;
    match value {
        1 => println!("Value is one"),
        2 => println!("Value is two"),
        _ => println!("Value is something else"), // The underscore (_) acts as a catch-all pattern
    }
}

```

### Comments

- Comments are used to add explanatory notes or documentation within the code, making it easier to understand
- Rust supports two types of comments:
  - Single-line comments: start with `//` and continue to the end of the line
  - Multi-line comments: enclosed between `/*` and `*/`, allowing for longer explanations
- Comments are ignored by the compiler and do not affect the execution of the program.
- They are useful for documenting the purpose of code, explaining complex logic, or providing context for future developers.
- Comments can also be used to temporarily disable code during development or debugging.
- It is a good practice to write clear and concise comments that explain the intent of the code, rather than restating what the code does.
- Rust also supports doc comments, which are used to generate documentation for functions, structs, and modules. These comments start with `///` and can be used to provide detailed information about the code's functionality after it and usage .
- Doc comments can be processed by tools like `rustdoc` to generate HTML documentation, making it easier for developers to understand how to use the code and its features.
- Comments can also be used to mark TODOs or FIXMEs, indicating areas of the code that need further attention or improvement.
- It is important to keep comments up to date as the code evolves, ensuring that they accurately reflect the current state and purpose of the code.
- Comments should be used judiciously, focusing on explaining the "why" behind the code rather than the "what," as the code itself should be self-explanatory when written clearly.

```RUST
// This is a single-line comment
let x = 5; // This is an inline comment
/* This is a multi-line comment
   It can span multiple lines
   and is useful for longer explanations */
// This is a doc comment for a function
/// This function adds two numbers together
fn add(a: i32, b: i32) -> i32 {
    a + b // This is an inline comment within the function
}
```

`rustdoc src/lib.rs` is used to generate documentation for the code. Saved in `doc/lib/index.html`
we can also give names to the crate using --crate-name flag, which is useful when generating documentation for a library crate.
we can also use `cargo doc` to generate documentation for the entire project, including dependencies. `cargo doc --open` will open the generated documentation in the default web browser.
`rustdoc --crate-name docs src/lib.rs -o <path>/docs/target/doc -L dependency=<path>/docs/target/debug/deps` , cargo doc calls out to rustdoc like this.
-o controls the output directory for the generated documentation.
-L dependency specifies the path to the compiled dependencies, allowing rustdoc to link against them when generating documentation.

- `//!` is used to write documentation comments at the crate level, which will be included in the generated documentation. ( inner documentation) , it is the root of the crate as nothing comes before it.
- `rustdoc` can create doc from readme files as well .

### Installation and Setup

- Install Rust using `rustup`, which is the recommended way to install Rust and its associated tools.
- `rustup` manages Rust versions and toolchains, allowing you to easily switch between different versions of Rust.
- To install Rust, run the following command in your terminal:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install stable
rustup component add rustfmt clippy
cargo install cargo-edit cargo-watch cargo-bench rustlings


```

- This command downloads and runs the Rust installation script, which will guide you through the installation process.
- After installation, you can verify that Rust is installed correctly by running:

```bash
rustc --version
```

- This command will display the version of the Rust compiler (`rustc`) that is installed on your system.
- You can also check the version of `cargo`, the Rust package manager, by running:

```bash
cargo --version
```

- This command will display the version of `cargo` that is installed on your system.

```
Install these VS Code Extensions:
rust-analyzer (main LSP)
Better TOML
CodeLLDB (for debugging)
Crates (shows crate versions inline)
```

- To create a new Rust project, use the `cargo` command:

```bash
cargo new my_project
cd my_project
```

- This will create a new directory called `my_project` with a basic Rust project structure, including a `Cargo.toml` file for managing dependencies and a `src/main.rs` file for your main Rust code.
- You can then build and run your project using:

```bash
cargo run
// or
cargo run --bin name
```

- This command will compile your Rust code and execute the resulting binary.
- To build the project without running it, use:

```bash
cargo build
```

- can use -q flag to suppress output
- To run tests, use:

```bash
cargo test
```

- This command will run any tests defined in your Rust code and display the results.
- To format your code according to Rust's style guidelines, use:

```bash
cargo fmt
```

- This command will automatically format your Rust code, making it more readable and consistent with Rust's style conventions.
- To check your code for common mistakes and improve its quality, use:

```bash
cargo clippy
```

- This command will run the Clippy linter, which provides suggestions for improving your Rust code
- To manage dependencies, you can add them to your `Cargo.toml` file under the `[dependencies]` section. For example:

```toml
[dependencies]
serde = "1.0" # Add the serde crate for serialization/deserialization
```

- After adding dependencies, you can run `cargo build` to download and compile them.
- To update dependencies, use:

```bash
cargo update
```

- This command will update your dependencies to the latest compatible versions based on the constraints specified in your `Cargo.toml` file.
- To remove unused dependencies, you can use the `cargo fix` command:

```bash
cargo fix --allow-dirty
```

- This command will automatically remove any unused dependencies from your `Cargo.toml` file and update your code to reflect the changes.
- To run a specific binary in a multi-binary project, use:

```bash
cargo run --bin my_binary
```

- This command will execute the specified binary within your project, allowing you to run different parts of your codebase as needed.
- To build a release version of your project, use:

```bash
cargo build --release
```

- This command will compile your Rust code with optimizations enabled, resulting in a faster binary suitable for production use.
- To clean up build artifacts and temporary files, use:

```bash
cargo clean
```

- This command will remove the `target` directory, which contains compiled binaries and other build artifacts, allowing you to start fresh with a clean build.

### Enumerations

- Data that can be one of the multiple different possibilities , --> Each Possibility is called a variant.
- Provides information about your program to the compiler --> helping in creation of more robust code.

```RUST
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let direction = Direction::Up; // Creating an instance of the Direction enum
    match direction {
        Direction::Up => println!("Moving Up"),
        Direction::Down => println!("Moving Down"),
        Direction::Left => println!("Moving Left"),
        Direction::Right => println!("Moving Right"),
    }
}

// :: is used to access the enum variants
// Enums can also have associated data, allowing you to store additional information with each variant
enum Shape {
    Circle(f64), // Circle with radius
    Rectangle(f64, f64), // Rectangle with width and height
    Triangle { base: f64, height: f64 }, // Triangle with named fields
}

fn main() {
    let shape = Shape::Circle(5.0); // Creating an instance of the Shape enum with Circle variant
    match shape {
        Shape::Circle(radius) => println!("Circle with radius: {}", radius),
        Shape::Rectangle(width, height) => println!("Rectangle with width: {}, height: {}", width, height),
        Shape::Triangle { base, height } => println!("Triangle with base: {}, height: {}", base, height),
    }
}
```

- Enums can also implement methods, allowing you to define behavior associated with the enum variants.

```RUST
impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle(width, height) => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }
}
fn main() {
    let shape = Shape::Rectangle(4.0, 5.0); // Creating an instance of the Shape enum with Rectangle variant
    println!("Area of the shape: {}", shape.area()); // Calling the area method on the shape instance
}
```

- `impl` is used to implement methods for the enum, allowing you to define behavior associated with the enum variants.

### structs

- A type that contains multiple pieces of data --> All or Nothing - cannot have some pieces of data and not others.
- Each piece of data is called a field, and each field can have its own type.
- Makes working with data easier by grouping related data together.
- Structs can be defined using the `struct` keyword, followed by the struct name and its fields.
- Fields can have explicit types, and the struct can also implement methods for behavior associated with the struct.
- Structs can be used to create complex data types that represent real-world entities
- Rust provides two types of structs: tuple structs and regular structs.
- Tuple structs are similar to tuples, but they have a name and can have different types for each field.
- Regular structs have named fields, allowing for more clarity and readability when accessing the data.

```RUST
// Tuple Struct Example
struct Point(i32, i32); // A tuple struct with two fields of type i32
fn main() {
    let point = Point(3, 4); // Creating an instance of the Point tuple struct
    println!("Point coordinates: ({}, {})", point.0, point.1); // Accessing fields using dot notation
}
// Regular Struct Example
struct Rectangle {
    width: f64, // Named field for width
    height: f64, // Named field for height
}
fn main() {
    let rect = Rectangle { width: 5.0, height: 10.0 }; // Creating an instance of the Rectangle struct
    println!("Rectangle dimensions: {} x {}", rect.width, rect.height); // Accessing fields using dot notation
}
```

- struct fields can have even enum as their type

```RUST
enum Color {
    Red,
    Green,
    Blue,
}
struct Circle {
    radius: f64,
    color: Color, // Field of type Color enum
}
fn main() {
    let circle = Circle {
        radius: 5.0,
        color: Color::Red, // Assigning a variant of the Color enum to the color
    };
    match circle.color { // use dot operator to access the color field
        Color::Red => println!("Circle is Red"),
        Color::Green => println!("Circle is Green"),
        Color::Blue => println!("Circle is Blue"),
    }
}
```

### tuples

- A type of record that can hold multiple values of different types.
- Store data anonymously --> no need to name fields.
- Useful to return pairs of data from functions or to group related values together.
- Can be destructured easily into variables for easier access.
- Tuples can be defined using parentheses `()` and can contain any number of elements, each with its own type.
- Tuples can be used to group related data together, making it easier to pass multiple values around in your code.
- Tuples can be nested, allowing you to create complex data structures by combining multiple tuples together.
- Tuples can also be used as function parameters or return values, allowing you to return multiple values from a function without needing to define a custom struct or enum.
- Tuples can be used to represent fixed-size collections of values, similar to arrays, but with the ability to have different types for each element.

```RUST
// Example of a tuple
let person: (&str, i32) = ("Alice", 30); // A tuple with a string and an integer
println!("Name: {}, Age: {}", person.0, person.1); // Accessing tuple elements using dot notation
// Destructuring a tuple into variables
let (name, age) = person; // Destructuring the tuple into two variables
println!("Name: {}, Age: {}", name, age); // Using the destructured variables
// Nested tuples
let nested_tuple: ((i32, i32), (f64, f64)) = ((1, 2), (3.5, 4.5)); // A tuple containing two tuples
let ((x1, y1), (x2, y2)) = nested_tuple; // Destructuring the nested tuple
println!("Coordinates: ({}, {}), ({}, {})", x1, y1, x2, y2); // Using the destructured variables
// Tuples as function parameters and return values
fn calculate_area(dimensions: (f64, f64)) -> f64 {
    dimensions.0 * dimensions.1 // Using the tuple elements to calculate area
}
let area = calculate_area((5.0, 10.0)); // Passing a tuple as a function argument
println!("Area: {}", area); // Outputting the calculated area
```

### expressions

- Rust is an expression-oriented language, meaning that most things are evaluated and return a value.
- Expression values coalesce to a single point --> can be used for nesting logic
- An expression is a piece of code that evaluates to a value.
- Expressions can be used to perform calculations, create data structures, or call functions.
- Rust expressions can be used in various contexts, such as assignments, function calls, and control flow statements.
- Expressions can be combined to create more complex expressions, allowing for powerful and flexible code

```rust

let x = 3;
let is_lt_5 = if x < 5 { true } else { false }; // This is an expression that evaluates to a boolean value
let is_gt_5 = x>5;
```

### Intermediate Memory Representation (IMR)

#### Basic Memory Refresh

- Memory is stored in binary data, which is a sequence of bits (0s and 1s).
- Computer optimised for bytes = 8 contiguous bits.
- Fully contiguous

#### Addresses

- All data in memoory has an address -->
  - Used to locate data in memory.
  - Always the same - only data changes.
- Usually dont utilize addresses directly --> Variables handle most of the work .

#### Offsets

- Offset is the distance from the start of a data structure to a specific element within it.
- Items can be located at an address using an Offset
- Offsets begin at 0
- Represent the number the bytes away from the original address --> Narmally deal with indexes instead.

#### Pointers

- A pointer is a variable that stores the address of another variable.
- Pointers are used to reference data in memory without copying it, allowing for efficient memory management
- Pointers can be mutable or immutable, depending on whether you want to change the data they point to.
- Rust has two types of pointers: references and raw pointers.
- References are safe pointers that ensure memory safety by enforcing borrowing rule s, while raw pointers are unsafe pointers that allow for more flexibility but require careful management to avoid memory issues.
- Pointers can be dereferenced to access the data they point to, allowing you to read or modify the value stored at that address.
- Rust's ownership model ensures that pointers are properly managed, preventing issues like dangling pointers or memory leaks.
  - Pointers can also be used to create complex data structures, such as linked lists or trees, by allowing nodes to reference each other.

```rust
let x = 5; // A variable with a value of 5
let y = &x; // A reference to the variable x
println!("Value of x: {}, Value of y: {}", x, y); // Accessing the value of x and the reference y
let mut z = 10; // A mutable variable
let w = &mut z; // A mutable reference to the variable z
*w += 5; // Modifying the value of z through the mutable reference w
println!("Value of z: {}, Value of w: {}", z, w); // Accessing the modified value of z and the reference w
```

### Ownership and Borrowing

#### Managing Memory

- Programs must track memory --> If they fail to do so , a leak occurs
- Rust utilizes an ownership model to manage memory safely and efficiently --> The owner of memory is responsible for cleaning up the memory.
- Memory can either be moved or borrowed.
- Ownership is a key concept in Rust that ensures memory safety without needing a garbage collector.
- Each value in Rust has a single owner, which is responsible for managing the memory associated with that value.
- When the owner goes out of scope, the memory is automatically cleaned up, preventing memory leaks and dangling pointers.
- Ownership can be transferred (moved) from one variable to another, meaning that the original variable can no longer access the value.
- Borrowing allows you to create references to a value without taking ownership, enabling multiple parts of the code to access the same data safely
- Rust enforces strict rules about ownership and borrowing at compile time, ensuring that data is accessed safely and preventing common memory-related bugs.
- Rust's ownership model also includes the concept of lifetimes, which helps the compiler understand how long references to data are valid, preventing dangling pointers and ensuring memory safety.

```rust
let x = String::from("Hello"); // x owns the String value
let y = x; // Ownership is moved from x to y, x can no longer be used
// println!("{}", x); // This would cause a compile-time error because x is no longer valid
println!("{}", y); // y can be used to access the value
let z = String::from("World"); // z owns the String value
let w = &z; // w is a reference to z, borrowing the value without taking ownership
println!("{}", w); // w can be used to access the value of z
// z can still be used because ownership was not moved, only borrowed

// move and borrow for functions
fn takes_ownership(s: String) { // s takes ownership of the String value
    println!("Taking ownership: {}", s);
}
fn borrows_reference(s: &String) { // s borrows a reference to the String value
    println!("Borrowing reference: {}", s);
}
fn main() {
    let my_string = String::from("Hello, Rust!"); // my_string owns the String value
    takes_ownership(my_string); // Ownership is moved to the function
    // println!("{}", my_string); // This would cause a compile-time error because my_string is no longer valid
    let another_string = String::from("Hello, World!"); // another_string owns the String value
    borrows_reference(&another_string); // Borrowing a reference to the String value
    println!("{}", another_string); // another_string can still be used because ownership was not moved, only borrowed
}
```

### Impl

- The `impl` keyword is used to define methods and associated functions for a type, allowing you to add behavior to structs, enums, and other types.
- It allows you to define methods that can be called on instances of the type, as well as associated functions that can be called without an instance.
- Methods defined within an `impl` block can access the fields of the type and perform operations on them, enabling encapsulation and abstraction.

```rust
struct Circle {
    radius: f64, // Field for the radius of the circle
}
impl Circle {
    // Method to calculate the area of the circle
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius // Using the radius field to calculate area
    }

    // Associated function to create a new Circle instance
    fn new(radius: f64) -> Self {
        Self { radius } // Returning a new Circle instance with the given radius
    }
}
fn main() {
    let circle = Circle::new(5.0); // Creating a new Circle instance using the associated function
    println!("Area of the circle: {}", circle.area()); // Calling the area method on the Circle instance
}
// if Self is used in the impl block, it refers to the type being implemented, allowing for more concise code when defining methods and associated functions.
```

- Have the name of type they are implemented for and the `impl` name same as the type binding is required.

### Vectors

- A vector is a dynamically sized array that can grow or shrink in size as needed.
- Vectors are implemented as a contiguous block of memory, allowing for efficient access to elements.
- Vectors can store elements of the same type and provide methods for adding, removing, and accessing elements.
- Vectors are part of the Rust standard library and can be used by importing the `std::vec` module.

```rust
use std::vec::Vec; // Importing the Vec type from the standard library
fn main() {
    let mut numbers: Vec<i32> = Vec::new(); // Creating a new empty vector of i32 type OR vec![1, 2, 3] can also be used to create a vector with initial values
    numbers.push(1); // Adding an element to the vector
    numbers.push(2); // Adding another element to the vector
    numbers.push(3); // Adding a third element to the vector

    println!("Numbers: {:?}", numbers); // Printing the contents of the vector

    // Accessing elements in the vector using indexing
    for i in 0..numbers.len() {
        println!("Element at index {}: {}", i, numbers[i]);
    }

    // Removing an element from the vector
    let removed_element = numbers.pop(); // Removes the last element and returns it
    println!("Removed element: {:?}", removed_element);
    println!("Numbers after removal: {:?}", numbers);
}
```

`Always use references to access elements , be it a function or for loop or anything , as in move it WILL get cleared up`

### Strings

- Two commonly used types of strings
  - String : A growable, heap-allocated string type that can be modified and resized.
  - &str : A string slice that represents a view into a string, typically used for read-only access to string data.
- Strings are used to represent text data in Rust, and they can be manipulated using various methods provided by the `String` and `&str` types.
- Strings can be created from string literals, converted from other types, or constructed using various methods.
- Strings can be concatenated, sliced, and modified using methods like `push`, `push_str`, and `replace`.
- String slices (`&str`) are often used for function parameters and return values
- String slices are more efficient for read-only access to string data, as they do not require ownership or allocation.
- Rust provides various methods for working with strings, such as `len`, `is_empty`, `to_uppercase`, and `to_lowercase`, allowing for easy manipulation and formatting of string data.
- Must use an owned String to store in a `struct`

```rust
fn main() {
    let mut my_string = String::from("Hello"); // Creating a new String
    my_string.push_str(", Rust!"); // Appending a string slice to the String
    println!("My string: {}", my_string); // Printing the contents of the String

    let my_str: &str = "This is a string slice"; // Creating a string
    // can be converted using .to_owned()
    println!("String slice: {}", my_str); // Printing the contents of the string slice

    // Concatenating two strings
    let another_string = String::from(" Welcome to Rust programming.");
    let concatenated = my_string + &another_string; // Using the + operator to concatenate strings
    println!("Concatenated string: {}", concatenated);

    // Slicing a string
    let sliced = &my_string[0..5]; // Slicing the first 5 characters of the String
    println!("Sliced string: {}", sliced);
}
```
