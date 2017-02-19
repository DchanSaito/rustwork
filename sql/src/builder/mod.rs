pub struct QueryBuilder {
  table: String,
  select: String,
  _where: String,
}

impl QueryBuilder {
  pub fn new() -> QueryBuilder {
    QueryBuilder {
      table: "".to_string(),
      select: "".to_string(),
      _where: "".to_string(),
    }
  }

  pub fn table(&mut self, arg: String) -> &mut QueryBuilder {
    self.table = arg;
    self
  }

  pub fn select(&mut self, arg: String) -> &mut QueryBuilder {
    self.select = arg;
    self
  }

  pub fn _where(&mut self, arg: String) -> &mut QueryBuilder {
    self._where = arg;
    self
  }

  pub fn execute(&self) -> String {
    let mut query = "".to_string();
    if self.select != "" {
      query.push_str("SELECT ");
      query.push_str(&self.select);
      query.push_str(" ");
    } else {
      // 例外を起こす
    }

    if self.table != "" {
      query.push_str("FROM ");
      query.push_str(&self.table);
      query.push_str(" ");
    } else {
      // 例外を起こす
    }

    if self._where != "" {
      query.push_str("WHERE ");
      query.push_str(&self._where);
      query.push_str(" ");
    }

    if query == "" {
      // 例外を起こす
    }

    query.pop();
    query.push_str(";");
    query
  }
}
