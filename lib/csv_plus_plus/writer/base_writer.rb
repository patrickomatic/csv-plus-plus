# typed: true
# frozen_string_literal: true

module CSVPlusPlus
  module Writer
    # Some shared functionality that all Writers should build on
    #
    # @attr options [Options] The supplied options - some of which are relevant for our writer instance
    # @attr runtime [Runtime] The current runtime - needed to resolve variables and display useful error messages.
    class BaseWriter
      attr_accessor :options, :runtime

      protected

      # Open a CSV outputter to +filename+
      #
      # @param options [Options] The supplied options.
      # @param runtime [Runtime] The current runtime.
      def initialize(options, runtime)
        @options = options
        @runtime = runtime
      end
    end
  end
end
