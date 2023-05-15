# typed: strict
# frozen_string_literal: true

module Helpers
  module GoogleSheets
    extend ::T::Sig

    # TODO: this should probably be more specific than allowing any paths
    PathMatcher =
      ::T.let(
        ->(_r1, _r2) { true },
        ::T.proc.params(_r1: ::T.untyped, _r2: ::T.untyped).returns(::T::Boolean)
      )
    public_constant :PathMatcher

    sig { returns(::String) }
    # This function allows us to use a specific ID for testing against a live Google Sheets but also it can
    # return a fake value if we don't need that (which is most of the time).
    #
    # @return [String]
    def self.test_google_sheet_id
      ::ENV.fetch('GOOGLE_SHEET_ID', 'skcksk1lw1ocks01xkskcls10paxl1cpslskdk20alxw')
    end
  end
end
