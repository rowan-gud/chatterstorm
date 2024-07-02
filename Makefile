.PHONY: install

install:
	pip install -r requirements.txt --break-system-packages

.PHONY: download-cards

download-cards:
	python download-card-data.py

.PHONY: run

run:
	cargo run --bin chatterstorm_parser
