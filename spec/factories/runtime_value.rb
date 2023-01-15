# frozen_string_literal: true

require_relative '../../lib/language/entities'

::FactoryBot.define do
  factory :runtime_value, class: ::CSVPlusPlus::Language::RuntimeValue do
    transient do
      resolve_fn { -> { build(:number_one) } }
    end

    initialize_with { new(resolve_fn) }
  end
end
