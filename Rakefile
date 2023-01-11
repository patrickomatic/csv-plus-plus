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
  test:integration:stocks
  test:integration:all_features
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
  namespace :integration do
    desc 'Test with the examples/stocks.csvpp template'
    task :stocks do
      sh %(./bin/csv++ -n "Sheet1" -g #{::ENV.fetch('GOOGLE_SHEET_ID', nil)} examples/stocks.csvpp)
    end

    desc 'Test with the examples/all_features.csvpp template'
    task :all_features do
      sh %(./bin/csv++ -n "Sheet2" -g #{::ENV.fetch('GOOGLE_SHEET_ID', nil)} examples/all_features.csvpp)
    end
  end
end
