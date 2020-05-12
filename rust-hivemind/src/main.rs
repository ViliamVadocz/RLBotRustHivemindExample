use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    rlbot::run_hive(
        MyHivemind {
            drone_indices: vec![]
        }
    )
}

struct MyHivemind {
    drone_indices: Vec<usize>
}

impl rlbot::Hivemind for MyHivemind {
    fn set_drone_indices(&mut self, indices: Vec<usize>) {
        self.drone_indices = indices;
    }

    fn tick(&mut self, packet: &rlbot::GameTickPacket) -> Vec<(usize, rlbot::ControllerState)> {
        println!("Running!")

        let mut inputs: Vec<(usize, rlbot::ControllerState)> = vec![];
        for &index in self.drone_indices.iter() {
            inputs.push(
                (
                    index,
                    rlbot::ControllerState {
                        throttle: 1.0,
                        steer: 1.0,
                        ..Default::default()
                    }
                )
            );
        }
        
        inputs
    }
}