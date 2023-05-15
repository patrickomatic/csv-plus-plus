# typed: false
# frozen_string_literal: true

::FactoryBot.define do
  factory :modifier, class: ::CSVPlusPlus::Modifier do
    transient do
      options { build(:file_options) }
      row_level { false }
    end

    initialize_with { new(options, row_level:) }

    factory :row_modifier do
      row_level { true }
    end
  end
end
