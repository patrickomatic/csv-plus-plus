# frozen_string_literal: true

require_relative '../../lib/language/entities'

ns = ::CSVPlusPlus::Language

::FactoryBot.define do
  factory :fn_call_add, class: ns::FunctionCall do
    initialize_with { new(:add, [build(:number_one), build(:number_two)]) }
  end

  factory :fn_call_foo, class: ns::FunctionCall do
    initialize_with { new('foo', [build(:variable_bar)]) }
  end

  factory :fn_call, class: ns::FunctionCall do
    transient do
      name { :foo }
      a { build(:string_foo) }
      b { build(:string_bar) }
      arguments { [] }
    end

    initialize_with { new(name, arguments.empty? ? [a, b] : arguments) }
  end
end
