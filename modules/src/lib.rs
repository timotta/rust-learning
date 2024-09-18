pub fn lib_function() {
    println!("This is a LIB function");
}

pub mod submodule_inline {
    pub fn submodule_inline_function() {
        println!("This is a submodule inline function");
    }
}

pub mod submodule_level1;