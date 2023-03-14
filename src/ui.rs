use egui::Context;
use egui_d3d11::DirectX11App;

pub static mut APP: DirectX11App<i32> = DirectX11App::new();

fn ui(ctx: &Context, i: &mut i32) {
    // ...
}
