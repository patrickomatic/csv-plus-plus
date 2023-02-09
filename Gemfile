# frozen_string_literal: true

source 'https://rubygems.org'

# for writing OOXML documents
gem 'caxlsx', '~> 3'

# google sheets api
gem 'google-apis-sheets_v4', '~> 0.2', require: false
# googleauth api
gem 'googleauth', '~> 1.3', require: false

group :development do
  # rake
  gem 'rake', '~> 13'
  # enforce standards
  gem 'rubocop', '~> 1.4', require: false
  # LSP provider for editor/rubocop support
  gem 'solargraph', '~> 0'
end

group :test do
  # factory builder for rspect tests
  gem 'factory_bot', '~> 6'
  # the chosen testing framework
  gem 'rspec', '~> 3'
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
