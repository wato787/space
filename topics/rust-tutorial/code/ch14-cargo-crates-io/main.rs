fn main() {
    let is_debug = cfg!(debug_assertions);
    println!("debug_assertions: {is_debug}");

    let profile = if is_debug { "dev" } else { "release" };
    println!("profile: {profile}");
}
