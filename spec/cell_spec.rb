require 'cell'
require 'modifier'

describe CSVPlusPlus::Cell do
  let(:modifier) { CSVPlusPlus::Modifier.new }
  let(:cell) { CSVPlusPlus::Cell.new(value, modifier) }

  describe "#value" do
    subject { cell.value }

    context "with leading and trailing spaces" do
      let(:value) { "   test   " }
      it { should eq("test") }
    end

    context "when it's all spaces" do
      let(:value) { "     " }
      it { should be_nil }
    end

    context "when it's nil" do
      let(:value) { nil }
      it { should be_nil }
    end
  end

  describe "#interpolate_variables!" do
    let(:variables) { { "rownum" => [:number, 1] } }
    before(:each) { cell.interpolate_variables!(variables) }

    subject { cell.ast }

    context "with variables to interpolate" do
      let(:value) { "=ADD(2, $$rownum)" }
      it { should eq([[:fn, "ADD"], [[:number, 2], [:number, 1]]]) }

      context "when the same value needs to be interpolated multiple times" do
        let(:value) { "=MINUS($$rownum, $$rownum)" }
        it { should eq([[:fn, "MINUS"], [[:number, 1], [:number, 1]]]) }
      end
    end
  end

  describe "#to_csv" do
    subject { cell.to_csv }

    context "with a function" do
      let(:value) { "=MULTIPLY(5, 5)" }

      it "goes through parsing and recreates the same value" do 
        expect(subject).to eq(value)
      end
    end

    context "with a variable" do
      let(:value) { "=$$foo" }
      before(:each) { cell.interpolate_variables!({ "foo" => [:number, 1] }) }

      it "goes through parsing and recreates the same value" do 
        expect(subject).to eq("=1")
      end
    end
  end
end
