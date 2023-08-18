use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::window::WindowBuilder;

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let mut input_helper = winit_input_helper::WinitInputHelper::new();
    let _window = WindowBuilder::new().build(&event_loop).unwrap();
    event_loop.run(move |event, _, control_flow|
        if input_helper.update(&event) && input_helper.close_requested() {
            *control_flow = ControlFlow::Exit
        }
    )
}
