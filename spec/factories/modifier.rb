# frozen_string_literal: true

require_relative '../../lib/modifier'

::FactoryBot.define do
  factory :modifier, class: ::CSVPlusPlus::Modifier do
    transient do
      repetitions { nil }
      row_level { false }
    end

    initialize_with { new(row_level:) }

    factory :row_modifier do
      row_level { true }

      factory :modifier_with_expand do
        after(:build) do |m|
          m.expand = build(:expand, repetitions: 2)
        end
      end

      factory :modifier_with_infinite_expand do
        after(:build) do |m|
          m.expand = build(:expand)
        end
      end
    end
  end
end
