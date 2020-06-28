use toy_http_server;

fn main() {
    let hello = toy_http_server::hello();
    println!("{}", hello);
}