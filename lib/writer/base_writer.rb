# frozen_string_literal: true

module CSVPlusPlus
  module Writer
    ##
    # Some shared functionality that all Writers should build on
    class BaseWriter
      attr_accessor :options

      # Open a CSV outputter to +filename+
      def initialize(options)
        @options = options
        load_requires
      end

      protected

      def load_requires; end
    end
  end
end
