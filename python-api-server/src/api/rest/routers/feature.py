from fastapi import Depends, routing

from src.api.rest.bootstrap import bootstrap
from src.api.rest.dtos import feature as feature_dtos
from src.services.feature_service import FeatureService

router = routing.APIRouter(prefix="/feature", tags=["feature"])


# Use normal def here to use threadpool instead since model is CPU bound.
# Using async def will block the event loop on such CPU bound task.
@router.post("/{feature_1_id}", response_model=feature_dtos.Response)
def process_feature(
    feature_1_id: int,
    request: feature_dtos.Request,
    feature_service: FeatureService = Depends(bootstrap),
):
    output = feature_service.process_feature(feature_1_id, request)
    return output
