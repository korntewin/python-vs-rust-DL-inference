import time
from pathlib import Path
from typing import Any, Iterator

import pandas as pd
from more_itertools import chunked
from sqlalchemy import create_engine
from sqlalchemy.dialects.postgresql import insert as pg_insert
from src.services.types.entities import Feature1
from utils import get_db_url

FEATURE_1_PARQUET_PATHS = sorted(list(Path("data").rglob("feature_1_*.parquet")))
BATCH_SIZE = 10_000


def row_dict_generator(
    df: pd.DataFrame, feature_columns: list[str]
) -> Iterator[dict[str, Any]]:
    for _, row in df.iterrows():
        yield {
            "id": int(row["feature_1_id"]),
            "feature": [float(row[c]) for c in feature_columns],
        }


def dump_feature_1(
    parquet_paths: list[Path] = FEATURE_1_PARQUET_PATHS,
    batch_size: int = BATCH_SIZE,
) -> None:
    start_time = time.perf_counter()
    df = pd.concat(
        [pd.read_parquet(parquet_path) for parquet_path in parquet_paths]
    ).drop_duplicates(subset=["feature_1_id"], keep="first")
    feature_columns = sorted(
        [c for c in df.columns if not c.startswith("feature") and c.startswith("f")]
    )

    engine = create_engine(get_db_url())
    with engine.begin() as conn:
        for batch in chunked(row_dict_generator(df, feature_columns), batch_size):
            insert_stmt = pg_insert(Feature1).values(batch)
            upsert_stmt = insert_stmt.on_conflict_do_update(
                index_elements=[Feature1.id],
                set_={
                    "feature": insert_stmt.excluded.feature,
                },
            )
            conn.execute(upsert_stmt)
    end_time = time.perf_counter()
    print(f"Time taken: {end_time - start_time} seconds")


if __name__ == "__main__":
    dump_feature_1()
