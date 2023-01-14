# frozen_string_literal: true

require_relative '../../lib/language/entities'

ns = ::CSVPlusPlus::Language

::FactoryBot.define do
  factory :string, class: ns::String do
    transient do
      s { '' }
    end
    initialize_with { new s }
  end

  factory :string_foo, class: ns::String do
    initialize_with { new 'foo' }
  end

  factory :string_bar, class: ns::String do
    initialize_with { new 'bar' }
  end
end
