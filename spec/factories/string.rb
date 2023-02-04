# frozen_string_literal: true

::FactoryBot.define do
  factory :string, class: ::CSVPlusPlus::Language::Entities::String do
    transient do
      s { '' }
    end

    initialize_with { new s }

    factory :string_foo do
      s { 'foo' }
    end

    factory :string_bar do
      s { 'bar' }
    end
  end
end
