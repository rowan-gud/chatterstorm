import requests
import json


def get_bulk_uri():
    res = requests.get("https://api.scryfall.com/bulk-data")

    data = json.loads(res.text)

    return next(x for x in data["data"] if x["type"] == "oracle_cards")["download_uri"]


def download_bulk_data(uri, filename):
    res = requests.get(uri)

    with open(filename, "w") as f:
        f.write(res.text)


download_url = get_bulk_uri()

download_bulk_data(download_url, "./chatterstorm_parser/data/cards.json")
