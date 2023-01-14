# frozen_string_literal: true

require_relative '../../lib/language/compiler'

::FactoryBot.define do
  factory :compiler, class: ::CSVPlusPlus::Language::Compiler do
    transient do
      options { build(:options) }
      runtime { build(:runtime) }
      scope { build(:scope) }
    end

    initialize_with { new(runtime:, options:, scope:) }
  end
end
