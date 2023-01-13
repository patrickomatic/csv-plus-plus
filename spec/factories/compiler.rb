# frozen_string_literal: true

require 'tempfile'
require_relative '../../lib/language/compiler'

::FactoryBot.define do
  factory :compiler, class: ::CSVPlusPlus::Language::Compiler do
    transient do
      filename { 'foo_stocks.csvpp' }
      verbose { false }
      input { '' }
      row_index { 0 }
      cell_index { nil }
      line_number { 1 }
      cell { nil }
    end

    initialize_with do
      new(input:, filename:, verbose:).tap do |instance|
        instance.row_index = row_index
        instance.line_number = line_number
        instance.cell = cell
        instance.cell_index = cell_index
      end
    end
  end
end
