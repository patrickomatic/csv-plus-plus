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

task default: ::RACC_FILES.keys.map(&:to_sym) + %i[rubocop:autocorrect_all sorbet:typecheck spec]

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
