# frozen_string_literal: true

require_relative '../../lib/language/entities'

ns = ::CSVPlusPlus::Language

::FactoryBot.define do
  factory :fn_bar, class: ns::Function do
    initialize_with do
      new(:bar, %i[], build(:fn_call, name: :indirect, arguments: [build(:string, s: 'bar')]))
    end
  end

  factory :fn_foo, class: ns::Function do
    initialize_with do
      new(:foo, %i[], build(:fn_call, name: :indirect, arguments: [build(:string, s: 'foo')]))
    end
  end

  factory :fn_add, class: ns::Function do
    initialize_with do
      new(
        :add,
        %i[a b],
        build(
          :fn_call,
          name: :add,
          arguments: [build(:variable, id: :a), build(:variable, id: :b)]
        )
      )
    end
  end
end
