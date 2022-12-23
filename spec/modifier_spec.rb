require 'modifier'

describe GSPush::Modifier do
  describe ".get_modifier_from_value" do
    let(:cell_value) { "foo,<[format=bold]>bar,baz" }
    let(:modifier) { Modifier.get_modifier_from_value(cell_value) }

    it "handles a simple format" do
      expect(modifier).to be_bold
    end
  end 
end
