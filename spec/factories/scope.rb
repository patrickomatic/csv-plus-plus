# frozen_string_literal: true

::FactoryBot.define do
  factory :scope, class: ::CSVPlusPlus::Scope do
    transient do
      runtime { build(:runtime) }
      variables { {} }
      functions { {} }
    end

    initialize_with { new(variables:, functions:, runtime:) }
  end
end
