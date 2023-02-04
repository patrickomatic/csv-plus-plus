# frozen_string_literal: true

::FactoryBot.define do
  factory :code_section, class: ::CSVPlusPlus::CodeSection do
    transient do
      variables { {} }
      functions { {} }
    end

    initialize_with { new(variables:, functions:) }
  end
end
