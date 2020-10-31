
This repository contains a basic reverse shell written in rust using the [winapi](https://github.com/retep998/winapi-rs) library.

Rust provides a standard lib for windows allowing access to ffi, so an alternate reverse shell could be created using [std::os::windows](https://doc.rust-lang.org/std/os/windows/index.html)

### Build:
x86/x64 for windows using [Cross](https://github.com/rust-embedded/cross)
```
cross build --release --target x86_64-pc-windows-gnu
```

### Todo:
* Add support for powershell
* Dynamic ip/port at compile/runtime
* Secure Bind shell
