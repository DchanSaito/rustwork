500_000.times do |i|
  t = CsvTest.new
  t.name = (1...100).map{ (65 + rand(26)).chr }.join
  t.hoge = (1...100).map{ (65 + rand(26)).chr }.join
  t.foo = (1...100).map{ (65 + rand(26)).chr }.join
  t.hogefoo = (1...100).map{ (65 + rand(26)).chr }.join
  t.hogehoge = (1...100).map{ (65 + rand(26)).chr }.join
  t.foofoo = (1...100).map{ (65 + rand(26)).chr }.join
  t.namehoge = (1...100).map{ (65 + rand(26)).chr }.join
  t.namefoo = (1...100).map{ (65 + rand(26)).chr }.join
  t.namehogefoo = (1...100).map{ (65 + rand(26)).chr }.join
  t.namehogehoge = (1...100).map{ (65 + rand(26)).chr }.join
  t.namefoofoo = (1...100).map{ (65 + rand(26)).chr }.join
  t.save!
  if(i % 1000 == 0)
    puts "#{i}個保存完了"
  end
end
