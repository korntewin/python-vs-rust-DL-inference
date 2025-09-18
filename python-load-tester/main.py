from pathlib import Path

import pandas as pd
from locust import FastHttpUser, task

REQUESTS_PATH = Path("data") / "requests.parquet"
API_KEY = "api_key"


class LoadTester(FastHttpUser):
    def on_start(self):
        self.requests_df = pd.read_parquet(REQUESTS_PATH)

    @task
    def feature(self):
        random_data = self.requests_df.sample(n=1)
        self.client.post(
            f"/feature/{int(random_data['feature_1_id'].values[0])}",
            json={
                "feature_2_ids": [
                    int(id) for id in random_data["feature_2_ids"].values[0].tolist()
                ],
                "latitude": float(random_data["latitude"].values[0]),
                "longitude": float(random_data["longitude"].values[0]),
                "max_dist": int(random_data["max_dist"].values[0]),
                "size": int(random_data["size"].values[0]),
                "sort_dist": bool(random_data["sort_dist"].values[0]),
            },
            headers={"X-API-Key": API_KEY},
            name="inference",
        )
