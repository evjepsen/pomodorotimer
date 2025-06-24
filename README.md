# Simple Pomodoro Timer

Running the _help_ command shows the available commands.

You can

- `Start`: start a pomodoro session consisting of a Work -> Break session.
- `Stop`: stop the session i.e. going back to idle.
- `Pause`: pause an ongoing session. Start again with `Start`.
- `Set <state> <duration in min>`: where the state is `Working` or `Breaking`.
- `stats <today, all-time>`
- `login <user-name>`: login or create a new user with username `<user-name>`.
- `whoami`: see who you are logged in as.
- `users`: see all users.

# Development environment

It is a standard rust project, so as long as you have rust installed along with cargo,
you can make changes and try them with

```bash
cargo run
```

RustRover ([link](https://www.jetbrains.com/rust/)) is a good IDE if that is something you want.

## Dependencies

Cargo handles most of it, except

- SQLite

Depending on your distribution you can install the dependencies with the following
<details>
<summary>Fedora</summary>

```bash
dnf install libsq3-devel
```

</details>
