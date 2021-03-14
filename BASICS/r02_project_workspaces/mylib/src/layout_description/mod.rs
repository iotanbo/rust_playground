
// For testing purposes, we made this submodule public
// so that library clients can use it directly
pub mod advantages;

// For testing purposes, we made this submodule private
mod disadvantages;


pub fn describe() {

    println!("Describing rust project layout with workspaces (this example).");
    println!("* Workspace layout advantages:");
    println!("{}", advantages::enumerate());

    println!("* Workspace layout disadvantages:");
    println!("{}", disadvantages::enumerate());
}

pub fn summarize() {
    println!("SUMMARY: workspaces are a viable solution at the moment (in JAN 2021);");
    println!("  But VS Code rust plugin performance seems to degrade, resulting in BUGGY CONTEXT HELP.");
    println!("  For small and medium projects it is recommended to use single-package project.");
}
