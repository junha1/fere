use fere_common::*;
use fere_resources::{surface, Mesh, Texture};
use std::sync::Arc;

pub type ChamberIndex = u32;

/// `RenderOp` variants that require this aren't supposed to be created by users.
///
/// DO NOT attempt to create this by yourself.
pub struct InternalOp {
    _creation_barrier: (),
}

impl InternalOp {
    pub fn do_not_call_this() -> Self {
        Self {
            _creation_barrier: (),
        }
    }
}

#[derive(Debug)]
pub struct Object {
    /// The mesh to render.
    pub mesh: Arc<Mesh>,

    /// Whether to count in the shadow phase or not.
    pub shadow: bool,

    /// Whether to count in the irradiance volume phase or not.
    pub irradiance_volume: bool,

    /// The model transformation.
    pub trans: Mat4,

    /// The index of the chamber this object belongs to
    pub chamber_index: ChamberIndex,
}

#[derive(Debug)]
pub struct DrawGeneral {
    /// The object
    pub object: Object,

    /// The surface to apply on the mesh.
    pub surface: surface::GeneralI,
}
impl From<DrawGeneral> for RenderOp {
    fn from(x: DrawGeneral) -> Self {
        Self::DrawGeneral(x)
    }
}

#[derive(Debug)]
pub struct DrawEmissiveStatic {
    /// The object
    pub object: Object,

    /// The surface to apply on the mesh.
    pub surface: surface::EmissiveStaticI,

    /// Whether to put additional point light to approximate the emission. Use (0, 1].
    ///
    /// It uses the average position and color for the given object and surface.
    pub point_light: Option<f32>,
}
impl From<DrawEmissiveStatic> for RenderOp {
    fn from(x: DrawEmissiveStatic) -> Self {
        Self::DrawEmissiveStatic(x)
    }
}

#[derive(Debug)]
pub struct DrawEmissiveDynamic {
    /// The object
    pub object: Object,

    /// The material sources used to represent the surface
    pub materials: surface::EmissiveMaterialI,

    /// The surface to apply on the mesh.
    pub surface: surface::EmissiveDynamic,
}
impl From<DrawEmissiveDynamic> for RenderOp {
    fn from(x: DrawEmissiveDynamic) -> Self {
        Self::DrawEmissiveDynamic(x)
    }
}

#[derive(Debug)]
pub struct DrawLine {
    pub pos1: Vec3,
    pub pos2: Vec3,
    pub color: IVec4,
    pub width: f32,
}
impl From<DrawLine> for RenderOp {
    fn from(x: DrawLine) -> Self {
        Self::DrawLine(x)
    }
}

#[derive(Debug)]
pub struct DrawWireFrame {
    /// The mesh to render.
    pub mesh: Arc<Mesh>,

    /// The model transformation.
    pub trans: Mat4,

    pub color: IVec4,
    pub width: f32,
}
impl From<DrawWireFrame> for RenderOp {
    fn from(x: DrawWireFrame) -> Self {
        Self::DrawWireFrame(x)
    }
}

#[derive(Debug)]
/// A point light that involves shadows and some additional effects.
pub struct AddMajorLight {
    /// The position of the light.
    pub pos: Vec3,

    /// The color of the light.
    pub color: Vec3,

    /// The X axis in camera space
    pub xdir: Vec3,

    /// The Y axis in camera space
    pub ydir: Vec3,

    /// Camera perspective in radian.
    pub perspective: f32,

    /// The index of the chamber this light belongs to
    pub chamber_index: ChamberIndex,
}
impl From<AddMajorLight> for RenderOp {
    fn from(x: AddMajorLight) -> Self {
        Self::AddMajorLight(x)
    }
}

#[derive(Debug)]
/// A major light which is omni-directional
pub struct MajorLightOmni {
    /// The position of the light.
    pub pos: Vec3,

    /// The color of the light.
    pub color: Vec3,

    /// The index of the chamber this light belongs to
    pub chamber_index: ChamberIndex,
}
impl From<MajorLightOmni> for RenderOp {
    fn from(x: MajorLightOmni) -> Self {
        RenderOp::Multiple(
            (0..6)
                .map(|i| {
                    let (xdir, ydir) = fere_common::geo::six_sides_dir(i);
                    AddMajorLight {
                        pos: x.pos,
                        color: x.color,
                        xdir,
                        ydir,
                        perspective: (90.0_f32).to_radians(),
                        chamber_index: x.chamber_index,
                    }
                    .into()
                })
                .collect(),
        )
    }
}

