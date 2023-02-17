# frozen_string_literal: true

require 'optparse'

module CSVPlusPlus
  # Hadle running the application with the given CLI flags
  module CLI
    # handle any CLI flags and launch the compiler
    def self.launch_compiler!
      options = parse_options
      ::CSVPlusPlus.apply_template_to_sheet!(::ARGF.read, ::ARGF.filename, options)
    rescue ::StandardError => e
      handle_error(e, options)
      exit(1)
    end

    private

    def self.handle_error(error, options)
      case error
      when ::CSVPlusPlus::Error
        handle_internal_error(error, options)
      when ::Google::Apis::ClientError
        handle_google_error(error, options)
      else
        # TODO: more if verbose?
        warn(error.message)
      end
    end
    private_class_method :handle_error

    def self.handle_internal_error(error, options)
      if error.is_a?(::CSVPlusPlus::Language::SyntaxError)
        warn(options.verbose ? error.to_verbose_trace : error.to_trace)
      else
        warn(error.message)
      end
    end
    private_class_method :handle_internal_error

    def self.handle_google_error(error, options)
      warn("Error making Google Sheets API request: #{error.message}")
      return unless options.verbose

      warn("#{error.status_code} Error making Google API request [#{error.message}]: #{error.body}")
    end
    private_class_method :handle_google_error

    def self.parse_options
      ::CSVPlusPlus::Options.new.tap do |options|
        option_parser(options).parse!
        validate_options(options)
      end
    end
    private_class_method :parse_options

    def self.validate_options(options)
      error_message = options.validate
      return if error_message.nil?

      warn(error_message)
      puts(option_parser)
      exit(1)
    end
    private_class_method :validate_options

    def self.option_parser(options)
      ::OptionParser.new do |parser|
        parser.on('-h', '--help', 'Show help information') do
          puts(parser)
          exit
        end

        ::SUPPORTED_CSVPP_FLAGS.each do |f|
          parser.on(f.short_flag, f.long_flag, f.description) { |v| f.handler.call(options, v) }
        end
      end
    end
    private_class_method :option_parser
  end
end
