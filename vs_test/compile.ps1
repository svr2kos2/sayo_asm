cargo run -p sayoasm --release -- -o vs_test/main.bin vs_test/main.s -l --listing-output vs_test/main.lst
cargo run -p sayo_uploader --release vs_test\main.bin --vpm 0x8089_0009_0014 --index 0
