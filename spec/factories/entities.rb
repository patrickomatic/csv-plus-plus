require_relative '../../lib/language/entities'

ns = CSVPlusPlus::Language

FactoryBot.define do
  factory :fn_call_add, class: ns::FunctionCall do
    initialize_with { new(:add, [build(:number_one), build(:number_two)]) }
  end

  factory :fn_call_foo, class: ns::FunctionCall do
    initialize_with { new("foo", [build(:variable_bar)]) }
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

  factory :true, class: ns::Boolean do
    initialize_with { new true }
  end

  factory :false, class: ns::Boolean do
    initialize_with { new false }
  end

  factory :number, class: ns::Number do
    transient do
      n { 0 }
    end
    initialize_with { new n }
  end

  factory :number_one, class: ns::Number do
    initialize_with { new 1 }
  end

  factory :number_two, class: ns::Number do
    initialize_with { new 2 }
  end

  factory :string, class: ns::String do
    transient do
      s { '' }
    end
    initialize_with { new s }
  end

  factory :string_foo, class: ns::String do
    initialize_with { new "foo" }
  end

  factory :string_bar, class: ns::String do
    initialize_with { new "bar" }
  end

  factory :variable, class: ns::Variable do
    transient do
      id { "foo" }
    end
    initialize_with { new id }
  end

  factory :variable_foo, class: ns::Variable do
    initialize_with { new "foo" }
  end

  factory :variable_bar, class: ns::Variable do
    initialize_with { new "bar" }
  end

  factory :cell_reference, class: ns::CellReference do
    transient do
      ref { "C1" }
    end
    initialize_with { new ref }
  end
end
