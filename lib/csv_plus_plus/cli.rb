# frozen_string_literal: true

require 'optparse'

module CSVPlusPlus
  # Handle running the application with the given CLI flags
  module CLI
    # handle any CLI flags and launch the compiler
    def self.launch_compiler!
      options = parse_options
      ::CSVPlusPlus.apply_template_to_sheet!(::ARGF.read, ::ARGF.filename, options)
    rescue ::CSVPlusPlus::Error => e
      if e.is_a?(::CSVPlusPlus::Language::SyntaxError)
        warn(options.verbose ? e.to_verbose_trace : e.to_trace)
      else
        warn(e.message)
      end
      exit(1)
    end

    private

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
