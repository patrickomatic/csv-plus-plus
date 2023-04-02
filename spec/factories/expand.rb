# typed: false
# frozen_string_literal: true

::FactoryBot.define do
  factory :expand, class: ::CSVPlusPlus::Modifier::Expand do
    transient do
      repetitions { nil }
      starts_at { nil }
    end

    initialize_with { new(repetitions:, starts_at:) }
  end
end
