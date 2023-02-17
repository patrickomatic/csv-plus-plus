# frozen_string_literal: true

module CSVPlusPlus
  module Writer
    # Some shared functionality that all Writers should build on
    class BaseWriter
      attr_accessor :options

      protected

      # Open a CSV outputter to +filename+
      def initialize(options)
        @options = options
        load_requires
      end

      def load_requires; end
    end
  end
end
