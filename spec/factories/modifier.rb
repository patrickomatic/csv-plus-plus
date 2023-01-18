# frozen_string_literal: true

require_relative '../../lib/modifier'

::FactoryBot.define do
  factory :modifier, class: ::CSVPlusPlus::Modifier do
    transient do
      repetitions { nil }
    end

    after(:build) do |m, e|
      m.expand = build(:expand, repetitions: e.repetitions) if e.repetitions
    end

    factory :row_modifier do
      row_level { true }
    end

    factory :modifier_with_expand do
      row_level { true }
      repetitions { 2 }
    end

    factory :modifier_with_infinite_expand do
      row_level { true }

      after(:build) do |m|
        m.expand = build(:expand)
      end
    end
  end
end
