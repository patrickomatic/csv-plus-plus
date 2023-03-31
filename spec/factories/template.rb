# frozen_string_literal: true

::FactoryBot.define do
  factory :template, class: ::CSVPlusPlus::Template do
    transient do
      rows { [] }
      runtime { build(:runtime) }
    end

    initialize_with { new(rows:, runtime:) }
  end
end
