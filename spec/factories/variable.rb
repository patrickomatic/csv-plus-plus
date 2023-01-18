# frozen_string_literal: true

require_relative '../../lib/language/entities'

::FactoryBot.define do
  factory :variable, class: ::CSVPlusPlus::Language::Variable do
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
