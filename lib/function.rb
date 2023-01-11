# frozen_string_literal: true

module CSVPlusPlus
  Function =
    ::Struct.new(:name, :arguments, :body, :line_number) do
      # to_s
      def to_s
        "#{line_number}:=#{name}(#{arguments}) { #{body} }"
      end
    end

  public_constant :Function
end
