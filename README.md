# Rust Refresher: A 20-Hour Practical Curriculum

> **Level:** Beginner Rust refresher\
> **Target:** 20 hours of focused practice\
> **Toolchain:** Stable Rust, Rust 2024 edition, Cargo\
> **Primary outcome:** Rebuild practical Rust fluency and prepare for a
> larger distributed-computing project.

## Scope and use of ChatGPT

This repository contains a structured, approximately 20-hour set of Rust
exercises and small applications. It starts with syntax and basic
algorithms, then progresses through ownership, borrowing, data
modelling, error handling, collections, traits, testing, serialization,
concurrency, networking, and a small multi-process node system.

The intended learner has general programming experience but is not
assumed to remember Rust syntax or idioms. Time estimates include
reading the Rust Book and API documentation, looking up syntax,
implementation, debugging, and basic testing. The estimates are planning
guidance, not deadlines.

**ChatGPT was used to design and organize this curriculum.** It was used
to propose the learning progression, exercise requirements, estimated
durations, Rust topics to investigate, suggested implementation
approaches, and completion criteria. The exercises are intended to be
implemented and understood by the learner. ChatGPT-generated material is
a study plan, not a substitute for the Rust documentation, independent
implementation, testing, or review. Any code added to this repository
should be written, tested, and understood by the learner.

The curriculum is deliberately scoped as a refresher rather than a
complete course in distributed systems. The final exercises introduce
communication between independent processes, but do not implement
production-grade reliability, consensus, replication, or fault
tolerance.

## Learning objectives

By the end of the curriculum, you should be able to:

-   Write basic Rust functions, control flow, and collection operations
    without repeatedly looking up every syntax detail.
-   Explain ownership, moves, borrowing, mutable references, and the
    borrow checker's constraints.
-   Model application data with structs and enums.
-   Use `Option`, `Result`, custom errors, and the `?` operator
    appropriately.
-   Work with vectors, hash maps, iterators, generics, traits, and
    modules.
-   Organize a Cargo project and write unit and integration tests.
-   Serialize and deserialize structured data.
-   Spawn threads and coordinate them using channels and synchronization
    primitives.
-   Build a basic TCP client/server application.
-   Run multiple independent node processes and exchange structured
    messages.

## Time budget

  Phase       Focus                                                     Estimated time
  ----------- ------------------------------------------------------- ----------------
  1           Syntax and fundamental algorithms                                     2h
  2           Ownership, borrowing, and references                                  3h
  3           Structs, enums, and error handling                                    3h
  4           Collections, iterators, generics, traits, and modules                 3h
  5           Testing, file handling, and serialization                             2h
  6           Concurrency and parallelism                                           3h
  7           Networking and distributed-computing mini-projects                    4h
  **Total**                                                                    **20h**

The estimates are based on a learner who knows programming concepts but
needs to consult Rust documentation and experiment with unfamiliar
syntax. If a foundational topic takes longer, prioritize understanding
over finishing within the estimate.

## Tooling and working conventions

Use:

-   Stable Rust.
-   Rust 2024 edition.
-   Cargo for project and dependency management.
-   `rustfmt` for formatting.
-   Clippy for linting.
-   The standard library before adding third-party crates, except where
    an exercise explicitly introduces a dependency.

Useful references:

