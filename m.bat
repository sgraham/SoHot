cargo run --bin patchgen -q init test/main.obj test/stuff.obj || exit /b
cargo run --bin patchgen -q patch 1 test/stuff.sohot_1.obj test/stuff.sohot_1.patched.obj || exit /b
