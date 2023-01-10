require_relative '../../lib/template'

FactoryBot.define do
  factory :template, class: CSVPlusPlus::Template do
    transient do
      execution_context { build(:execution_context) }
      code_section { build(:code_section) }
      rows { [] }
    end

    initialize_with { new(rows:, code_section:, execution_context:) }
  end
end

