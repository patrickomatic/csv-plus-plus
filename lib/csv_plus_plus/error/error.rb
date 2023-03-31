# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Error
    # An error thrown by our code (generally to be handled at the top level bin/ command)
    class Error < StandardError
      # TODO: perhaps give this a better name? something more descriptive than just Error
    end
  end
end
