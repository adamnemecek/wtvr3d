//! System for registering lights before rendering

use crate::component::{Light,Direction,Transform,Cone,Enabled};
use nalgebra::{Matrix4,Vector3};
use specs::{System,Write,ReadStorage, Join,Entities};

/// Resource for sharing light information between the light system and the rendering system
#[derive(Default)]
pub struct LightRepository {
    pub ambiant : Option<Light>,
    pub directional : Vec<(Light,Direction)>,
    pub point : Vec<(Light,Matrix4<f32>)>,
    pub spot : Vec<(Light,Matrix4<f32>,Direction,Cone)>,
}

pub struct LightingSystem;

impl<'a> System<'a> for LightingSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Light>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Direction>,
        ReadStorage<'a, Cone>,
        ReadStorage<'a, Enabled>,
        Write<'a, LightRepository>,
    );
    fn run(&mut self, (entities, lights, transforms, directions, cones, enableds, mut light_repository): Self::SystemData) {
        light_repository.ambiant = None;
        light_repository.directional.clear();
        light_repository.point.clear();
        light_repository.spot.clear();
        let mut ambiant = Light { color : Vector3::new(0.0,0.0,0.0), intensity : 0.0, attenuation : 0.0};
        let mut some_ambiant = false;
        for (entity,light, _) in (&entities, &lights, &enableds).join() {
            let direction_opt = directions.get(entity);
            let transform_opt = transforms.get(entity);
            let cone_opt = cones.get(entity);
            if let (Some(direction),None) = (direction_opt,cone_opt) {
                light_repository.directional.push((light.clone(),direction.clone()));
            }
            else if let (Some(transform),None,None) = (transform_opt,cone_opt,direction_opt){
                light_repository.point.push((light.clone(),transform.get_world_matrix()));
            }
            else if let (Some(direction),Some(cone),Some(transform)) = (direction_opt,cone_opt,transform_opt){
                light_repository.spot.push((light.clone(),transform.get_world_matrix(),direction.clone(),cone.clone()));
            }
            else if let (None,None,None) = (transform_opt,cone_opt,direction_opt) {
                some_ambiant = true;
                ambiant.color = ambiant.color * ambiant.intensity + light.color * light.intensity;
                ambiant.intensity = ambiant.intensity + light.intensity;
            }
        }
        if some_ambiant {
            light_repository.ambiant = Some(ambiant);
        }
    }
}