# script to run service to predict directly
from pathlib import Path

import pandas as pd
import requests
from src.api.rest.config import config
from src.api.rest.dtos import feature as feature_dtos

# sanity check
from src.services.adapters.feature import GetFeatureFromSQLAdapter
from src.services.feature_service import FeatureService
from src.services.model_service import get_model_service

REQUEST_PATH = Path("data") / "requests.parquet"

test_df = pd.read_parquet(REQUEST_PATH)

model_service = FeatureService(
    GetFeatureFromSQLAdapter(
        f"{config.POSTGRES_DIALECT}://{config.POSTGRES_USER}:{config.POSTGRES_PASSWORD}"
        f"@{config.POSTGRES_HOST}:{config.POSTGRES_PORT}/{config.POSTGRES_DB}"
    ),
    get_model_service("data/model.onnx"),
)

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
        f"http://localhost:8080/feature/{test_df.iloc[_id]['feature_1_id']}",
        json=request_json,
    )
    response_1 = requests.post(
        f"http://localhost:8081/feature/{test_df.iloc[_id]['feature_1_id']}",
        json=request_json,
    )
    response_2 = requests.post(
        f"http://localhost:8082/feature/{test_df.iloc[_id]['feature_1_id']}",
        json=request_json,
    )

    assert response.json() == response_1.json() == response_2.json()

print(output[0])
