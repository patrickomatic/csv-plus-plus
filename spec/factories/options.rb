# frozen_string_literal: true

require_relative '../../lib/options'

::FactoryBot.define do
  factory :options, class: ::CSVPlusPlus::Options do
    transient do
      create_if_not_exists { false }
    end

    after(:build) do |i, e|
      i.create_if_not_exists = e.create_if_not_exists
    end
  end
end
