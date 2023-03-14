# frozen_string_literal: true

::FactoryBot.define do
  factory :template, class: ::CSVPlusPlus::Template do
    transient do
      rows { [] }
      scope { build(:scope) }
    end

    initialize_with { new(rows:, scope:) }
  end
end
