mod btree;
use btree::Btree;

fn main() {
    let tfp = "chicchai.db";
    let wal = "chicchai.log";

    let mut btree = match Btree::<String, String>::new(&tfp.to_string(), &wal.to_string(), 100, 100)
    {
        Ok(v) => v,
        Err(e) => {
            println!("err {}", e.description());
            return;
        }
    };

    if let Err(e) = btree.insert("hello".to_string(), "world".to_string()) {
        println!("err {}", e.description());
        return;
    }

    if let Some(v) = btree.get("hello".to_string()) {
        println!("value: {}", v);
    }

    if let Err(e) = btree.close() {
        println!("err {}", e);
        return;
    }

    println!("good bye!")
}
