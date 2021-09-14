use godot_testicles::build::*;

fn main() -> std::io::Result<()> {
  setup(Config {
    godot_cmd: "godot-headless",
    gdnlib_path: "res://test/game-test.gdnlib",
    ..Config::default()
  })
}
