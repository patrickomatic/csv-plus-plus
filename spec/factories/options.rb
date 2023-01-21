# frozen_string_literal: true

require_relative '../../lib/options'

::FactoryBot.define do
  factory :options, class: ::CSVPlusPlus::Options do
    transient do
      create_if_not_exists { false }
      google_sheet_id { nil }
      output_filename { nil }
    end

    after(:build) do |i, e|
      i.create_if_not_exists = e.create_if_not_exists
      i.google_sheet_id = e.google_sheet_id if e.google_sheet_id
      i.output_filename = e.output_filename
    end
  end
end
