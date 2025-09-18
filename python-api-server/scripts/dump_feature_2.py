import time
from pathlib import Path
from typing import Any, Iterator

import pandas as pd
from geoalchemy2 import Geography
from more_itertools import chunked
from sqlalchemy import (
    create_engine,
    func,
)
from sqlalchemy.dialects.postgresql import insert as pg_insert
from src.services.types.entities import Feature2
from utils import get_db_url

FEATURE_2_PARQUET_PATH = Path("data") / "feature_2.parquet"
BATCH_SIZE = 10_000


def row_dict_generator(
    df: pd.DataFrame, feature_columns: list[str]
) -> Iterator[dict[str, Any]]:
    for _, row in df.iterrows():
        geog_value = func.ST_SetSRID(
            func.ST_MakePoint(row["longitude"], row["latitude"]), 4326
        ).cast(Geography(geometry_type="POINT", srid=4326))
        features = [float(row[c]) for c in feature_columns]

        yield {
            "id": int(row["feature_2_id"]),
            "geog": geog_value,
            "feature": features,
        }


def dump_feature_2(
    parquet_path: Path = FEATURE_2_PARQUET_PATH,
    batch_size: int = BATCH_SIZE,
) -> None:
    df = pd.read_parquet(parquet_path).drop_duplicates(
        subset=["feature_2_id"], keep="first"
    )
    feature_columns = sorted(
        [c for c in df.columns if not c.startswith("feature") and c.startswith("f")]
    )

    engine = create_engine(get_db_url())

    start_time = time.perf_counter()
    with engine.begin() as conn:
        for batch in chunked(row_dict_generator(df, feature_columns), batch_size):
            insert_stmt = pg_insert(Feature2).values(batch)
            upsert_stmt = insert_stmt.on_conflict_do_update(
                index_elements=[Feature2.id],
                set_={
                    "geog": insert_stmt.excluded.geog,
                    "feature": insert_stmt.excluded.feature,
                },
            )
            conn.execute(upsert_stmt)
    end_time = time.perf_counter()
    print(f"Time taken: {end_time - start_time} seconds")


if __name__ == "__main__":
    dump_feature_2()
