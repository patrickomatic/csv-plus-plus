# typed: false
# frozen_string_literal: true

::FactoryBot.define do
  factory :cell, class: ::CSVPlusPlus::Cell do
    transient do
      row_index { 0 }
      index { 0 }
      value { nil }
      modifier { build(:modifier) }
    end

    ast { nil }

    initialize_with { new(row_index:, index:, value:, modifier:) }
  end
end
