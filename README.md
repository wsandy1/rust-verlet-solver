![image](https://us-east-1.tixte.net/uploads/cdn.mrsandywilly.com/vL6Ajq6b7T.gif)
# Rust Verlet Solver
A simple Verlet integration solver written using the Rust SDL2 bindings.
Where's the friction?!

### Building
[cargo-vcpkg](https://github.com/mcgoo/cargo-vcpkg) is required in the build process in order to provide the `sdl2-gfx` binaries.
```
cargo install cargo-vcpkg
cargo vcpkg build
cargo build
```

Just run the binary and click anywhere inside the grey circular constraint to spawn a verlet physics object.
