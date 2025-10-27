::set PATH=%PATH%;c:\raylib\w64devkit\bin
d:
cd d:\coding\erp
del target\release\erp.exe
cargo build --release
target\release\erp.exe