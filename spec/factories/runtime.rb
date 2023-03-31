# frozen_string_literal: true

::FactoryBot.define do
  factory :runtime, class: ::CSVPlusPlus::Runtime do
    transient do
      row_index { 0 }
      cell_index { nil }
      line_number { 1 }
      cell { nil }
      filename { 'foo.csvpp' }
      input { '' }
      functions { {} }
      variables { {} }
    end

    initialize_with { new(input:, filename:, functions:, variables:) }

    after(:build) do |i, e|
      i.cell = e.cell
      i.cell_index = e.cell_index
      i.line_number = e.line_number
      i.row_index = e.row_index
    end
  end
end
