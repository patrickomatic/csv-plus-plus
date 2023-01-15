# frozen_string_literal: true

module CSVPlusPlus
  # The Google-specific options a user can supply
  GoogleOptions =
    ::Struct.new(:sheet_id) do
      # to_s
      def to_s
        "GoogleOptions(sheet_id: #{sheet_id})"
      end
    end

  public_constant :GoogleOptions

  # The options a user can supply
  class Options
    attr_accessor :backup, :create_if_not_exists, :key_values, :offset, :sheet_name, :verbose
    attr_reader :google

    # initialize
    def initialize
      @offset = [0, 0]
      @create_if_not_exists = false
      @key_values = {}
      @verbose = false
      @google = ::CSVPlusPlus::GoogleOptions.new
    end

    # to_s
    def to_s
      "Options(create_if_not_exists: #{@create_if_not_exists}, google: #{@google}, key_values: #{@key_values}, " \
        "offset: #{@offset}, verbose: #{@verbose})"
    end
  end
end
