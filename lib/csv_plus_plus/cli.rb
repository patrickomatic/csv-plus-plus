# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  # Handle running the application with the given CLI flags
  #
  # @attr options [Options, nil] The parsed CLI options
  class CLI
    extend ::T::Sig

    sig { returns(::CSVPlusPlus::Options) }
    attr_accessor :options

    sig { void }
    # Handle CLI flags and launch the compiler
    #
    # @return [CLI]
    def self.launch_compiler!
      cli = new
      cli.main
    rescue ::StandardError => e
      ::T.must(cli).handle_error(e)
      exit(1)
    end

    sig { void }
    # Initialize and parse the CLI flags provided to the program
    def initialize
      @options = ::T.let(::CSVPlusPlus::Options.new, ::CSVPlusPlus::Options)
      parse_options!
    end

    sig { void }
    # Compile the given template using the given CLI flags
    def main
      ::CSVPlusPlus.apply_template_to_sheet!(::ARGF.read, ::ARGF.filename, @options)
    end

    sig { params(error: ::StandardError).void }
    # Nicely handle a given error.  How it's handled depends on if it's our error and if @options.verbose
    #
    # @param error [CSVPlusPlus::Error, Google::Apis::ClientError, StandardError]
    def handle_error(error)
      # make sure that we're on a newline (verbose mode might be in the middle of printing a benchmark)
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

    sig { void }
    # Handle the supplied command line options, setting +@options+ or throw an error if anything is invalid
    def parse_options!
      option_parser.parse!
      validate_options
    rescue ::OptionParser::InvalidOption => e
      raise(::CSVPlusPlus::Error::Error, e.message)
    end

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
      case error
      when ::CSVPlusPlus::Error::SyntaxError
        warn(@options.verbose ? error.to_verbose_trace : error.to_trace)
      else
        warn(error.message)
      end
    end

    sig { params(error: ::Google::Apis::ClientError).void }
    def handle_google_error(error)
      warn("Error making Google Sheets API request: #{error.message}")
      return unless @options.verbose

      warn("#{error.status_code} Error making Google API request [#{error.message}]: #{error.body}")
    end

    sig { void }
    def validate_options
      error_message = @options.validate
      return if error_message.nil?

      puts(option_parser)
      raise(::CSVPlusPlus::Error::Error, error_message)
    end

    sig { returns(::OptionParser) }
    def option_parser
      ::OptionParser.new do |parser|
        parser.on('-h', '--help', 'Show help information') do
          puts(parser)
          exit
        end

        ::SUPPORTED_CSVPP_FLAGS.each do |f|
          parser.on(f.short_flag, f.long_flag, f.description) { |v| f.handler.call(@options, v) }
        end
      end
    end
  end
end
