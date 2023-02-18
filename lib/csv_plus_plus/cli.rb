# frozen_string_literal: true

require 'optparse'

module CSVPlusPlus
  # Handle running the application with the given CLI flags
  class CLI
    # handle any CLI flags and launch the compiler
    def self.launch_compiler!
      cli = new
      cli.compile!
    rescue ::StandardError => e
      cli.handle_error(e)
      exit(1)
    end

    # initialize
    def initialize
      parse_options!
    end

    # compile the given template, using the given CLI flags
    def compile!
      ::CSVPlusPlus.apply_template_to_sheet!(::ARGF.read, ::ARGF.filename, @options)
    end

    # (nicely) handle a given error.  how it's handled depends on if it's our error and if @options.verbose
    def handle_error(error)
      case error
      when ::CSVPlusPlus::Error
        handle_internal_error(error)
      when ::Google::Apis::ClientError
        handle_google_error(error)
      else
        # TODO: more if verbose?
        warn(error.message)
      end
    end

    private

    def handle_internal_error(error)
      if error.is_a?(::CSVPlusPlus::Language::SyntaxError)
        warn(@options.verbose ? error.to_verbose_trace : error.to_trace)
      else
        warn(error.message)
      end
    end

    def handle_google_error(error)
      warn("Error making Google Sheets API request: #{error.message}")
      return unless @options.verbose

      warn("#{error.status_code} Error making Google API request [#{error.message}]: #{error.body}")
    end

    def parse_options!
      @options = ::CSVPlusPlus::Options.new
      option_parser.parse!
      validate_options
    end

    def validate_options
      error_message = @options.validate
      return if error_message.nil?

      puts(option_parser)
      raise(::CSVPlusPlus::Error, error_message)
    end

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
