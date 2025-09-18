import os


def get_db_url() -> str:
    host = os.environ.get("POSTGRES_HOST", "localhost")
    port = os.environ.get("POSTGRES_PORT", "5432")
    user = os.environ.get("POSTGRES_USER", "postgres")
    password = os.environ.get("POSTGRES_PASSWORD", "password")
    db = os.environ.get("POSTGRES_DB", "postgres")
    return f"postgresql+psycopg://{user}:{password}@{host}:{port}/{db}"
