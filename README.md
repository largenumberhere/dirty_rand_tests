# Dirty_rand_tests
Contains `dirty_rand`, the worst random number generator you'll ever see. Made with unsafe rust and love.
Made for some fun and practice working with unsafe rust. Please feel free to get in contact or create an issue if you find any code-correctness concerns.
It explores the idea of using initialized memory to seed random integers.
 
### Use dirty_rand as a library
**No warranty or guarantees are included for using `dirty_rand` in your own project. Doing so is at your own risk!** Dirty rand is **not** cryptographically secure. See the inline documentation for more reasons why you should not use this library.
1. You will obviously need cargo installed and have made a new project.
    This root folder contains only tests for `dirty_rand`. To use dirty_rand as a library in your project, add under the `[dependencies]` section  in your `Cargo.toml`:
    ```toml
    dirty_rand = {git = "https://github.com/largenumberhere/dirty_rand_tests.git"}
    ```
    Cargo will search this repository and find the crate `dirty_rand`.

2. Alternatively, if you desire you may clone this repository and specify its path. You will need to change the path in the example bellow.
    ```toml
    dirty_rand = {path = "C:\\users\\yourname\\Documents\\dirty_rand_test\\dirty_rand"}
    ```

