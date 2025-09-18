"""
Domain logic configuration
"""

from pydantic_settings import BaseSettings


class Config(BaseSettings):
    MODEL_PATH: str = "data/model.pt"


config = Config()
