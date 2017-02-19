mod builder;

fn main() {
  let query = builder::QueryBuilder::new()
    .select("*".to_string())
    .table("table1".to_string())
    ._where("id == 1".to_string())
    .execute();
  println!("{}", query);
}
