# frozen_string_literal: true

::FactoryBot.define do
  factory :template, class: ::CSVPlusPlus::Template do
    transient do
      rows { [] }
      code_section { build(:code_section) }
    end

    initialize_with { new(rows:, code_section:) }
  end
end
