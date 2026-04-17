fn main() {
    uniffi::generate_scaffolding("src/cs_mobile_core.udl").expect("uniffi scaffolding");
}
