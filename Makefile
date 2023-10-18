quick_dev:
	cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"

watch:
	cargo watch -q -c -w src/ -x run