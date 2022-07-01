use crate::{messages::FtoC, GBC};

pub fn draw(gbc: &GBC, draw_ctx: &imgui::DrawContext) {
    let ui = draw_ctx.ui();
    imgui::gui::Window::new("Control")
        .always_auto_resize(true)
        .build(ui, || {
            let paused = gbc.state.ctrl.paused;
            if paused {
                if ui.button("resume") {
                    gbc.msgq.send(FtoC::SetPausedState(false));
                }
            } else {
                if ui.button("pause") {
                    gbc.msgq.send(FtoC::SetPausedState(true));
                }
            }
            ui.enabled(paused, || {
                if ui.button("step") {
                    gbc.msgq.send(FtoC::Step(1));
                }
                ui.same_line();
                if ui.button("step over") {
                    gbc.msgq.send(FtoC::StepOver);
                }
            });
        });
}
