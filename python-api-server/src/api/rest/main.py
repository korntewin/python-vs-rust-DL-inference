from contextlib import asynccontextmanager

from fastapi import FastAPI, Header, HTTPException, Security
from fastapi.middleware.gzip import GZipMiddleware

from src.api.rest.bootstrap import bootstrap
from src.api.rest.config import config
from src.api.rest.routers import feature


@asynccontextmanager
async def lifespan(app: FastAPI):
    print("Starting, warming up model...")
    _ = bootstrap()
    print("Model warmed up")
    yield
    print("Shutting down, flushed all user events...")


# Add Gzipmiddleware to conditionally compress responses
# to reduce network bandwidth
app = FastAPI(lifespan=lifespan)
app.add_middleware(GZipMiddleware)


def api_key_auth(x_api_key: str = Header()):
    if x_api_key != config.API_KEY.get_secret_value():
        print(f"Invalid API key: {x_api_key}")
        raise HTTPException(status_code=401, detail="Invalid API key")
    return x_api_key


app.include_router(feature.router, dependencies=[Security(api_key_auth)])


@app.get("/health")
@app.get("/healthz")
def health_check():
    return {"message": "OK"}
