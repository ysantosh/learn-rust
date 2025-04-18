# Get started with Installation
## Installation
### Installation on Ubuntu
* Install rust. 
  ```
  curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
  ```
  *NOTE* :choose default installation and proceed
* Set env variable
  ```
  source "$HOME/.cargo/env"
  rustup update
  rustup toolchain install stable
       output : stable-x86_64-unknown-linux-gnu unchanged - rustc 1.86.0 (05f9846f8 2025-03-31)
  
  ```
* Verify installation and get essentials
  ```  
  rustc --version
  sudo apt update
  sudo apt install build-essential
  ```
* Check git and if it is not installed then install it
  ```
  sudo apt update
  sudo apt install git
  ```
## Create your first program and execute
* Create directory and cd to it or use cargo to build one- Ref : https://doc.rust-lang.org/cargo/getting-started/first-steps.html
  ```
  cargo new rusttest1
  cd rusttest1
     tree .
          ├── Cargo.toml
          └── src
            └── main.rs
  ```
* file main.rs
  ```
  vim main.rs
     fn main() {
    println!("Congratulations! You have installed your Rust program and it works.");
     }
  ```
* Compile and run
  ```
  cargo build
  ./target/debug/rusttest1
  OR 
  rustc main.rs && ./main
  ```
* Build and execute in the same command
  ```
  cargo run src/main.rs
  ```
## Running a script to check db connect time for postgre SQL and issue faced.
* main.rs script is in this directory itself and we just replaced it at rusttest1/src and execute
 ```
  cargo run src/main.rs
  ```
#### Issue use of unresolved module
  ```
  app@oo-querier-1644432354-1-2438998920:~/rusttest1$ cargo run src/main.rs
   Compiling rusttest1 v0.1.0 (/home/app/rusttest1)
error[E0432]: unresolved import `anyhow`
 --> src/main.rs:1:5
  |
1 | use anyhow::Result;
  |     ^^^^^^ use of unresolved module or unlinked crate `anyhow`
  |
  = help: if you wanted to use a crate named `anyhow`, use `cargo add anyhow` to add it to your `Cargo.toml`
```
*Solution* : cargo add anyhow ; cargo add sqlx ; cargo add clap ; 

#### Feature not there
```
error[E0432]: unresolved import `sqlx::postgres`
  --> src/main.rs:3:11
   |
3  | use sqlx::postgres::PgPool;
   |           ^^^^^^^^ could not find `postgres` in `sqlx`
   |
note: found an item that was configured out
  --> /home/app/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sqlx-0.8.5/src/lib.rs:56:13
   |
56 |     self as postgres, PgConnection, PgExecutor, PgPool, PgTransaction, Postgres,
   |             ^^^^^^^^
note: the item is gated behind the `postgres` feature
  --> /home/app/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sqlx-0.8.5/src/lib.rs:52:7
   |
52 | #[cfg(feature = "postgres")]
   |       ^^^^^^^^^^^^^^^^^^^^

```
*Solution* : Add feature to Cargo.toml -- sqlx = { version = "0.8.5", features = ["postgres"]} --

#### derive parser issue
```
error: cannot find derive macro `Parser` in this scope
 --> src/main.rs:6:10
  |
6 | #[derive(Parser)]
  |          ^^^^^^
  |
note: `Parser` is imported here, but it is only a trait, without a derive macro
 --> src/main.rs:2:5
  |
2 | use clap::Parser;
  |     ^^^^^^^^^^^^

```
*Solution* : Add feature to Cargo.toml -- clap = { version = "4.5.36", features = ["derive"] }  --

#### unresolved tokio
```
error[E0433]: failed to resolve: use of unresolved module or unlinked crate `tokio`
  --> src/main.rs:29:3
   |
29 | #[tokio::main]
   |   ^^^^^ use of unresolved module or unlinked crate `tokio`

error[E0752]: `main` function is not allowed to be `async`
  --> src/main.rs:30:1
   |
30 | async fn main() -> Result<()> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `main` function is not allowed to be `async`

Some errors have detailed explanations: E0433, E0752.
For more information about an error, try `rustc --explain E0433`.
```
*Solution* : cargo add tokio ; add feature main -- tokio = {version = "1.44.2", features = ["full"]} --

#### Compilation successful 
```
./target/debug/rusttest1 -d "postgres://mlsanivia_admin:${pass109}@10.22.59.109:5432/mls_db"
Iteration: 0

thread 'main' panicked at /home/app/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/sqlx-core-0.8.5/src/pool/inner.rs:58:24:
either the `runtime-async-std` or `runtime-tokio` feature must be enabled
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
*Solution* : Tried adding runtime-async-std etc but the above error was for sqlx
Solved by adding runtime-tokio feature in sqlx -- sqlx = { version = "0.8.5", features = ["runtime-tokio", "postgres"]} --

#### DB connect issue
```
Iteration: 0
Error: error returned from database: no pg_hba.conf entry for host "xxxxx", user "xxxx", database "xxxx", no encryption

Caused by:
    no pg_hba.conf entry for host "xxxxx", user "xxxxx", database "xxxx", no encryption
```
*Solution* : Confusing error. This can be solved by adding tls to sqlx
Solved by adding "tls-native-tls" -- sqlx = { version = "0.8.5", features = ["runtime-tokio", "postgres", "tls-native-tls"] } --

#### Solved results
```
./target/debug/rusttest1 -d "postgres://xxxx:${xxx}@XXXXX:5432/xxxx"
Iteration: 0
Connect time: 254 ms, Ping time: 71 ms 
Iteration: 1
Connect time: 250 ms, Ping time: 70 ms 
Iteration: 2
Connect time: 260 ms, Ping time: 74 ms 
Iteration: 3
Connect time: 263 ms, Ping time: 75 ms 
Iteration: 4
Connect time: 240 ms, Ping time: 66 ms 
```
