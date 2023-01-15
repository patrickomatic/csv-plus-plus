# frozen_string_literal: true

require_relative '../../lib/language/entities'

::FactoryBot.define do
  factory :fn, class: ::CSVPlusPlus::Language::Function do
    transient do
      arguments { [] }
      body { nil }
      name { [] }
    end

    initialize_with { new(name, arguments, body) }

    factory :fn_bar do
      name { :bar }
      body { build(:fn_call, name: :indirect, arguments: [build(:string, s: 'bar')]) }
    end

    factory :fn_foo do
      name { :foo }
      body { build(:fn_call, name: :indirect, arguments: [build(:string, s: 'foo')]) }
    end

    factory :fn_add do
      name { :add }
      arguments { %i[a b] }
      body { build(:fn_call, name: :add, arguments: [build(:variable, id: :a), build(:variable, id: :b)]) }
    end
  end
end
