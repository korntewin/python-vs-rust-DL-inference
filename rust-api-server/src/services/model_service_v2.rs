use burn::{
    backend::LibTorch, nn::{Linear, LinearConfig, Relu, Sigmoid}, prelude::*,
    record::{FullPrecisionSettings, Record},
};
use burn_import::safetensors::SafetensorsFileRecorder;

type TorchBackend = LibTorch;

#[derive(Module, Debug)]
struct RankNet<B: Backend> {
    l1: Linear<B>,
    l2: Linear<B>,
    l2_relu: Relu,
    l3: Linear<B>,
    l3_sigmoid: Sigmoid,
}

impl<B: Backend> RankNet<B> {
    fn init(device: &B::Device) -> Self {
        Self {
            l1: LinearConfig::new(30 + 10, 256).with_bias(true).init(device),
            l2: LinearConfig::new(256, 128).with_bias(true).init(device),
            l2_relu: Relu::new(),
            l3: LinearConfig::new(128, 1).with_bias(true).init(device),
            l3_sigmoid: Sigmoid::new(),
        }
    }

    fn forward(&self, x: Tensor<B, 2>) -> Tensor<B, 1> {
        let x = self.l1.forward(x);
        let x = self.l2.forward(x);
        let x = self.l2_relu.forward(x);
        let x = self.l3.forward(x);
        let x = self.l3_sigmoid.forward(x);
        x
    }
}

#[derive(Clone)]
pub struct ModelService<B: Backend> {
    model: RankNet<B>,
    device: B::Device,
}

impl<B: Backend> ModelService<B> {
    pub fn new<B: Backend>(model_path: &str) -> Self<B> {
        let device = Default::default();

        let record = SafetensorsFileRecorder::<FullPrecisionSettings>::default()
        .load(model_path, &device)
        .expect("Failed to load model");

        Self {
            model: RankNet::<B>::init(&device).load_record(record),
            device: Default::default(),
        } 
    }

    pub fn predict(&self, x: Tensor<B, 2>) -> Tensor<B, 1> {
        self.model.forward(x)
    }
    
    pub fn transform_feature(&self, feature_1: &[f32], feature_2: &[Vec<f32>]) -> Option<Tensor<B, 2>> {
        let n = feature_2.len();
        let u_dim = feature_1.len();
        let r_dim = if n > 0 {
            feature_2[0].len()
        } else {
            0;
        };

        if n == 0 || u_dim == 0 || r_dim == 0 {
            return None;
        }
        
        let tiled_user = feature_2
            .iter()
            .map(|_| feature_1.iter().cloned())
            .collect::<Vec<Vec<f32>>>();
        let user_t = Tensor::from_floats(tiled_user, &self.device);
        
        let rest_flat = feature_2
            .iter()
            .map(|row| row.iter().cloned())
            .collect::<Vec<Vec<f32>>>();
        let rest_t = Tensor::from_floats(rest_flat, &self.device);
        
        let x = Tensor::cat(&[&user_t, &rest_t], 1);
        x
    }

    pub fn predict(&self, x: Tensor<B, 2>) -> Option<Tensor<B, 1>> {
        let x = self.model.forward(x);
        Some(x)
    }
}
