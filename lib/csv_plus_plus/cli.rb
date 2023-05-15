# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  # Handle running the application with the supported +CLIFlag+s
  #
  # @attr options [Options] The parsed CLI options
  class CLI
    extend ::T::Sig

    sig { returns(::CSVPlusPlus::Options::Options) }
    attr_accessor :options

    sig { returns(::CSVPlusPlus::SourceCode) }
    attr_accessor :source_code

    sig { void }
    # Handle CLI flags and launch the compiler
    #
    # @return [CLI]
    def self.launch_compiler!
      new.main
    rescue ::StandardError => e
      warn(e.message)
      exit(1)
    end

    sig { void }
    # Initialize and parse the CLI flags provided to the program
    def initialize
      opts = parse_options

      @source_code = ::T.let(::CSVPlusPlus::SourceCode.new(source_code_filename), ::CSVPlusPlus::SourceCode)
      @options = ::T.let(apply_options(opts), ::CSVPlusPlus::Options::Options)
    end

    sig { void }
    # Compile the given template using the given CLI flags
    def main
      ::CSVPlusPlus.cli_compile(source_code, options)
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
          parser.on(f.short_flag, f.long_flag, f.description)
        end
      end
    end

    sig { params(opts: ::T::Hash[::Symbol, ::String]).returns(::CSVPlusPlus::Options::Options) }
    def apply_options(opts)
      ::CSVPlusPlus::Options.from_cli_flags(opts, source_code.filename).tap do |options|
        opts.each do |key, value|
          ::T.must(::CSVPlusPlus::FLAG_HANDLERS[key]).call(options, value) if ::CSVPlusPlus::FLAG_HANDLERS.key?(key)
        end
      end
    end

    sig { returns(::T::Hash[::Symbol, ::String]) }
    def parse_options
      {}.tap do |opts|
        option_parser.parse!(into: opts)
      end
    rescue ::OptionParser::InvalidOption => e
      puts(option_parser)
      raise(::CSVPlusPlus::Error::CLIError, e.message)
    end

    sig { returns(::String) }
    # NOTE: this must be called after #parse_options, since #parse_options modifiers +ARGV+
    def source_code_filename
      ::ARGV.pop || raise(
        ::CSVPlusPlus::Error::CLIError,
        'You must specify a source (.csvpp) file to compile as the last argument'
      )
    end
  end
end
