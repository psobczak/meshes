mod terrain;

use bevy::prelude::App;
use bevy_template::GamePlugin;

fn main() {
    App::new().add_plugin(GamePlugin).run();
}
