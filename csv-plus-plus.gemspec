# frozen_string_literal: true

require 'rake'

::Gem::Specification.new do |s|
  s.name        = 'csv-plus-plus'
  s.version     = '0.0.1'
  s.licenses    = ['MIT']
  s.summary     = 'A CSV-based programming language'
  # todo
  s.description = 'A CSV-based programming language'
  s.authors     = ['Patrick Carroll']
  s.email       = 'patrick@patrickomatic.com'
  s.files       = ::FileList['lib/**/*.rb']
  s.homepage    = 'https://github.com/patrickomatic/csv-plus-plus'
  s.metadata['rubygems_mfa_required'] = 'true'
  s.required_ruby_version = '>= 3.1'
end
