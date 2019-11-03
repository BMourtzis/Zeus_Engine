pub mod render;

#[macro_use]
extern crate log;

// use render::ZeusEngine;
// use render::ProgramProc;

fn main() {
    // let mut program_proc: ProgramProc = Default::default();
    // let mut app = ZeusEngine::new(&program_proc.events_loop);
    // program_proc.main_loop(&mut app);
    
    render::render();
}