-   [The Rust Programming Language (The Rust
    Book)](https://doc.rust-lang.org/book/)
-   [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
-   [Rust Standard Library](https://doc.rust-lang.org/std/)
-   [The Cargo Book](https://doc.rust-lang.org/cargo/)
-   [Serde documentation](https://serde.rs/)
-   [serde_json API
    documentation](https://docs.rs/serde_json/latest/serde_json/)

A useful workflow for each exercise:

1.  Read the relevant concept in the Rust Book or API documentation.
2.  Write a small experiment if a syntax or ownership rule is unclear.
3.  Implement the exercise yourself.
4.  Compile frequently and read compiler diagnostics carefully.
5.  Add tests for expected behavior and failure cases.
6.  Run `cargo fmt`, `cargo clippy`, and `cargo test`.
7.  Record any concepts or compiler errors that were particularly
    instructive in `LEARNINGS.md`.

Do not optimize prematurely. The priority is understanding Rust's rules
and expressing familiar programming patterns idiomatically.

------------------------------------------------------------------------

# Phase 1 --- Syntax and fundamental algorithms

**Time budget:** 2 hours\
**Exercises:** 4

### Objective

Restore familiarity with Rust syntax, expressions, functions, control
flow, primitive types, strings, and basic collections.

These are deliberately simple problems. The purpose is to practise
writing Rust, not to test algorithmic problem-solving ability.

### Exercise 1.1 --- FizzBuzz

**Estimated time:** 20 minutes

Write a program that prints numbers from 1 to 100, following the
standard FizzBuzz rules:

-   Multiples of 3 print `Fizz`.
-   Multiples of 5 print `Buzz`.
-   Multiples of both print `FizzBuzz`.
-   Everything else prints the number.

**Rust concepts to practise:**

-   `fn main()`
-   Variables and mutability (`let`, `mut`)
-   Integer types
-   `if`, `else if`, and `else`
-   `for` loops and ranges
-   `println!` and formatting

**Suggested approach:**

Implement the logic in a function that accepts an integer and returns a
`String`. Let `main` handle iteration and printing. This separates
business logic from output.

### Exercise 1.2 --- Number statistics

**Estimated time:** 25 minutes

Write a function that receives a collection of integers and calculates:

-   Minimum value.
-   Maximum value.
-   Sum.
-   Arithmetic mean.
-   Number of even and odd values.

**Rust concepts to practise:**

-   Functions and return values.
-   Slices (`&[i32]`).
-   Loops.
-   Conditional expressions.
-   Integer versus floating-point arithmetic.

**Suggested approach:**

Start with a regular loop and a handful of mutable accumulator
variables. Do not use iterators yet; you will revisit this problem in
Phase 4.

Decide what should happen when the input collection is empty. Avoid
silently dividing by zero or inventing a minimum/maximum value.

### Exercise 1.3 --- String reversal and palindrome detection

**Estimated time:** 30 minutes

Implement two functions:

1.  Reverse a string.
2.  Determine whether a string is a palindrome.

For the palindrome function, ignore capitalization and whitespace.

Examples:

``` text
"Never odd or even" -> true
"Rust is great"     -> false
```

**Rust concepts to practise:**

-   `String` versus `&str`.
-   String methods.
-   Iteration.
-   Character handling.
-   Ownership when returning strings.

**Suggested approach:**

Normalize the input first, then compare its characters from opposite
ends or compare the normalized sequence with its reverse.

Rust strings are UTF-8 encoded. Do not assume that indexing a `String`
by integer position works like indexing a character array in some other
languages. Start with ASCII input if necessary, but understand the
limitation and investigate Unicode character handling if time permits.

### Exercise 1.4 --- Simple command-line calculator

**Estimated time:** 45 minutes

Implement a calculator supporting:

-   Addition.
-   Subtraction.
-   Multiplication.
-   Division.
-   Remainder.

The program should receive two numbers and an operator, then produce a
result.

**Rust concepts to practise:**

-   Functions.
-   `match`.
-   Standard input.
-   Parsing strings into numbers.
-   Basic error handling.

**Suggested approach:**

Separate calculation from input/output. For example, design a function
conceptually similar to:

``` rust
fn calculate(a: f64, b: f64, operator: char) -> /* choose a return type */
```

Decide how the function should represent an invalid operator, invalid
input, and division by zero. You will study the appropriate Rust
error-handling types in Phase 3.

------------------------------------------------------------------------

# Phase 2 --- Ownership, borrowing, and references

**Time budget:** 3 hours\
**Exercises:** 3

### Objective

Understand the most important conceptual difference between Rust and
many other programming languages.

Ownership and borrowing influence practically every component of a
distributed application. Spend time understanding the compiler's
reasoning rather than trying to work around it.

### Exercise 2.1 --- String manipulation without unnecessary cloning

**Estimated time:** 45 minutes

Create a program that:

1.  Creates or accepts a `String`.
2.  Passes it to a function that prints its length.
3.  Passes it to another function that checks whether it contains a
    particular substring.
4.  Finally, prints the original string in `main`.

**Requirements:**

-   Avoid cloning the string.
-   Avoid transferring ownership unnecessarily.
-   Use references wherever appropriate.

**Concepts to investigate:**

-   Ownership transfer.
-   Borrowing.
-   Immutable references.
-   Mutable references.
-   Borrowing rules.

**Suggested approach:**

Consider which functions actually need ownership and which only need
temporary access to the data. Be able to explain why each function
accepts `String`, `&String`, or `&str`.

### Exercise 2.2 --- Small inventory manager

**Estimated time:** 1 hour

Create a program that manages a collection of products.

Each product has:

-   Name.
-   Quantity.
-   Price.

Implement functions to:

-   Add a product.
-   Remove a product.
-   Change its quantity.
-   Find a product by name.
-   Calculate the total inventory value.

**Requirements:**

-   Store products in a `Vec`.
-   Avoid cloning products unnecessarily.
-   Use references when reading product information.
-   Use mutable references when modifying products.

**Concepts to investigate:**

-   Structs.
-   `Vec<T>`.
-   References.
-   `iter()` and `iter_mut()`.
-   `find()`.

**Suggested approach:**

Keep the collection in `main` and pass references to functions that
operate on it. Pay attention to the difference between borrowing the
entire collection and borrowing an individual element.

### Exercise 2.3 --- Fix the borrow checker

**Estimated time:** 1 hour 15 minutes

Create a separate Rust project containing several intentionally
incorrect snippets. Make each one compile without simply adding
`.clone()` everywhere.

**Problem A --- Ownership moved**

``` rust
fn main() {
    let name = String::from("Rust");

    let another_name = name;

    println!("{}", name);
}
```

**Problem B --- Conflicting borrows**

``` rust
fn main() {
    let mut numbers = vec![1, 2, 3];

    let first = &numbers[0];

    numbers.push(4);

    println!("{}", first);
}
```

**Problem C --- Mutable borrowing**

``` rust
fn main() {
    let mut value = String::from("hello");

    let reference = &value;

    reference.push_str(" world");
}
```

**Requirements:**

-   Fix each example.
-   Explain why the original version failed.
-   Implement at least one alternative solution for each problem.
-   Avoid using `.clone()` as a universal solution.

**Concepts to investigate:**

-   Move semantics.
-   Shared versus exclusive references.
-   Mutable borrowing.
-   Reference lifetimes and scope.
-   Non-lexical lifetimes.

**Expected outcome:**

Become comfortable treating compiler errors as explanations of ownership
constraints rather than arbitrary restrictions.

------------------------------------------------------------------------

# Phase 3 --- Structs, enums, and error handling

**Time budget:** 3 hours\
**Exercises:** 3

### Objective

Learn how Rust represents application state and handles invalid
operations. These concepts are important when designing network
messages, node states, and protocol responses.

### Exercise 3.1 --- Task manager data model

**Estimated time:** 45 minutes

Create a task manager with the following data model.

A task contains:

-   ID.
-   Description.
-   Status.
-   Priority.

Use enums to represent task status and priority. For example:

``` rust
enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

enum Priority {
    Low,
    Medium,
    High,
}
```

Implement functions or methods to:

-   Create a task.
-   Change its status.
-   Change its priority.
-   Display a task.
-   Determine whether a task is completed.

**Concepts to investigate:**

-   Structs.
-   Enums.
-   `impl` blocks.
-   Methods and associated functions.
-   Pattern matching.
-   `match` and `if let`.

**Suggested approach:**

Put behavior related to a task inside its `impl` block rather than
implementing everything as free functions.

### Exercise 3.2 --- Proper error handling

**Estimated time:** 1 hour

Extend the task manager.

Implement a function that retrieves a task by ID. Choose a return type
that represents the possibility that the task does not exist.

Then implement a function that changes a task's status and can fail if:

-   The task does not exist.
-   The requested transition is invalid.

For example, completing an already completed task should return an
error.

**Concepts to investigate:**

-   `Option<T>`.
-   `Result<T, E>`.
-   `Ok` and `Err`.
-   `Some` and `None`.
-   The `?` operator.
-   Custom error enums.
-   `unwrap()` versus proper error handling.

**Suggested approach:**

Use `Option` when a value might simply be absent. Use `Result` when an
operation can fail for a meaningful reason. Avoid `unwrap()` in
application logic.

### Exercise 3.3 --- Configuration parser

**Estimated time:** 1 hour 15 minutes

Create a small application that reads configuration values from a text
file.

Example configuration:

``` text
server_name=node_1
port=8080
max_connections=100
debug=true
```

Implement:

-   Reading the file.
-   Parsing key-value pairs.
-   Converting values into appropriate types.
-   Validating required fields.
-   Returning useful errors when configuration is invalid.

**Concepts to investigate:**

-   File I/O.
-   `std::fs`.
-   `std::str::FromStr`.
-   `Result`.
-   Custom error types.
-   Struct initialization.

**Suggested approach:**

Parse the file into a temporary collection of key-value pairs, then
construct a typed configuration struct. Do not introduce a configuration
library yet; the purpose is to understand parsing and error propagation.

------------------------------------------------------------------------

# Phase 4 --- Collections, iterators, generics, traits, and modules

**Time budget:** 3 hours\
**Exercises:** 3

### Objective

Become familiar with the tools used to structure larger Rust
applications.

### Exercise 4.1 --- Refactor number statistics using iterators

**Estimated time:** 40 minutes

Return to Exercise 1.2 and rewrite the number statistics program using
Rust iterators.

Investigate and use methods such as:

-   `iter()`
-   `filter()`
-   `map()`
-   `fold()`
-   `sum()`
-   `min()` and `max()`

**Requirements:**

Implement both the original loop-based and iterator-based versions.
Compare their readability and understand what each iterator operation
does.

**Concepts to investigate:**

-   Iterators.
-   Closures.
-   Iterator adapters.
-   Lazy evaluation.
-   Ownership and borrowing in iterator chains.

### Exercise 4.2 --- Generic data container

**Estimated time:** 50 minutes

Create a generic container called `Storage<T>`.

It should support:

-   Adding an item.
-   Removing an item by index.
-   Retrieving an item by index.
-   Getting the number of stored items.
-   Checking whether it is empty.

Use a `Vec<T>` internally.

**Concepts to investigate:**

-   Generics.
-   Generic structs.
-   Generic methods.
-   Trait bounds.
-   References returned from functions.
-   `impl<T>` syntax.

**Suggested approach:**

Implement the basic container without trait bounds first. Introduce
trait bounds only when a particular operation requires them.

### Exercise 4.3 --- Traits and modules

**Estimated time:** 1 hour 30 minutes

Refactor your task manager into a proper Cargo project.

Suggested structure:

``` text
task_manager/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── lib.rs
    ├── task.rs
    ├── manager.rs
    └── storage.rs
```

Create a trait called `StorageBackend`. It should define operations such
as:

-   Insert a task.
-   Retrieve a task.
-   Remove a task.

Implement the trait for an in-memory storage backend using `HashMap`.

**Concepts to investigate:**

-   Traits.
-   Trait implementations.
-   Visibility (`pub`).
-   Modules.
-   `mod` and `use`.
-   `HashMap`.
-   Trait bounds and `where` clauses.

**Suggested approach:**

Keep the trait independent of the concrete storage implementation. The
task manager should depend on the abstraction rather than directly
depending on the `HashMap`.

This introduces dependency abstraction, which will become useful when
you have multiple storage or communication backends.

------------------------------------------------------------------------

# Phase 5 --- Testing, file handling, and serialization

**Time budget:** 2 hours\
**Exercises:** 2

### Objective

Learn how to make Rust applications testable and work with external
data.

### Exercise 5.1 --- Add tests to the task manager

**Estimated time:** 45 minutes

Return to your task manager and add tests for:

-   Creating tasks.
-   Retrieving existing tasks.
-   Retrieving nonexistent tasks.
-   Changing task status.
-   Invalid state transitions.
-   Removing tasks.

**Concepts to investigate:**

-   `#[test]`.
-   `assert_eq!`.
-   `assert!`.
-   `#[cfg(test)]`.
-   Unit tests.
-   Integration tests.
-   Test organization in Cargo.

**Requirements:**

Write at least eight tests, including successful and unsuccessful
operations.

Run them using:

``` bash
cargo test
```

### Exercise 5.2 --- Serialize and deserialize data

**Estimated time:** 1 hour 15 minutes

Extend your task manager to save its state to a JSON file and load it
again.

Use the `serde` and `serde_json` crates.

Implement:

-   Saving all tasks.
-   Loading tasks.
-   Handling missing files.
-   Handling malformed JSON.
-   Testing that data survives a save/load cycle.

**Concepts to investigate:**

-   External dependencies.
-   `Cargo.toml`.
-   Serialization.
-   Deserialization.
-   Derive macros.
-   `#[derive(Serialize, Deserialize)]`.
-   Integration testing.

**Resources:**

-   [Serde documentation](https://serde.rs/)
-   [serde_json API
    documentation](https://docs.rs/serde_json/latest/serde_json/)

**Why this matters:**

Distributed applications constantly serialize information before
transmitting it between processes. JSON is a convenient first format
before exploring more specialized binary protocols.

------------------------------------------------------------------------

# Phase 6 --- Concurrency and parallelism

**Time budget:** 3 hours\
**Exercises:** 3

### Objective

Understand how Rust handles concurrent execution and shared data. This
phase is directly relevant to the intended distributed-computing
project.

### Exercise 6.1 --- Parallel number processing

**Estimated time:** 45 minutes

Create a program that:

1.  Generates a large collection of integers.
2.  Splits it into several chunks.
3.  Spawns a thread for each chunk.
4.  Calculates the sum of each chunk in parallel.
5.  Combines the results.

Compare the result against a sequential implementation.

**Concepts to investigate:**

-   `std::thread`.
-   `thread::spawn`.
-   `JoinHandle`.
-   `join()`.
-   Moving ownership into threads.
-   The `move` keyword.
-   `Send` and `Sync`.

**Suggested approach:**

Use `thread::spawn` and move each chunk into its corresponding thread.
Start with a small number of threads. Do not introduce thread pools or
asynchronous runtimes.

### Exercise 6.2 --- Producer-consumer application

**Estimated time:** 1 hour

Implement a producer-consumer system.

Requirements:

-   One producer generates jobs.
-   Multiple consumers process jobs.
-   Jobs are communicated through channels.
-   Consumers print the jobs they process.
-   The program terminates cleanly when all jobs have been processed.

**Concepts to investigate:**

-   `std::sync::mpsc`.
-   Channels.
-   `Sender` and `Receiver`.
-   Multiple consumers.
-   Thread synchronization.
-   Channel disconnection.
-   Ownership transfer between threads.

**Suggested approach:**

Use an MPSC channel for communication. Start with one producer and one
consumer, then extend it to multiple consumers.

Think about what happens when the producer finishes and how consumers
know there is no more work. Note that the standard library's MPSC
receiver is not directly cloneable; investigate an appropriate way to
distribute work to multiple consumers, such as a shared receiver
protected by a mutex or a separate channel per worker.

### Exercise 6.3 --- Shared state and synchronization

**Estimated time:** 1 hour 15 minutes

Create a shared counter accessed by multiple threads.

Requirements:

-   Spawn ten threads.
-   Each thread increments the counter 1,000 times.
-   All threads operate on the same counter.
-   The final result must be 10,000.

Implement the counter using:

-   `Arc<Mutex<T>>`.

Then investigate how the implementation would differ using:

-   `Arc<AtomicUsize>`.

**Concepts to investigate:**

-   `Arc<T>`.
-   `Mutex<T>`.
-   `MutexGuard`.
-   Atomic types.
-   Interior mutability.
-   Thread-safe shared ownership.
-   Lock contention.

**Suggested approach:**

First implement the mutex version. Understand why `Arc` is needed in
addition to `Mutex`.

Then implement the atomic version and compare the two approaches. Do not
attempt to optimize performance; focus on understanding the
synchronization guarantees.

------------------------------------------------------------------------

# Phase 7 --- Networking and distributed-computing mini-projects

**Time budget:** 4 hours\
**Exercises:** 3

### Objective

Combine the Rust concepts you have practised into small applications
resembling the building blocks of a distributed system.

These exercises are deliberately more open-ended. You should now be
capable of structuring a small project without being told exactly where
every function belongs.

### Exercise 7.1 --- TCP echo server and client

**Estimated time:** 1 hour 15 minutes

Build two separate Cargo binaries:

-   A TCP server.
-   A TCP client.

The server should:

1.  Listen on a configurable TCP port.
2.  Accept incoming connections.
3.  Read messages from connected clients.
4.  Send received messages back.
5.  Handle clients disconnecting unexpectedly.

The client should:

1.  Connect to the server.
2.  Send messages from standard input.
3.  Receive responses.
4.  Terminate cleanly when instructed.

**Concepts to investigate:**

-   `std::net::TcpListener`.
-   `std::net::TcpStream`.
-   `Read` and `Write` traits.
-   Buffered I/O (`BufReader`).
-   `std::io::Result`.
-   TCP connections and disconnections.
-   Message framing.

**Suggested approach:**

Start with a server that accepts one connection and echoes one message.
Extend it to handle multiple messages over the same connection. Finally,
investigate how to handle multiple clients concurrently. One thread per
connection is sufficient for this exercise.

**Important:**

TCP is a byte stream, not a message-based protocol. The receiver needs a
way to determine where one message ends and the next begins. For this
exercise, newline-delimited messages are sufficient.

### Exercise 7.2 --- Concurrent job-processing server

**Estimated time:** 1 hour 15 minutes

Combine networking and concurrency knowledge.

Build a server that accepts jobs from TCP clients and processes them
using worker threads.

Example job:

``` json
{
  "job_id": 42,
  "operation": "sum",
  "numbers": [10, 20, 30, 40]
}
```

The server should:

1.  Accept TCP connections.
2.  Receive jobs.
3.  Deserialize incoming JSON.
4.  Submit jobs to a shared work queue.
5.  Have multiple worker threads process jobs.
6.  Return results to the requesting client.

Support at least two operations:

-   Sum a collection of numbers.
-   Calculate the maximum value in a collection.

**Concepts to investigate:**

-   TCP networking.
-   Serde serialization/deserialization.
-   Channels.
-   Worker threads.
-   Shared application state.
-   `Arc` and `Mutex`.
-   Custom enums for job types.
-   Error propagation.

**Suggested architecture:**

``` text
TCP clients
    |
    v
TCP server
    |  Accept connections and deserialize jobs
    v
Job queue
    |
    +------ Worker 1
    +------ Worker 2
    +------ Worker 3
```

One implementation detail to consider is how the server associates a
worker's result with the correct client connection.

Initially, keep the client connection open while waiting for its result.
A sophisticated asynchronous response system is not required.

### Exercise 7.3 --- Mini distributed node system

**Estimated time:** 1 hour 30 minutes

This is the final exercise and the closest to the intended project.

Build a simple application consisting of multiple independent nodes.

Each node should:

-   Have a unique node ID.
-   Listen on a TCP port.
-   Accept messages from other nodes.
-   Send messages to other nodes.
-   Maintain a local counter.
-   Respond to status requests.

Implement at least three message types:

``` rust
enum Message {
    Ping { sender_id: String },
    GetStatus,
    IncrementCounter,
}
```

You can extend this with a response enum.

**Required behaviour:**

1.  Start three instances of the application using different ports and
    IDs.
2.  Configure each instance with the addresses of the other nodes.
3.  Send a ping from one node to another.
4.  Request another node's status.
5.  Increment a remote node's counter.
6.  Display the responses.

**Concepts to investigate:**

-   TCP communication.
-   Serialization and deserialization.
-   Enums as protocol message types.
-   Structuring a network protocol.
-   Configuration management.
-   Error handling.
-   Multiple independent processes.
-   Separation of application logic and transport logic.

**Suggested project structure:**

``` text
node/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── node.rs
    ├── protocol.rs
    ├── network.rs
    └── config.rs
```

Suggested responsibilities:

  Module          Responsibility
  --------------- -------------------------------
  `main.rs`       Application entry point
  `node.rs`       Node state and business logic
  `protocol.rs`   Message definitions
  `network.rs`    TCP communication
  `config.rs`     Node configuration

**Important scope boundary:**

Do not implement leader election, consensus, replication, or distributed
synchronization yet. The goal is to understand the mechanics of multiple
independent Rust processes communicating over a network.

Each node should maintain its own state. Do not simulate three nodes
using three threads sharing the same application state. Run three
separate processes.

------------------------------------------------------------------------

# Completion criteria

Use this checklist to assess whether the curriculum achieved its
purpose.

## Rust fundamentals

-   [ ] Write functions and control flow without constantly consulting
    syntax references.
-   [ ] Understand `String`, `&str`, primitive types, and collections.

## Ownership and type system

-   [ ] Explain ownership transfers, borrowing, and mutable references.
-   [ ] Use structs, enums, `Option`, and `Result` appropriately.
-   [ ] Understand basic generics and trait implementations.

## Application development

-   [ ] Organize a project into modules.
-   [ ] Use Cargo dependencies.
-   [ ] Write unit and integration tests.
-   [ ] Serialize and deserialize structured data.

## Concurrency and networking

-   [ ] Spawn and coordinate threads.
-   [ ] Use channels and shared synchronized state.
-   [ ] Create a TCP client and server.
-   [ ] Run multiple independent processes that exchange structured
    messages.

You do not need to be an expert in every concept. You should be able to
recognize when to use each feature, implement a basic solution, and find
the relevant documentation when something unfamiliar arises.

------------------------------------------------------------------------

# Working recommendations

1.  **Do not use AI to generate complete solutions.** Use documentation,
    compiler diagnostics, and AI explanations to understand unfamiliar
    syntax. Write the implementations yourself. The objective is to
    rebuild your ability to express ideas in Rust.

2.  **Avoid unnecessary dependencies.** Use the standard library
    wherever possible. The exceptions in this curriculum are `serde` and
    `serde_json`, which introduce useful real-world tooling.

3.  **Use Clippy and formatting regularly.** Run the following commands:

    ``` bash
    cargo fmt
    cargo clippy
    cargo test
    ```

    Treat warnings as things to investigate, not simply things to
    suppress.

4.  **Keep your projects.** Do not discard the exercises after
    completing them. Some will become useful reference implementations
    when you start your actual project.

5.  **Do not introduce Tokio yet.** The standard library's threading and
    networking APIs are sufficient for this refresher. Understanding
    synchronous networking and concurrency first will make asynchronous
    Rust easier to reason about later.

## Suggested repository organization

Keep each exercise in its own Cargo project, or group related exercises
into a Cargo workspace if you are comfortable with workspaces. A simple
initial layout could be:

``` text
rust-refresher/
├── README.md
├── LEARNINGS.md
├── phase-1/
│   ├── fizzbuzz/
│   ├── number-statistics/
│   ├── palindrome/
│   └── calculator/
├── phase-2/
│   ├── borrowing-strings/
│   ├── inventory/
│   └── borrow-checker/
├── phase-3/
│   ├── task-manager/
│   ├── error-handling/
│   └── config-parser/
├── phase-4/
│   ├── iterator-statistics/
│   ├── generic-storage/
│   └── task-manager-refactor/
├── phase-5/
│   └── task-manager-tests-and-json/
├── phase-6/
│   ├── parallel-sum/
│   ├── producer-consumer/
│   └── shared-counter/
└── phase-7/
    ├── tcp-echo/
    ├── job-server/
    └── distributed-nodes/
```

This is a suggested layout, not a requirement. The important point is to
keep exercises independently runnable and avoid turning the entire
curriculum into one large application too early.

## What to study after this curriculum

After completing the 20-hour refresher, a useful next stage for the
intended distributed-computing project would cover:

-   Tokio and asynchronous Rust.
-   Futures and `async`/`await`.
-   Distributed messaging patterns.
-   Network protocol design.
-   Fault tolerance, timeouts, and retries.
-   Node discovery and coordination.
-   Distributed consistency and consensus.

These topics belong in a subsequent learning stage rather than being
squeezed into this refresher.

------------------------------------------------------------------------

## Progress log

Record the date, actual time spent, and any concepts that required
additional investigation. This makes it easier to identify which Rust
topics need another pass before starting the main project.

  Phase                                      Estimated time   Actual time Completed
  ---------------------------------------- ---------------- ------------- -----------
  1\. Syntax and algorithms                              2h               
  2\. Ownership and borrowing                            3h               
  3\. Structs, enums, and errors                         3h               
  4\. Collections, traits, and modules                   3h               
  5\. Testing and serialization                          2h               
  6\. Concurrency                                        3h               
  7\. Networking and distributed systems                 4h               
  **Total**                                         **20h**               

**Recommended companion file:** `LEARNINGS.md`. Use it as a concise
personal reference for Rust syntax, ownership rules, compiler errors,
standard-library APIs, and design decisions discovered while completing
the exercises.
