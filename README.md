# rust countdown timer
Application that takes a user input, of time in seconds and counts down to zero,
and terminates the application

## Technologies used
* Rust
* Cargo
* Docker

### Prerequisites
#### Rust
Make sure you have the correct rust installed, see: [Installing Rust](https://www.rust-lang.org/learn/get-started)
The version of rust used in this project could be found here: [Cargo.toml](Cargo.toml)
To verify the version of rust installed, run the following command:
```bash script
rustc --version
```

#### Cargo
Make sure you have cargo installed using this command:
> Note: installing Rust using rustup will also install cargo
```bash script
cargo --version
```

#### Docker
Make sure you have docker installed, see: [Install Docker Engine](https://docs.docker.com/engine/install/) using this command:
```bash script
docker --version
```

### Build code
Build the code without running it
```bash script
cargo build
```

### Test code
Run all the tests
```bash script
cargo test
```

#### Run the application locally
##### Run code
Run the application locally
```bash script
cargo run
```

##### Run with docker
###### Create docker image of the app
Creating a docker image should be as simple as
```bash script
docker build -t rust-countdown-timer .
```

##### Running the docker image
```bash script
docker run -p 8080:8080 rust-countdown-timer
```

## Contact
This project is maintained by [MikAoJk](CODEOWNERS)
Questions and/or feature requests?
Please create an [issue](https://github.com/MikAoJk/rust-countdown-timer/issues)