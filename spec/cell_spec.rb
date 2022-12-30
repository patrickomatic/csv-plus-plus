require 'cell'

describe CSVPlusPlus::Cell do
  let(:cell) { CSVPlusPlus::Cell.new(value) }

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
    let(:variables) { { "rownum" => [:literal, "1"] } }
    before(:each) { cell.interpolate_variables!(variables) }

    subject { cell.ast }

    context "with variables to interpolate" do
      let(:value) { "=2 + $$rownum" }
      it { should eq([[:fn, "ADD"], [[:literal, "2"], [:literal, "1"]]]) }

      context "when the same value needs to be interpolated multiple times" do
        let(:value) { "=$$rownum - $$rownum" }
        it { should eq([[:fn, "MINUS"], [[:literal, "1"], [:literal, "1"]]]) }
      end
    end
  end

  describe "#to_csv" do
    subject { cell.to_csv }

    context "with a variable" do
      let(:value) { "=MULTIPLY(5, 5)" }

      it "goes through parsing and recreates the same value" do 
        expect(subject).to eq(value)
      end
    end
  end
end
