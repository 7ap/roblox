use egui::*;
use egui_d3d11::DirectX11App;
use egui_notify::Toasts;

pub static mut APP: DirectX11App<i32> = DirectX11App::new();
pub static mut TOASTS: Toasts = Toasts::new();

pub fn ui(ctx: &Context, _: &mut i32) {
    Window::new("Settings").show(ctx, |ui| {
        ctx.settings_ui(ui);
    });

    unsafe {
        TOASTS.show(ctx);
    }
}
