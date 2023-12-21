use crate::impl_Interpolate;
use nalgebra::{Quaternion, Vector1, Vector2, Vector3, Vector4, Vector5, Vector6};
use num_traits::Float;

impl_Interpolate!(f32, Vector1<f32>, core::f32::consts::PI);
impl_Interpolate!(f32, Vector2<f32>, core::f32::consts::PI);
impl_Interpolate!(f32, Vector3<f32>, core::f32::consts::PI);
impl_Interpolate!(f32, Vector4<f32>, core::f32::consts::PI);
impl_Interpolate!(f32, Vector5<f32>, core::f32::consts::PI);
impl_Interpolate!(f32, Vector6<f32>, core::f32::consts::PI);
impl_Interpolate!(f32, Quaternion<f32>, core::f32::consts::PI);

impl_Interpolate!(f64, Vector1<f64>, core::f64::consts::PI);
impl_Interpolate!(f64, Vector2<f64>, core::f64::consts::PI);
impl_Interpolate!(f64, Vector3<f64>, core::f64::consts::PI);
impl_Interpolate!(f64, Vector4<f64>, core::f64::consts::PI);
impl_Interpolate!(f64, Vector5<f64>, core::f64::consts::PI);
impl_Interpolate!(f64, Vector6<f64>, core::f64::consts::PI);
impl_Interpolate!(f64, Quaternion<f64>, core::f64::consts::PI);
