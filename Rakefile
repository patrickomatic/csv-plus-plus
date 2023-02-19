# frozen_string_literal: true

require_relative 'lib/csv_plus_plus/version'
require 'dotenv/load'
require 'rspec/core/rake_task'
require 'rubocop/rake_task'

::RSpec::Core::RakeTask.new(:spec)
::RuboCop::RakeTask.new

RACC_FILES = {
  'lib/csv_plus_plus/language/code_section.tab.rb': 'parsers/code_section.y',
  'lib/csv_plus_plus/language/cell_value.tab.rb': 'parsers/cell_value.y',
  'lib/csv_plus_plus/modifier.tab.rb': 'parsers/modifier.y'
}.freeze

task default: ::RACC_FILES.keys.map(&:to_sym) + %i[
  rubocop:autocorrect_all
  spec
  test:csv:all_features
  test:excel:all_features
  test:google_sheets:stocks
  test:google_sheets:all_features
]

::RACC_FILES.each do |dep, source|
  desc "Use racc to generate parser file #{dep}"
  file dep => source do |t|
    sh "racc -o #{t.name} #{t.prerequisites.join(' ')}"
  end
end

desc 'Remove generated files'
task :clean do
  sh "rm -f #{::RACC_FILES.keys.join(' ')} csv_plus_plus-*.gem examples/all_features-*.csv"
end

namespace :docs do
  desc 'Build Yard docs locally'
  task :yard do
    sh 'bundle exec yard'
  end
end

namespace :gem do
  # based on the current version, the name of the built gem
  def gem_file
    "csv_plus_plus-#{::CSVPlusPlus::VERSION}.gem"
  end

  # create a tag in git for the current version
  def git_tag_version
    version_tag = "v#{::CSVPlusPlus::VERSION}"
    sh("git tag | grep '^#{version_tag}$'") do |tagged|
      return if tagged

      sh("git tag #{version_tag}")
      sh('git push origin --tags')
    end
  end

  desc 'Build a new release'
  task :build do
    sh 'gem build csv_plus_plus.gemspec'
  end

  desc 'Install the gem locally'
  task install: :build do
    sh "gem install #{gem_file}"
  end

  desc 'Publish the built release'
  task publish: :install do
    git_tag_version
    sh "gem push #{gem_file}"
  end
end

namespace :test do
  namespace :google_sheets do
    # get GOOGLE_SHEET_ID which should be set in the env (or warn)
    def with_google_sheet_id
      google_sheet_id = ::ENV.fetch('GOOGLE_SHEET_ID', nil)
      if google_sheet_id
        yield(google_sheet_id)
      else
        warn('GOOGLE_SHEET_ID is not defined')
      end
    end

    desc 'Test examples/stocks.csvpp outputting to Google Sheets'
    task :stocks do
      with_google_sheet_id do |google_sheet_id|
        sh %(./bin/csv++ -v -n "Sheet1" -g #{google_sheet_id} examples/stocks.csvpp)
      end
    end

    desc 'Test examples/all_features.csvpp outputting to Google Sheets'
    task :all_features do
      with_google_sheet_id do |google_sheet_id|
        sh %(./bin/csv++ --verbose -n "Sheet2" -g #{google_sheet_id} examples/all_features.csvpp)
      end
    end
  end

  namespace :csv do
    desc 'Test examples/all_features.csvpp outputting to CSV'
    task :all_features do
      sh %(./bin/csv++ -b --verbose --output examples/all_features.csv examples/all_features.csvpp)
    end
  end

  namespace :excel do
    desc 'Test examples/all_features.csvpp outputting to Excel'
    task :all_features do
      sh %(./bin/csv++ -v -n "Excel test" -o examples/all_features.xlsx examples/all_features.csvpp)
    end
  end
end
