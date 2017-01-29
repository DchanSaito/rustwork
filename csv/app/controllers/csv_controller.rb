require 'csv'
require 'ffi'

class CsvController < ApplicationController
  def index
  end

  def ruby
    start_time = Time.now

    ruby_csv = CSV.generate do |csv|
      data_column = CsvTest.column_names
      data_column.delete("created_at")
      data_column.delete("updated_at")
      csv << data_column
      CsvTest.limit(10).each do |data|
        csv << data.attributes.values_at(*data_column)
      end
    end

    puts '------------------------------------------------------------'
    puts '処理にかかった時間'
    puts Time.now - start_time
    puts '------------------------------------------------------------'

    respond_to do |format|
      format.csv { send_data ruby_csv }
    end
  end

  module CSVmaker
    extend FFI::Library
    ffi_lib Rails.root + 'rust/target/release/libcsv.dylib'
    attach_function :make_csv, [], :string
  end

  def rust
    start_time = Time.now

    rust_csv = CSVmaker.make_csv

    puts '------------------------------------------------------------'
    puts '処理にかかった時間'
    puts Time.now - start_time
    puts '------------------------------------------------------------'

    respond_to do |format|
      format.csv { send_data rust_csv }
    end
  end
end
