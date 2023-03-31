# typed: false
# frozen_string_literal: true

::FactoryBot.define do
  factory :cell_reference, class: ::CSVPlusPlus::Entities::CellReference do
    transient do
      cell_index { nil }
      ref { nil }
      row_index { nil }
      scoped_to_expand { nil }
    end

    initialize_with { new(cell_index:, ref:, row_index:, scoped_to_expand:) }
  end
end
