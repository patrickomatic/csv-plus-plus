# frozen_string_literal: true

::FactoryBot.define do
  factory :template, class: ::CSVPlusPlus::Template do
    transient do
      rows { [] }
    end

    initialize_with { new(rows:) }
  end
end
