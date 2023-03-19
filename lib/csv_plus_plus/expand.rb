# frozen_string_literal: true

module CSVPlusPlus
  Expand =
    ::Struct.new(:repetitions) do
      # Does this infinitely expand?
      #
      # @return [boolean]
      def infinite?
        repetitions.nil?
      end

      # @return [::String]
      def to_s
        "Expand #{repetitions || 'infinity'}"
      end
    end

  public_constant :Expand
end
