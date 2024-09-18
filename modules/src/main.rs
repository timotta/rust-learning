use modules::{self, submodule_inline, submodule_level1, submodule_level1::submodule_level2};

fn main() {
    modules::lib_function();
    submodule_inline::submodule_inline_function();
    submodule_level1::submodule_level1_function();
    submodule_level2::submodule_level2_function();
}
