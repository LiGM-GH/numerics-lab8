default:
    just --list

run part_number:
    PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig/ cargo run --locked {{part_number}}

run-win part_number:
    PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig/ cross build --target x86_64-pc-windows-gnu --locked
    wine ./target/x86_64-pc-windows-gnu/debug/lab8.exe {{part_number}}
