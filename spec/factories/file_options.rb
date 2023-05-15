# typed: false
# frozen_string_literal: true

::FactoryBot.define do
  factory :file_options, class: ::CSVPlusPlus::Options::FileOptions do
    transient do
      create_if_not_exists { false }
      output_filename { 'text.xlsx' }
      sheet_name { 'Test' }
    end

    initialize_with { new(sheet_name, output_filename) }

    after(:build) do |i, e|
      i.create_if_not_exists = e.create_if_not_exists
      i.sheet_name = e.sheet_name
    end
  end
end
