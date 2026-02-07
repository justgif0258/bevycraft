use bevy::prelude::*;
use bevycraft_app::prelude::*;
use bevycraft_core::prelude::*;
use bevycraft_world::prelude::*;

fn main() {
    let mut packed = PackedArrayU32::with_bit_length(4096, 12);

    println!("{:?}", packed);

    /*
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GameRegistries::default())
        .add_systems(Startup, init)
        .run();
    */
}

fn init(root: Res<GameRegistries>) {
    let result = root.get_registered::<Block>(&ResourceId::parse("cobblestone").unwrap());

    println!("{:#?}", result);
}
