from src.api.rest.dtos import feature as feature_dtos
from src.services.adapters.feature import GetFeaturePort
from src.services.model_service import ModelService


class FeatureService:
    def __init__(
        self,
        feature_adapter: GetFeaturePort,
        model_service: ModelService,
    ):
        self.feature_adapter = feature_adapter
        self.model_service = model_service

    def process_feature(  # noqa
        self,
        feature_1_id: int,
        request: feature_dtos.Request,
    ) -> feature_dtos.Response:
        feature_1 = self.feature_adapter.get_feature_1(feature_1_id)
        rem_ids, feature_2, displacements = self.feature_adapter.get_feature_2(
            request.feature_2_ids,
            request.latitude,
            request.longitude,
            request.max_dist,
        )

        outputs = []
        if len(feature_2) > 0:
            x = self.model_service.transform_feature(feature_1, feature_2)
            y_pred = self.model_service.predict(x)

            # output
            result = [
                {"id": rid, "score": float(prob), "displacement": float(distance_m)}
                for rid, prob, distance_m in zip(
                    rem_ids,
                    y_pred.numpy(),
                    displacements,
                )
            ]
            result.sort(
                key=lambda x: x["displacement"] if request.sort_dist else x["score"],
                reverse=False if request.sort_dist else True,
            )
            outputs = [feature_dtos.Feature2Entity(**r) for r in result[: request.size]]

        output = feature_dtos.Response(outputs=outputs)
        return output
