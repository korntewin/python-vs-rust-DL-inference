use crate::api::dtos::{Output, Request, Response};
use crate::services::adapters::GetFeaturePort;
use crate::services::model_service::ModelService;
use anyhow::Result;
use burn::backend::Candle;
use log::debug;
use std::sync::Arc;

#[derive(Clone)]
pub struct FeatureService {
    feature_adapter: Arc<dyn GetFeaturePort>,
    model_service: ModelService<Candle>,
}

impl FeatureService {
    pub fn new(
        feature_adapter: Arc<dyn GetFeaturePort>,
        model_service: ModelService<Candle>,
    ) -> Self {
        Self {
            feature_adapter,
            model_service,
        }
    }

    pub async fn process_feature(&self, feature_1_id: i64, request: Request) -> Result<Response> {
        let feature_1 = self.feature_adapter.query_feature_1(feature_1_id).await?;
        let (feature_2_ids, feature_2, displacements) = self
            .feature_adapter
            .query_feature_2(
                request.feature_2_ids,
                request.latitude,
                request.longitude,
                request.max_dist,
            )
            .await?;
        let x = self.model_service.transform_feature(&feature_1, &feature_2);

        if x.is_none() {
            return Ok(Response { outputs: vec![] });
        }

        let x = x.unwrap();

        debug!("x shape: {:?}", x.shape());
        let y_pred = self.model_service.predict(&x);
        let y_pred: Vec<f32> = y_pred
            .to_data()
            .to_vec()
            .map_err(|e| anyhow::anyhow!("Failed to convert tensor to vector: {:?}", e))?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::services::adapters::GetFeatureFromSQLAdapter;

    #[tokio::test]
    async fn test_recommend() -> Result<()> {
        let config = Config::new();
        let feature_adapter = GetFeatureFromSQLAdapter::new(config.clone()).await?;
        let model_service = ModelService::new(&"../data/model.safetensors".to_string());
        let feature_service = FeatureService::new(Arc::new(feature_adapter), model_service);
        let request = Request {
            feature_2_ids: vec![45751],
            latitude: 13.632991200188561,
            longitude: 100.76561620014209,
            size: 300,
            max_dist: 6000,
            sort_dist: false,
        };
        let response = feature_service.process_feature(785628, request).await?;
        println!("response: {:?}", response);
        Ok(())
    }
}
