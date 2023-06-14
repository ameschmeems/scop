use math;
use math::vector::Vector3;
use gl;
use std::vec::Vec;
use crate::model;
use crate::camera::Camera;
use sdl2;

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

    pub fn process_input(&mut self, delta_time: f32, event: &sdl2::event::Event)
    {
        self.camera.update_camera(delta_time, event);
        for model in &mut self.models
        {
            model.update_pos(event);
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