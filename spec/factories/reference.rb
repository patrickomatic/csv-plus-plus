# typed: false
# frozen_string_literal: true

::FactoryBot.define do
  factory :reference, class: ::CSVPlusPlus::Entities::Reference do
    transient do
      ref { nil }
      a1_ref { nil }
    end

    initialize_with { new(ref:, a1_ref:) }

    factory :variable_foo do
      ref { 'foo' }
    end

    factory :variable_bar do
      ref { 'bar' }
    end
  end
end
