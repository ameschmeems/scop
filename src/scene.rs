use math;
use math::vector::Vector3;
use gl;
use std::vec::Vec;
use crate::model;
use crate::camera::Camera;

pub struct Scene
{
    models: Vec<model::Mesh>,
    projection: math::matrix::Matrix4,
    pub camera: Camera
}

impl Scene
{
    pub fn new(models: Vec<model::Mesh>, projection: math::matrix::Matrix4, camera: Camera) -> Self
    {
        Self {
            models,
            projection,
            camera
        }
    }

    pub fn draw(&self, camera_pos: &Vector3, camera_front: &Vector3, camera_up: &Vector3)
    {
        for model in &self.models
        {
            model.render(self.camera.view());
        }
    }
}