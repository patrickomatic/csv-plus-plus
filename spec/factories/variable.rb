# typed: false
# frozen_string_literal: true

::FactoryBot.define do
  factory :variable, class: ::CSVPlusPlus::Entities::Variable do
    transient do
      id { nil }
    end

    initialize_with { new id }

    factory :variable_foo do
      id { 'foo' }
    end

    factory :variable_bar do
      id { 'bar' }
    end
  end
end
