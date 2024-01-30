RUST_BACKTRACE=1 RUST_LOG=info cargo run --bin train -- --print_training_loss --entities_per_domain=1096 --scenario_name=$1
