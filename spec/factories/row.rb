# frozen_string_literal: true

::FactoryBot.define do
  factory :row, class: ::CSVPlusPlus::Row do
    transient do
      index { 0 }
      cells { [] }
      modifier { build(:modifier) }
    end

    initialize_with { new(index, cells, modifier) }
  end
end
