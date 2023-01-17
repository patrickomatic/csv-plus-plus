# frozen_string_literal: true

require 'dotenv/load'
require 'rspec/core/rake_task'
require 'rubocop/rake_task'

RACC_FILES = {
  'lib/language/code_section.tab.rb': 'parsers/code_section.y',
  'lib/language/cell_value.tab.rb': 'parsers/cell_value.y',
  'lib/modifier.tab.rb': 'parsers/modifier.y'
}.freeze

task default: ::RACC_FILES.keys.map(&:to_sym) + %i[
  rubocop:autocorrect_all
  spec
  test:csv:all_features
  test:google_sheets:stocks
  test:google_sheets:all_features
]

::RSpec::Core::RakeTask.new(:spec)

::RuboCop::RakeTask.new

::RACC_FILES.each do |dep, source|
  desc "Compile #{dep}"
  file dep => source do |t|
    sh "racc -o #{t.name} #{t.prerequisites.join(' ')}"
  end
end

desc 'Remove generated files'
task :clean do
  sh "rm -f #{::RACC_FILES.keys.join(' ')}"
end

namespace :test do
  namespace :google_sheets do
    google_sheet_id = ::ENV.fetch('GOOGLE_SHEET_ID', nil)

    desc 'Test with the examples/stocks.csvpp template'
    task :stocks do
      sh %(./bin/csv++ -v -n "Sheet1" -g #{google_sheet_id} examples/stocks.csvpp)
    end

    desc 'Test with the examples/all_features.csvpp template outputting to Google Sheets'
    task :all_features do
      sh %(./bin/csv++ --verbose -n "Sheet2" -g #{google_sheet_id} examples/all_features.csvpp)
    end
  end

  namespace :csv do
    desc 'Test with the examples/all_features.csvpp template outputting to CSV'
    task :all_features do
      sh %(./bin/csv++ --verbose -n "Sheet2" --output examples/all_features.csv examples/all_features.csvpp)
    end
  end
end
