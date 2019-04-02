use graphite_render::{Client, Path};

fn main() {
    let url = "http://localhost/".parse().unwrap();
    let client = Client::new(url).unwrap();

    let path = Path::new("carbon.agents")
        .dot("57a805d90413-a")
        .values(&["cpuUsage", "memUsage"])
        .alias_by_current_node();
    
    let response = client.request(&[&path]).unwrap();
    
    println!("{}", path);
    println!("{:#?}", response);
}
