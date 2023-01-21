# frozen_string_literal: true

require_relative '../../lib/options'

::FactoryBot.define do
  factory :google_options, class: ::CSVPlusPlus::GoogleOptions do
    sheet_id { 'skcksk1lw1ocks01xkskcls10paxl1cpslskdk20alxw' }
  end
end
