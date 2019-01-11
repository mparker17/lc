# Building

1. Ensure you have the Rust toolchain:

        curl https://sh.rustup.rs -sSf | sh
        rustup update
        rustup component add clippy
        rustup component add rustfmt

2. Clone this repository:

        git clone --recursive git@github.com:mparker17/lc.git
        cd lc

3. Compile for debugging...

        cargo build  # This will place the executable at target/debug/lc

    ...or for release...

        cargo build --release  # This will place the executable at target/release/lc

# Making changes

1. Fork this project and clone your fork.
2. Create a new branch:

        git checkout -b change_branchname

3. Repeat as necessary until you consider your changes ready to submit:

    1. Make some changes.
    2. Run linters:

            cargo clippy
            cargo fmt

        ... if the linters suggest changes, make those changes, or explain in
        the commit message why it does not make sense to do so.
    3. Run automated tests:

            cargo test

        ... if any automated tests fail, change your code so that it doesn't
        break the test, or change the test and explain in the commit message why
        it needed to change.
    4. Commit your changes. Ideally, one commit should contain one logical
        change; the first line of the commit message should succinctly explain
        that change, and if necessary, more information about the change can be
        added to the rest of the commit message.
4. Push your branch to your fork.
5. Submit a pull request.

# Packaging

Docker:

        docker build -f packaging/docker/Dockerfile .
