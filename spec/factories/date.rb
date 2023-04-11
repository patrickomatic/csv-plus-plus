# typed: false
# frozen_string_literal: true

::FactoryBot.define do
  factory :date, class: ::CSVPlusPlus::Entities::Date do
    transient do
      value { '4/25/2023' }
    end

    initialize_with { new value }
  end
end
