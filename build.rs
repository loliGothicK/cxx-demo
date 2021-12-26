fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/semaphore.cc")
        .flag_if_supported("-std=c++20")
        .compile("cxx-demo");
}
