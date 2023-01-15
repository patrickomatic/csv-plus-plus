# frozen_string_literal: true

require_relative '../../lib/language/entities'

::FactoryBot.define do
  factory :fn_call, class: ::CSVPlusPlus::Language::FunctionCall do
    transient do
      name { :foo }
      a { build(:string_foo) }
      b { build(:string_bar) }
      arguments { [] }
    end

    initialize_with { new(name, arguments.empty? ? [a, b] : arguments) }

    factory :fn_call_add do
      name { :add }
      arguments { [build(:number_one), build(:number_two)] }
    end

    factory :fn_call_foo do
      name { 'foo' }
      arguments { [build(:variable_bar)] }
    end
  end
end
