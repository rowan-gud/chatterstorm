# Chatterstorm

## Dependencies

- Rust
- Python (idk version I have 3.12) (optional)
- Make (optional)

### Install Python Dependencies

> NOTE: Python is only used as a script runner to download the cards.json file from scryfall into `chatterstorm_parser/data/cards.json`. You can just do that manually if you don't want to use Python

```bash
pip install -r requirements.txt
```

or

```bash
make install
```

> NOTE: The above will run with `--break-system-packages` not sure if it's just a mac thing but I had to run it with that flag.

## Running

```
make run
```

or

Normal Rust way

```bash
cargo run --bin chatterstorm_parser
```
