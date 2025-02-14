use crate::core::*;

///
/// An effect that simulates fog, ie. the entire screen gets hazy white when objects are far away.
///
pub struct FogEffect {
    pub color: Vec3,
    pub density: f32,
    pub animation: f32,
    image_effect: ImageEffect,
}

impl FogEffect {
    pub fn new(gl: &Context) -> ThreeDResult<FogEffect> {
        Ok(FogEffect {
            color: vec3(0.8, 0.8, 0.8),
            density: 0.2,
            animation: 0.1,
            image_effect: ImageEffect::new(gl, include_str!("shaders/fog.frag"))?,
        })
    }

    pub fn apply(
        &self,
        camera: &Camera,
        depth_texture: &DepthTargetTexture2D,
        time: f32,
    ) -> ThreeDResult<()> {
        let render_states = RenderStates {
            write_mask: WriteMask::COLOR,
            blend: Blend::TRANSPARENCY,
            cull: Cull::Back,
            ..Default::default()
        };

        self.image_effect.use_texture("depthMap", depth_texture)?;
        self.image_effect.use_uniform(
            "viewProjectionInverse",
            (camera.projection() * camera.view()).invert().unwrap(),
        )?;
        self.image_effect.use_uniform("fogColor", self.color)?;
        self.image_effect.use_uniform("fogDensity", self.density)?;
        self.image_effect.use_uniform("animation", self.animation)?;
        self.image_effect.use_uniform("time", 0.001 * time)?;
        self.image_effect
            .use_uniform("eyePosition", camera.position())?;

        self.image_effect.apply(render_states, camera.viewport())?;
        Ok(())
    }
}
