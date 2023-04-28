# typed: false
# frozen_string_literal: true

::FactoryBot.define do
  factory :scope, class: ::CSVPlusPlus::Runtime::Scope do
    transient do
      functions { {} }
      variables { {} }
    end

    initialize_with { new(functions:, variables:) }
  end
end
