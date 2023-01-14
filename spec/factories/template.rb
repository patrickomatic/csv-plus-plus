# frozen_string_literal: true

require_relative '../../lib/template'

::FactoryBot.define do
  factory :template, class: ::CSVPlusPlus::Template do
    transient do
      code_section { build(:code_section) }
      rows { [] }
    end

    initialize_with { new(rows:, code_section:) }
  end
end
