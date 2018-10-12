use glm;


pub struct Camera {
    pub camera: glm::TMat4<f32>,
    pub camera_pos: glm::TVec3<f32>,
    pub camera_front: glm::TVec3<f32>,
    pub camera_up: glm::TVec3<f32>,

}

impl Camera {
    pub fn new() -> Self{

        let camera_pos = glm::vec3(0.0, 0.0, 3.0);
        let camera_front = glm::vec3(0.0, 0.0,-1.0);
        let camera_up = glm::vec3(0.0, 1.0, 0.0);
        let camera = glm::look_at_rh(
            &camera_pos, //eye
            &(camera_pos + camera_front), //center
            &camera_up //up
        );

        Self {
            camera,
            camera_pos,
            camera_front,
            camera_up,
        }

    }

    pub fn update(&mut self) {
        self.camera = glm::look_at_rh(
            &self.camera_pos, //eye
            &(self.camera_pos + self.camera_front), //center
            &self.camera_up //up
        );
    }
    
}