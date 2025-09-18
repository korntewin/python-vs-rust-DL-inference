# script to run service to predict directly
import os
from pathlib import Path

import pandas as pd
import requests
from src.api.rest.config import config
from src.api.rest.dtos import feature as feature_dtos
from src.services.model_service import PyTorchModelService

REQUEST_PATH = Path("data") / "requests.parquet"

test_df = pd.read_parquet(REQUEST_PATH)

ENDPOINTS = [
    os.getenv("PYTHON_PYTORCH_API_SERVER_URL", "http://localhost:8080"),
    os.getenv("PYTHON_ONNX_API_SERVER_URL", "http://localhost:8081"),
    os.getenv("RUST_API_SERVER_URL", "http://localhost:8082"),
]

print(ENDPOINTS)


def assert_equal(response_1, response_2):
    data1 = response_1.json()["outputs"]
    data2 = response_2.json()["outputs"]
    for d1, d2 in zip(data1, data2):
        assert d1["id"] == d2["id"]
        assert round(d1["score"], 2) == round(d2["score"], 2)
        assert int(round(d1["displacement"], 0)) == int(round(d2["displacement"], 0))


PyTorchModelService(model_path="data/model.pt")


output = [[], [], []]
for _id in range(100):
    request = feature_dtos.Request(
        feature_2_ids=test_df.iloc[_id]["feature_2_ids"],
        latitude=test_df.iloc[_id]["latitude"],
        longitude=test_df.iloc[_id]["longitude"],
        max_dist=test_df.iloc[_id]["max_dist"],
        size=test_df.iloc[_id]["size"],
        sort_dist=test_df.iloc[_id]["sort_dist"],
    )
    request_json = request.model_dump()
    response = requests.post(
        f"{ENDPOINTS[0]}/feature/{test_df.iloc[_id]['feature_1_id']}",
        json=request_json,
        headers={"X-API-Key": config.API_KEY.get_secret_value()},
    )
    response_1 = requests.post(
        f"{ENDPOINTS[1]}/feature/{test_df.iloc[_id]['feature_1_id']}",
        json=request_json,
        headers={"X-API-Key": config.API_KEY.get_secret_value()},
    )
    response_2 = requests.post(
        f"{ENDPOINTS[2]}/feature/{test_df.iloc[_id]['feature_1_id']}",
        json=request_json,
    )

    assert_equal(response, response_1)
    assert_equal(response, response_2)

    if _id % 10 == 0:
        print(f"Processed {_id} requests")

print("Sanity check passed")
