# Project: Finding The Longest Possible Substring Without Repeating Characters Using The Rust Programming Language

### Brief overview of the project
This project is a Rust application that finds the longest substring of a given string that does not contain repeating characters. After the user enter the first string and execute, he is further prompted again to decide whether they want to try another string or exit the program.

### Steps to follow in order to execute the rust code

- Once on your local machine, clone the repository using the command:
```
git clone <repository url>

```

- Ensure that rust is installed on your local machine by running the command:
```
rustc --version

```

- If rust is not install, run the command below to install it:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

```

- Once the repository is successfully clone on your local machine, change to the project directory by running on the terminal:
```
cd <project-name>

```

- Once within the project, compile and run the Rust application using the command below which start the program:
```
cargo run

```

- If you want to test the functionality of the rust code, use the command:
```
cargo test

```

- If you have docker install in your local and want to instead execute the code using the container, run the command below which builds the project:
```
docker build -t <project-name> .

```

- Once the docker image is successfully built, run the  application inside a Docker container using the command below where the container is deleted once exited from it:
```
docker run --rm <project-name>

```
