from functools import cache

from src.api.rest.config import config
from src.services.adapters.feature import GetFeatureFromSQLAdapter
from src.services.config import config as service_config
from src.services.feature_service import FeatureService
from src.services.model_service import get_model_service


@cache
def bootstrap():
    feature_adapter = GetFeatureFromSQLAdapter(
        f"{config.POSTGRES_DIALECT}://{config.POSTGRES_USER}:{config.POSTGRES_PASSWORD}"
        f"@{config.POSTGRES_HOST}:{config.POSTGRES_PORT}/{config.POSTGRES_DB}"
    )

    model_service = get_model_service(service_config.MODEL_PATH)
    recommendation_service = FeatureService(
        feature_adapter,
        model_service,
    )
    return recommendation_service
