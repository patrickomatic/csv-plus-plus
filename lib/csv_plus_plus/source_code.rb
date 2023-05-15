# typed: strict
# frozen_string_literal: true

module CSVPlusPlus
  # Information about the unparsed source code
  class SourceCode
    extend ::T::Sig

    sig { returns(::String) }
    attr_reader :input

    sig { returns(::Pathname) }
    attr_reader :filename

    sig { returns(::Integer) }
    attr_reader :length_of_csv_section

    sig { returns(::Integer) }
    attr_reader :length_of_code_section

    sig { returns(::Integer) }
    attr_reader :length_of_file

    sig { params(filename: ::String, input: ::T.nilable(::String)).void }
    # @param filename [::String] The name of the file the source came from.
    def initialize(filename, input: nil)
      @filename = ::T.let(::Pathname.new(filename), ::Pathname)
      @input = ::T.let(input || read_file, ::String)

      lines = @input.split(/[\r\n]/)
      @length_of_file = ::T.let(lines.length, ::Integer)
      @length_of_code_section = ::T.let(count_code_section_lines(lines), ::Integer)
      @length_of_csv_section = ::T.let(@length_of_file - @length_of_code_section, ::Integer)
    end

    sig { params(line_number: ::Integer).returns(::T::Boolean) }
    # Does the given +line_number+ land in the code section of the file? (which includes the --- separator)
    #
    # @param line_number [Integer]
    #
    # @return [T::Boolean]
    def in_code_section?(line_number)
      line_number <= @length_of_code_section
    end

    sig { params(line_number: ::Integer).returns(::T::Boolean) }
    # Does the given +line_number+ land in the CSV section of the file?
    #
    # @param line_number [Integer]
    #
    # @return [T::Boolean]
    def in_csv_section?(line_number)
      line_number > @length_of_code_section
    end

    private

    sig { returns(::String) }
    def read_file
      raise(::CSVPlusPlus::Error::CLIError, "Source file #{@filename} does not exist") unless ::File.exist?(@filename)

      ::File.read(@filename)
    end

    sig { params(lines: ::T::Array[::String]).returns(::Integer) }
    def count_code_section_lines(lines)
      eoc = ::CSVPlusPlus::Lexer::END_OF_CODE_SECTION
      lines.include?(eoc) ? (lines.take_while { |l| l != eoc }).length + 1 : 0
    end
  end
end
