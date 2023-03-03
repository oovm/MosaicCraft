
#[derive(Debug, Copy, Clone)]
pub enum MosaicShape {
    DiskMatrix,
    DiamondMatrix,
    BoxMatrix,
    IdentityMatrix,
    CrossMatrix,
    GaussianMatrix,
    Custom(),
}
