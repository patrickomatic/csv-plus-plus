# frozen_string_literal: true

require_relative '../../lib/template'

::FactoryBot.define do
  factory :template, class: ::CSVPlusPlus::Template do
    transient do
      scope { build(:scope) }
      rows { [] }
    end

    initialize_with { new(rows:, scope:) }
  end
end
