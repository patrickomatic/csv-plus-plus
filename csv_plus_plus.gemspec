# frozen_string_literal: true

require_relative 'lib/csv_plus_plus/version'
require 'rake'

# rubocop:disable Metrics/BlockLength
::Gem::Specification.new do |s|
  s.name        = 'csv_plus_plus'
  s.summary     = 'A CSV-based programming language'
  s.description = <<~DESCRIPTION
    A programming language built on top of CSV.  You can define functions and variables to use in your spreadsheet,#{' '}
    then compile it to Excel, CSV, Google Sheets, etc.
  DESCRIPTION
  s.authors     = ['Patrick Carroll']
  s.email       = 'patrick@patrickomatic.com'
  s.version     = ::CSVPlusPlus::VERSION
  s.license     = 'MIT'
  s.files       = ::FileList['bin/csv++', 'lib/**/*.rb', 'README.md', 'CHANGELOG.md']
  s.homepage    = 'https://github.com/patrickomatic/csv-plus-plus'

  s.executables = %w[csv++]

  s.metadata = {
    rubygems_mfa_required: 'true',
    bug_tracker_uri: 'https://github.com/patrickomatic/csv-plus-plus/issues',
    documentation_uri: 'https://www.rubydoc.info/gems/csv_plus_plus/',
    github_repo: 'git://github.com/patrickomatic/csv_plus_plus',
    homepage_uri: 'https://github.com/patrickomatic/csv_plus_plus',
    source_code_uri: 'https://github.com/patrickomatic/csv_plus_plus',
    changelog_uri: 'https://github.com/patrickomatic/csv_plus_plus/blob/main/CHANGELOG.md',
    funding_uri: 'eth:patrickomatic.eth',
    # rubocop:disable Style/StringHashKeys
    'rubygems_mfa_required' => 'true'
    # rubocop:enable Style/StringHashKeys
  }

  s.required_ruby_version = '>= 3.1'

  s.add_runtime_dependency('google-apis-sheets_v4', '~> 0.2')
  s.add_runtime_dependency('googleauth', '~> 1.3')
  s.add_runtime_dependency('rubyXL', '~> 3.4')

  s.add_development_dependency('bundler', '~> 2')
  s.add_development_dependency('rake', '~> 13')
  s.add_development_dependency('rubocop', '~> 1.4')
end
# rubocop:enable Metrics/BlockLength
