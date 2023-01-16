# frozen_string_literal: true

require_relative '../../lib/language/entities'

::FactoryBot.define do
  factory :number, class: ::CSVPlusPlus::Language::Number do
    transient do
      n { 0 }
    end

    initialize_with { new n }

    factory :number_zero do
      n { 0 }
    end

    factory :number_one do
      n { 1 }
    end

    factory :number_two do
      n { 2 }
    end
  end
end