# frozen_string_literal: true

::FactoryBot.define do
  factory :boolean_true, class: ::CSVPlusPlus::Entities::Boolean do
    initialize_with { new true }
  end

  factory :boolean_false, class: ::CSVPlusPlus::Entities::Boolean do
    initialize_with { new false }
  end
end
