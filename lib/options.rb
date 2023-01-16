# frozen_string_literal: true

module CSVPlusPlus
  # The Google-specific options a user can supply
  GoogleOptions =
    ::Struct.new(:sheet_id) do
      # Return a string with a verbose description of what we're doing with the options
      def verbose_summary
        <<~SUMMARY
          ## Google Sheets Options

          > Sheet ID | #{sheet_id}
        SUMMARY
      end

      # to_s
      def to_s
        "GoogleOptions(sheet_id: #{sheet_id})"
      end
    end

  public_constant :GoogleOptions

  # The options a user can supply
  class Options
    attr_accessor :backup, :create_if_not_exists, :key_values, :offset, :output_filename, :sheet_name, :verbose
    attr_reader :google

    # initialize
    def initialize
      @offset = [0, 0]
      @create_if_not_exists = false
      @key_values = {}
      @verbose = false
    end

    # Set the Google Sheet ID
    def google_sheet_id=(sheet_id)
      @google = ::CSVPlusPlus::GoogleOptions.new(sheet_id)
    end

    # Returns an error string or nil if there are no validation problems
    def validate
      return if @google || @output_filename

      'You must supply either a Google Sheet ID or an output file'
    end

    # Return a string with a verbose description of what we're doing with the options
    def verbose_summary
      <<~SUMMARY
        ===================================================
        # Options

        > Input filename           | #{@filename}
        > Sheet name               | #{@sheet_name}
        > Create sheet if it does  | #{@create_if_not_exists}
          not exist?
        > Spreadsheet cell-offset  | #{@offset[1]}
        > Spreadsheet row-offset   | #{@offset[0]}
        > User-supplied key-values | #{@key_values}

        ## Output

        > Backup                   | #{@backup}
        > Output filename          | #{@output_filename}
        #{@google&.verbose_summary || ''}
        ===================================================
      SUMMARY
    end

    # to_s
    def to_s
      "Options(create_if_not_exists: #{@create_if_not_exists}, google: #{@google}, key_values: #{@key_values}, " \
        "offset: #{@offset}, sheet_name: #{@sheet_name}, verbose: #{@verbose})"
    end
  end
end
