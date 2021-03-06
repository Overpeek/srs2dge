pub use srs2dge_link_static::*;

#[cfg(feature = "dynamic")]
macro_rules! fix_clippy_single_component_path_imports {
    () => {
        #[allow(clippy::all)]
        #[allow(unused_imports)]
        use srs2dge_link_dynamic;
    };
}

#[cfg(feature = "dynamic")]
fix_clippy_single_component_path_imports!();
