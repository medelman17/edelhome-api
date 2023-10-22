use hue_rs::discover_bridges;

#[tokio::main]
async fn main() {
    let bridges = discover_bridges().await;
    println!("Bridges: {:?}", bridges);
}
