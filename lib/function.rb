module GSPush
  Function = Struct.new(:name, :arguments, :body, :line_number) do
    def to_s
      "#{line_number}:=#{name}(#{arguments}) { #{body} }"
    end
  end
end


