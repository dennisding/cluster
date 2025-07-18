

mod cluster;

fn main() {
    println!("hello world!");

    let cluster = cluster::Cluster::new();

    cluster.print_self();
}