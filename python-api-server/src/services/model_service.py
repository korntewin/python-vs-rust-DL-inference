from abc import ABC, abstractmethod

import numpy as np
import onnx
import onnxruntime
import torch


class ModelService(ABC):
    @staticmethod
    def transform_feature(
        feature_1: list[float], feature_2: list[list[float]]
    ) -> torch.Tensor:
        return torch.tensor(
            np.hstack(
                (
                    np.tile(feature_1, (len(feature_2), 1)),
                    feature_2,
                )
            ),
            dtype=torch.float32,
        )

    @abstractmethod
    def predict(self, x: torch.Tensor) -> torch.Tensor:
        pass


def get_model_service(model_path: str) -> ModelService:
    if model_path.endswith(".pt"):
        return PyTorchModelService(model_path)
    elif model_path.endswith(".onnx"):
        return ONNXModelService(model_path)
    else:
        raise ValueError(f"Invalid model service: {model_path}")


class PyTorchModelService(ModelService):
    def __init__(self, model_path: str):
        self.model_path = model_path
        self.model: torch.jit.ScriptModule = torch.jit.load(self.model_path)
        self.model.eval()

    def predict(self, x: torch.Tensor) -> torch.Tensor:
        with torch.no_grad():
            output = self.model(x)
            output = torch.sigmoid(output)
        return output


class ONNXModelService(ModelService):
    def __init__(self, model_path: str):
        self.model_path = model_path
        self.model = onnx.load(self.model_path)
        self.ort_session = onnxruntime.InferenceSession(
            self.model_path,
        )

    def predict(self, x: torch.Tensor) -> torch.Tensor:
        output = torch.from_numpy(self.ort_session.run(None, {"input": x.numpy()})[0])
        output = torch.sigmoid(output)
        return output
