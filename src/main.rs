use crate::table::{Table, TableOwner};

mod table;
mod values;

fn main() {
    println!("Hello, world!");

    let table_owner = TableOwner::new();

    let table = Table::new(&table_owner);

    table.insert("Hello".into(), "World".into());
    table.insert(1.into(), "Goodbye".into());
    dbg!(&table);

    table.remove(&0.into());
    table.remove(&1.into());
    table.remove(&2.into());
    dbg!(&table);

    table.insert(true.into(), table.clone().into());
    dbg!(&table);
}
