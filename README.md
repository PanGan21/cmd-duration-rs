# cmd-duration-service

A Rust command-line tool for executing commands while tracking and displaying how long they have been running. This utility allows you to monitor the execution duration in real-time, printing the duration at the end of each output line. It's a useful tool for tracking long-running commands or scripts and provides clean, user-friendly output.

## Features

- <b>Real-time Execution Duration</b>:: Continuously shows the duration the command has been running.
- <b>Command Output Handling</b>:: Prints the command's output along with the real-time duration.
- <b>Flexible</b>:: Works with any command or script and displays output incrementally.
- <b>Clear and Simple Output</b>:: Keeps your terminal clean, displaying the duration at the right edge of your screen.

### Installation

To install cmd-duration-service as a command-line tool, simply run:

```bash
cd cmd-duration-service && cargo install --path .
```

### Usage

#### Command-Line Tool

To use the command-line tool, specify the command you want to run and any arguments. For example:

```bash
# Track the duration of a sleep command
cmd-duration-service sleep 5
```

The output will look like:

```bash
<command output>
                                    command running since: 00:00:05
```

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.
