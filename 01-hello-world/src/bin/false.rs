// Program which simulates the false command, which always returns failure code (different from 0)
fn main() {
    //std::process::exit(1);
    std::process::abort();
}