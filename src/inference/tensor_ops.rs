use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Basic tensor representation for inference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tensor {
    pub shape: Vec<usize>,
    pub data: Vec<f32>,
    pub dtype: DataType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    Float32,
    Float16,
    Int32,
    Int64,
}

impl Tensor {
    pub fn new(shape: Vec<usize>, data: Vec<f32>) -> Self {
        Self {
            shape,
            data,
            dtype: DataType::Float32,
        }
    }

    pub fn zeros(shape: Vec<usize>) -> Self {
        let size: usize = shape.iter().product();
        Self {
            shape,
            data: vec![0.0; size],
            dtype: DataType::Float32,
        }
    }

    pub fn ones(shape: Vec<usize>) -> Self {
        let size: usize = shape.iter().product();
        Self {
            shape,
            data: vec![1.0; size],
            dtype: DataType::Float32,
        }
    }

    pub fn random(shape: Vec<usize>) -> Self {
        let size: usize = shape.iter().product();
        let data: Vec<f32> = (0..size)
            .map(|_| rand::random::<f32>() * 2.0 - 1.0)
            .collect();
        Self {
            shape,
            data,
            dtype: DataType::Float32,
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn reshape(&mut self, new_shape: Vec<usize>) -> Result<(), String> {
        let new_size: usize = new_shape.iter().product();
        if new_size != self.data.len() {
            return Err("Cannot reshape tensor: size mismatch".to_string());
        }
        self.shape = new_shape;
        Ok(())
    }
}

/// Basic tensor operations
pub trait TensorOps {
    fn add(&self, other: &Tensor) -> Result<Tensor, String>;
    fn multiply(&self, other: &Tensor) -> Result<Tensor, String>;
    fn matmul(&self, other: &Tensor) -> Result<Tensor, String>;
    fn relu(&self) -> Tensor;
    fn softmax(&self) -> Tensor;
    fn transpose(&self) -> Tensor;
}

impl TensorOps for Tensor {
    fn add(&self, other: &Tensor) -> Result<Tensor, String> {
        if self.shape != other.shape {
            return Err("Shape mismatch for addition".to_string());
        }
        
        let data: Vec<f32> = self.data.iter()
            .zip(other.data.iter())
            .map(|(a, b)| a + b)
            .collect();
            
        Ok(Tensor::new(self.shape.clone(), data))
    }

    fn multiply(&self, other: &Tensor) -> Result<Tensor, String> {
        if self.shape != other.shape {
            return Err("Shape mismatch for multiplication".to_string());
        }
        
        let data: Vec<f32> = self.data.iter()
            .zip(other.data.iter())
            .map(|(a, b)| a * b)
            .collect();
            
        Ok(Tensor::new(self.shape.clone(), data))
    }

    fn matmul(&self, other: &Tensor) -> Result<Tensor, String> {
        // Simple matrix multiplication for 2D tensors
        if self.shape.len() != 2 || other.shape.len() != 2 {
            return Err("MatMul only supports 2D tensors".to_string());
        }
        
        let (m, k) = (self.shape[0], self.shape[1]);
        let (k2, n) = (other.shape[0], other.shape[1]);
        
        if k != k2 {
            return Err("Matrix dimensions don't match for multiplication".to_string());
        }
        
        let mut result = vec![0.0; m * n];
        
        for i in 0..m {
            for j in 0..n {
                for k_idx in 0..k {
                    result[i * n + j] += self.data[i * k + k_idx] * other.data[k_idx * n + j];
                }
            }
        }
        
        Ok(Tensor::new(vec![m, n], result))
    }

    fn relu(&self) -> Tensor {
        let data: Vec<f32> = self.data.iter()
            .map(|&x| x.max(0.0))
            .collect();
        Tensor::new(self.shape.clone(), data)
    }

    fn softmax(&self) -> Tensor {
        let max_val = self.data.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let exp_data: Vec<f32> = self.data.iter()
            .map(|&x| (x - max_val).exp())
            .collect();
        let sum_exp: f32 = exp_data.iter().sum();
        
        let data: Vec<f32> = exp_data.iter()
            .map(|&x| x / sum_exp)
            .collect();
            
        Tensor::new(self.shape.clone(), data)
    }

    fn transpose(&self) -> Tensor {
        if self.shape.len() != 2 {
            return self.clone(); // Return self for non-2D tensors
        }
        
        let (rows, cols) = (self.shape[0], self.shape[1]);
        let mut data = vec![0.0; self.data.len()];
        
        for i in 0..rows {
            for j in 0..cols {
                data[j * rows + i] = self.data[i * cols + j];
            }
        }
        
        Tensor::new(vec![cols, rows], data)
    }
} 