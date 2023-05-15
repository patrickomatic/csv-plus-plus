# typed: false
# frozen_string_literal: true

::FactoryBot.define do
  factory :source_code, class: ::CSVPlusPlus::SourceCode do
    transient do
      input do
        <<~INPUT
          # this is a comment
          foo := 42
          bar := A1

          ---
          foo,bar,baz
          =$$foo,=$$bar,"baz"
        INPUT
      end
      filename { 'test.csvpp' }
    end

    initialize_with { new(filename, input:) }
  end
end
