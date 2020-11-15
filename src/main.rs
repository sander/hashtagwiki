mod document;
mod authoring;
mod publishing;

#[tokio::main]
async fn main() {
    authoring::serve(([127, 0, 0, 1], 3030)).await;
}