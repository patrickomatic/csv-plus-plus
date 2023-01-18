# frozen_string_literal: true

require_relative '../../lib/language/entities'

::FactoryBot.define do
  factory :cell_reference, class: ::CSVPlusPlus::Language::Entities::CellReference do
    transient do
      ref { 'C1' }
    end

    initialize_with { new ref }
  end
end
