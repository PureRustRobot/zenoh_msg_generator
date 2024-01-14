# zenoh_msg_generator
generate original message struct derive serde(serialize, deserialize) from .msg files

[![Rust](https://github.com/PureRustRobot/zenoh_msg_generator/actions/workflows/rust.yml/badge.svg)](https://github.com/PureRustRobot/zenoh_msg_generator/actions/workflows/rust.yml)

# Usage
### Create new message package
```
cargo run -- new [package_name] [msg_file_name(.msg)]
```
You have to put the file directly under this package

**Example**
```
cargo run -- new hello_msgs Hello.msg
```

### Add new message to package
```
cargo run -- add [package_name] [msg_file_name(.msg)]
```

**Example**
```
cargo run -- new hello_msgs Motii.msg
```

# Result
- hello_msgs
  - Cargo.toml
  - src
    - lib.rs
    - msg.rs
