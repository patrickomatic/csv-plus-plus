require 'cell_value_parser.tab'

describe GSPush::CellValueParser do
  describe "#parse" do 
    subject { GSPush::CellValueParser.new.parse(cell_value) }
    describe "without a formula" do
      let(:cell_value) { "just a value" }
      it { should be_nil }
    end

    describe "with an infix formula" do
      let(:cell_value) { "=5 * 5" }
      it { should eq([[:fn, "MULTIPLY"], [[:literal, "5"], [:literal, "5"]]]) }
    end

    describe "with a variable" do
      let(:cell_value) { "=$$foo" }
      it { should eq([:literal, "$$foo"]) }
    end

    describe "with a prefix formula" do
      let(:cell_value) { "=ADD(1, 2)" }
      it { should eq([[:fn, "ADD"], [[:literal, "1"], [:literal, "2"]]]) }
    end
  end
end