#[derive(Debug)]
/// A plain poing light
pub struct AddPointLight {
    /// The position of the light.
    pub pos: Vec3,

    /// The color of the light.
    pub color: Vec3,

    /// The index of the chamber this light belongs to
    pub chamber_index: ChamberIndex,
}
impl From<AddPointLight> for RenderOp {
    fn from(x: AddPointLight) -> Self {
        Self::AddPointLight(x)
    }
}

#[derive(Debug)]
pub struct AddAmbientLight {
    /// The color of the light.
    pub color: Vec3,

    /// Enabling omni-lighting.
    pub omni: bool,

    /// The index of the chamber this light belongs to
    pub chamber_index: ChamberIndex,
}
impl From<AddAmbientLight> for RenderOp {
    fn from(x: AddAmbientLight) -> Self {
        Self::AddAmbientLight(x)
    }
}

/// Shades a chamber with irradiance volume. Use only once for a chamber.
#[derive(Debug)]
pub struct ShadeWithIv {
    /// The index of the chamber to apply irradiance volume.
    pub chamber_index: ChamberIndex,

    /// A weight to control the intensity of illumination. Use [0, 1].
    pub weight: f32,
}
impl From<ShadeWithIv> for RenderOp {
    fn from(x: ShadeWithIv) -> Self {
        Self::ShadeWithIv(x)
    }
}

#[derive(Debug)]
pub struct DrawImage {
    pub texture: Arc<Texture>,

    /// Of the center of the image, from the center of the screen as (0, 0).
    pub pos: Vec2,
    /// In scale.
    pub size: Vec2,
    pub rotation: f32,

    pub blend_mode: (),
    pub color: Vec4,
}
impl From<DrawImage> for RenderOp {
    fn from(x: DrawImage) -> Self {
        Self::DrawImage(x)
    }
}

#[derive(Debug)]
pub struct DrawBillboard {
    pub texture: Arc<Texture>,

    pub depth_test: bool,
    pub depth_write: bool,

    /// Of the center of the image, from the center of the screen as (0, 0).
    pub pos: Vec3,
    /// In scale.
    pub size: Vec2,
    pub rotation: f32,
    pub blend_mode: (),
    pub color: Vec4,
}
impl From<DrawBillboard> for RenderOp {
    fn from(x: DrawBillboard) -> Self {
        Self::DrawBillboard(x)
    }
}

#[derive(Debug)]
pub struct VisualizeProbes {
    pub chamber_index: ChamberIndex,
}
impl From<VisualizeProbes> for RenderOp {
    fn from(x: VisualizeProbes) -> Self {
        Self::VisualizeProbes(x)
    }
}

#[derive(Debug)]
pub struct ShowInternalTexture {
    pub name: String,

    /// Of the left-bottom of the image, from the left-bottom of the screen as (0, 0).
    pub pos: Vec2,

    /// In scale.
    pub size: Vec2,
}
impl From<ShowInternalTexture> for RenderOp {
    fn from(x: ShowInternalTexture) -> Self {
        Self::ShowInternalTexture(x)
    }
}

pub enum RenderOp {
    // Internal opertions controlled by the frame
    StartFrame(InternalOp),
    Abort(InternalOp),
    EndFrame(InternalOp),

    // Special operations to configure the chamber
    SetCamera(SetCamera),

    // Draw various objects
    DrawLine(DrawLine),
    DrawWireFrame(DrawWireFrame),
    DrawGeneral(DrawGeneral),
    DrawEmissiveStatic(DrawEmissiveStatic),
    DrawEmissiveDynamic(DrawEmissiveDynamic),

    // Add various lights
    AddMajorLight(AddMajorLight),
    AddPointLight(AddPointLight),
    AddAmbientLight(AddAmbientLight),

    // Perform global illumination
    ShadeWithIv(ShadeWithIv),

    // 2D Renderings
    DrawImage(DrawImage),
    DrawBillboard(DrawBillboard),

    // Various debugging tools
    VisualizeProbes(VisualizeProbes),
    ShowInternalTexture(ShowInternalTexture),

    // Meta operations
    Multiple(Vec<RenderOp>),
}
impl From<SetCamera> for RenderOp {
    fn from(x: SetCamera) -> Self {
        Self::SetCamera(x)
    }
}
