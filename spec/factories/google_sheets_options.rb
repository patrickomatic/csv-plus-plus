# typed: false
# frozen_string_literal: true

::FactoryBot.define do
  factory :google_sheets_options, class: ::CSVPlusPlus::Options::GoogleSheetsOptions do
    transient do
      create_if_not_exists { false }
      sheet_id { ::Helpers::GoogleSheets.test_google_sheet_id }
      sheet_name { 'Test' }
    end

    initialize_with { new(sheet_name, sheet_id) }

    after(:build) do |i, e|
      i.create_if_not_exists = e.create_if_not_exists
      i.sheet_name = e.sheet_name
    end
  end
end
