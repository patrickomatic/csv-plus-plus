# frozen_string_literal: true

require_relative '../../lib/language/scope'

::FactoryBot.define do
  factory :scope, class: ::CSVPlusPlus::Language::Scope do
    transient do
      code_section { build(:code_section) }
    end

    initialize_with { new(code_section) }
  end
end
