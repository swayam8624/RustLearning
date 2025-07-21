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
