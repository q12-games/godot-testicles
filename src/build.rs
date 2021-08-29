use std::env;
use std::fs;

fn get_runner_script(lib: &str) -> String {
  return format!(
    r#"
extends Node

var gdn

func _ready():
  gdn = GDNative.new()
  var status = false

  gdn.library = load('{}')

  var args = OS.get_cmdline_args()
  var test_pattern
  if args.size() > 1:
      test_pattern = args[1]

  if gdn.initialize():
      status = gdn.call_native('standard_varcall', 'run_tests', [self, test_pattern])

      gdn.terminate()
  else:
      print('ERR: Could not load the gdnative library')

  if status:
      print('SUCC: Test ran successfully')
  else:
      print('FAIL: Tests failed')
      OS.exit_code = 1

  get_tree().quit()
"#,
    lib = lib
  );
}

fn get_main(runner: &str) -> String {
  return format!(
    r#"[gd_scene load_steps=2 format=2]

[ext_resource path="{runner}" type="Script" id=1]

[node name="Node" type="Node" index="0"]

script = ExtResource( 1 )"#,
    runner = runner,
  );
}

fn get_run_test_script(config: &Config, scene: &str) -> String {
  return format!(
    r#"
#!/usr/bin/env bash
# NOTE: DO NOT include this in version control

{godot} --path . {scene} "$@";
"#,
    godot = config.godot_cmd,
    scene = scene,
  );
}

pub struct Config<'a> {
  pub godot_cmd: &'a str,
  pub test_script_path: &'a str,
  pub gdnlib_path: &'a str,
}

impl Config<'_> {
  pub fn default() -> Self {
    Self {
      godot_cmd: "godot",
      test_script_path: "./run-tests.sh",
      gdnlib_path: "res://test/lib.gdnlib",
    }
  }
}

pub fn setup(config: Config<'_>) -> std::io::Result<()> {
  let out_dir = env::var("OUT_DIR").expect("Env OUT_DIR not set");

  // Create runner gdscript file
  let runner_path = format!("{}/{}", out_dir, "test-runner.gd");
  let runner_script = get_runner_script(config.gdnlib_path);
  fs::write(&runner_path, runner_script)?;

  // Create main node file
  let main_scene_path = format!("{}/{}", out_dir, "Main.tscn");
  let main_scene = get_main(&runner_path);
  fs::write(&main_scene_path, main_scene)?;

  // Create run-tests.sh file
  let test_script = get_run_test_script(&config, &main_scene_path);
  fs::write(&config.test_script_path, test_script)?;

  Ok(())
}
