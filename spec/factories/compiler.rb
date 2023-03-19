# frozen_string_literal: true

::FactoryBot.define do
  factory :compiler, class: ::CSVPlusPlus::Compiler do
    transient do
      options { build(:options) }
      scope { build(:scope, runtime: runtime || build(:runtime)) }
      runtime { nil }
    end

    initialize_with { new(runtime: scope.runtime, options:, scope:) }
  end
end
