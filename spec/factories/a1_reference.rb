# typed: false
# frozen_string_literal: true

::FactoryBot.define do
  factory :a1_reference, class: ::CSVPlusPlus::A1Reference do
    transient do
      cell_index { nil }
      ref { nil }
      row_index { nil }
      scoped_to_expand { nil }
    end

    initialize_with { new(cell_index:, ref:, row_index:, scoped_to_expand:) }
  end
end
