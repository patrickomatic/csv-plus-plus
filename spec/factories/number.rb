# frozen_string_literal: true

require_relative '../../lib/language/entities'

ns = ::CSVPlusPlus::Language

::FactoryBot.define do
  factory :number, class: ns::Number do
    transient do
      n { 0 }
    end
    initialize_with { new n }
  end

  factory :number_zero, class: ns::Number do
    initialize_with { new 0 }
  end

  factory :number_one, class: ns::Number do
    initialize_with { new 1 }
  end

  factory :number_two, class: ns::Number do
    initialize_with { new 2 }
  end
end
