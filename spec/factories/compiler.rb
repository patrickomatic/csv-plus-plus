# frozen_string_literal: true

::FactoryBot.define do
  factory :compiler, class: ::CSVPlusPlus::Compiler do
    transient do
      options { build(:options) }
      runtime { build(:runtime) }
    end

    initialize_with { new(runtime:, options:) }
  end
end
