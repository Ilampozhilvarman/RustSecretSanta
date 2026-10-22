# Secret Santa

A small command-line Secret Santa generator written in Rust.

It randomly assigns every participant a recipient, makes sure nobody is assigned
to themselves, and then reveals each assignment privately by passing the
computer around.

## Usage

Run the program:

```sh
cargo run --release
```

Enter one name per line. Type `END` when you are done. Names must be unique,
and at least two people are required.

```text
!Enter one name per line. Type END when you're done!
Enter a list of people:
--------------------------------------------------
Alice
Bob
Carol
END
```

After the assignments are ready, pass the computer to each person in turn.
Each person presses Enter to reveal only their own recipient, then presses
Enter again to hide it before passing the computer on.

## Build

```sh
cargo build --release
```

The executable is written to `target/release/rust-secret-santa-alg` on macOS
and Linux, or `target/release/rust-secret-santa-alg.exe` on Windows.

The GitHub workflow can also build a Windows `.exe` artifact manually from the
Actions tab.

## Tests

```sh
cargo test
```
