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
      new.main
    rescue ::StandardError
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
      ::CSVPlusPlus.cli_compile(::ARGF.read, ::ARGF.filename, @options)
    end

    private

    sig { returns(::OptionParser) }
    def option_parser
      ::OptionParser.new do |parser|
        parser.on('-h', '--help', 'Show help information') do
          puts(parser)
          exit
        end

        ::CSVPlusPlus::SUPPORTED_CSVPP_FLAGS.each do |f|
          parser.on(f.short_flag, f.long_flag, f.description) { |v| f.handler.call(@options, v) }
        end
      end
    end

    sig { void }
    # Handle the supplied command line options, setting +@options+ or throw an error if anything is invalid
    def parse_options!
      option_parser.parse!
      validate_options
    rescue ::OptionParser::InvalidOption => e
      raise(::CSVPlusPlus::Error::CLIError, e.message)
    end

    sig { void }
    def validate_options
      error_message = @options.validate
      return if error_message.nil?

      puts(option_parser)
      raise(::CSVPlusPlus::Error::CLIError, error_message)
    end
  end
end
