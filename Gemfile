# frozen_string_literal: true

source 'https://rubygems.org'

# google sheets api
gem 'google-apis-sheets_v4', '~> 0.2'
# googleauth api
gem 'googleauth', '~> 1.3'

group :development do
  # config
  gem 'dotenv', '~> 2.8'
  # rake
  gem 'rake', '~> 13'
  # enforce standards
  gem 'rubocop', '~> 1.4'
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
end
