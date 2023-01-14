# frozen_string_literal: true

require_relative '../../lib/language/entities'

ns = ::CSVPlusPlus::Language

::FactoryBot.define do
  factory :cell_reference, class: ns::CellReference do
    transient do
      ref { 'C1' }
    end
    initialize_with { new ref }
  end
end
