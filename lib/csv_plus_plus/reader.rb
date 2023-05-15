# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  # Classes which can read spreadsheets in our various formats.
  module Reader
  end
end

require_relative './reader/reader'

require_relative './reader/csv'
require_relative './reader/google_sheets'
require_relative './reader/rubyxl'
