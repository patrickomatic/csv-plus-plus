# frozen_string_literal: true

source 'https://rubygems.org'

# google drive api
gem 'google-apis-drive_v3', '~> 0.3', require: false
# google sheets api
gem 'google-apis-sheets_v4', '~> 0.2', require: false
# googleauth api
gem 'googleauth', '~> 1.3', require: false

# for writing xlsx files
gem 'rubyXL', '~> 3.4'

# type checking
gem 'sorbet-static-and-runtime', '~> 0.5'

group :development do
  # rake
  gem 'rake', '~> 13'
  # enforce standards
  gem 'rubocop', '~> 1.4', require: false
  # LSP provider for editor/rubocop support
  gem 'solargraph', '~> 0'
  # type generation
  gem 'tapioca', '~> 0.11'
end

group :test do
  # factory builder for rspect tests
  gem 'factory_bot', '~> 6'
  # the chosen testing framework
  gem 'rspec', '~> 3'
  # fixes some oddities with doubles disagreeing with runtime types
  gem 'rspec-sorbet', '~> 1'
  # code coverage
  gem 'simplecov', '~> 0.2', require: false
  # testing external APIS
  gem 'vcr', '~> 6', require: false
  # mocking web requests
  gem 'webmock', '~> 3', require: false
end

group :development, :test do
  # config
  gem 'dotenv', '~> 2.8', require: false
end
