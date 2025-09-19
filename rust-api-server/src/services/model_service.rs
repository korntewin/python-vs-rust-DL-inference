/*
Candle implementation of model service
*/
use candle_core::{DType, Device, Result, Tensor};
use candle_nn::ops::sigmoid;
use candle_nn::{Linear, Module, VarBuilder, linear};

#[derive(Clone)]
struct RankNet {
    l1: Linear,
    l2: Linear,
    l3: Linear,
}

impl RankNet {
    fn new(vb: VarBuilder) -> Result<Self> {
        let input_dim = 30 + 10;
        let vp = vb.pp("model");
        Ok(Self {
            l1: linear(input_dim, 256, vp.pp("0"))?,
            l2: linear(256, 128, vp.pp("1"))?,
            l3: linear(128, 1, vp.pp("3"))?,
        })
    }
}

impl Module for RankNet {
    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let x = self.l1.forward(&x)?;
        let x = self.l2.forward(&x)?.relu()?;
        self.l3.forward(&x)
    }
}

#[derive(Clone)]
pub struct ModelService {
    model: RankNet,
}

impl ModelService {
    pub fn new(model_path: &str) -> Result<Self> {
        let device = Device::Cpu;
        let vb =
            unsafe { VarBuilder::from_mmaped_safetensors(&[model_path], DType::F32, &device)? };
        let model = RankNet::new(vb)?;
        Ok(Self { model })
    }

    pub fn predict(&self, x: &Tensor) -> Result<Tensor> {
        let x = self.model.forward(&x)?;
        sigmoid(&x)
    }

    pub fn transform_feature(&self, feature_1: &[f32], feature_2: &[Vec<f32>]) -> Result<Tensor> {
        let n = feature_2.len();
        let u_dim = feature_1.len();
        let r_dim = if n > 0 { feature_2[0].len() } else { 0 };

        if n == 0 || u_dim == 0 || r_dim == 0 {
            return Tensor::from_vec(Vec::<f32>::new(), (0, u_dim + r_dim), &Device::Cpu);
        }

        let tiled_f1: Vec<f32> = feature_2
            .iter()
            .flat_map(|_| feature_1.iter().cloned())
            .collect();
        let f1 = Tensor::from_vec(tiled_f1, (n, u_dim), &Device::Cpu)?;

        let tiled_f2: Vec<f32> = feature_2
            .iter()
            .flat_map(|row| row.iter().cloned())
            .collect();
        let f2 = Tensor::from_vec(tiled_f2, (n, r_dim), &Device::Cpu)?;

        // hstack
        let x = Tensor::cat(&[&f1, &f2], 1)?;
        Ok(x)
    }
}
