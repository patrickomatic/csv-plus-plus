# frozen_string_literal: true

require_relative '../../lib/language/entities'

ns = ::CSVPlusPlus::Language

::FactoryBot.define do
  factory :variable, class: ns::Variable do
    transient do
      id { 'foo' }
    end
    initialize_with { new id }
  end

  factory :variable_foo, class: ns::Variable do
    initialize_with { new 'foo' }
  end

  factory :variable_bar, class: ns::Variable do
    initialize_with { new 'bar' }
  end
end
