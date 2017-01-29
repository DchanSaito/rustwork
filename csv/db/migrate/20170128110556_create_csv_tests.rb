class CreateCsvTests < ActiveRecord::Migration[5.0]
  def change
    create_table :csv_tests do |t|
      t.string :name
      t.string :hoge
      t.string :foo
      t.string :hogefoo
      t.string :hogehoge
      t.string :foofoo
      t.string :namehoge
      t.string :namefoo
      t.string :namehogefoo
      t.string :namehogehoge
      t.string :namefoofoo

      t.timestamps
    end
  end
end
