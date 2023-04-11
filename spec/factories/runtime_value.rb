# typed: false
# frozen_string_literal: true

::FactoryBot.define do
  factory :runtime_value, class: ::CSVPlusPlus::Runtime::Value do
    transient do
      resolve_fn { -> { build(:number_one) } }
    end

    initialize_with { new(resolve_fn) }
  end
end
