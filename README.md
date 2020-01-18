# Mars

Mars is a proof of concept Rust port of the mach build tool used by the [Servo browser engine](https://github.com/servo/servo).

## Usage

Currently, Mars can build Servo on linux x86-64 platforms.

```bash
# First clone the Mars repo and use `cargo build` to generate
# the `mars` binary
git clone https://github.com/JoshMcguigan/mars.git
cd mars
cargo build

# Then run the `mars` binary from inside a servo repository
cd ~/path/to/servo
../path/to/mars/target/debug/mars build --dev
```

## Code style

For now I've attempted to port mach (which is Python based) to Rust literally, rather than translating to idiomatic Rust. This is an attempt to make it easier to see which parts of mach have been ported and which have not.

The plan is if work on Mars continues to the point where it reaches feature parity with mach (or nearly so), then it could be slowly migrated to a more idiomatic style.

## Motivation

My initial goal for writing Mars was to learn how mach works, and more generally what is required of build tooling for a large project.

Secondarily, I generally prefer reading/writing Rust over Python and I believe there others in the Servo community who feel the same (Servo itself is written in Rust). See [this Servo issue](https://github.com/servo/servo/issues/18343) for some discussion of this. It may be worth noting that I did not see that discussion before starting Mars.
