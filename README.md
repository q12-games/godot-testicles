# godot-testicles
Write unit tests for your godot-rust project in rust!


## Getting started

#### Create a test workspace
* Create a `test` directory inside your project root
* Add `test` to workspace members in your Cargo.toml as follows
```toml
[workspace]
members = [
  "test"
]
```
* Inside the `test` directory, run `cargo --init --name game_test` to setup a rust project
* Add godot-testicles as a dependency. ([Example](./example/Cargo.toml.example))

#### Setup test runner
* Create a gdnlib file and place it in the `test` directory ([Example](./example/test.gdnlib))
* Create a `build.rs` file in your `test` directory ([Example](./example/build.rs))
* Add `run-tests.sh` to your `.gitignore` as it will be generated in the `test` directory
* In your `lib.rs`, use the `run_tests` macro to run test modules as below
```rust
use godot_testicles::*;

gdnative::godot_init!(init); // Use your init function with classes

run_tests! {
  example_test;
}
```
* Create `example_test.rs` in your `src`
```rust
use gdnative::prelude::*;
use godot_testicles::*;

testicles! {
  fn simple_test() {
    d!("1 should equal 1");
    expect!(1).to_equal(1);
  }
  
  fn testing_example_class() {
    d!("Example should process without errors");
    let root = get_root_node()?;

    let example = node!(Node, {}, |node: TRef<Node>| node.set_script(get_script("Example")), [
      node!(Label, { name: "Text", text: "Count: 0" }, [])
    ]);

    root.add_child(example, false);

    process_frame(&example); // Will run _process and _physics_process in Example class
  }
}
```

#### Run your test
* Run `cargo build` in the root of your project
* Run `./test/run_tests.sh` to run all of your testicles



## License
This project is licensed under [MIT License](./LICENSE)

