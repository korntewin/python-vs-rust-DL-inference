use crate::api::dtos::{Request, Response, Output};
use crate::services::adapters::GetFeaturePort;
use crate::services::model_service::ModelService;
use anyhow::Result;
use candle_core::Tensor;
use log::debug;
use std::sync::Arc;

#[derive(Clone)]
pub struct FeatureService {
    feature_adapter: Arc<dyn GetFeaturePort>,
    model_service: ModelService,
}

impl FeatureService {
    pub fn new(feature_adapter: Arc<dyn GetFeaturePort>, model_service: ModelService) -> Self {
        Self {
            feature_adapter,
            model_service,
        }
    }

    pub async fn process_feature(
        &self,
        feature_1_id: i64,
        request: Request,
    ) -> Result<Response> {
        let feature_1 = self.feature_adapter.query_feature_1(feature_1_id).await?;
        let (feature_2_ids, feature_2, displacements) = self.feature_adapter
            .query_feature_2(
                request.feature_2_ids,
                request.latitude,
                request.longitude,
                request.max_dist,
            )
            .await?;
        let x = self
            .model_service
            .transform_feature(&feature_1, &feature_2)?;

        if x.shape().dim(0).unwrap() == 0 {
            return Ok(Response {
                outputs: vec![],
            });
        }

        debug!("x shape: {:?}", x.shape());
        let y_pred: Tensor = self.model_service.predict(&x)?;
        let y_pred: Vec<f32> = y_pred.squeeze(1)?.to_vec1::<f32>()?;
        debug!("y_pred: {:?}", y_pred);

        let mut outputs = Vec::new();
        for (i, feature_2_id) in feature_2_ids.iter().enumerate() {
            let displacement = displacements[i];
            let score = y_pred[i] as f64;
            let output = Output {
                id: *feature_2_id,
                score,
                displacement,
            };
            outputs.push(output);
        }

        // sort outputs by displacement or score
        if request.sort_dist {
            outputs.sort_by(|a, b| a.displacement.cmp(&b.displacement));
        } else {
            outputs.sort_by(|a, b| b.score.total_cmp(&a.score));
        }

        // select top n outputs
        outputs = outputs.into_iter().take(request.size as usize).collect();

        let response = Response { outputs };
        Ok(response)
    }
}
