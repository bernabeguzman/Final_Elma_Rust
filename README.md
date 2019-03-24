# ELMA running on Rust Language

Project Goals:
-- Create an ELMA project running On Rust Language
-- I will translate all the c++ code created for elma to rust
-- I will succeed if I am able to port the main parts of elma code to rust
  and the code compiles
-- I will use docker, and may need a rust compiler of some sort

Five Milestones:
1. Research what is needed to run a Rust Project 
2. Create a new docker for Rust project
3. Set up the docker to run Rust
4. Translate elma core code
5. Create unit test/ or create testing platform

# HW_9 update
Finished step 1, and step 2 to research and create rust project using docker.
Next I will begin porting all the elma c++ code to rust and creat a test environment
What may be challenging is creating docs and tests for the rust language I will
have to learn what king of libraries are out there and how to used them. 

Getting the code
===
```bash
git clone https://github.com/bernabeguzman/Final_Elma_Rust
```

Building and starting the Docker Image
===
```bash
cd Final_Elma_Rust
docker build -t hack .
docker run ti -v $PWD/Final_Elma_Rust:/src/ hack
```
The build may take up several minutes.

Creating a new project using cargo 
===
```bash
cargo new ELMA_Rust2
```

Compile the project 
===
```bash
cd ELMA_Rust1
cargo build 
cargo run
```

Creating documentation
===
```bash
cargo doc
```
To get a new documents directory use the following command
```bash
cargo new doc
```

Clean up the project 
===
```bash
cargo clean
```

Running test files 
===
```bash
cargo test 
```
If you would like to see console println! comments use
```bash
cargo test -- --nocapture
```

No makefile needed using cargo framework for rust 