# GPU TRACER

Monitor GPU usage on top of nvidia-smi with TUI.

_Note: FThis is based on CPU solution: [https://github.com/yarenty/app_tracker](https://github.com/yarenty/app_tracker)._

## UI (TUI)

TODO add screenshot

## Build

Build it. Or don't. It's not like we're going to force you.

"The journey of a thousand miles begins with a single step." - Lao Tzu, probably while waiting for a Rust compilation.

```shell
cargo build -r
```

## Run

```shell
cargo run  -r -- -o out.csv 
```

This is how you run it. If you can follow these instructions, you're overqualified. 

"The only thing we have to fear is fear itself." - Franklin D. Roosevelt, definitely not talking about command-line arguments.

## Usage

```shell
gpu-tracer 0.4.0
Tracing GPU.

USAGE:
    gpu-tracer [OPTIONS] 

ARGS:
    <APPLICATION>    Application to be run as child process (alternatively provide PID of
                     running app)

OPTIONS:
    -h, --help                 Print help information
    -l, --log <LOG>            Set custom log level: info, debug, trace [default: info]
    -o, --output <OUTPUT>      Name of output CSV file with all readings - for further investigations
    -p, --pid <PID>            PID of external process
    -r, --refresh <REFRESH>    Refresh rate in milliseconds [default: 1000]
    -V, --version              Print version information

```

The command-line options. We have them. Use them, or don't. It's all the same to us.

## Example output

```log

```

Look at those logs. Aren't you impressed?

## CSV persistence

Example output.csv file:

```csv

```

CSV. Because tables are essential to human progress. Or something like that. 

"The only true wisdom is in knowing you know nothing." - Socrates, who definitely knew nothing about CSV.

## [CHANGELOG](CHANGELOG.md)

If you're still here, you must really enjoy pain. Or you are really interested in history. Or both.

## Activity


We're still alive! Barely.
