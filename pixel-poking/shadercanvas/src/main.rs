use shadercanvas::run;

fn main() {
    pollster::block_on(run());  // do not use in browser
}