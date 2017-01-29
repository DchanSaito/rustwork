extern crate mysql;
extern crate libc;

use libc::*;
use std::ffi::{CString};

pub struct CsvTest {
    id: i32,
    name: String,
    hoge: String,
    foo: String,
    hogefoo: String,
    hogehoge: String,
    foofoo: String,
    namehoge: String,
    namefoo: String,
    namehogefoo: String,
    namehogehoge: String,
    namefoofoo: String,
}

#[no_mangle]
pub extern fn make_csv() -> *const c_char {
    // DBの接続
    let pool = mysql::Pool::new("mysql://root@localhost:3307/csv_test_development").unwrap();

    // SELECT
    let select_all: Vec<CsvTest> =
        pool.prep_exec("SELECT id, name, hoge, foo, hogefoo, hogehoge, foofoo, namehoge, namefoo, namehogefoo, namehogehoge, namefoofoo from csv_tests LIMIT 10", ())
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (id, name, hoge, foo, hogefoo, hogehoge, foofoo, namehoge, namefoo, namehogefoo, namehogehoge, namefoofoo) = mysql::from_row(row);
                CsvTest {
                    id: id,
                    name: name,
                    hoge: hoge,
                    foo: foo,
                    hogefoo: hogefoo,
                    hogehoge: hogehoge,
                    foofoo: foofoo,
                    namehoge: namehoge,
                    namefoo: namefoo,
                    namehogefoo: namehogefoo,
                    namehogehoge: namehogehoge,
                    namefoofoo: namefoofoo,
                }
            }).collect()
        }).unwrap();

    // CSVの作成
    let mut rust_csv: String;
    rust_csv = format!("\"id\",\"name\",\"hoge\",\"foo\",\"hogefoo\",\"hogehoge\",\"foofoo\",\"namehoge\",\"namefoo\",\"namehogefoo\",\"namehogehoge\",\"namefoofoo\"\n");
    for i in &select_all {
        let record: String = format!("\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n",
                                     i.id.to_string(),
                                     i.name,
                                     i.hoge,
                                     i.foo,
                                     i.hogefoo,
                                     i.hogehoge,
                                     i.foofoo,
                                     i.namehoge,
                                     i.namefoo,
                                     i.namehogefoo,
                                     i.namehogehoge,
                                     i.namefoofoo
        );

        rust_csv.push_str(&record);
    }

    CString::new(rust_csv).unwrap().into_raw()
}
