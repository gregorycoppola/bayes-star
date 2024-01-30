RUST_BACKTRACE=1 RUST_LOG=info cargo run --bin test -- --print_training_loss --entities_per_domain=1024 --test_example=0 --scenario_name=$1
