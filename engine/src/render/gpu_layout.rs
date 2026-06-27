use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct StarGpu {
    pub position_mag: [f32; 4],
    pub color_flags: [u32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct StarGpuCompact {
    pub pos_xy: u32,
    pub pos_z_mag: u32,
    pub color_flags: u32,
    pub extra: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star_gpu_layout_is_32_bytes() {
        assert_eq!(std::mem::size_of::<StarGpu>(), 32);
        assert_eq!(std::mem::align_of::<StarGpu>(), 4);
    }

    #[test]
    fn star_gpu_compact_layout_is_16_bytes() {
        assert_eq!(std::mem::size_of::<StarGpuCompact>(), 16);
        assert_eq!(std::mem::align_of::<StarGpuCompact>(), 4);
    }
}
