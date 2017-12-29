extern crate glm;

use shapes::Axis;

pub struct Camera {
    up_vector : glm::Vector3<f32>,
    position_vector : glm::Vector3<f32>,
    pointing_vector : glm::Vector3<f32>,
    view_matrix : glm::Matrix4<f32>
}

impl Camera {
    pub fn new(pos : glm::Vector3<f32>, pointing : glm::Vector3<f32> ) -> Camera {
        let view = glm::ext::look_at(pointing, pos, glm::Vector3::<f32>::new(0.0, 1.0, 0.0));
        Camera{up_vector : glm::Vector3::<f32>::new(0.0, 1.0, 0.0), 
                position_vector : pos,
                pointing_vector : pos + pointing,
                view_matrix : view}
    }

    pub fn get_view_matrix(&mut self) -> glm::Matrix4<f32> {

        self.view_matrix = glm::ext::look_at(self.position_vector,
                                            self.pointing_vector,
                                            self.up_vector);

        self.view_matrix
    }

    pub fn translate(&mut self, amount : f32, axis : Axis) {
        match axis {
            Axis::XAxis => {
                self.position_vector = glm::Vector3::<f32>::new(self.pointing_vector.x + amount, 
                                                                self.position_vector.y,
                                                                self.position_vector.z);
            }
            Axis::YAxis => {
                self.position_vector = glm::Vector3::<f32>::new(self.pointing_vector.x, 
                                                                self.position_vector.y + amount,
                                                                self.position_vector.z);
            }
            Axis::ZAxis => {
                self.position_vector = glm::Vector3::<f32>::new(self.position_vector.x, 
                                                                self.position_vector.y,
                                                                self.position_vector.z + amount);
            }
        }
    }

    pub fn pan(&mut self, degrees : f32, axis : Axis) {
        let rotation_axis = match axis {
            Axis::XAxis => glm::Vector3::<f32>::new(1.0, 0.0, 0.0),
            Axis::YAxis => glm::Vector3::<f32>::new(0.0, 1.0, 0.0),
            Axis::ZAxis => glm::Vector3::<f32>::new(0.0, 0.0, 1.0)
        };

        // using the Rodrigues rotation formula
        self.pointing_vector = (self.pointing_vector * degrees.cos()) + 
                                (glm::cross(rotation_axis, self.pointing_vector) * degrees.sin()) +
                                (rotation_axis * glm::dot(rotation_axis, self.pointing_vector) * (1.0 - degrees.cos()));


    }
}
