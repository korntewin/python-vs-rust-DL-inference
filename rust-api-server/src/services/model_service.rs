/*
Burn implementation of model service
*/
use burn::nn::{Linear, LinearConfig, Relu, Sigmoid};
use burn::prelude::*;
use burn::record::{FullPrecisionSettings, Recorder};
use burn_import::safetensors::{LoadArgs, SafetensorsFileRecorder};

#[derive(Module, Debug)]
pub struct RankNet<B: Backend> {
    linear1: Linear<B>,
    linear2: Linear<B>,
    relu: Relu,
    linear3: Linear<B>,
    sigmoid: Sigmoid,
}

impl<B: Backend> RankNet<B> {
    pub fn init(device: &B::Device) -> Self {
        Self {
            linear1: LinearConfig::new(40, 256).with_bias(true).init(device),
            linear2: LinearConfig::new(256, 128).with_bias(true).init(device),
            relu: Relu::new(),
            linear3: LinearConfig::new(128, 1).with_bias(true).init(device),
            sigmoid: Sigmoid::new(),
        }
    }

    pub fn forward(&self, input: Tensor<B, 2>) -> Tensor<B, 2> {
        let x = self.linear1.forward(input);
        let x = self.linear2.forward(x);
        let x = self.relu.forward(x);
        let x = self.linear3.forward(x);
        self.sigmoid.forward(x)
    }
}

#[derive(Clone)]
pub struct ModelService<B: Backend> {
    model: RankNet<B>,
    device: B::Device,
}

impl<B: Backend> ModelService<B> {
    pub fn new(model_path: &str) -> Self {
        let device = Default::default();
        let load_args = LoadArgs::new(model_path.into())
            .with_key_remap(r"^model.0.(.*)$", "linear1.$1")
            .with_key_remap(r"^model.1.(.*)$", "linear2.$1")
            .with_key_remap(r"^model.3.(.*)$", "linear3.$1");
        let record = SafetensorsFileRecorder::<FullPrecisionSettings>::default()
            .load(load_args, &device)
            .expect("Failed to load model");
        let model = RankNet::<B>::init(&device).load_record(record);
        Self { model, device }
    }

    pub fn transform_feature(
        &self,
        feature_1: &[f32],
        feature_2: &[Vec<f32>],
    ) -> Option<Tensor<B, 2>> {
        let n = feature_2.len();
        let u_dim = feature_1.len();
        let r_dim = if n > 0 { feature_2[0].len() } else { 0 };

        if n == 0 || u_dim == 0 || r_dim == 0 {
            return None;
        }

        // Flatten the user features and repeat for each item in feature_2
        let mut combined_data: Vec<f32> = Vec::new();
        let rows = feature_2.len();
        let cols = feature_1.len() + feature_2[0].len();

        for item in feature_2.iter() {
            combined_data.extend_from_slice(feature_1);
            combined_data.extend_from_slice(item);
        }

        // Create tensor from flat data and reshape it to 2D
        let tensor_data = TensorData::new(combined_data, vec![rows, cols]);
        let x = Tensor::from_data(tensor_data, &self.device);
        Some(x)
    }

    pub fn predict(&self, x: &Tensor<B, 2>) -> Tensor<B, 2> {
        self.model.forward(x.clone())
    }
}
