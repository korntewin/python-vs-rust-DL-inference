"""
All components that are interact with real-world IO should be declared as abstract class.
For easier changing infra without impacting core logic aka Hexagonal Architecture.
"""

from abc import ABC, abstractmethod

from geoalchemy2 import Geography
from sqlalchemy import create_engine, func, select
from sqlalchemy.orm import sessionmaker

from src.services.types.entities import Feature1, Feature2


class GetFeaturePort(ABC):
    @abstractmethod
    def get_feature_1(self, feature_1_id: int) -> list[float]:
        pass

    @abstractmethod
    def get_feature_2(
        self,
        feature_2_ids: list[int],
        latitude: float,
        longitude: float,
        max_dist: float,
    ) -> tuple[list[int], list[list[float]], list[float]]:
        pass


class GetFeatureFromSQLAdapter(GetFeaturePort):
    def __init__(self, db_url: str):
        self.db_url = db_url
        self.engine = create_engine(db_url)
        self.Session = sessionmaker(self.engine)

    def get_feature_1(self, feature_1_id: int) -> list[float]:
        with self.Session() as session:
            feature_1 = session.execute(
                select(Feature1).where(Feature1.id == feature_1_id)
            ).scalar_one()
            return feature_1.feature

    def get_feature_2(
        self,
        feature_2_ids: list[int],
        latitude: float,
        longitude: float,
        max_dist: float,
    ) -> tuple[list[int], list[list[float]], list[float]]:
        with self.Session() as session:
            pt = func.ST_SetSRID(func.ST_MakePoint(longitude, latitude), 4326).cast(
                Geography(geometry_type="POINT", srid=4326)
            )
            distance_m = func.ST_Distance(Feature2.geog, pt)

            stmt = (
                select(Feature2, distance_m)
                .where(Feature2.id.in_(feature_2_ids))
                .where(distance_m < max_dist)
            )

            rows = session.execute(stmt).all()
        ids = [feature_2.id for feature_2, _ in rows]
        features = [feature_2.feature for feature_2, _ in rows]
        displacements = [distance_m for _, distance_m in rows]
        return ids, features, displacements
