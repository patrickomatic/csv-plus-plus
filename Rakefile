RACC_FILES = {
  "lib/code_section.tab.rb" => "parsers/code_section.y",
  "lib/cell_value.tab.rb" => "parsers/cell_value.y",
}

task default: RACC_FILES.keys.map(&:to_sym) + %i[spec]

begin
  require 'rspec/core/rake_task'
  RSpec::Core::RakeTask.new(:spec)
rescue LoadError
end

RACC_FILES.each do |dep, source|
  file dep => source do |t|
    sh "racc -o #{t.name} #{t.prerequisites.join(' ')}"
  end
end

task :clean do
  sh "rm -f #{RACC_FILES.keys.join(' ')}"
end
