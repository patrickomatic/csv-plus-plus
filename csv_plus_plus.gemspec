# frozen_string_literal: true

require_relative 'lib/csv_plus_plus/version'
require 'rake'

::Gem::Specification.new do |s|
  s.name        = 'csv_plus_plus'
  s.version     = ::CSVPlusPlus::VERSION
  s.license     = 'MIT'
  s.summary     = 'A CSV-based programming language'
  s.description = 'A programming language built on top of CSV'
  s.authors     = ['Patrick Carroll']
  s.email       = 'patrick@patrickomatic.com'
  s.files       = ::FileList['lib/**/*.rb']
  s.homepage    = 'https://github.com/patrickomatic/csv-plus-plus'
  s.metadata['rubygems_mfa_required'] = 'true'
  s.required_ruby_version = '>= 3.1'
end
