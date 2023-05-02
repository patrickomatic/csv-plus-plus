# frozen_string_literal: true

require_relative 'lib/csv_plus_plus/version'
require 'dotenv/load'
require 'rspec/core/rake_task'
require 'rubocop/rake_task'

::RSpec::Core::RakeTask.new(:spec)
::RuboCop::RakeTask.new

RACC_FILES = {
  'lib/csv_plus_plus/parser/code_section.tab.rb': 'parsers/code_section.y',
  'lib/csv_plus_plus/parser/cell_value.tab.rb': 'parsers/cell_value.y',
  'lib/csv_plus_plus/parser/modifier.tab.rb': 'parsers/modifier.y'
}.freeze

task default: ::RACC_FILES.keys.map(&:to_sym) + %i[
  rubocop:autocorrect_all
  sorbet:typecheck
  spec
  test:csv:all_features
  test:excel:all_features
  test:excel:loan
  test:google_sheets:all_features
  test:google_sheets:crypto_wallet
  test:google_sheets:stocks
]

::RACC_FILES.each do |dep, source|
  desc "Use racc to generate parser file #{dep}"
  file dep => source do |t|
    sh "bundle exec racc -o #{t.name} #{t.prerequisites.join(' ')}"
  end
end

namespace :racc do
  desc 'Debug racc grammars'
  task :debug do
    ::RACC_FILES.each do |_, source|
      sh "bundle exec racc -vt #{source}"
    end
  end
end

desc 'Remove generated files'
task :clean do
  sh "rm -f #{::RACC_FILES.keys.join(' ')} csv_plus_plus-*.gem examples/all_features-*.csv"
  sh 'rm -rf coverage/ doc/ .yardoc/'
  sh 'rm -rf parsers/*.tab.rb modifier.output code_section.output cell_value.output'
end

namespace :docs do
  desc 'Build Yard docs locally'
  task :yard do
    sh 'bundle exec yard --fail-on-warning'
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
  task build: 'docs:yard' do
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

namespace :sorbet do
  desc 'Run a typecheck with Sorbet'
  task :typecheck do
    sh 'bundle exec srb tc'
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

    desc 'Test examples/all_features/all_features.csvpp outputting to Google Sheets'
    task :all_features do
      with_google_sheet_id do |google_sheet_id|
        sh %(./bin/csv++ --verbose -n "Sheet2" -g #{google_sheet_id} examples/all_features/all_features.csvpp)
      end
    end

    desc 'Test examples/crypto_wallet/crypto_wallet.csvpp outputting to Google Sheets'
    task :crypto_wallet do
      with_google_sheet_id do |google_sheet_id|
        sh %(./bin/csv++ --verbose -n "Crypto Wallet" -g #{google_sheet_id} examples/crypto_wallet/crypto_wallet.csvpp)
      end
    end

    desc 'Test examples/loan/loan.csvpp outputting to Google Sheets'
    task :loan do
      with_google_sheet_id do |google_sheet_id|
        sh %(./bin/csv++ --verbose -n "Loan Test" -g #{google_sheet_id} examples/loan/loan.csvpp)
      end
    end

    desc 'Test examples/stocks/stocks.csvpp outputting to Google Sheets'
    task :stocks do
      with_google_sheet_id do |google_sheet_id|
        sh %(./bin/csv++ -v -n "Sheet1" -g #{google_sheet_id} examples/stocks/stocks.csvpp)
      end
    end
  end

  namespace :csv do
    desc 'Test examples/all_features/all_features.csvpp outputting to CSV'
    task :all_features do
      sh %(./bin/csv++ \
            -b \
            --verbose \
            --output examples/all_features/all_features.csv \
            examples/all_features/all_features.csvpp
          )
    end
  end

  namespace :excel do
    desc 'Test examples/all_features/all_features.csvpp outputting to Excel'
    task :all_features do
      sh %(./bin/csv++ -v -n "All Features" -o examples/all_features/all_features.xlsx \
            examples/all_features/all_features.csvpp)
    end

    desc 'Test examples/loan/loan.csvpp outputting to Excel'
    task :loan do
      sh %(./bin/csv++ -v -n "Loan" -o examples/loan/loan.xlsx examples/loan/loan.csvpp)
    end
  end
end
