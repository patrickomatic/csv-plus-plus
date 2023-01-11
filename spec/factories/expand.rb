# frozen_string_literal: true

require_relative '../../lib/modifier'

::FactoryBot.define do
  factory :expand, class: ::CSVPlusPlus::Expand do
    transient do
      repetitions { nil }
    end

    initialize_with { new(repetitions) }
  end
end
