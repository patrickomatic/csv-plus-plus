require_relative '../../lib/modifier'

FactoryBot.define do
  factory :modifier, class: CSVPlusPlus::Modifier do
  end

  factory :row_modifier, class: CSVPlusPlus::Modifier do
    after(:build) do |m|
      m.row_level = true
    end
  end

  factory :modifier_with_expand, class: CSVPlusPlus::Modifier do
    transient do 
      repetitions { nil }
    end

    after(:build) do |m, e|
      m.row_level = true
      m.expand = CSVPlusPlus::Modifier::Expand.new e.repetitions
    end
  end
end
