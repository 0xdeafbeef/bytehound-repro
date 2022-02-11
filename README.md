# How to reproduce

1. Clone this repo
2. `cargo build --release --example simple_node `
3. Set your ip in `config.yaml:ip_address`
4. ` wget https://github.com/koute/bytehound/releases/download/0.8.0/bytehound-x86_64-unknown-linux-gnu.tgz && tar xf bytehound-x86_64-unknown-linux-gnu.tgz`
5. `LD_PRELOAD=./libbytehound.so ./target/release/examples/simple_node --config config.yaml --global-config ton-global.config-test.json`
6. Hangs forever. Hangs with both jemalloc or system allocator.  
It hangs before any rocksdb operations (you can delete config and see it's still doing nothing)
