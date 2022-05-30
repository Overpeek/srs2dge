use srs2dge::prelude::*;

//

struct App {
    target: Target,
}

//

impl App {
    async fn init(target: &EventLoopTarget) -> Self {
        let engine = Engine::new();
        let target = engine.new_target_default(target).await.unwrap();
        Self { target }
    }
}

impl Runnable for App {
    fn event(&mut self, _: Event, _: &EventLoopTarget, _: &mut ControlFlow) {}

    fn draw(&mut self) {
        let mut frame = self.target.get_frame();
        frame.primary_render_pass();
        self.target.finish_frame(frame);
    }
}

//

main_app!(async App);
