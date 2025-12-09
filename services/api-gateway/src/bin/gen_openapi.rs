use api_gateway::openapi;

fn main() {
    let spec = openapi::generate_openapi_spec();
    println!("{}", spec);
}
