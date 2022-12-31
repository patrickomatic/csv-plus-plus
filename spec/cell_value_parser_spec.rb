require 'cell_value.tab'

describe CSVPlusPlus::CellValueParser do
  describe "#parse" do 
    subject { CSVPlusPlus::CellValueParser.new.parse(cell_value) }

    describe "without a formula" do
      let(:cell_value) { "just a value" }
      it { should be_nil }
    end

    describe "an infix formula" do
      let(:cell_value) { "=MULTIPLY(5, 5)" }
      it { should eq([[:fn, "MULTIPLY"], [[:number, 5], [:number, 5]]]) }
    end

    describe "a variable" do
      let(:cell_value) { "=$$foo" }
      it { should eq([:var, "foo"]) }
    end

    describe "a prefix formula" do
      let(:cell_value) { "=ADD(1, 2)" }
      it { should eq([[:fn, "ADD"], [[:number, 1], [:number, 2]]]) }
    end

    describe "a double quoted string" do
      let(:cell_value) { '="test"' }
      it { should eq([:string, "test"]) }
    end
  end
end
