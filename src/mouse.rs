use imgui::ImGui;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Mouse {
    pub pos: (i32, i32),
    pub pressed: (bool, bool, bool),
    pub wheel: f32,
}

impl Mouse {
    pub fn update(&mut self, imgui: &mut ImGui) {
        imgui.set_mouse_pos(self.pos.0 as f32, self.pos.1 as f32);
        imgui.set_mouse_down([
            self.pressed.0,
            self.pressed.1,
            self.pressed.2,
            false,
            false,
        ]);
        imgui.set_mouse_wheel(self.wheel);
        self.wheel = 0.0;    
    }
}