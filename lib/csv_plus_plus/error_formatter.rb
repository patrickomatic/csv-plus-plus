# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  # Handle any errors potentially thrown during compilation.  This could be anything from a user error (for example
  # calling with invalid csvpp code) to an error calling Google Sheets API or writing to the filesystem.
  class ErrorFormatter
    extend ::T::Sig

    sig do
      params(options: ::CSVPlusPlus::Options, runtime: ::CSVPlusPlus::Runtime::Runtime).void
    end
    # @param runtime [Runtime::Runtime]
    # @param  [Runtime::Runtime]
    def initialize(options:, runtime:)
      @options = options
      @runtime = runtime
    end

    sig { params(error: ::StandardError).void }
    # Nicely handle a given error.  How it's handled depends on if it's our error and if @options.verbose
    #
    # @param error [CSVPlusPlus::Error, Google::Apis::ClientError, StandardError]
    def handle_error(error)
      # make sure that we're on a newline (verbose mode will probably be in the middle of printing a benchmark)
      puts("\n\n") if @options.verbose

      case error
      when ::CSVPlusPlus::Error::Error
        handle_internal_error(error)
      when ::Google::Apis::ClientError
        handle_google_error(error)
      else
        unhandled_error(error)
      end
    end

    private

    sig { params(error: ::StandardError).void }
    # An error was thrown that we weren't planning on
    def unhandled_error(error)
      warn(
        <<~ERROR_MESSAGE)
          An unexpected error was encountered.  Please try running again with --verbose and
          reporting the error at: https://github.com/patrickomatic/csv-plus-plus/issues/new'
        ERROR_MESSAGE

      return unless @options.verbose

      warn(error.full_message)
      warn("Cause: #{error.cause}") if error.cause
    end

    sig { params(error: ::CSVPlusPlus::Error::Error).void }
    def handle_internal_error(error)
      warn(with_position(error))
      handle_wrapped_error(::T.must(error.wrapped_error)) if error.wrapped_error
    end

    sig { params(wrapped_error: ::StandardError).void }
    def handle_wrapped_error(wrapped_error)
      return unless @options.verbose

      warn(wrapped_error.full_message)
      warn((wrapped_error.backtrace || []).join("\n")) if wrapped_error.backtrace
    end

    sig { params(error: ::Google::Apis::ClientError).void }
    def handle_google_error(error)
      warn("Error making Google Sheets API request: #{error.message}")
      return unless @options.verbose

      warn("#{error.status_code} Error making Google API request [#{error.message}]: #{error.body}")
    end

    sig { params(error: ::CSVPlusPlus::Error::Error).returns(::String) }
    # Output a user-helpful string that references the runtime state
    #
    # @params error_message [String] The error message to be prefixed with a filename and position
    #
    # @return [String]
    def with_position(error)
      message = error.error_message
      case error
      when ::CSVPlusPlus::Error::PositionalError
        "#{message_prefix}#{cell_index} #{message}"
      else
        message
      end
    end

    sig { returns(::String) }
    def cell_index
      if @runtime.parsing_csv_section?
        "[#{@runtime.position.row_index},#{@runtime.position.cell_index}]"
      else
        ''
      end
    end

    sig { returns(::String) }
    def message_prefix
      line_number = @runtime.position.line_number
      filename = @runtime.source_code.filename

      line_str = ":#{line_number}"
      "#{filename}#{line_str}"
    end
  end
end
