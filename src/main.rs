use comfy_table::Table;
use rustnetting::*;

fn main() {
    let config = Config::new().unwrap();

    let mut ip_add: String = config.ip_add.clone();

    let mut table = Table::new();

    table.set_header(vec!["ip_add", "broadcast", "next"]);

    for slash in config.requirements {
        let (next, broadcast) = next_add(&ip_add, slash);

        table.add_row(vec![&ip_add, &broadcast, &next]);

        ip_add = next;
    }

    println!("{table}")
}
