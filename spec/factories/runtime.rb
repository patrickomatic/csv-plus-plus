# typed: false
# frozen_string_literal: true

::FactoryBot.define do
  factory :runtime, class: ::CSVPlusPlus::Runtime do
    transient do
      position { build(:position) }
      scope { build(:scope) }
      source_code { build(:source_code) }
    end

    initialize_with { new(source_code:, position:, scope:) }
  end
end
