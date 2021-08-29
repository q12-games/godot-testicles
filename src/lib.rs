#[macro_use]

pub mod build;
mod expect;
pub use expect::*;

use gdnative::{
  api::{GDNativeLibrary, NativeScript, Node},
  prelude::Unique,
  GodotObject, Ref, TRef,
};

static mut ROOT_NODE: Option<Ref<Node>> = None;
pub fn get_root_node() -> Result<TRef<'static, Node>, String> {
  unsafe { ROOT_NODE.map(|n| n.assume_safe()) }.ok_or_else(|| "ERR: Unable to get root node".to_string())
}
pub fn set_root_node(node: Ref<Node>) {
  unsafe { ROOT_NODE = Some(node) };
}

pub fn process_frame(node: &Node) {
  node.notification(Node::NOTIFICATION_PROCESS, false);
  node.notification(Node::NOTIFICATION_PHYSICS_PROCESS, false);
}

pub fn process_frames(n: i32, node: &Node) {
  for _ in 0..n {
    process_frame(node);
  }
}

pub fn cleanup(root: TRef<'static, Node>) {
  while root.get_child_count() > 0 {
    let child = root.get_child(0).unwrap();
    root.remove_child(child);
    unsafe { child.assume_safe().queue_free() };
  }
}

pub fn get_script(class_name: &str) -> Ref<NativeScript, Unique> {
  let gdn = GDNativeLibrary::current_library();
  let ns = NativeScript::new();

  ns.set_class_name(class_name);
  ns.set_library(unsafe { gdn.assume_shared() });

  ns
}

#[macro_export]
macro_rules! get_path {
  () => {
    stdext::function_name!().to_string().replace("quintessence_tests::", "")
  };
}

#[macro_export]
macro_rules! run_tests {
  { $( $test:ident ; )* } => {
    $(mod $test;)*

    fn _run_tests(pattern: &Option<String>) -> godot_testicles::AssertionResult {
      $($test::run(stringify!($test), pattern)?;)*

      Ok(())
    }

    #[no_mangle]
    pub extern "C" fn run_tests(
      _data: *mut gdnative::libc::c_void,
      args: *const gdnative::core_types::VariantArray,
    ) -> gdnative::sys::godot_variant {
      #![allow(clippy::not_unsafe_ptr_arg_deref)]
      let args = unsafe { args.as_ref() };
      let args = args.unwrap();

      // Passed args
      let root = args.get(0).try_to_object::<gdnative::api::Node>().unwrap();

      let test_pattern = args.get(1).try_to_string();
      if let Some(p) = &test_pattern {
        gdnative::godot_print!(">> Running tests matching \"{}\"\n", p);
      }

      // Run tests
      godot_testicles::set_root_node(root);
      let status = _run_tests(&test_pattern);

      // Error forwarding
      if let Err(msg) = &status {
        gdnative::godot_print!("{}", msg);
      }

      gdnative::core_types::Variant::from_bool(status.is_ok()).forget()
    }
  }
}

#[macro_export]
macro_rules! d {
  ($val:expr) => {
    gdnative::godot_print!("[ {} ] => {}", godot_testicles::get_path!(), $val);
  };
}

#[macro_export]
macro_rules! testicles {
  ($(fn $fn_name:ident() $body:block)*) => {
    $(#[inline] fn $fn_name() -> godot_testicles::AssertionResult {
      $body
      godot_testicles::cleanup(godot_testicles::get_root_node()?);
      Ok(())
    })*

    pub fn run(prefix: &str, pattern: &Option<String>) -> godot_testicles::AssertionResult {
      let skip_prefix = "__skip_";
      match pattern {
        Some(pattern) => {
          let pattern = regex::Regex::new(&pattern[..]).map_err(|_| "Invalid test path pattern".to_string())?;

          $(
            let path = format!("{}::{}", prefix, stringify!($fn_name));
            if !stringify!($fn_name).starts_with(skip_prefix) && pattern.is_match(&path[..]) {
              $fn_name()?;
            }
          )*
        }
        None => {
          $(
            if !stringify!($fn_name).starts_with(skip_prefix) {
              $fn_name()?;
            }
          )*
        }
      }

      Ok(())
    }
  };
}

#[macro_export]
macro_rules! node {
  (
    $type:ty,
    { $( $key:ident : $value:expr , )* },
    $setup: expr,
    [ $( $child:expr , )* ]
  ) => {{
    let node = unsafe { <$type>::new().into_shared().assume_safe() };

    $(node.set(stringify!($key), $value);)*
    $setup(node);

    $(node.add_child($child, false);)*

    node
  }}
}
