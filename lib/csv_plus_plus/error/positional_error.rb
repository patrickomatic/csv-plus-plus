# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Error
    # Methods that can be included into a class to denote that it's result it dependent on the current
    # +Runtime::Position+
    module PositionalError
      extend ::T::Sig
      extend ::T::Helpers

      interface!
    end
  end
end
