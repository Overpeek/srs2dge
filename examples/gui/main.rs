use srs2dge::prelude::*;

//

struct App {
    target: Target,

    ubo: UniformBuffer<Mat4>,
    shader: Colored2DShader,

    gui: Gui,
}

//

impl App {
    async fn init(target: &EventLoopTarget) -> Self {
        let engine = Engine::new();
        let target = engine.new_target_default(target).await.unwrap();

        let ubo = UniformBuffer::new(&target, 1);
        let shader = Colored2DShader::new(&target);

        let mut gui = Gui::new(&target);
        gui.generate(target, frame);
        gui.add(Empty::new());

        Self {
            target,
            ubo,
            shader,
            gui,
        }
    }
}

impl Runnable for App {
    fn event(&mut self, event: Event, _: &EventLoopTarget, _: &mut ControlFlow) {
        self.gui.event(&event);
    }

    fn draw(&mut self) {
        let mut frame = self.target.get_frame();
        let (vbo, ibo, i) = self.gui.generate(&mut self.target, &mut frame);
        let unit = self.shader.bind_unit(&self.ubo);
        frame
            .primary_render_pass()
            .bind_vbo(vbo)
            .bind_ibo(ibo)
            .bind_unit(&unit)
            .draw_indexed(0..i, 0, 0..1);
        self.target.finish_frame(frame);
    }
}

//

main_app!(async App);
