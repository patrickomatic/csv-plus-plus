# frozen_string_literal: true

require_relative '../../lib/language/entities'

ns = ::CSVPlusPlus::Language

::FactoryBot.define do
  factory :boolean_true, class: ns::Boolean do
    initialize_with { new true }
  end

  factory :boolean_false, class: ns::Boolean do
    initialize_with { new false }
  end
end
