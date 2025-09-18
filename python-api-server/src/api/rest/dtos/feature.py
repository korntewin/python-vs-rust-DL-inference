from pydantic import BaseModel


class Request(BaseModel):
    feature_2_ids: list[int]
    latitude: float
    longitude: float
    size: int = 20
    max_dist: int = 5000
    sort_dist: bool = False


class Feature2Entity(BaseModel):
    id: int
    score: float
    displacement: float


class Response(BaseModel):
    outputs: list[Feature2Entity]
