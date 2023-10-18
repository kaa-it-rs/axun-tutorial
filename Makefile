quick_dev:
	cargo watch -q -c -w examples/ -x "run --example quick_dev"

watch:
	cargo watch -q -c -w src/ -x run