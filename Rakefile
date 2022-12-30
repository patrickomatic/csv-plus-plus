require 'dotenv/load'

RACC_FILES = {
  "lib/code_section.tab.rb" => "parsers/code_section.y",
  "lib/cell_value.tab.rb" => "parsers/cell_value.y",
  "lib/modifier.tab.rb" => "parsers/modifier.y",
}

task default: RACC_FILES.keys.map(&:to_sym) + %i[
  spec
  test:integration:stocks
  test:integration:all_modifiers
]

begin
  require 'rspec/core/rake_task'
  RSpec::Core::RakeTask.new(:spec)
rescue LoadError
end

RACC_FILES.each do |dep, source|
  file dep => source do |t|
    sh "racc -o #{t.name} #{t.prerequisites.join(' ')}"
  end
end

task :clean do
  sh "rm -f #{RACC_FILES.keys.join(' ')}"
end

namespace :test do
  namespace :integration do
    task :stocks do
      sh %Q!./bin/csv++ -n "Test: Stocks" -i #{ENV['GOOGLE_SHEET_ID']} examples/stocks.csvpp!
    end

    task :all_modifiers do
      sh %Q!./bin/csv++ -n "Test: All Modifiers" -i #{ENV['GOOGLE_SHEET_ID']} examples/all_modifiers.csvpp!
    end
  end
end
