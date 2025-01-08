pub mod gpu;
pub(crate) mod draw;

use bytemuck::{Pod, Zeroable};
use pollster::FutureExt;
use wgpu::util::DeviceExt;

