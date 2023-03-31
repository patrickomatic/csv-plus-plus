# typed: false
# frozen_string_literal: true

::FactoryBot.define do
  factory :options, class: ::CSVPlusPlus::Options do
    transient do
      create_if_not_exists { false }
      google_sheet_id { nil }
      output_filename { nil }
      sheet_name { 'Test' }
    end

    trait :with_google_sheet_id do
      google_sheet_id { ::ENV.fetch('GOOGLE_SHEET_ID', 'skcksk1lw1ocks01xkskcls10paxl1cpslskdk20alxw') }
    end

    after(:build) do |i, e|
      i.create_if_not_exists = e.create_if_not_exists
      i.google_sheet_id = e.google_sheet_id if e.google_sheet_id
      i.output_filename = e.output_filename
      i.sheet_name = e.sheet_name
    end
  end
end
