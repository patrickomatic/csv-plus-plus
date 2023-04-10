# typed: false
# frozen_string_literal: true

::FactoryBot.define do
  factory :fn_call, class: ::CSVPlusPlus::Entities::FunctionCall do
    transient do
      name { :foo }
      arguments { [] }
      infix { false }
    end

    initialize_with { new(name, arguments, infix:) }

    factory :fn_call_add do
      name { :add }
      arguments { [build(:number_one), build(:number_two)] }
      infix { true }
    end

    factory :fn_call_foo do
      name { :foo }
      arguments { [build(:variable_bar)] }
    end
  end
end
