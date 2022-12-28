task default: %i[
  lib/code_section_parser.tab.rb
  spec
]

begin
  require 'rspec/core/rake_task'
  RSpec::Core::RakeTask.new(:spec)
rescue LoadError
end

file "lib/code_section_parser.tab.rb" => ["racc/code_section_parser.y"] do |t|
  sh "racc -o #{t.name} #{t.prerequisites.join(' ')}"
end

