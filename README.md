# rust-countdown-timer

### Prerequisites
Make sure you have the rust installed using this command:
#### Rust
```bash script
rustc --version
```

#### Cargo
Make sure you have cargo installed using this command:
```bash script
cargo --version
```

#### Docker
Make sure you have docker installed using this command:
```bash script
docker --version
```

### Build code
Build the code without running it
```bash script
cargo build
```

### Test code
Build the code and run all the tests
```bash script
cargo test
```

#### Running the application locally
##### Run code
Build the code and run all the tests
```bash script
cargo run
```

##### Run with docker
###### Create docker image of the app
Creating a docker image should be as simple as
``` bash
docker build -t rust-countdown-timer .
```

##### Running the docker image
```bash 
docker run -p 8080:8080 rust-countdown-timer
```

## Contact
This project is maintained by [MikAoJk](CODEOWNERS)