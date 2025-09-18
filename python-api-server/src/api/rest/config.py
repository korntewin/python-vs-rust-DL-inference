"""
Infrastructure configuration
"""

from pydantic import SecretStr
from pydantic_settings import BaseSettings


class Config(BaseSettings):
    POSTGRES_HOST: str = "localhost"
    POSTGRES_PORT: int = 5432
    POSTGRES_USER: str = "postgres"
    POSTGRES_PASSWORD: str = "password"
    POSTGRES_DB: str = "postgres"
    API_KEY: SecretStr = SecretStr("api_key")
    POSTGRES_DIALECT: str = "postgresql+psycopg"


config = Config()
