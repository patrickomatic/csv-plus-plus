# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  module Options
    # The options a user can supply (via CLI flags)
    #
    # @attr backup [Boolean] Create a backup of the spreadsheet before writing
    # @attr create_if_not_exists [Boolean] Create the spreadsheet if it does not exist?
    # @attr key_values [Hash] Additional variables that can be supplied to the template
    # @attr offset [Array<Integer>] An [x, y] offset (array with two integers)
    # @attr sheet_name [String] The name of the spreadsheet to write to
    # @attr verbose [Boolean] Include extra verbose output?
    class Options
      extend ::T::Sig
      extend ::T::Helpers

      abstract!

      sig { returns(::T::Boolean) }
      attr_accessor :backup

      sig { returns(::T::Boolean) }
      attr_accessor :create_if_not_exists

      sig { returns(::T::Hash[::Symbol, ::CSVPlusPlus::Entities::Entity]) }
      attr_accessor :key_values

      sig { returns(::T::Array[::Integer]) }
      attr_accessor :offset

      sig { returns(::String) }
      attr_accessor :sheet_name

      sig { returns(::T::Boolean) }
      attr_accessor :verbose

      sig { params(sheet_name: ::String).void }
      # Initialize a defaul +Options+ object
      def initialize(sheet_name)
        @sheet_name = sheet_name
        @offset = ::T.let([0, 0], ::T::Array[::Integer])
        @create_if_not_exists = ::T.let(false, ::T::Boolean)
        @key_values = ::T.let({}, ::T::Hash[::Symbol, ::CSVPlusPlus::Entities::Entity])
        @verbose = ::T.let(false, ::T::Boolean)
        @backup = ::T.let(false, ::T::Boolean)
      end

      sig { abstract.returns(::CSVPlusPlus::Options::OutputFormat) }
      # Given the options, figure out which type of +OutputFormat+ we'll be writing to
      #
      # @return [Options::OutputFormat]
      def output_format; end

      sig { abstract.returns(::String) }
      # Return a string with a verbose description of what we're doing with the options
      #
      # @return [String]
      def verbose_summary; end

      protected

      sig { params(str: ::String).returns(::String) }
      # Return a string with a verbose description of what we're doing with the options
      #
      # @return [String]
      def shared_summary(str)
        <<~SUMMARY
          #{summary_divider}

          # csv++ Command Options

          > Sheet name                          | #{@sheet_name}
          > Create sheet if it does not exist?  | #{@create_if_not_exists}
          > Spreadsheet row-offset              | #{@offset[0]}
          > Spreadsheet cell-offset             | #{@offset[1]}
          > User-supplied key-values            | #{@key_values}
          > Verbose                             | #{@verbose}
          > Backup                              | #{@backup}

          ## Output Options

          #{str}

          #{summary_divider}
        SUMMARY
      end

      private

      sig { returns(::String) }
      def summary_divider
        '========================================================================='
      end
    end
  end
end
