from geoalchemy2 import Geography
from sqlalchemy import ARRAY, BigInteger, Column, Float
from sqlalchemy.orm import declarative_base

Base = declarative_base()


class Feature1(Base):
    __tablename__ = "feature_1"
    id = Column(BigInteger, primary_key=True)
    feature = Column(ARRAY(Float))


class Feature2(Base):
    __tablename__ = "feature_2"
    id: int = Column(BigInteger, primary_key=True)
    geog = Column(Geography(geometry_type="POINT", srid=4326), nullable=False)
    feature = Column(ARRAY(Float))
