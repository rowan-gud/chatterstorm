# Chatterstorm Overview

## Workspace Members

- `chatterstorm_core` - shared game objects
- `chatterstorm_parser` - parse raw card data into structured, playable data
- `chatterstorm_game_server` - authoritative game server
- `chatterstorm_client` - the pretty (hopefully) part

## Workflow

The idea here is to do as little work as we have to with raw card data. The oracle text should be pre-parsed into an intermediary (or final) representation which can be easily loaded and registered in the game server.

- Pull the oracle-cards bulk list from [Scryfall](https://scryfall.com/docs/api/bulk-data) nightly
- Parse each card's oracle text into a playable game object
- Serialize the game object using something like [bincode](https://docs.rs/bincode/latest/bincode/).
- Store the relevant raw card data like name, image uris, etc. as well as the serialized game object data into a database (SQLite).
- When the game engine receives a command to play a card using an ID, load it from the database and deserialize the game object.
- Register the game object in the game engine.

